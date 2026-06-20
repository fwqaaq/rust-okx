//! Integration tests for the private block-trading WebSocket channels.
//!
//! Covers `rfqs`, `quotes`, and `struc-block-trades`. These channels are on the
//! **business** endpoint (`/ws/v5/business`) with credentials, not the standard
//! private endpoint. Block-trading events are sparse (they require active
//! block-trading counterparties), so these tests only verify login and
//! subscription acknowledgements. Any push that does arrive during the window
//! is parsed with the typed models to exercise them.

use std::time::Duration;

use rust_okx::ws::channels::block;
use rust_okx::ws::model::{BlockQuoteUpdate, BlockRfqUpdate, StructureBlockTradeUpdate};
use rust_okx::{OkxWs, WsEvent};

use super::common::credentials;

/// Subscribe to `rfqs`, `quotes`, and `struc-block-trades` on the **business**
/// WebSocket endpoint with credentials. Waits for all three subscription
/// acknowledgements.
///
/// Block-trading pushes are sparse; any push that arrives is still parsed with
/// the appropriate typed model to exercise the deserialization path.
#[tokio::test]
async fn block_private_channels_subscribe() {
    let Some(creds) = credentials() else {
        eprintln!("skipping block_private_channels_subscribe: OKX credentials not set");
        return;
    };

    let connect = tokio::time::timeout(
        Duration::from_secs(10),
        OkxWs::business().credentials(creds).connect(),
    )
    .await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping block_private_channels_subscribe: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping block_private_channels_subscribe: connect timed out");
            return;
        }
    };

    let args = [
        block::rfqs(),
        block::quotes(),
        block::structure_block_trades(),
    ];
    if let Err(err) = ws.subscribe(&args).await {
        eprintln!("skipping block_private_channels_subscribe: subscribe failed: {err}");
        return;
    }

    let mut logged_in = false;
    let mut rfqs_acked = false;
    let mut quotes_acked = false;
    let mut struc_acked = false;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping block_private_channels_subscribe: timed out \
                     (logged_in={logged_in}, rfqs={rfqs_acked}, quotes={quotes_acked}, struc={struc_acked})"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Login)) => {
                        logged_in = true;
                    }
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "rfqs" => {
                        rfqs_acked = true;
                    }
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "quotes" => {
                        quotes_acked = true;
                    }
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "struc-block-trades" => {
                        struc_acked = true;
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "rfqs" => {
                        let _: Vec<BlockRfqUpdate> = push
                            .parse()
                            .expect("rfqs push should parse as Vec<BlockRfqUpdate>");
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "quotes" => {
                        let _: Vec<BlockQuoteUpdate> = push
                            .parse()
                            .expect("quotes push should parse as Vec<BlockQuoteUpdate>");
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "struc-block-trades" => {
                        let _: Vec<StructureBlockTradeUpdate> = push
                            .parse()
                            .expect("struc-block-trades push should parse as Vec<StructureBlockTradeUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping block_private_channels_subscribe: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(WsEvent::Disconnected)) | Ok(None) => {
                        eprintln!("skipping block_private_channels_subscribe: connection closed");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Err(err) => {
                        eprintln!("skipping block_private_channels_subscribe: receive failed: {err}");
                        return;
                    }
                }

                if logged_in && rfqs_acked && quotes_acked && struc_acked {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&args)
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}
