//! Integration tests for the authenticated **account** endpoints.
//!
//! The read-only suite runs against BOTH the live (main) account and the demo
//! (simulated) account, whichever credentials are present. Each account is
//! tested independently and skips if its credential set is missing. These tests
//! never mutate account state — they only issue read (`GET`) requests.

mod common;

use common::{demo_client, live_client};
use rust_okx::api::account::{
    AccountInstrumentsRequest, BillsRequest, FeeRatesRequest, LeverageRequest,
    MaxAvailableSizeRequest,
};
use rust_okx::model::{InstType, TradeMode};
use rust_okx::{Error, OkxClient};

/// Run every read-only account endpoint and assert each one succeeds (OKX
/// `code == "0"`, i.e. no [`rust_okx::Error::Api`]) and its payload parses.
/// `label` identifies the account ("live" or "demo") in failure messages.
async fn read_only_suite(client: &OkxClient, label: &str) {
    // OKX error codes that mean "this endpoint is not available under the
    // account's current mode/feature set" rather than a real failure. Tests run
    // across different account configurations, so mode-gated endpoints tolerate
    // these and report them as skipped instead of failing.
    const ACCOUNT_MODE_CODES: &[&str] = &["51010"];

    macro_rules! ok {
        ($call:expr, $endpoint:literal) => {
            $call
                .await
                .unwrap_or_else(|e| panic!("[{label}] {} failed: {e}", $endpoint))
        };
    }

    // Like `ok!`, but accepts an account-mode error ([`ACCOUNT_MODE_CODES`]) as
    // a skip so the suite passes regardless of the account's trading mode.
    macro_rules! ok_or_mode_skip {
        ($call:expr, $endpoint:literal) => {
            match $call.await {
                Ok(_) => {}
                Err(Error::Api { code, message })
                    if ACCOUNT_MODE_CODES.contains(&code.as_str()) =>
                {
                    eprintln!(
                        "[{label}] {} unavailable in this account mode ({code} {message}); skipped",
                        $endpoint
                    );
                }
                Err(e) => panic!("[{label}] {} failed: {e}", $endpoint),
            }
        };
    }

    // GET /api/v5/account/config — account configuration (uid, account level,
    // position mode). Always returns exactly one row.
    let config = ok!(client.account().get_account_config(), "account/config");
    assert!(!config.is_empty(), "[{label}] config should return one row");

    // GET /api/v5/account/balance — trading-account balance summary.
    let balance = ok!(client.account().get_balance(None), "account/balance");
    assert!(
        !balance.is_empty(),
        "[{label}] balance should return one row"
    );

    // GET /api/v5/account/positions — open positions (may legitimately be empty).
    ok!(
        client.account().get_positions(None, None),
        "account/positions"
    );

    // GET /api/v5/account/account-position-risk — per-account risk snapshot.
    ok!(
        client.account().get_position_risk(None),
        "account/account-position-risk"
    );

    // GET /api/v5/account/trade-fee — SPOT trade fee rates.
    ok!(
        client
            .account()
            .get_fee_rates(&FeeRatesRequest::new(InstType::Spot)),
        "account/trade-fee"
    );

    // GET /api/v5/account/leverage-info — leverage for a cross-margin SWAP
    // (requires a derivatives-capable account mode).
    ok_or_mode_skip!(
        client
            .account()
            .get_leverage(&LeverageRequest::new(TradeMode::Cross).inst_id("BTC-USDT-SWAP")),
        "account/leverage-info"
    );

    // GET /api/v5/account/max-avail-size — max available size for a spot pair.
    ok!(
        client
            .account()
            .get_max_avail_size(&MaxAvailableSizeRequest::new("BTC-USDT", TradeMode::Cash)),
        "account/max-avail-size"
    );

    // GET /api/v5/account/greeks — account greeks (requires options/PM mode).
    ok_or_mode_skip!(client.account().get_greeks(None), "account/greeks");

    // GET /api/v5/account/bills — recent account bills (last 5).
    ok!(
        client
            .account()
            .get_account_bills(&BillsRequest::new().limit(5)),
        "account/bills"
    );

    // GET /api/v5/account/instruments — SPOT instruments available to the account.
    ok!(
        client
            .account()
            .get_account_instruments(&AccountInstrumentsRequest::new(InstType::Spot)),
        "account/instruments"
    );
}

/// Read-only suite against the **live (main) account**.
/// Skips when `OKX_API_KEY`/`OKX_API_SECRET`/`OKX_PASSPHRASE` are not set.
#[tokio::test]
async fn live_account_read_only() {
    let Some(client) = live_client() else {
        eprintln!("skipping live_account_read_only: OKX_API_* env vars not set");
        return;
    };
    read_only_suite(&client, "live").await;
}

/// Read-only suite against the **demo (simulated) account**, using the
/// dedicated demo-environment key.
/// Skips when `OKX_DEMO_API_KEY`/`OKX_DEMO_API_SECRET`/`OKX_DEMO_PASSPHRASE` are
/// not set.
#[tokio::test]
async fn demo_account_read_only() {
    let Some(client) = demo_client() else {
        eprintln!("skipping demo_account_read_only: OKX_DEMO_API_* env vars not set");
        return;
    };
    read_only_suite(&client, "demo").await;
}
