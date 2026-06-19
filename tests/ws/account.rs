//! Integration tests for the private `account` WebSocket channel.

use std::time::Duration;

use rust_okx::ws::channels::account;
use rust_okx::ws::model::AccountUpdate;
use rust_okx::{OkxWs, WsEvent};

use super::common::credentials;

/// Verifies the full private WebSocket flow for the `account` channel:
/// connect → auto-login → subscribe → receive snapshot push → parse rows.
///
/// OKX always sends a snapshot on subscribe, so this test passes regardless
/// of whether the account holds any USDT balance.
#[tokio::test]
async fn account_login_subscribe_and_parse() {
    let Some(creds) = credentials() else {
        eprintln!("skipping: OKX credentials not set");
        return;
    };

    let mut ws = OkxWs::private(creds)
        .connect()
        .await
        .expect("connect to OKX WebSocket");

    ws.subscribe(&[account::account_by_currency("USDT", None)])
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
                    Some(WsEvent::Subscribed(arg)) if arg.channel == "account" => {
                        eprintln!("subscribed to account");
                        subscribed = true;
                    }
                    Some(WsEvent::Push(push)) if push.arg.channel == "account" => {
                        eprintln!("push received: action={:?}", push.action);
                        let rows: Vec<AccountUpdate> =
                            push.parse().expect("parse AccountUpdate rows");
                        eprintln!("parsed {} account row(s)", rows.len());
                        assert_eq!(
                            push.action.as_deref(),
                            Some("snapshot"),
                            "initial push should be a snapshot"
                        );
                        assert!(!rows.is_empty(), "account snapshot must contain at least one row");
                        assert!(
                            !rows[0].u_time.as_str().is_empty(),
                            "uTime must be populated"
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
    assert!(subscribed, "never received WsEvent::Subscribed for account");
    assert!(got_push, "never received an account push");

    ws.close().await.expect("close");
}
