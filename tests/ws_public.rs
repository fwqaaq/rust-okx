#![cfg(feature = "websocket")]

//! Integration tests against OKX public WebSocket channels.

use std::time::Duration;

use rust_okx::api::market::Ticker;
use rust_okx::{Arg, OkxRegion, OkxWs, WsEvent};

/// `WS / public tickers` — subscribe to `tickers` for `BTC-USDT`, wait for an
/// acknowledgement and at least one ticker push, then unsubscribe and close.
#[tokio::test]
async fn public_tickers_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(
        Duration::from_secs(10),
        OkxWs::public(OkxRegion::Global).connect(),
    )
    .await;
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
