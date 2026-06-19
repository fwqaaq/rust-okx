use super::signed_client;
use crate::test_util::MockTransport;

use super::super::{
    ManagedSubAccountBillsRequest, SubAccountBillsRequest, SubAccountFundingBalancesRequest,
    SubAccountMaxWithdrawalRequest, SubAccountTradingBalancesRequest, SubAccountTransferRequest,
};

#[tokio::test]
async fn get_subaccount_trading_balances_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"adjEq":"101.46752000000001","availEq":"624719833286","borrowFroz":"0","details":[{"autoLendStatus":"off","availBal":"101.5","availEq":"101.5","cashBal":"101.5","ccy":"USDT","disEq":"101.46752000000001","eq":"101.5","eqUsd":"101.46752000000001","frozenBal":"0","interest":"0","isoEq":"0","isoLiab":"0","liab":"0","ordFrozen":"0","uTime":"1663854334734","upl":"0","crossLiab":"0","uplLiab":"0"}],"imr":"0","isoEq":"0","mgnRatio":"","mmr":"0","notionalUsd":"0","ordFroz":"0","totalEq":"101.46752000000001","uTime":"1739332269934","upl":"0"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_trading_balances(&SubAccountTradingBalancesRequest::new("test-1"))
        .await
        .unwrap();
    assert_eq!(rows[0].total_eq.as_str(), "101.46752000000001");
    assert_eq!(rows[0].details[0].ccy, "USDT");
    assert_eq!(rows[0].details[0].avail_bal.as_str(), "101.5");

    let req = mock.captured();
    assert_eq!(req.query(), Some("subAcct=test-1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_subaccount_funding_balances_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"availBal":"37.11827078","bal":"37.11827078","ccy":"ETH","frozenBal":"0"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_funding_balances(&SubAccountFundingBalancesRequest::new("test-1"))
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "ETH");
    assert_eq!(rows[0].avail_bal.as_str(), "37.11827078");
    assert_eq!(rows[0].frozen_bal.as_str(), "0");

    let req = mock.captured();
    assert_eq!(req.query(), Some("subAcct=test-1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_subaccount_max_withdrawal_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"BTC","maxWd":"3","maxWdEx":"","spotOffsetMaxWd":"3","spotOffsetMaxWdEx":""},{"ccy":"ETH","maxWd":"15","maxWdEx":"","spotOffsetMaxWd":"15","spotOffsetMaxWdEx":""},{"ccy":"USDT","maxWd":"10600","maxWdEx":"","spotOffsetMaxWd":"10600","spotOffsetMaxWdEx":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_max_withdrawal(&SubAccountMaxWithdrawalRequest::new("test-1"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].max_wd.as_str(), "3");
    assert_eq!(rows[2].ccy, "USDT");
    assert_eq!(rows[2].max_wd.as_str(), "10600");

    let req = mock.captured();
    assert_eq!(req.query(), Some("subAcct=test-1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_subaccount_bills_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"amt":"1.1","billId":"89887685","ccy":"USDT","subAcct":"hahatest1","ts":"1712560959000","type":"0"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_bills(&SubAccountBillsRequest::new())
        .await
        .unwrap();
    assert_eq!(rows[0].bill_id, "89887685");
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].amt.as_str(), "1.1");
    assert_eq!(rows[0].sub_acct, "hahatest1");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/asset/subaccount/bills"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_subaccount_managed_bills_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"billId":"12344","type":"1","ccy":"BTC","amt":"2","subAcct":"test-1","subUid":"xxxxxx","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_managed_bills(&ManagedSubAccountBillsRequest::new())
        .await
        .unwrap();
    assert_eq!(rows[0].bill_id, "12344");
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].sub_uid, "xxxxxx");

    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/asset/subaccount/managed-subaccount-bills")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn transfer_between_subaccounts_posts_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"transId":"12345"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        SubAccountTransferRequest::new("USDT", "10", "6", "6", "subAcct001", "subAcct002");

    let rows = client
        .sub_account()
        .transfer_between_subaccounts(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].trans_id, "12345");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/subaccount/transfer"));
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "USDT");
    assert_eq!(sent["fromSubAccount"], "subAcct001");
    assert_eq!(sent["toSubAccount"], "subAcct002");
}
