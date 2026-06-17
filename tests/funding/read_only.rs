use crate::common::{env_non_empty, live_client_or_skip};
use rust_okx::api::funding::{
    DepositHistoryRequest, DepositWithdrawStatusRequest, FundingBillsRequest,
    WithdrawalHistoryRequest,
};

#[tokio::test]
async fn funding_currency_and_balance_endpoints_parse() {
    let Some(client) = live_client_or_skip("funding_currency_and_balance_endpoints_parse") else {
        return;
    };

    // API: GET /api/v5/asset/currencies
    // STATUS: LIVE — authenticated, read-only.
    let currencies = client
        .funding()
        .get_currencies(Some("USDT"))
        .await
        .expect("asset/currencies");
    assert!(currencies.iter().any(|row| row.ccy == "USDT"));

    // API: GET /api/v5/asset/balances
    // STATUS: LIVE — authenticated, read-only.
    let balances = client
        .funding()
        .get_balances(None)
        .await
        .expect("asset/balances");
    assert!(balances.iter().all(|row| !row.ccy.is_empty()));

    // API: GET /api/v5/asset/non-tradable-assets
    // STATUS: LIVE — authenticated, read-only.
    let assets = client
        .funding()
        .get_non_tradable_assets(None)
        .await
        .expect("asset/non-tradable-assets");
    assert!(assets.iter().all(|row| !row.ccy.is_empty()));

    // API: GET /api/v5/asset/asset-valuation
    // STATUS: LIVE — authenticated, read-only.
    let valuation = client
        .funding()
        .get_asset_valuation(Some("USD"))
        .await
        .expect("asset/asset-valuation");
    assert!(valuation.len() <= 1);
}

#[tokio::test]
async fn funding_address_and_history_endpoints_parse() {
    let Some(client) = live_client_or_skip("funding_address_and_history_endpoints_parse") else {
        return;
    };
    let ccy = env_non_empty("OKX_TEST_DEPOSIT_CCY").unwrap_or_else(|| "USDT".to_owned());

    // API: GET /api/v5/asset/deposit-address
    // STATUS: LIVE — authenticated, read-only.
    let addresses = client
        .funding()
        .get_deposit_address(&ccy)
        .await
        .expect("asset/deposit-address");
    assert!(addresses.iter().all(|row| row.ccy == ccy));

    // API: GET /api/v5/asset/deposit-history
    // STATUS: LIVE — authenticated, read-only.
    let deposits = client
        .funding()
        .get_deposit_history(&DepositHistoryRequest::new().limit(5))
        .await
        .expect("asset/deposit-history");
    assert!(deposits.iter().all(|row| !row.ccy.is_empty()));

    // API: GET /api/v5/asset/withdrawal-history
    // STATUS: LIVE — authenticated, read-only.
    let withdrawals = client
        .funding()
        .get_withdrawal_history(&WithdrawalHistoryRequest::new().limit(5))
        .await
        .expect("asset/withdrawal-history");
    assert!(withdrawals.iter().all(|row| !row.ccy.is_empty()));

    // API: GET /api/v5/asset/bills
    // STATUS: LIVE — authenticated, read-only.
    let bills = client
        .funding()
        .get_bills(&FundingBillsRequest::new().limit(5))
        .await
        .expect("asset/bills");
    assert!(bills.iter().all(|row| !row.ccy.is_empty()));
}

#[tokio::test]
async fn funding_transfer_state_parses_when_id_is_configured() {
    let test = "funding_transfer_state_parses_when_id_is_configured";
    let Some(client) = live_client_or_skip(test) else {
        return;
    };
    let Some(trans_id) = env_non_empty("OKX_TEST_TRANSFER_ID") else {
        eprintln!("skipping {test}: OKX_TEST_TRANSFER_ID is not set");
        return;
    };
    let transfer_type = env_non_empty("OKX_TEST_TRANSFER_TYPE");

    // API: GET /api/v5/asset/transfer-state
    // STATUS: LIVE/ENV — read-only but requires a real transfer ID.
    let rows = client
        .funding()
        .transfer_state(&trans_id, transfer_type.as_deref())
        .await
        .expect("asset/transfer-state");
    assert!(!rows.is_empty());
}

#[tokio::test]
async fn funding_deposit_withdraw_status_parses_when_filter_is_configured() {
    let test = "funding_deposit_withdraw_status_parses_when_filter_is_configured";
    let Some(client) = live_client_or_skip(test) else {
        return;
    };

    let mut request = DepositWithdrawStatusRequest::new();
    let mut has_filter = false;
    if let Some(value) = env_non_empty("OKX_TEST_STATUS_WD_ID") {
        request = request.withdrawal_id(value);
        has_filter = true;
    }
    if let Some(value) = env_non_empty("OKX_TEST_STATUS_TX_ID") {
        request = request.tx_id(value);
        has_filter = true;
    }
    if let Some(value) = env_non_empty("OKX_TEST_STATUS_CCY") {
        request = request.currency(value);
        has_filter = true;
    }
    if let Some(value) = env_non_empty("OKX_TEST_STATUS_TO") {
        request = request.to(value);
        has_filter = true;
    }
    if let Some(value) = env_non_empty("OKX_TEST_STATUS_CHAIN") {
        request = request.chain(value);
        has_filter = true;
    }
    if !has_filter {
        eprintln!("skipping {test}: set at least one OKX_TEST_STATUS_* filter");
        return;
    }

    // API: GET /api/v5/asset/deposit-withdraw-status
    // STATUS: LIVE/ENV — read-only but requires a transaction/address filter.
    let rows = client
        .funding()
        .get_deposit_withdraw_status(&request)
        .await
        .expect("asset/deposit-withdraw-status");
    assert!(
        rows.iter()
            .all(|row| !row.state.is_empty() || !row.ccy.is_empty())
    );
}
