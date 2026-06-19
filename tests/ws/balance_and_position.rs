//! Integration tests for the private `balance_and_position` WebSocket channel.

use std::time::Duration;

use rust_okx::ws::channels::account;
use rust_okx::ws::model::BalanceAndPositionUpdate;
use rust_okx::{OkxWs, WsEvent};

use super::common::credentials;

/// Verifies the full private WebSocket flow for the `balance_and_position` channel:
/// connect → auto-login → subscribe → receive snapshot push → parse rows.
///
/// OKX sends a snapshot push even when there are no open positions or balances,
/// so this test passes regardless of account state.
///
/// Note: `event_type` for this channel lives inside each data row (not the push
/// envelope), so `push.action` will be `None`.
#[tokio::test]
async fn balance_and_position_login_subscribe_and_parse() {
    let Some(creds) = credentials() else {
        eprintln!("skipping: OKX credentials not set");
        return;
    };

    let mut ws = OkxWs::private(creds)
        .connect()
        .await
        .expect("connect to OKX WebSocket");

    ws.subscribe(&[account::balance_and_position()])
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
                    Some(WsEvent::Subscribed(arg)) if arg.channel == "balance_and_position" => {
                        eprintln!("subscribed to balance_and_position");
                        subscribed = true;
                    }
                    Some(WsEvent::Push(push)) if push.arg.channel == "balance_and_position" => {
                        eprintln!("push received: action={:?}", push.action);
                        let rows: Vec<BalanceAndPositionUpdate> =
                            push.parse().expect("parse BalanceAndPositionUpdate rows");
                        eprintln!("parsed {} row(s)", rows.len());
                        assert!(!rows.is_empty(), "snapshot must contain at least one row");
                        assert!(
                            !rows[0].p_time.as_str().is_empty(),
                            "pTime must be populated"
                        );
                        assert!(
                            rows[0].event_type == "snapshot"
                                || rows[0].event_type == "snapshot_updates",
                            "unexpected event_type: {}",
                            rows[0].event_type,
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
        "never received WsEvent::Subscribed for balance_and_position"
    );
    assert!(got_push, "never received a balance_and_position push");

    ws.close().await.expect("close");
}
