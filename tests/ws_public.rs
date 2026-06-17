#![cfg(feature = "websocket")]

//! Integration tests against OKX public WebSocket channels.
//!
//! These tests intentionally parse channel payloads with the WebSocket models
//! from [`rust_okx::ws::model`] instead of reusing REST response types. This
//! keeps the examples aligned with OKX's WebSocket payload schemas.

use std::time::Duration;

use rust_okx::ws::channels;
use rust_okx::ws::model::{CandleUpdate, StatusUpdate, TickerUpdate};
use rust_okx::{OkxWs, WsEvent};

/// `WS / public tickers` — subscribe to `tickers` for `BTC-USDT`, wait for an
/// acknowledgement and at least one typed ticker push, then unsubscribe and
/// close.
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

    let arg = channels::market::tickers("BTC-USDT");
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
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "tickers"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "tickers"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<TickerUpdate> = push
                            .parse()
                            .expect("tickers push should parse as Vec<TickerUpdate>");

                        pushed = rows.iter().any(|row| row.inst_id == "BTC-USDT");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
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
/// acknowledgement.
///
/// Status pushes are sparse, so the test does not require one. When a status
/// push arrives before the acknowledgement, it is still parsed with
/// [`StatusUpdate`] to exercise the channel model.
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
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "status" => {
                        let _: Vec<StatusUpdate> = push
                            .parse()
                            .expect("status push should parse as Vec<StatusUpdate>");
                    }
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
/// wait for an acknowledgement and one typed candle push, then unsubscribe and
/// close.
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

    let arg = channels::market::candlesticks("candle1m", "BTC-USDT");
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
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "candle1m"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "candle1m"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<CandleUpdate> = push
                            .parse()
                            .expect("candle push should parse as Vec<CandleUpdate>");

                        pushed = !rows.is_empty();
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
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
