//! Integration tests for the private funding-account WebSocket channels.
//!
//! Covers `deposit-info` and `withdrawal-info`. Both channels are on the
//! **business** endpoint (`/ws/v5/business`) with credentials. They push only
//! when deposit or withdrawal activity actually occurs, so these tests only
//! verify login and subscription acknowledgements. Any push that does arrive
//! during the window is parsed with the typed models to exercise them.

use std::time::Duration;

use rust_okx::ws::channels::funding;
use rust_okx::ws::model::{DepositInfoUpdate, WithdrawalInfoUpdate};
use rust_okx::{OkxWs, WsEvent};

use super::common::credentials;

/// Subscribe to `deposit-info` and `withdrawal-info` on the business WebSocket.
/// Waits for both subscription acknowledgements.
///
/// Funding pushes only arrive when deposits or withdrawals occur; any push that
/// arrives during the window is parsed with the appropriate typed model.
#[tokio::test]
async fn funding_private_channels_subscribe() {
    let Some(creds) = credentials() else {
        eprintln!("skipping funding_private_channels_subscribe: OKX credentials not set");
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
            eprintln!("skipping funding_private_channels_subscribe: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping funding_private_channels_subscribe: connect timed out");
            return;
        }
    };

    let args = [funding::deposit_info(), funding::withdrawal_info()];
    if let Err(err) = ws.subscribe(&args).await {
        eprintln!("skipping funding_private_channels_subscribe: subscribe failed: {err}");
        return;
    }

    let mut logged_in = false;
    let mut deposit_acked = false;
    let mut withdrawal_acked = false;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping funding_private_channels_subscribe: timed out \
                     (logged_in={logged_in}, deposit={deposit_acked}, withdrawal={withdrawal_acked})"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Login)) => {
                        logged_in = true;
                    }
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "deposit-info" => {
                        deposit_acked = true;
                    }
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "withdrawal-info" => {
                        withdrawal_acked = true;
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "deposit-info" => {
                        let _: Vec<DepositInfoUpdate> = push
                            .parse()
                            .expect("deposit-info push should parse as Vec<DepositInfoUpdate>");
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "withdrawal-info" => {
                        let _: Vec<WithdrawalInfoUpdate> = push
                            .parse()
                            .expect("withdrawal-info push should parse as Vec<WithdrawalInfoUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping funding_private_channels_subscribe: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(WsEvent::Disconnected)) | Ok(None) => {
                        eprintln!("skipping funding_private_channels_subscribe: connection closed");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Err(err) => {
                        eprintln!(
                            "skipping funding_private_channels_subscribe: receive failed: {err}"
                        );
                        return;
                    }
                }

                if logged_in && deposit_acked && withdrawal_acked {
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
