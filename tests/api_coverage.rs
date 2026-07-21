//! Executable audit for REST integration-test organization and API status annotations.
use std::collections::{BTreeMap, BTreeSet};

const IMPLEMENTED_ENDPOINTS: &[(&str, &str)] = &[
    ("GET", "/api/v5/account/account-position-risk"),
    ("GET", "/api/v5/account/adjust-leverage-info"),
    ("GET", "/api/v5/account/balance"),
    ("GET", "/api/v5/account/bills"),
    ("GET", "/api/v5/account/bills-archive"),
    ("GET", "/api/v5/account/bills-history-archive"),
    ("GET", "/api/v5/account/config"),
    ("GET", "/api/v5/account/greeks"),
    ("GET", "/api/v5/account/instruments"),
    ("GET", "/api/v5/account/interest-accrued"),
    ("GET", "/api/v5/account/interest-limits"),
    ("GET", "/api/v5/account/interest-rate"),
    ("GET", "/api/v5/account/leverage-info"),
    ("GET", "/api/v5/account/max-avail-size"),
    ("GET", "/api/v5/account/max-loan"),
    ("GET", "/api/v5/account/max-size"),
    ("GET", "/api/v5/account/max-withdrawal"),
    ("GET", "/api/v5/account/move-positions-history"),
    ("GET", "/api/v5/account/position-tiers"),
    ("GET", "/api/v5/account/positions"),
    ("GET", "/api/v5/account/positions-history"),
    ("GET", "/api/v5/account/precheck-set-delta-neutral"),
    ("GET", "/api/v5/account/risk-state"),
    ("GET", "/api/v5/account/set-account-switch-precheck"),
    ("GET", "/api/v5/account/spot-borrow-repay-history"),
    ("GET", "/api/v5/account/subtypes"),
    ("GET", "/api/v5/account/trade-fee"),
    ("GET", "/api/v5/asset/asset-valuation"),
    ("GET", "/api/v5/asset/balances"),
    ("GET", "/api/v5/asset/bills"),
    ("GET", "/api/v5/asset/convert/currencies"),
    ("GET", "/api/v5/asset/convert/currency-pair"),
    ("GET", "/api/v5/asset/convert/history"),
    ("GET", "/api/v5/asset/currencies"),
    ("GET", "/api/v5/asset/deposit-address"),
    ("GET", "/api/v5/asset/deposit-history"),
    ("GET", "/api/v5/asset/deposit-lightning"),
    ("GET", "/api/v5/asset/deposit-withdraw-status"),
    ("GET", "/api/v5/asset/non-tradable-assets"),
    ("GET", "/api/v5/asset/transfer-state"),
    ("GET", "/api/v5/asset/withdrawal-history"),
    ("GET", "/api/v5/finance/flexible-loan/borrow-currencies"),
    ("GET", "/api/v5/finance/flexible-loan/collateral-assets"),
    ("GET", "/api/v5/finance/flexible-loan/interest-accrued"),
    ("GET", "/api/v5/finance/flexible-loan/loan-history"),
    ("GET", "/api/v5/finance/flexible-loan/loan-info"),
    (
        "GET",
        "/api/v5/finance/flexible-loan/max-collateral-redeem-amount",
    ),
    ("GET", "/api/v5/finance/savings/balance"),
    ("GET", "/api/v5/finance/savings/lending-history"),
    ("GET", "/api/v5/finance/savings/lending-rate-history"),
    ("GET", "/api/v5/finance/savings/lending-rate-summary"),
    ("GET", "/api/v5/finance/staking-defi/eth/apy-history"),
    ("GET", "/api/v5/finance/staking-defi/eth/balance"),
    ("GET", "/api/v5/finance/staking-defi/eth/product-info"),
    (
        "GET",
        "/api/v5/finance/staking-defi/eth/purchase-redeem-history",
    ),
    ("GET", "/api/v5/finance/staking-defi/offers"),
    ("GET", "/api/v5/finance/staking-defi/orders-active"),
    ("GET", "/api/v5/finance/staking-defi/orders-history"),
    ("GET", "/api/v5/finance/staking-defi/sol/apy-history"),
    ("GET", "/api/v5/finance/staking-defi/sol/balance"),
    ("GET", "/api/v5/finance/staking-defi/sol/product-info"),
    (
        "GET",
        "/api/v5/finance/staking-defi/sol/purchase-redeem-history",
    ),
    ("GET", "/api/v5/market/block-ticker"),
    ("GET", "/api/v5/market/block-tickers"),
    ("GET", "/api/v5/market/books"),
    ("GET", "/api/v5/market/candles"),
    ("GET", "/api/v5/market/exchange-rate"),
    ("GET", "/api/v5/market/history-candles"),
    ("GET", "/api/v5/market/history-trades"),
    ("GET", "/api/v5/market/index-candles"),
    ("GET", "/api/v5/market/index-components"),
    ("GET", "/api/v5/market/index-tickers"),
    ("GET", "/api/v5/market/mark-price-candles"),
    ("GET", "/api/v5/market/option/instrument-family-trades"),
    ("GET", "/api/v5/market/platform-24-volume"),
    ("GET", "/api/v5/market/ticker"),
    ("GET", "/api/v5/market/tickers"),
    ("GET", "/api/v5/market/trades"),
    ("GET", "/api/v5/public/convert-contract-coin"),
    ("GET", "/api/v5/public/delivery-exercise-history"),
    ("GET", "/api/v5/public/discount-rate-interest-free-quota"),
    ("GET", "/api/v5/public/estimated-price"),
    ("GET", "/api/v5/public/funding-rate"),
    ("GET", "/api/v5/public/funding-rate-history"),
    ("GET", "/api/v5/public/instrument-tick-bands"),
    ("GET", "/api/v5/public/instruments"),
    ("GET", "/api/v5/public/insurance-fund"),
    ("GET", "/api/v5/public/interest-rate-loan-quota"),
    ("GET", "/api/v5/public/mark-price"),
    ("GET", "/api/v5/public/market-data-history"),
    ("GET", "/api/v5/public/open-interest"),
    ("GET", "/api/v5/public/opt-summary"),
    ("GET", "/api/v5/public/option-trades"),
    ("GET", "/api/v5/public/position-tiers"),
    ("GET", "/api/v5/public/price-limit"),
    ("GET", "/api/v5/public/time"),
    ("GET", "/api/v5/public/underlying"),
    ("GET", "/api/v5/trade/easy-convert-currency-list"),
    ("GET", "/api/v5/trade/easy-convert-history"),
    ("GET", "/api/v5/trade/fills"),
    ("GET", "/api/v5/trade/fills-history"),
    ("GET", "/api/v5/trade/one-click-repay-currency-list"),
    ("GET", "/api/v5/trade/one-click-repay-currency-list-v2"),
    ("GET", "/api/v5/trade/one-click-repay-history"),
    ("GET", "/api/v5/trade/one-click-repay-history-v2"),
    ("GET", "/api/v5/trade/order"),
    ("GET", "/api/v5/trade/order-algo"),
    ("GET", "/api/v5/trade/orders-algo-history"),
    ("GET", "/api/v5/trade/orders-algo-pending"),
    ("GET", "/api/v5/trade/orders-history"),
    ("GET", "/api/v5/trade/orders-history-archive"),
    ("GET", "/api/v5/trade/orders-pending"),
    ("POST", "/api/v5/account/activate-option"),
    ("POST", "/api/v5/account/bills-history-archive"),
    ("POST", "/api/v5/account/move-positions"),
    ("POST", "/api/v5/account/position-builder"),
    ("POST", "/api/v5/account/position/margin-balance"),
    ("POST", "/api/v5/account/set-account-level"),
    ("POST", "/api/v5/account/set-auto-earn"),
    ("POST", "/api/v5/account/set-auto-loan"),
    ("POST", "/api/v5/account/set-auto-repay"),
    ("POST", "/api/v5/account/set-collateral-assets"),
    ("POST", "/api/v5/account/set-fee-type"),
    ("POST", "/api/v5/account/set-greeks"),
    ("POST", "/api/v5/account/set-isolated-mode"),
    ("POST", "/api/v5/account/set-leverage"),
    ("POST", "/api/v5/account/set-position-mode"),
    ("POST", "/api/v5/account/set-riskOffset-amt"),
    ("POST", "/api/v5/account/set-settle-currency"),
    ("POST", "/api/v5/account/simulated_margin"),
    ("POST", "/api/v5/account/spot-manual-borrow-repay"),
    ("POST", "/api/v5/asset/cancel-withdrawal"),
    ("POST", "/api/v5/asset/convert/estimate-quote"),
    ("POST", "/api/v5/asset/convert/trade"),
    ("POST", "/api/v5/asset/transfer"),
    ("POST", "/api/v5/asset/withdrawal"),
    ("POST", "/api/v5/asset/withdrawal-lightning"),
    ("POST", "/api/v5/finance/flexible-loan/adjust-collateral"),
    ("POST", "/api/v5/finance/flexible-loan/max-loan"),
    ("POST", "/api/v5/finance/savings/purchase-redempt"),
    ("POST", "/api/v5/finance/savings/set-lending-rate"),
    ("POST", "/api/v5/finance/staking-defi/cancel"),
    ("POST", "/api/v5/finance/staking-defi/eth/purchase"),
    ("POST", "/api/v5/finance/staking-defi/eth/redeem"),
    ("POST", "/api/v5/finance/staking-defi/eth/cancel-redeem"),
    ("POST", "/api/v5/finance/staking-defi/purchase"),
    ("POST", "/api/v5/finance/staking-defi/redeem"),
    ("POST", "/api/v5/finance/staking-defi/sol/purchase"),
    ("POST", "/api/v5/finance/staking-defi/sol/redeem"),
    ("POST", "/api/v5/trade/amend-algos"),
    ("POST", "/api/v5/trade/amend-batch-orders"),
    ("POST", "/api/v5/trade/amend-order"),
    ("POST", "/api/v5/trade/batch-orders"),
    ("POST", "/api/v5/trade/cancel-algos"),
    ("POST", "/api/v5/trade/cancel-batch-orders"),
    ("POST", "/api/v5/trade/cancel-order"),
    ("POST", "/api/v5/trade/close-position"),
    ("POST", "/api/v5/trade/easy-convert"),
    ("POST", "/api/v5/trade/one-click-repay"),
    ("POST", "/api/v5/trade/one-click-repay-v2"),
    ("POST", "/api/v5/trade/order"),
    ("POST", "/api/v5/trade/order-algo"),
];

const ENDPOINT_SOURCES: &[&str] = &[
    include_str!("../src/api/account/endpoints.rs"),
    include_str!("../src/api/finance/endpoints.rs"),
    include_str!("../src/api/funding/endpoints.rs"),
    include_str!("../src/api/market/endpoints.rs"),
    include_str!("../src/api/public_data/endpoints.rs"),
    include_str!("../src/api/trade/endpoints.rs"),
    include_str!("../src/api/convert/api.rs"),
];

const TEST_SOURCES: &[(&str, &str)] = &[
    ("account/read_only.rs", include_str!("account/read_only.rs")),
    ("account/loans.rs", include_str!("account/loans.rs")),
    ("account/todo.rs", include_str!("account/todo.rs")),
    ("convert/read_only.rs", include_str!("convert/read_only.rs")),
    ("convert/todo.rs", include_str!("convert/todo.rs")),
    ("finance/read_only.rs", include_str!("finance/read_only.rs")),
    ("finance/todo.rs", include_str!("finance/todo.rs")),
    ("funding/read_only.rs", include_str!("funding/read_only.rs")),
    ("funding/todo.rs", include_str!("funding/todo.rs")),
    ("market/core.rs", include_str!("market/core.rs")),
    (
        "market/derivatives.rs",
        include_str!("market/derivatives.rs"),
    ),
    ("market/todo.rs", include_str!("market/todo.rs")),
    ("public_data/core.rs", include_str!("public_data/core.rs")),
    (
        "public_data/history.rs",
        include_str!("public_data/history.rs"),
    ),
    (
        "public_data/options_and_quotas.rs",
        include_str!("public_data/options_and_quotas.rs"),
    ),
    ("public_data/todo.rs", include_str!("public_data/todo.rs")),
    ("trade/read_only.rs", include_str!("trade/read_only.rs")),
    ("trade/todo.rs", include_str!("trade/todo.rs")),
    ("lifecycle_demo.rs", include_str!("lifecycle_demo.rs")),
];

fn annotations() -> BTreeMap<(String, String), Vec<String>> {
    let mut annotations = BTreeMap::new();

    for (file, source) in TEST_SOURCES {
        let mut pending = Vec::new();
        for (line_number, line) in source.lines().enumerate() {
            let line = line.trim();
            if let Some(api) = line.strip_prefix("// API: ") {
                let (method, path) = api.split_once(' ').unwrap_or_else(|| {
                    panic!("{file}:{} malformed API annotation: {api}", line_number + 1)
                });
                pending.push((method.to_owned(), path.to_owned(), line_number + 1));
            } else if let Some(status) = line.strip_prefix("// STATUS: ") {
                assert!(
                    !pending.is_empty(),
                    "{file}:{} STATUS has no preceding API annotation",
                    line_number + 1
                );
                assert!(
                    status.starts_with("LIVE")
                        || status.starts_with("TODO")
                        || status.starts_with("DEMO"),
                    "{file}:{} unsupported STATUS value: {status}",
                    line_number + 1
                );
                for (method, path, api_line) in pending.drain(..) {
                    let key = (method, path);
                    let _ = api_line;
                    annotations
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(status.to_owned());
                }
            }
        }
        assert!(
            pending.is_empty(),
            "{file} has API annotations without a following STATUS"
        );
    }

    annotations
}

#[test]
fn every_rest_endpoint_has_live_demo_or_todo_status() {
    let expected: BTreeSet<_> = IMPLEMENTED_ENDPOINTS
        .iter()
        .map(|(method, path)| ((*method).to_owned(), (*path).to_owned()))
        .collect();
    let annotated = annotations();
    let actual: BTreeSet<_> = annotated.keys().cloned().collect();

    let missing: Vec<_> = expected.difference(&actual).cloned().collect();
    let unknown: Vec<_> = actual.difference(&expected).cloned().collect();

    assert!(
        missing.is_empty(),
        "missing API status annotations: {missing:#?}"
    );
    assert!(
        unknown.is_empty(),
        "annotations for unknown APIs: {unknown:#?}"
    );
}

#[test]
fn todo_statuses_explain_why_the_case_is_not_live() {
    for ((method, path), statuses) in annotations() {
        for status in statuses {
            if status.starts_with("TODO") {
                assert!(
                    status.contains('—'),
                    "TODO status for {method} {path} must include a reason after an em dash"
                );
            }
        }
    }
}

#[test]
fn endpoint_path_registry_matches_current_source() {
    let source_paths: BTreeSet<String> = ENDPOINT_SOURCES
        .iter()
        .flat_map(|source| source.split('"'))
        .filter(|value| value.starts_with("/api/v5/"))
        .map(str::to_owned)
        .collect();
    let registry_paths: BTreeSet<String> = IMPLEMENTED_ENDPOINTS
        .iter()
        .map(|(_, path)| (*path).to_owned())
        .collect();

    let missing: Vec<_> = source_paths.difference(&registry_paths).cloned().collect();
    let stale: Vec<_> = registry_paths.difference(&source_paths).cloned().collect();

    assert!(
        missing.is_empty(),
        "endpoint paths missing from coverage registry: {missing:#?}"
    );
    assert!(
        stale.is_empty(),
        "stale endpoint paths in coverage registry: {stale:#?}"
    );
}
