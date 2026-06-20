use std::collections::VecDeque;
use std::io::stdout;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures_util::StreamExt;
use ratatui::prelude::*;
use rust_okx::api::account::BalanceRequest;
use rust_okx::api::market::CandlesRequest;
use tokio::sync::mpsc;

mod app;
mod credentials;
mod ui;
mod views;

use app::{App, AppMsg, OhlcBar, Tab};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // --- Blocking startup: credential + instrument prompts ---
    let (creds, inst_id, bar) = tokio::task::block_in_place(startup_prompts)?;

    println!("正在获取账户数据...");

    // --- REST: initial data ---
    let rest = rust_okx::OkxClient::builder()
        .credentials(creds.clone())
        .build();

    let balances = rest
        .account()
        .get_balance(BalanceRequest::default())
        .await
        .unwrap_or_default();

    let config = rest
        .account()
        .get_account_config()
        .await
        .unwrap_or_default();
    let perm = config.first().map(|c| c.perm.clone()).unwrap_or_default();

    let raw_candles = rest
        .market()
        .get_candlesticks(&CandlesRequest {
            inst_id: &inst_id,
            bar: Some(&bar),
            limit: Some(100),
        })
        .await
        .unwrap_or_default();

    // OKX returns newest first; reverse so oldest is at front (left of chart)
    let candles: VecDeque<OhlcBar> = raw_candles.into_iter().rev().map(OhlcBar::from).collect();

    let app = App {
        tab: Tab::Account,
        balances,
        perm,
        inst_id: inst_id.clone(),
        bar: bar.clone(),
        candles,
        tickers: VecDeque::new(),
    };

    // --- Background WS tasks ---
    let (tx, mut rx) = mpsc::channel::<AppMsg>(64);

    let tx_c = tx.clone();
    let inst_c = inst_id.clone();
    let bar_c = bar.clone();
    tokio::spawn(async move {
        if let Err(e) = candle_ws_task(inst_c, bar_c, tx_c).await {
            eprintln!("Candle WS error: {e}");
        }
    });

    let tx_t = tx.clone();
    let inst_t = inst_id.clone();
    tokio::spawn(async move {
        if let Err(e) = ticker_ws_task(inst_t, tx_t).await {
            eprintln!("Ticker WS error: {e}");
        }
    });

    // --- Enter TUI ---
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let result = run_tui(&mut terminal, app, &mut rx).await;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    result
}

fn startup_prompts() -> Result<(rust_okx::Credentials, String, String)> {
    use credentials::{load_or_prompt, prompt_line};

    let creds = load_or_prompt()?;
    let inst_raw = prompt_line("交易对 (默认 BTC-USDT): ");
    let inst_id = if inst_raw.is_empty() {
        "BTC-USDT".to_owned()
    } else {
        inst_raw
    };
    let bar_raw = prompt_line("K 线周期 (1m/5m/15m/1H/4H/1D, 默认 1m): ");
    let bar = if bar_raw.is_empty() {
        "1m".to_owned()
    } else {
        bar_raw
    };
    Ok((creds, inst_id, bar))
}

async fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    mut app: App,
    rx: &mut mpsc::Receiver<AppMsg>,
) -> Result<()> {
    let mut event_stream = EventStream::new();
    let mut tick = tokio::time::interval(Duration::from_millis(200));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                terminal.draw(|f| ui::render(f, &app))?;
            }
            Some(Ok(Event::Key(key))) = event_stream.next() => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Tab | KeyCode::Right => app.next_tab(),
                        KeyCode::Left | KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Char('1') => app.tab = Tab::Account,
                        KeyCode::Char('2') => app.tab = Tab::Chart,
                        KeyCode::Char('3') => app.tab = Tab::Feed,
                        _ => {}
                    }
                }
            }
            Some(msg) = rx.recv() => {
                match msg {
                    AppMsg::Candle(cu) => app.push_candle(cu),
                    AppMsg::Ticker(tu) => app.push_ticker(tu),
                }
            }
        }
    }
}

async fn candle_ws_task(inst_id: String, bar: String, tx: mpsc::Sender<AppMsg>) -> Result<()> {
    use rust_okx::ws::channels::market;
    use rust_okx::ws::model::CandleUpdate;
    use rust_okx::{OkxWs, WsEvent};

    let arg = market::candlesticks(&bar, &inst_id);
    let mut ws = OkxWs::business().connect().await?;
    ws.subscribe(std::slice::from_ref(&arg)).await?;

    loop {
        match ws.next_event().await? {
            Some(WsEvent::Push(push)) if push.arg.channel.starts_with("candle") => {
                let rows: Vec<CandleUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::Candle(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Disconnected) | None => break,
            _ => {}
        }
    }
    Ok(())
}

async fn ticker_ws_task(inst_id: String, tx: mpsc::Sender<AppMsg>) -> Result<()> {
    use rust_okx::ws::channels::market;
    use rust_okx::ws::model::TickerUpdate;
    use rust_okx::{OkxWs, WsEvent};

    let arg = market::tickers(&inst_id);
    let mut ws = OkxWs::public().connect().await?;
    ws.subscribe(std::slice::from_ref(&arg)).await?;

    loop {
        match ws.next_event().await? {
            Some(WsEvent::Push(push)) if push.arg.channel == "tickers" => {
                let rows: Vec<TickerUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::Ticker(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Disconnected) | None => break,
            _ => {}
        }
    }
    Ok(())
}
