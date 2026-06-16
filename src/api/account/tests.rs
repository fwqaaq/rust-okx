use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn get_balance_signs_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
        {"totalEq":"10000","adjEq":"9500","uTime":"1597026383085","details":[
            {"ccy":"USDT","eq":"10000","cashBal":"10000","availBal":"9000","frozenBal":"1000"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let balances = client.account().get_balance(None).await.unwrap();
    assert_eq!(balances[0].total_eq.as_str(), "10000");
    assert_eq!(balances[0].details[0].ccy, "USDT");
    assert_eq!(balances[0].details[0].avail_bal.as_str(), "9000");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert!(req.uri.ends_with("/api/v5/account/balance"));
    assert!(req.is_signed(), "authenticated endpoint must be signed");
}

#[tokio::test]
async fn get_positions_passes_filters() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instType":"SWAP","instId":"BTC-USDT-SWAP","posId":"1","posSide":"long",
         "mgnMode":"cross","pos":"1","avgPx":"42000","upl":"5","lever":"10","liqPx":"38000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let positions = client
        .account()
        .get_positions(Some(crate::model::InstType::Swap), Some("BTC-USDT-SWAP"))
        .await
        .unwrap();
    assert_eq!(positions[0].inst_id, "BTC-USDT-SWAP");
    assert_eq!(positions[0].pos_side, crate::model::PositionSide::Long);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn missing_credentials_is_configuration_error() {
    let mock = MockTransport::new("{}");
    let client = OkxClient::with_transport(mock).build();
    let err = client.account().get_balance(None).await.unwrap_err();
    assert!(matches!(err, crate::Error::Configuration(_)));
}

#[tokio::test]
async fn get_position_risk_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
        {"adjEq":"1000","ts":"1597026383085","balData":[{"ccy":"USDT","eq":"1000"}],
         "posData":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","posSide":"long","mgnMode":"cross","pos":"1"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let risk = client
        .account()
        .get_position_risk(Some(crate::model::InstType::Swap))
        .await
        .unwrap();
    assert_eq!(risk[0].adj_eq.as_str(), "1000");
    assert_eq!(risk[0].pos_data[0].inst_id, "BTC-USDT-SWAP");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_config_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
        {"uid":"1","acctLv":"2","posMode":"net_mode","greeksType":"PA","autoLoan":false}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let config = client.account().get_account_config().await.unwrap();
    assert_eq!(config[0].pos_mode, "net_mode");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/account/config"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_bills_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"billId":"1","instType":"SPOT","ccy":"USDT","mgnMode":"cash","type":"1","subType":"2",
         "sz":"10","bal":"100","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::BillsRequest::new()
        .inst_type(crate::model::InstType::Spot)
        .currency("USDT")
        .bill_type("1")
        .limit(1);

    let bills = client.account().get_account_bills(&request).await.unwrap();
    assert_eq!(bills[0].bill_id, "1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&ccy=USDT&type=1&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_bills_archive_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"billId":"2","instType":"SPOT","ccy":"USDT","type":"1","sz":"10","bal":"100","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::BillsArchiveRequest::new()
        .filters(super::BillsRequest::new().currency("USDT"))
        .begin("100")
        .end("200");

    let bills = client
        .account()
        .get_account_bills_archive(&request)
        .await
        .unwrap();
    assert_eq!(bills[0].bill_id, "2");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=USDT&begin=100&end=200"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_position_mode_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"posMode":"net_mode"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_position_mode("net_mode")
        .await
        .unwrap();
    assert_eq!(result[0].pos_mode, "net_mode");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-position-mode"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["posMode"], "net_mode");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_leverage_posts_builder_body() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT-SWAP","mgnMode":"cross","posSide":"long","lever":"10"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::SetLeverageRequest::new("10", crate::model::TradeMode::Cross)
        .inst_id("BTC-USDT-SWAP")
        .position_side(crate::model::PositionSide::Long);

    let result = client.account().set_leverage(&request).await.unwrap();
    assert_eq!(result[0].lever.as_str(), "10");

    let req = mock.captured();
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["lever"], "10");
    assert_eq!(sent["mgnMode"], "cross");
    assert_eq!(sent["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["posSide"], "long");
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_leverage_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT-SWAP","mgnMode":"cross","posSide":"long","lever":"10"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        super::LeverageRequest::new(crate::model::TradeMode::Cross).inst_id("BTC-USDT-SWAP");

    let result = client.account().get_leverage(&request).await.unwrap();
    assert_eq!(result[0].mgn_mode, crate::model::TradeMode::Cross);

    let req = mock.captured();
    assert_eq!(req.query(), Some("mgnMode=cross&instId=BTC-USDT-SWAP"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_order_size_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT","maxBuy":"1","maxSell":"2"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        super::MaxOrderSizeRequest::new("BTC-USDT", crate::model::TradeMode::Cash).price("42000");

    let result = client.account().get_max_order_size(&request).await.unwrap();
    assert_eq!(result[0].max_buy.as_str(), "1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&tdMode=cash&px=42000"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_avail_size_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT","availBuy":"1","availSell":"2"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::MaxAvailableSizeRequest::new("BTC-USDT", crate::model::TradeMode::Cash)
        .reduce_only(false);

    let result = client.account().get_max_avail_size(&request).await.unwrap();
    assert_eq!(result[0].avail_sell.as_str(), "2");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instId=BTC-USDT&tdMode=cash&reduceOnly=false")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fee_rates_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instType":"SPOT","instId":"BTC-USDT","category":"1","maker":"-0.0001","taker":"0.001","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FeeRatesRequest::new(crate::model::InstType::Spot).inst_id("BTC-USDT");

    let result = client.account().get_fee_rates(&request).await.unwrap();
    assert_eq!(result[0].maker.as_str(), "-0.0001");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&instId=BTC-USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_withdrawal_queries_currency() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ccy":"USDT","maxWd":"100","maxWdEx":"90"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .get_max_withdrawal(Some("USDT"))
        .await
        .unwrap();
    assert_eq!(result[0].max_wd.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_positions_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instType":"SWAP","instId":"BTC-USDT-SWAP","posId":"1","mgnMode":"cross",
         "type":"2","realizedPnl":"5","cTime":"1597026383085","uTime":"1597026383999"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::PositionsHistoryRequest::new()
        .inst_type(crate::model::InstType::Swap)
        .inst_id("BTC-USDT-SWAP")
        .limit(1);

    let result = client
        .account()
        .get_positions_history(&request)
        .await
        .unwrap();
    assert_eq!(result[0].realized_pnl.as_str(), "5");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instType=SWAP&instId=BTC-USDT-SWAP&limit=1")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_risk_state_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"atRisk":"false","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().get_risk_state().await.unwrap();
    assert_eq!(result[0].at_risk, "false");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/account/risk-state"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn demo_trading_sets_simulated_header() {
    let body = r#"{"code":"0","msg":"","data":[
        {"uid":"1","acctLv":"2","posMode":"net_mode","greeksType":"PA","autoLoan":false}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone())
        .credentials(Credentials::new("key", "secret", "pass"))
        .demo_trading(true)
        .build();

    let config = client.account().get_account_config().await.unwrap();
    assert_eq!(config[0].acct_lv, "2");

    let req = mock.captured();
    assert_eq!(
        req.headers
            .get("x-simulated-trading")
            .and_then(|v| v.to_str().ok()),
        Some("1")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn adjust_margin_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT-SWAP","posSide":"long","type":"add","amt":"100"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AdjustMarginRequest::new(
        "BTC-USDT-SWAP",
        crate::model::PositionSide::Long,
        "add",
        "100",
    )
    .loan_transfer(true);

    let result = client.account().adjust_margin(&request).await.unwrap();
    assert_eq!(result[0].amt.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/position/margin-balance"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["posSide"], "long");
    assert_eq!(sent["type"], "add");
    assert_eq!(sent["amt"], "100");
    assert_eq!(sent["loanTrans"], true);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_instruments_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instType":"SWAP","instId":"BTC-USDT-SWAP","uly":"BTC-USDT","instFamily":"BTC-USDT",
         "baseCcy":"BTC","quoteCcy":"USDT","settleCcy":"USDT"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AccountInstrumentsRequest::new()
        .inst_type(crate::model::InstType::Swap)
        .inst_id("BTC-USDT-SWAP");

    let result = client
        .account()
        .get_account_instruments(&request)
        .await
        .unwrap();
    assert_eq!(result[0].settle_ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_loan_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT","mgnMode":"cross","mgnCcy":"USDT","maxLoan":"1000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::MaxLoanRequest::new("BTC-USDT", crate::model::TradeMode::Cross)
        .margin_currency("USDT");

    let result = client.account().get_max_loan(&request).await.unwrap();
    assert_eq!(result[0].max_loan.as_str(), "1000");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instId=BTC-USDT&mgnMode=cross&mgnCcy=USDT")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_interest_accrued_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USDT","ccy":"USDT","mgnMode":"cross","interest":"1",
         "interestRate":"0.0001","liab":"100","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::InterestAccruedRequest::new()
        .inst_id("BTC-USDT")
        .currency("USDT")
        .limit(1);

    let result = client
        .account()
        .get_interest_accrued(&request)
        .await
        .unwrap();
    assert_eq!(result[0].interest_rate.as_str(), "0.0001");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&ccy=USDT&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_interest_rate_queries_currency() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","interestRate":"0.0001"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .get_interest_rate(Some("USDT"))
        .await
        .unwrap();
    assert_eq!(result[0].ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_greeks_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"greeksType":"PA"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().set_greeks("PA").await.unwrap();
    assert_eq!(result[0].greeks_type, "PA");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-greeks"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["greeksType"], "PA");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_isolated_mode_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"isoMode":"automatic","type":"MARGIN"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_isolated_mode("automatic", "MARGIN")
        .await
        .unwrap();
    assert_eq!(result[0].iso_mode, "automatic");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-isolated-mode"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["isoMode"], "automatic");
    assert_eq!(sent["type"], "MARGIN");
    assert!(req.is_signed());
}

#[tokio::test]
async fn borrow_repay_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ccy":"USDT","side":"borrow","amt":"100","ordId":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::BorrowRepayRequest::new("USDT", "borrow", "100").order_id("1");

    let result = client.account().borrow_repay(&request).await.unwrap();
    assert_eq!(result[0].ord_id, "1");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/borrow-repay"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "USDT");
    assert_eq!(sent["side"], "borrow");
    assert_eq!(sent["amt"], "100");
    assert_eq!(sent["ordId"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_borrow_repay_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ccy":"USDT","side":"borrow","amt":"100","ordId":"1","state":"2","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::BorrowRepayHistoryRequest::new()
        .currency("USDT")
        .limit(1);

    let result = client
        .account()
        .get_borrow_repay_history(&request)
        .await
        .unwrap();
    assert_eq!(result[0].state, "2");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=USDT&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_interest_limits_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ccy":"USDT","rate":"0.0001","loanQuota":"1000","usedLoan":"100"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::InterestLimitsRequest::new()
        .limit_type("1")
        .currency("USDT");

    let result = client
        .account()
        .get_interest_limits(&request)
        .await
        .unwrap();
    assert_eq!(result[0].loan_quota.as_str(), "1000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("type=1&ccy=USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_simulated_margin_posts_body_and_omits_unset_fields() {
    let body = r#"{"code":"0","msg":"","data":[
        {"imr":"10","mmr":"5","mr":"100","notionalUsd":"1000",
         "details":[{"instId":"BTC-USDT-SWAP","pos":"1","imr":"10","mmr":"5","upl":"2"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::SimulatedMarginRequest::new()
        .inst_type(crate::model::InstType::Swap)
        .simulated_positions(vec![
            super::SimulatedPosition::new("BTC-USDT-SWAP")
                .position("1")
                .leverage("10"),
        ]);

    let result = client
        .account()
        .get_simulated_margin(&request)
        .await
        .unwrap();
    assert_eq!(result[0].details[0].upl.as_str(), "2");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/simulated_margin"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instType"], "SWAP");
    assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["simPos"][0]["pos"], "1");
    assert_eq!(sent["simPos"][0]["lever"], "10");
    assert!(sent.get("inclRealPos").is_none());
    assert!(sent["simPos"][0].get("avgPx").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_greeks_queries_currency() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ccy":"BTC","deltaBS":"1","deltaPA":"0.9","gammaBS":"0.1","thetaBS":"-0.01","vegaBS":"2"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().get_greeks(Some("BTC")).await.unwrap();
    assert_eq!(result[0].delta_pa.as_str(), "0.9");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_position_tiers_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        {"instType":"OPTION","uly":"BTC-USD","instFamily":"BTC-USD","posType":"1","minSz":"0","maxSz":"100"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AccountPositionTiersRequest::new()
        .inst_type(crate::model::InstType::Option)
        .underlying("BTC-USD");

    let result = client
        .account()
        .get_account_position_tiers(&request)
        .await
        .unwrap();
    assert_eq!(result[0].max_sz.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=OPTION&uly=BTC-USD"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_risk_offset_type_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"type":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().set_risk_offset_type("1").await.unwrap();
    assert_eq!(result[0].risk_offset_type, "1");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-riskOffset-type"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["type"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_auto_loan_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"autoLoan":"true"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().set_auto_loan(true).await.unwrap();
    assert_eq!(result[0].auto_loan, "true");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-auto-loan"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["autoLoan"], true);
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_account_level_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"acctLv":"2"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().set_account_level("2").await.unwrap();
    assert_eq!(result[0].acct_lv, "2");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-account-level"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["acctLv"], "2");
    assert!(req.is_signed());
}

#[tokio::test]
async fn activate_option_posts_empty_body() {
    let body = r#"{"code":"0","msg":"","data":[{"result":"true"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().activate_option().await.unwrap();
    assert_eq!(result[0].result, "true");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/activate-option"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent, serde_json::json!({}));
    assert!(req.is_signed());
}

#[tokio::test]
async fn position_builder_posts_body_and_omits_unset_fields() {
    let body = r#"{"code":"0","msg":"","data":[
        {"acctLv":"2","adjEq":"1000","imr":"10","mmr":"5","mr":"100",
         "posData":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","pos":"1","avgPx":"42000","upl":"2"}],
         "assetData":[{"ccy":"USDT","eq":"1000"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::PositionBuilderRequest::new()
        .account_level("2")
        .include_real_positions_and_equity(false)
        .simulated_positions(vec![
            super::SimulatedPosition::new("BTC-USDT-SWAP").position("1"),
        ])
        .simulated_assets(vec![super::SimulatedAsset::new("USDT").equity("1000")]);

    let result = client.account().position_builder(&request).await.unwrap();
    assert_eq!(result[0].pos_data[0].inst_id, "BTC-USDT-SWAP");
    assert_eq!(result[0].asset_data[0].eq.as_str(), "1000");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/position-builder"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["acctLv"], "2");
    assert_eq!(sent["inclRealPosAndEq"], false);
    assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["simAsset"][0]["ccy"], "USDT");
    assert!(sent.get("lever").is_none());
    assert!(sent["simPos"][0].get("avgPx").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn vip_loan_queries_are_signed_and_flexible() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ordId":"vip1","ccy":"USDT","amt":"100","ts":"1597026383085"}]}"#;

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::VipInterestAccruedRequest::new().param("ccy", "USDT");
    let rows = client
        .account()
        .get_vip_interest_accrued(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(mock.captured().query(), Some("ccy=USDT"));
    assert!(mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::VipInterestDeductedRequest::new().param("ordId", "vip1");
    client
        .account()
        .get_vip_interest_deducted(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("ordId=vip1"));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::VipLoanOrderListRequest::new().param("ccy", "USDT");
    client
        .account()
        .get_vip_loan_order_list(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/vip-loan-order-list?ccy=USDT")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::VipLoanOrderDetailRequest::new().param("ordId", "vip1");
    client
        .account()
        .get_vip_loan_order_detail(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("ordId=vip1"));
}

#[tokio::test]
async fn fixed_loan_endpoints_are_signed_and_flexible() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ordId":"loan1","ccy":"USDT","amt":"100","rate":"0.01","state":"live"}]}"#;

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanBorrowingLimitRequest::new().param("ccy", "USDT");
    let rows = client
        .account()
        .get_fixed_loan_borrowing_limit(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].amt.as_str(), "100");
    assert_eq!(mock.captured().query(), Some("ccy=USDT"));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanBorrowingQuoteRequest::new()
        .param("ccy", "USDT")
        .param("amt", "100");
    client
        .account()
        .fixed_loan_borrowing_quote(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/fixed-loan/borrowing-quote")
    );
    assert!(mock.captured().body_str().contains(r#""amt":"100""#));
    assert!(mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanBorrowingOrderRequest::new().param("quoteId", "q1");
    client
        .account()
        .fixed_loan_borrowing_order(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/fixed-loan/borrowing-order")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanAmendBorrowingOrderRequest::new()
        .param("ordId", "loan1")
        .param("reborrow", "true");
    client
        .account()
        .amend_fixed_loan_borrowing_order(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/fixed-loan/amend-borrowing-order")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanManualReborrowRequest::new().param("ordId", "loan1");
    client
        .account()
        .fixed_loan_manual_reborrow(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/fixed-loan/manual-reborrow")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanRepayBorrowingOrderRequest::new()
        .param("ordId", "loan1")
        .param("amt", "10");
    client
        .account()
        .repay_fixed_loan_borrowing_order(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/fixed-loan/repay-borrowing-order")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FixedLoanBorrowingOrdersListRequest::new().param("state", "live");
    client
        .account()
        .get_fixed_loan_borrowing_orders_list(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("state=live"));
}

#[tokio::test]
async fn spot_borrow_repay_and_auto_settings_are_signed() {
    let body = r#"{"code":"0","msg":"","data":[
        {"ccy":"USDT","ordId":"spot1","sCode":"0","sMsg":"","state":"success"}]}"#;

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::SpotManualBorrowRepayRequest::new()
        .param("ccy", "USDT")
        .param("side", "borrow")
        .param("amt", "10");
    let rows = client
        .account()
        .spot_manual_borrow_repay(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/account/spot-manual-borrow-repay")
    );
    assert!(mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::SetAutoRepayRequest::new().bool_param("autoRepay", true);
    client.account().set_auto_repay(&request).await.unwrap();
    assert!(mock.captured().body_str().contains(r#""autoRepay":true"#));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::SpotBorrowRepayHistoryRequest::new().param("ccy", "USDT");
    client
        .account()
        .get_spot_borrow_repay_history(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("ccy=USDT"));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::SetAutoEarnRequest::new().bool_param("autoEarn", true);
    client.account().set_auto_earn(&request).await.unwrap();
    assert!(mock.captured().body_str().contains(r#""autoEarn":true"#));
}
