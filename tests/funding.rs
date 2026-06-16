//! Integration tests for the authenticated Funding / Asset endpoints.
//!
//! Read-only tests run with `OKX_API_KEY`, `OKX_API_SECRET`, and
//! `OKX_PASSPHRASE`. Mutating asset tests require
//! `OKX_ENABLE_ASSET_MUTATION=1` plus endpoint-specific variables.

mod common;

use std::env;

use common::live_client;
use rust_okx::OkxClient;
use rust_okx::api::funding::{
    DepositHistoryRequest, DepositLightningRequest, DepositWithdrawStatusRequest,
    FundingBillsRequest, FundsTransferRequest, WithdrawalHistoryRequest,
    WithdrawalLightningRequest, WithdrawalRequest,
};

fn client_or_skip(test: &str) -> Option<OkxClient> {
    let client = live_client();
    if client.is_none() {
        eprintln!("skipping {test}: OKX_API_* env vars not set");
    }
    client
}

fn env_non_empty(var: &str) -> Option<String> {
    let _ = dotenvy::dotenv();
    env::var(var).ok().filter(|value| !value.is_empty())
}

fn mutation_enabled(test: &str) -> bool {
    let _ = dotenvy::dotenv();
    let enabled = env::var("OKX_ENABLE_ASSET_MUTATION").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skipping {test}: set OKX_ENABLE_ASSET_MUTATION=1 to run asset mutation test");
    }
    enabled
}

fn required_env(test: &str, var: &str) -> Option<String> {
    let value = env_non_empty(var);
    if value.is_none() {
        eprintln!("skipping {test}: {var} is not set");
    }
    value
}

#[tokio::test]
async fn get_currencies_live() {
    let Some(client) = client_or_skip("get_currencies_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_currencies(Some("USDT"))
        .await
        .expect("asset/currencies");
    assert!(
        !rows.is_empty(),
        "USDT should have at least one currency row"
    );
    assert!(rows.iter().any(|row| row.ccy == "USDT"));
}

#[tokio::test]
async fn get_balances_live() {
    let Some(client) = client_or_skip("get_balances_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_balances(None)
        .await
        .expect("asset/balances");
    for row in rows {
        assert!(
            !row.ccy.is_empty(),
            "balance row should include a currency code"
        );
    }
}

#[tokio::test]
async fn get_non_tradable_assets_live() {
    let Some(client) = client_or_skip("get_non_tradable_assets_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_non_tradable_assets(None)
        .await
        .expect("asset/non-tradable-assets");
    for row in rows {
        assert!(!row.ccy.is_empty(), "asset row should include a currency");
    }
}

#[tokio::test]
async fn get_deposit_address_live() {
    let Some(client) = client_or_skip("get_deposit_address_live") else {
        return;
    };
    let ccy = env_non_empty("OKX_TEST_DEPOSIT_CCY").unwrap_or_else(|| "USDT".to_owned());

    let rows = client
        .funding()
        .get_deposit_address(&ccy)
        .await
        .expect("asset/deposit-address");
    for row in rows {
        assert_eq!(row.ccy, ccy);
    }
}

#[tokio::test]
async fn get_deposit_history_live() {
    let Some(client) = client_or_skip("get_deposit_history_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_deposit_history(&DepositHistoryRequest::new().limit(5))
        .await
        .expect("asset/deposit-history");
    for row in rows {
        assert!(!row.ccy.is_empty(), "deposit row should include currency");
    }
}

#[tokio::test]
async fn get_withdrawal_history_live() {
    let Some(client) = client_or_skip("get_withdrawal_history_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_withdrawal_history(&WithdrawalHistoryRequest::new().limit(5))
        .await
        .expect("asset/withdrawal-history");
    for row in rows {
        assert!(
            !row.ccy.is_empty(),
            "withdrawal row should include currency"
        );
    }
}

#[tokio::test]
async fn get_bills_live() {
    let Some(client) = client_or_skip("get_bills_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_bills(&FundingBillsRequest::new().limit(5))
        .await
        .expect("asset/bills");
    for row in rows {
        assert!(!row.ccy.is_empty(), "bill row should include currency");
    }
}

#[tokio::test]
async fn get_asset_valuation_live() {
    let Some(client) = client_or_skip("get_asset_valuation_live") else {
        return;
    };

    let rows = client
        .funding()
        .get_asset_valuation(Some("USD"))
        .await
        .expect("asset/asset-valuation");
    assert!(
        rows.len() <= 1,
        "asset valuation should return zero or one summary row"
    );
}

#[tokio::test]
async fn transfer_state_live() {
    let test = "transfer_state_live";
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(trans_id) = required_env(test, "OKX_TEST_TRANSFER_ID") else {
        return;
    };
    let transfer_type = env_non_empty("OKX_TEST_TRANSFER_TYPE");

    let rows = client
        .funding()
        .transfer_state(&trans_id, transfer_type.as_deref())
        .await
        .expect("asset/transfer-state");
    assert!(!rows.is_empty(), "transfer-state should return a row");
}

#[tokio::test]
async fn get_deposit_withdraw_status_live() {
    let test = "get_deposit_withdraw_status_live";
    let Some(client) = client_or_skip(test) else {
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
        eprintln!("skipping {test}: set OKX_TEST_STATUS_* filters");
        return;
    }

    let rows = client
        .funding()
        .get_deposit_withdraw_status(&request)
        .await
        .expect("asset/deposit-withdraw-status");
    for row in rows {
        assert!(
            !row.state.is_empty() || !row.ccy.is_empty(),
            "status row should include state or currency"
        );
    }
}

#[tokio::test]
async fn funds_transfer_live_mutation() {
    let test = "funds_transfer_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(ccy) = required_env(test, "OKX_TEST_TRANSFER_CCY") else {
        return;
    };
    let Some(amt) = required_env(test, "OKX_TEST_TRANSFER_AMT") else {
        return;
    };
    let Some(from) = required_env(test, "OKX_TEST_TRANSFER_FROM") else {
        return;
    };
    let Some(to) = required_env(test, "OKX_TEST_TRANSFER_TO") else {
        return;
    };

    let mut request = FundsTransferRequest::new(ccy, amt, from, to);
    if let Some(value) = env_non_empty("OKX_TEST_TRANSFER_TYPE") {
        request = request.transfer_type(value);
    }

    let rows = client
        .funding()
        .funds_transfer(&request)
        .await
        .expect("asset/transfer");
    assert!(!rows.is_empty(), "transfer should return a transfer id");
}

#[tokio::test]
async fn withdrawal_live_mutation() {
    let test = "withdrawal_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(ccy) = required_env(test, "OKX_TEST_WITHDRAWAL_CCY") else {
        return;
    };
    let Some(amt) = required_env(test, "OKX_TEST_WITHDRAWAL_AMT") else {
        return;
    };
    let Some(dest) = required_env(test, "OKX_TEST_WITHDRAWAL_DEST") else {
        return;
    };
    let Some(to_addr) = required_env(test, "OKX_TEST_WITHDRAWAL_TO_ADDR") else {
        return;
    };

    let mut request = WithdrawalRequest::new(ccy, amt, dest, to_addr);
    if let Some(value) = env_non_empty("OKX_TEST_WITHDRAWAL_CHAIN") {
        request = request.chain(value);
    }
    if let Some(value) = env_non_empty("OKX_TEST_WITHDRAWAL_AREA_CODE") {
        request = request.area_code(value);
    }
    if let Some(value) = env_non_empty("OKX_TEST_WITHDRAWAL_CLIENT_ID") {
        request = request.client_id(value);
    }
    if let Some(value) = env_non_empty("OKX_TEST_WITHDRAWAL_TO_ADDR_TYPE") {
        request = request.to_addr_type(value);
    }

    let rows = client
        .funding()
        .withdrawal(&request)
        .await
        .expect("asset/withdrawal");
    assert!(!rows.is_empty(), "withdrawal should return a withdrawal id");
}

#[tokio::test]
async fn cancel_withdrawal_live_mutation() {
    let test = "cancel_withdrawal_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(wd_id) = required_env(test, "OKX_TEST_CANCEL_WITHDRAWAL_ID") else {
        return;
    };

    let rows = client
        .funding()
        .cancel_withdrawal(&wd_id)
        .await
        .expect("asset/cancel-withdrawal");
    assert!(!rows.is_empty(), "cancel-withdrawal should return a row");
}

#[tokio::test]
async fn purchase_redempt_live_mutation() {
    let test = "purchase_redempt_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(ccy) = required_env(test, "OKX_TEST_PURCHASE_REDEMPT_CCY") else {
        return;
    };
    let Some(amt) = required_env(test, "OKX_TEST_PURCHASE_REDEMPT_AMT") else {
        return;
    };
    let Some(side) = required_env(test, "OKX_TEST_PURCHASE_REDEMPT_SIDE") else {
        return;
    };
    let Some(rate) = required_env(test, "OKX_TEST_PURCHASE_REDEMPT_RATE") else {
        return;
    };

    let rows = client
        .funding()
        .purchase_redempt(&ccy, &amt, &side, &rate)
        .await
        .expect("asset/purchase_redempt");
    assert!(!rows.is_empty(), "purchase_redempt should return a row");
}

#[tokio::test]
async fn convert_dust_assets_live_mutation() {
    let test = "convert_dust_assets_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(ccy) = required_env(test, "OKX_TEST_DUST_CCY") else {
        return;
    };

    let rows = client
        .funding()
        .convert_dust_assets(&[ccy.as_str()])
        .await
        .expect("asset/convert-dust-assets");
    assert!(!rows.is_empty(), "dust conversion should return a row");
}

#[tokio::test]
async fn get_deposit_lightning_live_mutation() {
    let test = "get_deposit_lightning_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(ccy) = required_env(test, "OKX_TEST_LIGHTNING_CCY") else {
        return;
    };
    let Some(amt) = required_env(test, "OKX_TEST_LIGHTNING_AMT") else {
        return;
    };

    let rows = client
        .funding()
        .get_deposit_lightning(&DepositLightningRequest::new(ccy, amt))
        .await
        .expect("asset/deposit-lightning");
    assert!(
        !rows.is_empty(),
        "deposit-lightning should return an invoice"
    );
}

#[tokio::test]
async fn withdrawal_lightning_live_mutation() {
    let test = "withdrawal_lightning_live_mutation";
    if !mutation_enabled(test) {
        return;
    }
    let Some(client) = client_or_skip(test) else {
        return;
    };
    let Some(ccy) = required_env(test, "OKX_TEST_LIGHTNING_CCY") else {
        return;
    };
    let Some(invoice) = required_env(test, "OKX_TEST_LIGHTNING_INVOICE") else {
        return;
    };

    let mut request = WithdrawalLightningRequest::new(ccy, invoice);
    if let Some(value) = env_non_empty("OKX_TEST_LIGHTNING_MEMO") {
        request = request.memo(value);
    }

    let rows = client
        .funding()
        .withdrawal_lightning(&request)
        .await
        .expect("asset/withdrawal-lightning");
    assert!(
        !rows.is_empty(),
        "withdrawal-lightning should return a withdrawal id"
    );
}
