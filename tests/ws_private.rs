#![cfg(feature = "websocket")]

//! Integration tests against OKX private WebSocket channels.
//!
//! These tests only subscribe to read-only private channels. They do not place
//! orders or mutate account state. Push payloads are parsed with the typed
//! models from [`rust_okx::ws::model`].

use std::env;
use std::time::Duration;

use rust_okx::ws::channels;
use rust_okx::ws::model::{AccountUpdate, OrderUpdate, SpreadOrderUpdate};
use rust_okx::{Credentials, OkxWs, WsEvent};

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
/// to read-only account/order channels.
///
/// The account channel normally sends an initial snapshot, so this test waits
/// for and parses one [`AccountUpdate`]. The orders channel only pushes when an
/// order changes; an order push is parsed as [`OrderUpdate`] when one happens,
/// but it is not required for the test to pass.
#[tokio::test]
async fn private_login_and_read_only_subscriptions() {
    let Some(credentials) = live_credentials() else {
        eprintln!("skipping private_login_and_read_only_subscriptions: OKX_API_* env vars not set");
        return;
    };

    let connect = tokio::time::timeout(
        Duration::from_secs(10),
        OkxWs::private(credentials).connect(),
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

    let args = [channels::trade::orders("ANY")];
    if let Err(err) = ws.subscribe(&args).await {
        eprintln!("skipping private_login_and_read_only_subscriptions: subscribe failed: {err}");
        return;
    }

    let mut logged_in = false;
    let mut account_subscribed = false;
    let mut orders_subscribed = false;
    let mut account_push_parsed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping private_login_and_read_only_subscriptions: timed out waiting for account snapshot");
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
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "account" => {
                        let rows: Vec<AccountUpdate> = push
                            .parse()
                            .expect("account push should parse as Vec<AccountUpdate>");

                        account_push_parsed = !rows.is_empty();
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "orders" => {
                        let _: Vec<OrderUpdate> = push
                            .parse()
                            .expect("orders push should parse as Vec<OrderUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
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

                if logged_in
                    && account_subscribed
                    && orders_subscribed
                    && account_push_parsed
                {
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

/// `WS / business private spread` — log in with live credentials on the
/// business endpoint and subscribe to the read-only spread order channel.
///
/// Spread-order pushes only occur when spread orders change. The test therefore
/// requires login and subscription acknowledgements, and parses any push that
/// happens during that process as [`SpreadOrderUpdate`].
#[tokio::test]
async fn business_private_spread_subscription_uses_env_login() {
    let Some(credentials) = live_credentials() else {
        eprintln!(
            "skipping business_private_spread_subscription_uses_env_login: OKX_API_* env vars not set"
        );
        return;
    };

    let connect = tokio::time::timeout(
        Duration::from_secs(10),
        OkxWs::business().credentials(credentials).connect(),
    )
    .await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!(
                "skipping business_private_spread_subscription_uses_env_login: connect failed: {err}"
            );
            return;
        }
        Err(_) => {
            eprintln!(
                "skipping business_private_spread_subscription_uses_env_login: connect timed out"
            );
            return;
        }
    };

    let arg = channels::spread::orders();
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!(
            "skipping business_private_spread_subscription_uses_env_login: subscribe failed: {err}"
        );
        return;
    }

    let mut logged_in = false;
    let mut subscribed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping business_private_spread_subscription_uses_env_login: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Login)) => logged_in = true,
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "sprd-orders" => {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "sprd-orders" => {
                        let _: Vec<SpreadOrderUpdate> = push
                            .parse()
                            .expect("sprd-orders push should parse as Vec<SpreadOrderUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!("skipping business_private_spread_subscription_uses_env_login: OKX WS error {code}: {msg}");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping business_private_spread_subscription_uses_env_login: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping business_private_spread_subscription_uses_env_login: receive failed: {err}");
                        return;
                    }
                }

                if logged_in && subscribed {
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
