//! Integration test for the private `positions` WebSocket channel.
//!
//! Run with:
//! ```
//! OKX_DEMO=1 cargo test -F websocket --test ws_positions -- --ignored
//! ```
//!
//! Required env vars (or `.env` file): `OKX_API_KEY`, `OKX_API_SECRET`, `OKX_PASSPHRASE`.
//! `OKX_DEMO` defaults to `1` (demo trading) for safety.

#![cfg(feature = "websocket")]

use std::time::Duration;

use rust_okx::ws::channels::account;
use rust_okx::ws::model::PositionUpdate;
use rust_okx::{Credentials, OkxWs, WsEvent};

fn credentials() -> Option<Credentials> {
    let _ = dotenvy::dotenv();
    Some(Credentials::new(
        std::env::var("OKX_API_KEY").ok()?,
        std::env::var("OKX_API_SECRET").ok()?,
        std::env::var("OKX_PASSPHRASE").ok()?,
    ))
}

fn is_demo() -> bool {
    std::env::var("OKX_DEMO")
        .map(|v| v != "0" && v != "false")
        .unwrap_or(true)
}

/// Verifies the full private WebSocket flow for the `positions` channel:
/// connect → auto-login → subscribe → receive snapshot push → parse rows.
///
/// OKX sends a snapshot push even when there are no open positions, so this
/// test passes regardless of account state.
#[tokio::test]
#[ignore = "requires OKX_API_KEY / OKX_API_SECRET / OKX_PASSPHRASE env vars"]
async fn positions_login_subscribe_and_parse() {
    let Some(creds) = credentials() else {
        eprintln!("skipping: OKX credentials not set");
        return;
    };

    let demo = is_demo();
    eprintln!("connecting (demo={})", demo);

    let mut ws = OkxWs::private(creds)
        .demo_trading(demo)
        .connect()
        .await
        .expect("connect to OKX WebSocket");

    ws.subscribe(&[account::positions("ANY", None)])
        .await
        .expect("send subscribe");

    let deadline = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(deadline);

    let (mut logged_in, mut subscribed, mut got_push) = (false, false, false);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                panic!(
                    "timed out (logged_in={logged_in}, subscribed={subscribed}, got_push={got_push})"
                );
            }
            event = ws.next_event() => {
                match event.expect("recv event") {
                    Some(WsEvent::Login) => {
                        eprintln!("login ok");
                        logged_in = true;
                    }
                    Some(WsEvent::Subscribed(arg)) if arg.channel == "positions" => {
                        eprintln!("subscribed to positions");
                        subscribed = true;
                    }
                    Some(WsEvent::Push(push)) if push.arg.channel == "positions" => {
                        eprintln!(
                            "push received: action={:?}",
                            push.action
                        );
                        let rows: Vec<PositionUpdate> =
                            push.parse().expect("parse PositionUpdate rows");
                        eprintln!("parsed {} position row(s)", rows.len());
                        // Snapshot action is "snapshot"; event-driven updates are "event_update".
                        // The channel may also send empty snapshots.
                        assert!(
                            matches!(
                                push.action.as_deref(),
                                Some("snapshot") | Some("event_update") | None
                            ),
                            "unexpected action: {:?}",
                            push.action,
                        );
                        got_push = true;
                        break;
                    }
                    Some(WsEvent::Error { code, msg }) => {
                        panic!("OKX WS error {code}: {msg}");
                    }
                    Some(WsEvent::Disconnected) | None => {
                        panic!("disconnected before receiving push");
                    }
                    Some(_) => {}
                }
            }
        }
    }

    assert!(logged_in, "never received WsEvent::Login");
    assert!(
        subscribed,
        "never received WsEvent::Subscribed for positions"
    );
    assert!(got_push, "never received a positions push");

    ws.close().await.expect("close");
}
