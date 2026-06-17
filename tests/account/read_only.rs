use crate::common::{demo_client, live_client};
use rust_okx::api::account::{
    AccountInstrumentsRequest, BillsRequest, FeeRatesRequest, LeverageRequest,
    MaxAvailableSizeRequest,
};
use rust_okx::model::{InstType, TradeMode};
use rust_okx::{Error, OkxClient};

async fn read_only_suite(client: &OkxClient, label: &str) {
    const ACCOUNT_MODE_CODES: &[&str] = &["51010"];

    macro_rules! live {
        ($call:expr, $endpoint:literal) => {
            $call
                .await
                .unwrap_or_else(|error| panic!("[{label}] {} failed: {error}", $endpoint))
        };
    }

    macro_rules! live_or_mode_todo {
        ($call:expr, $endpoint:literal) => {
            match $call.await {
                Ok(_) => {}
                Err(Error::Api { code, message })
                    if ACCOUNT_MODE_CODES.contains(&code.as_str()) =>
                {
                    eprintln!(
                        "[{label}] TODO {}: unavailable in this account mode ({code} {message})",
                        $endpoint
                    );
                }
                Err(error) => panic!("[{label}] {} failed: {error}", $endpoint),
            }
        };
    }

    // API: GET /api/v5/account/config
    // STATUS: LIVE — authenticated, read-only.
    let config = live!(client.account().get_account_config(), "account/config");
    assert!(!config.is_empty(), "[{label}] config should return one row");

    // API: GET /api/v5/account/balance
    // STATUS: LIVE — authenticated, read-only.
    let balance = live!(client.account().get_balance(None), "account/balance");
    assert!(
        !balance.is_empty(),
        "[{label}] balance should return one row"
    );

    // API: GET /api/v5/account/positions
    // STATUS: LIVE — authenticated, read-only; an empty response is valid.
    live!(
        client.account().get_positions(None, None),
        "account/positions"
    );

    // API: GET /api/v5/account/account-position-risk
    // STATUS: LIVE — authenticated, read-only.
    live!(
        client.account().get_position_risk(None),
        "account/account-position-risk"
    );

    // API: GET /api/v5/account/trade-fee
    // STATUS: LIVE — authenticated, read-only.
    live!(
        client
            .account()
            .get_fee_rates(&FeeRatesRequest::new(InstType::Spot)),
        "account/trade-fee"
    );

    // API: GET /api/v5/account/leverage-info
    // STATUS: LIVE/MODE-TODO — read-only but account-mode dependent.
    live_or_mode_todo!(
        client
            .account()
            .get_leverage(&LeverageRequest::new(TradeMode::Cross).inst_id("BTC-USDT-SWAP")),
        "account/leverage-info"
    );

    // API: GET /api/v5/account/max-avail-size
    // STATUS: LIVE — authenticated, read-only.
    live!(
        client
            .account()
            .get_max_avail_size(&MaxAvailableSizeRequest::new("BTC-USDT", TradeMode::Cash)),
        "account/max-avail-size"
    );

    // API: GET /api/v5/account/greeks
    // STATUS: LIVE/MODE-TODO — read-only but options/PM-mode dependent.
    live_or_mode_todo!(client.account().get_greeks(None), "account/greeks");

    // API: GET /api/v5/account/bills
    // STATUS: LIVE — authenticated, read-only.
    live!(
        client
            .account()
            .get_account_bills(&BillsRequest::new().limit(5)),
        "account/bills"
    );

    // API: GET /api/v5/account/instruments
    // STATUS: LIVE — authenticated, read-only.
    live!(
        client
            .account()
            .get_account_instruments(&AccountInstrumentsRequest::new(InstType::Spot)),
        "account/instruments"
    );
}

#[tokio::test]
async fn live_account_read_only() {
    let Some(client) = live_client() else {
        eprintln!("skipping live_account_read_only: OKX_API_* env vars not set");
        return;
    };
    read_only_suite(&client, "live").await;
}

#[tokio::test]
async fn demo_account_read_only() {
    let Some(client) = demo_client() else {
        eprintln!("skipping demo_account_read_only: OKX_DEMO_API_* env vars not set");
        return;
    };
    read_only_suite(&client, "demo").await;
}
