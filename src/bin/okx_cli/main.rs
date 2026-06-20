use std::io::stdout;
use std::sync::Arc;

use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures_util::StreamExt;
use ratatui::prelude::*;
use rust_okx::api::trade::{CancelOrderRequest, PlaceOrderRequest};
use rust_okx::{Credentials, OkxClient};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

mod app;
mod config;
mod credentials;
mod okx_config;
mod tasks;
mod ui;
mod views;

use app::{
    App, BAR_OPTIONS, DEFAULT_WATCHLIST, LogLevel, PendingAction, StreamKind, StreamState, Tab,
};
use clap::Parser;
use config::{CliArgs, RuntimeConfig, validate_bar};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let args = CliArgs::parse();
    let profile_name = args.profile.clone();
    let (creds, profile_demo, base_url) =
        tokio::task::block_in_place(|| credentials::load_or_prompt(profile_name.as_deref()))?;

    let mut config = RuntimeConfig::from_args(args)?;
    config.demo = config.demo || profile_demo;
    validate_bar(&config.bar)?;

    eprintln!(
        "Starting OKX TUI: {} {} {} trade_enabled={}",
        config.mode_label(),
        config.inst_id,
        config.bar,
        config.trade_enabled
    );

    let builder = OkxClient::builder()
        .credentials(creds.clone())
        .demo_trading(config.demo);
    let rest = Arc::new(
        if let Some(url) = base_url {
            builder.base_url(url)
        } else {
            builder.region(config.region)
        }
        .build(),
    );

    let mut app = App::new(config.clone());
    app.apply_rest_snapshot(tasks::fetch_rest_snapshot(&rest, &config.inst_id, &config.bar).await);

    let (tx, mut rx) = mpsc::channel(256);

    let watchlist_insts: Vec<String> = DEFAULT_WATCHLIST.iter().map(|s| s.to_string()).collect();
    let watchlist_handle = tasks::spawn_watchlist_ws(watchlist_insts, tx.clone());

    let mut handles = TaskHandles::spawn(rest.clone(), creds.clone(), &app, tx.clone());

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let result = run_tui(
        &mut terminal,
        &mut app,
        &mut rx,
        tx.clone(),
        rest,
        creds,
        &mut handles,
    )
    .await;

    handles.abort();
    watchlist_handle.abort();
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    result
}

struct TaskHandles {
    rest: JoinHandle<()>,
    market: JoinHandle<()>,
    candles: JoinHandle<()>,
    private: JoinHandle<()>,
}

impl TaskHandles {
    fn spawn(
        rest_client: Arc<OkxClient>,
        credentials: Credentials,
        app: &App,
        tx: mpsc::Sender<app::AppMsg>,
    ) -> Self {
        Self {
            rest: tasks::spawn_periodic_rest_refresh(
                rest_client,
                app.config.inst_id.clone(),
                app.config.bar.clone(),
                app.config.refresh_ms,
                tx.clone(),
            ),
            market: tasks::spawn_market_ws(app.config.inst_id.clone(), tx.clone()),
            candles: tasks::spawn_candle_ws(
                app.config.inst_id.clone(),
                app.config.bar.clone(),
                tx.clone(),
            ),
            private: tasks::spawn_private_ws(credentials, app.config.demo, tx),
        }
    }

    fn abort(&self) {
        self.rest.abort();
        self.market.abort();
        self.candles.abort();
        self.private.abort();
    }
}

async fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    rx: &mut mpsc::Receiver<app::AppMsg>,
    tx: mpsc::Sender<app::AppMsg>,
    rest: Arc<OkxClient>,
    creds: Credentials,
    handles: &mut TaskHandles,
) -> Result<()> {
    let mut event_stream = EventStream::new();
    let mut tick = tokio::time::interval(std::time::Duration::from_millis(150));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                terminal.draw(|f| ui::render(f, app))?;
            }
            Some(Ok(Event::Key(key))) = event_stream.next() => {
                if key.kind == KeyEventKind::Press
                    && handle_key(key.code, app, &rest, &tx, &creds, handles).await?
                {
                    return Ok(());
                }
            }
            Some(msg) = rx.recv() => {
                app.apply_msg(msg);
            }
        }
    }
}

async fn handle_key(
    key: KeyCode,
    app: &mut App,
    rest: &Arc<OkxClient>,
    tx: &mpsc::Sender<app::AppMsg>,
    creds: &Credentials,
    handles: &mut TaskHandles,
) -> Result<bool> {
    if handle_confirmation(key, app, rest).await? {
        return Ok(false);
    }
    if handle_bar_picker(key, app, rest, tx, creds, handles).await? {
        return Ok(false);
    }
    if handle_symbol_input(key, app, rest, tx, creds, handles).await? {
        return Ok(false);
    }

    match key {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        KeyCode::Tab | KeyCode::Right => app.next_tab(),
        KeyCode::Left | KeyCode::BackTab => app.prev_tab(),
        KeyCode::Char(c @ '1'..='7') if app.tab != Tab::Trade => {
            app.set_tab_by_number((c as u8 - b'0') as usize);
        }
        KeyCode::Char('r') => refresh_now(app, rest).await,
        KeyCode::Char('p') => {
            app.paused = !app.paused;
            app.log(
                LogLevel::Info,
                format!(
                    "stream display {}",
                    if app.paused { "paused" } else { "resumed" }
                ),
            );
        }
        KeyCode::Char('/') => {
            app.symbol_editing = true;
            app.symbol_input = app.config.inst_id.clone();
        }
        KeyCode::Char('b') => app.bar_picking = true,
        KeyCode::Down if app.tab == Tab::Orders => app.select_next_order(),
        KeyCode::Up if app.tab == Tab::Orders => app.select_prev_order(),
        KeyCode::Char('c') if app.tab == Tab::Orders => {
            app.build_cancel_confirmation();
        }
        KeyCode::Down if app.tab == Tab::Watchlist => app.select_next_watchlist(),
        KeyCode::Up if app.tab == Tab::Watchlist => app.select_prev_watchlist(),
        KeyCode::Enter if app.tab == Tab::Watchlist => {
            if let Some(inst) = app.active_watchlist_inst() {
                if inst != app.config.inst_id {
                    change_instrument(app, rest, tx, creds, handles, inst).await?;
                }
            }
        }
        _ if app.tab == Tab::Trade => handle_trade_key(key, app),
        _ => {}
    }

    Ok(false)
}

async fn handle_bar_picker(
    key: KeyCode,
    app: &mut App,
    rest: &Arc<OkxClient>,
    tx: &mpsc::Sender<app::AppMsg>,
    creds: &Credentials,
    handles: &mut TaskHandles,
) -> Result<bool> {
    if !app.bar_picking {
        return Ok(false);
    }
    match key {
        KeyCode::Char(c @ '1'..='9') => {
            let idx = (c as u8 - b'1') as usize;
            if let Some(&bar) = BAR_OPTIONS.get(idx) {
                app.bar_picking = false;
                change_bar(app, rest, tx, creds, handles, bar.to_owned()).await?;
            }
        }
        KeyCode::Esc => app.bar_picking = false,
        _ => {}
    }
    Ok(true)
}

async fn handle_confirmation(key: KeyCode, app: &mut App, rest: &Arc<OkxClient>) -> Result<bool> {
    let Some(confirmation) = app.confirmation.clone() else {
        return Ok(false);
    };

    match key {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.confirmation = None;
            match confirmation.action {
                PendingAction::PlaceOrder => submit_order(app, rest).await,
                PendingAction::CancelOrder { ord_id } => cancel_order(app, rest, ord_id).await,
            }
            Ok(true)
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.confirmation = None;
            app.trade.message = "操作已取消".to_owned();
            Ok(true)
        }
        _ => Ok(true),
    }
}

async fn handle_symbol_input(
    key: KeyCode,
    app: &mut App,
    rest: &Arc<OkxClient>,
    tx: &mpsc::Sender<app::AppMsg>,
    creds: &Credentials,
    handles: &mut TaskHandles,
) -> Result<bool> {
    if !app.symbol_editing {
        return Ok(false);
    }

    match key {
        KeyCode::Enter => {
            let next = app.symbol_input.trim().to_ascii_uppercase();
            app.symbol_editing = false;
            if !next.is_empty() && next != app.config.inst_id {
                change_instrument(app, rest, tx, creds, handles, next).await?;
            }
        }
        KeyCode::Esc => {
            app.symbol_editing = false;
            app.symbol_input = app.config.inst_id.clone();
        }
        KeyCode::Backspace => {
            app.symbol_input.pop();
        }
        KeyCode::Char(c) if c.is_ascii_alphanumeric() || c == '-' => {
            app.symbol_input.push(c.to_ascii_uppercase());
        }
        _ => {}
    }
    Ok(true)
}

fn handle_trade_key(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Enter => {
            app.build_place_confirmation();
        }
        KeyCode::Char('c') => {
            app.build_cancel_confirmation();
        }
        KeyCode::Char('s') => app.trade.side.toggle(),
        KeyCode::Char('o') => app.trade.order_type.toggle(),
        KeyCode::Char('m') => app.trade.trade_mode.cycle(),
        KeyCode::Down | KeyCode::Up => app.trade.next_field(),
        KeyCode::Backspace => app.trade.backspace(),
        KeyCode::Char(c) => app.trade.push_char(c),
        _ => {}
    }
}

async fn refresh_now(app: &mut App, rest: &Arc<OkxClient>) {
    app.set_status(
        StreamKind::Rest,
        StreamState::Connecting,
        "manual refresh".to_owned(),
    );
    let snapshot = tasks::fetch_rest_snapshot(rest, &app.config.inst_id, &app.config.bar).await;
    app.apply_rest_snapshot(snapshot);
    app.log(LogLevel::Info, "REST refreshed".to_owned());
}

async fn change_instrument(
    app: &mut App,
    rest: &Arc<OkxClient>,
    tx: &mpsc::Sender<app::AppMsg>,
    creds: &Credentials,
    handles: &mut TaskHandles,
    inst_id: String,
) -> Result<()> {
    app.log(LogLevel::Info, format!("switch instrument -> {inst_id}"));
    let snapshot = tasks::fetch_rest_snapshot(rest, &inst_id, &app.config.bar).await;
    app.set_market(inst_id, snapshot.candles.clone());
    app.apply_rest_snapshot(snapshot);
    handles.abort();
    *handles = TaskHandles::spawn(rest.clone(), creds.clone(), app, tx.clone());
    Ok(())
}

async fn change_bar(
    app: &mut App,
    rest: &Arc<OkxClient>,
    tx: &mpsc::Sender<app::AppMsg>,
    creds: &Credentials,
    handles: &mut TaskHandles,
    bar: String,
) -> Result<()> {
    let snapshot = tasks::fetch_rest_snapshot(rest, &app.config.inst_id, &bar).await;
    app.set_bar(bar.clone(), snapshot.candles.clone());
    app.apply_rest_snapshot(snapshot);
    app.log(LogLevel::Info, format!("switch bar -> {bar}"));
    handles.abort();
    *handles = TaskHandles::spawn(rest.clone(), creds.clone(), app, tx.clone());
    Ok(())
}

async fn submit_order(app: &mut App, rest: &Arc<OkxClient>) {
    let mut request = PlaceOrderRequest::new(
        app.config.inst_id.clone(),
        app.trade.trade_mode.as_trade_mode(),
        app.trade.side.as_order_side(),
        app.trade.order_type.as_order_type(),
        app.trade.size.clone(),
    );
    if app.trade.order_type == app::TradeTypeInput::Limit {
        request = request.price(app.trade.price.clone());
    }

    match rest.trade().place_order(&request).await {
        Ok(rows) => {
            let result = rows
                .first()
                .map(|row| format!("sCode={} ordId={} {}", row.s_code, row.ord_id, row.s_msg))
                .unwrap_or_else(|| "empty place-order response".to_owned());
            app.trade.message = result.clone();
            app.log(LogLevel::Info, format!("place order: {result}"));
            refresh_now(app, rest).await;
        }
        Err(error) => {
            app.trade.message = error.to_string();
            app.log(LogLevel::Error, format!("place order failed: {error}"));
        }
    }
}

async fn cancel_order(app: &mut App, rest: &Arc<OkxClient>, ord_id: String) {
    match rest
        .trade()
        .cancel_order(&CancelOrderRequest::by_order_id(
            &app.config.inst_id,
            &ord_id,
        ))
        .await
    {
        Ok(rows) => {
            let result = rows
                .first()
                .map(|row| format!("sCode={} ordId={} {}", row.s_code, row.ord_id, row.s_msg))
                .unwrap_or_else(|| "empty cancel-order response".to_owned());
            app.trade.message = result.clone();
            app.log(LogLevel::Info, format!("cancel order {ord_id}: {result}"));
            refresh_now(app, rest).await;
        }
        Err(error) => {
            app.trade.message = error.to_string();
            app.log(LogLevel::Error, format!("cancel order failed: {error}"));
        }
    }
}
