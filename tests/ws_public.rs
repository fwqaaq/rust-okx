#![cfg(feature = "websocket")]

//! Integration tests against OKX public WebSocket channels.

use std::time::Duration;

use rust_okx::api::market::Candle;
use rust_okx::api::market::Ticker;
use rust_okx::ws::channels;
use rust_okx::{Arg, OkxWs, WsEvent};

/// `WS / public tickers` — subscribe to `tickers` for `BTC-USDT`, wait for an
/// acknowledgement and at least one ticker push, then unsubscribe and close.
#[tokio::test]
async fn public_tickers_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_tickers_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_tickers_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = Arg::new("tickers").inst_id("BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_tickers_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_tickers_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) => {
                        if ack.channel == "tickers" && ack.inst_id.as_deref() == Some("BTC-USDT") {
                            subscribed = true;
                        }
                    }
                    Ok(Some(WsEvent::Push(push))) => {
                        if push.arg.channel == "tickers" {
                            let rows: Vec<Ticker> = push.parse().expect("ticker push should parse");
                            if rows.iter().any(|row| row.inst_id == "BTC-USDT") {
                                pushed = true;
                            }
                        }
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => panic!("OKX WS error {code}: {msg}"),
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_tickers_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_tickers_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / status` — subscribe to the public status channel and wait for an
/// acknowledgement. Status pushes are sparse, so this only requires the ack.
#[tokio::test]
async fn public_status_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_status_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_status_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::status::status();
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_status_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_status_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "status" => break,
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!("skipping public_status_channel_subscribes: OKX WS error {code}: {msg}");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_status_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_status_channel_subscribes: receive failed: {err}");
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / business candle1m` — subscribe to the business candlestick channel,
/// wait for an acknowledgement and one candle push, then unsubscribe and close.
#[tokio::test]
async fn business_candle_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::business().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping business_candle_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping business_candle_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = Arg::new("candle1m").inst_id("BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping business_candle_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping business_candle_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) => {
                        if ack.channel == "candle1m" && ack.inst_id.as_deref() == Some("BTC-USDT") {
                            subscribed = true;
                        }
                    }
                    Ok(Some(WsEvent::Push(push))) => {
                        if push.arg.channel == "candle1m" {
                            let rows: Vec<Candle> = push.parse().expect("candle push should parse");
                            if !rows.is_empty() {
                                pushed = true;
                            }
                        }
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => panic!("OKX WS error {code}: {msg}"),
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping business_candle_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping business_candle_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}
