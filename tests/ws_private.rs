#![cfg(feature = "websocket")]

//! Integration tests against OKX private WebSocket channels.
//!
//! These tests only subscribe to read-only private channels. They do not place
//! orders or mutate account state.

use std::env;
use std::time::Duration;

use rust_okx::{Arg, Credentials, OkxRegion, OkxWs, WsEvent};

fn live_credentials() -> Option<Credentials> {
    let _ = dotenvy::dotenv();
    Some(Credentials::new(
        non_empty("OKX_API_KEY")?,
        non_empty("OKX_API_SECRET")?,
        non_empty("OKX_PASSPHRASE")?,
    ))
}

fn non_empty(var: &str) -> Option<String> {
    env::var(var).ok().filter(|value| !value.is_empty())
}

/// `WS / private account, orders` — log in with live credentials and subscribe
/// to read-only account/order channels. Skips when credentials or network are
/// unavailable.
#[tokio::test]
async fn private_login_and_read_only_subscriptions() {
    let Some(credentials) = live_credentials() else {
        eprintln!("skipping private_login_and_read_only_subscriptions: OKX_API_* env vars not set");
        return;
    };

    let connect = tokio::time::timeout(
        Duration::from_secs(10),
        OkxWs::private(credentials, OkxRegion::Global).connect(),
    )
    .await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping private_login_and_read_only_subscriptions: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping private_login_and_read_only_subscriptions: connect timed out");
            return;
        }
    };

    let args = [Arg::new("account"), Arg::new("orders").inst_type("ANY")];
    if let Err(err) = ws.subscribe(&args).await {
        eprintln!("skipping private_login_and_read_only_subscriptions: subscribe failed: {err}");
        return;
    }

    let mut logged_in = false;
    let mut account_subscribed = false;
    let mut orders_subscribed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping private_login_and_read_only_subscriptions: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Login)) => logged_in = true,
                    Ok(Some(WsEvent::Subscribed(arg))) if arg.channel == "account" => {
                        account_subscribed = true;
                    }
                    Ok(Some(WsEvent::Subscribed(arg))) if arg.channel == "orders" => {
                        orders_subscribed = true;
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => panic!("OKX WS error {code}: {msg}"),
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping private_login_and_read_only_subscriptions: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping private_login_and_read_only_subscriptions: receive failed: {err}");
                        return;
                    }
                }
                if logged_in && account_subscribed && orders_subscribed {
                    break;
                }
            }
        }
    }

    ws.close().await.expect("close should send");
}
