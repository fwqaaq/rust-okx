use std::sync::Arc;

use anyhow::Result;
use rust_okx::api::account::{BalanceRequest, PositionsRequest};
use rust_okx::api::market::CandlesRequest;
use rust_okx::api::trade::OrderListRequest;
use rust_okx::model::InstType;
use rust_okx::ws::channels::{account, market};
use rust_okx::ws::model::{CandleUpdate, OrderBookUpdate, TickerUpdate, TradeUpdate};
use rust_okx::{Credentials, OkxClient, OkxWs, WsEvent};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::app::{AppMsg, LogLevel, OhlcBar, RestSnapshot, StreamKind, StreamState};

pub async fn fetch_rest_snapshot(client: &OkxClient, inst_id: &str, bar: &str) -> RestSnapshot {
    let balances = client
        .account()
        .get_balance(BalanceRequest::default())
        .await
        .unwrap_or_default();
    let config = client
        .account()
        .get_account_config()
        .await
        .unwrap_or_default();
    let perm = config
        .first()
        .map(|row| row.perm.clone())
        .unwrap_or_default();
    let positions = client
        .account()
        .get_positions(&PositionsRequest { inst_id: Some(inst_id), ..Default::default() })
        .await
        .unwrap_or_default();
    let orders = client
        .trade()
        .get_order_list(&OrderListRequest::new().inst_id(inst_id).limit(20))
        .await
        .unwrap_or_default();
    let candles = client
        .market()
        .get_candlesticks(&CandlesRequest { inst_id, bar: Some(bar), limit: Some(120) })
        .await
        .unwrap_or_default()
        .into_iter()
        .rev()
        .map(OhlcBar::from)
        .collect();

    RestSnapshot {
        balances,
        positions,
        orders,
        candles,
        perm,
    }
}

pub fn spawn_periodic_rest_refresh(
    client: Arc<OkxClient>,
    inst_id: String,
    bar: String,
    refresh_ms: u64,
    tx: mpsc::Sender<AppMsg>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(refresh_ms));
        loop {
            interval.tick().await;
            let _ = tx
                .send(AppMsg::Status(
                    StreamKind::Rest,
                    StreamState::Connecting,
                    "refreshing".to_owned(),
                ))
                .await;
            let snapshot = fetch_rest_snapshot(&client, &inst_id, &bar).await;
            if tx.send(AppMsg::RestSnapshot(snapshot)).await.is_err() {
                return;
            }
        }
    })
}

pub fn spawn_candle_ws(inst_id: String, bar: String, tx: mpsc::Sender<AppMsg>) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = candle_ws_task(inst_id, bar, tx.clone()).await {
            let _ = tx
                .send(AppMsg::Status(
                    StreamKind::Candles,
                    StreamState::Error,
                    error.to_string(),
                ))
                .await;
        }
    })
}

pub fn spawn_market_ws(inst_id: String, tx: mpsc::Sender<AppMsg>) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = market_ws_task(inst_id, tx.clone()).await {
            let _ = tx
                .send(AppMsg::Status(
                    StreamKind::Market,
                    StreamState::Error,
                    error.to_string(),
                ))
                .await;
        }
    })
}

pub fn spawn_watchlist_ws(instruments: Vec<String>, tx: mpsc::Sender<AppMsg>) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = watchlist_ws_task(instruments, tx.clone()).await {
            let _ = tx
                .send(AppMsg::Log(
                    LogLevel::Error,
                    format!("watchlist WS: {error}"),
                ))
                .await;
        }
    })
}

pub fn spawn_private_ws(
    credentials: Credentials,
    demo: bool,
    tx: mpsc::Sender<AppMsg>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = private_ws_task(credentials, demo, tx.clone()).await {
            let _ = tx
                .send(AppMsg::Status(
                    StreamKind::Private,
                    StreamState::Error,
                    error.to_string(),
                ))
                .await;
        }
    })
}

async fn candle_ws_task(inst_id: String, bar: String, tx: mpsc::Sender<AppMsg>) -> Result<()> {
    tx.send(AppMsg::Status(
        StreamKind::Candles,
        StreamState::Connecting,
        format!("{inst_id} candle{bar}"),
    ))
    .await
    .ok();

    let arg = market::candlesticks(format!("candle{bar}"), &inst_id);
    let mut ws = OkxWs::business().connect().await?;
    ws.subscribe(std::slice::from_ref(&arg)).await?;

    loop {
        match ws.next_event().await? {
            Some(WsEvent::Subscribed(_)) => {
                tx.send(AppMsg::Status(
                    StreamKind::Candles,
                    StreamState::Subscribed,
                    format!("candle{bar} subscribed"),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Reconnected) => {
                tx.send(AppMsg::Status(
                    StreamKind::Candles,
                    StreamState::Reconnecting,
                    "reconnected".to_owned(),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Push(push)) if push.arg.channel.starts_with("candle") => {
                let rows: Vec<CandleUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::Candle(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Error { code, msg }) => {
                tx.send(AppMsg::Log(
                    LogLevel::Error,
                    format!("candle WS {code}: {msg}"),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Disconnected) | None => return Ok(()),
            _ => {}
        }
    }
}

async fn market_ws_task(inst_id: String, tx: mpsc::Sender<AppMsg>) -> Result<()> {
    tx.send(AppMsg::Status(
        StreamKind::Market,
        StreamState::Connecting,
        inst_id.clone(),
    ))
    .await
    .ok();

    let args = [
        market::tickers(&inst_id),
        market::trades(&inst_id),
        market::order_book("books5", &inst_id),
    ];
    let mut ws = OkxWs::public().connect().await?;
    ws.subscribe(&args).await?;

    loop {
        match ws.next_event().await? {
            Some(WsEvent::Subscribed(arg)) => {
                tx.send(AppMsg::Status(
                    StreamKind::Market,
                    StreamState::Subscribed,
                    format!("{} subscribed", arg.channel),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Reconnected) => {
                tx.send(AppMsg::Status(
                    StreamKind::Market,
                    StreamState::Reconnecting,
                    "reconnected".to_owned(),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Push(push)) if push.arg.channel == "tickers" => {
                let rows: Vec<TickerUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::Ticker(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Push(push)) if push.arg.channel == "trades" => {
                let rows: Vec<TradeUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::Trade(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Push(push)) if push.arg.channel == "books5" => {
                let rows: Vec<OrderBookUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::Book(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Error { code, msg }) => {
                tx.send(AppMsg::Log(
                    LogLevel::Error,
                    format!("market WS {code}: {msg}"),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Disconnected) | None => return Ok(()),
            _ => {}
        }
    }
}

async fn watchlist_ws_task(instruments: Vec<String>, tx: mpsc::Sender<AppMsg>) -> Result<()> {
    let args: Vec<_> = instruments.iter().map(|id| market::tickers(id)).collect();
    let mut ws = OkxWs::public().connect().await?;
    ws.subscribe(&args).await?;

    loop {
        match ws.next_event().await? {
            Some(WsEvent::Push(push)) if push.arg.channel == "tickers" => {
                let rows: Vec<TickerUpdate> = push.parse()?;
                for row in rows {
                    if tx.send(AppMsg::WatchlistTicker(row)).await.is_err() {
                        return Ok(());
                    }
                }
            }
            Some(WsEvent::Disconnected) | None => return Ok(()),
            _ => {}
        }
    }
}

async fn private_ws_task(
    credentials: Credentials,
    demo: bool,
    tx: mpsc::Sender<AppMsg>,
) -> Result<()> {
    tx.send(AppMsg::Status(
        StreamKind::Private,
        StreamState::Connecting,
        "login".to_owned(),
    ))
    .await
    .ok();

    let args = [
        account::account_by_currency("USDT", Some("5000")),
        account::positions(InstType::Swap.as_str(), Some("5000")),
    ];
    let mut ws = OkxWs::private(credentials)
        .demo_trading(demo)
        .connect()
        .await?;
    ws.subscribe(&args).await?;

    loop {
        match ws.next_event().await? {
            Some(WsEvent::Login) => {
                tx.send(AppMsg::Status(
                    StreamKind::Private,
                    StreamState::Connecting,
                    "logged in".to_owned(),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Subscribed(arg)) => {
                tx.send(AppMsg::Status(
                    StreamKind::Private,
                    StreamState::Subscribed,
                    format!("{} subscribed", arg.channel),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Reconnected) => {
                tx.send(AppMsg::Status(
                    StreamKind::Private,
                    StreamState::Reconnecting,
                    "reconnected".to_owned(),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Push(push)) => {
                tx.send(AppMsg::Log(
                    LogLevel::Info,
                    format!("private push: {}", push.arg.channel),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Error { code, msg }) => {
                tx.send(AppMsg::Log(
                    LogLevel::Error,
                    format!("private WS {code}: {msg}"),
                ))
                .await
                .ok();
            }
            Some(WsEvent::Disconnected) | None => return Ok(()),
            _ => {}
        }
    }
}
