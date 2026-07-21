use crate::api::account::{
    AccountSwitchPrecheckRequest, AccountSwitchPresetRequest, PrecheckSetDeltaNeutralRequest,
    SetFeeTypeRequest, SetSettleCurrencyRequest,
};
use crate::test_util::MockTransport;

use super::signed_client;

#[tokio::test]
async fn precheck_set_delta_neutral_uses_documented_query() {
    let body = r#"{"code":"0","msg":"","data":[{"unmatchedInfoCheck":[{"type":"delta_risk","deltaLever":"2","ordList":["123"],"posList":["456"]}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = PrecheckSetDeltaNeutralRequest::new("1");

    let result = client
        .account()
        .precheck_set_delta_neutral(&request)
        .await
        .unwrap();
    assert_eq!(result[0].unmatched_info_check[0].unmatched_type, "delta_risk");
    assert_eq!(
        result[0].unmatched_info_check[0].delta_lever.as_str(),
        "2"
    );
    assert_eq!(result[0].unmatched_info_check[0].ord_list, ["123"]);
    assert_eq!(result[0].unmatched_info_check[0].pos_list, ["456"]);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("stgyType=1"));
    assert!(req
        .uri
        .contains("/api/v5/account/precheck-set-delta-neutral?"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_settle_currency_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"settleCcy":"USDC"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_settle_currency(&SetSettleCurrencyRequest::new("USDC"))
        .await
        .unwrap();
    assert_eq!(result[0].settle_ccy, "USDC");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-settle-currency"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["settleCcy"], "USDC");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_fee_type_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"feeType":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_fee_type(&SetFeeTypeRequest::new("1"))
        .await
        .unwrap();
    assert_eq!(result[0].fee_type, "1");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-fee-type"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["feeType"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn precheck_account_switch_decodes_margin_checks() {
    let body = r#"{"code":"0","msg":"","data":[{"sCode":"4","curAcctLv":"4","acctLv":"3","riskOffsetType":"","unmatchedInfoCheck":[{"type":"asset_validation","totalAsset":"1000","posList":[]}],"posList":[{"posId":"2005456500916518912","lever":"10"}],"posTierCheck":[{"instFamily":"BTC-USDT","instType":"SWAP","pos":"100","lever":"10","maxSz":"50"}],"mgnBf":{"acctAvailEq":"1000","mgnRatio":"4.5","details":""},"mgnAft":{"acctAvailEq":"900","mgnRatio":"3.8","details":[{"ccy":"USDT","availEq":"900","mgnRatio":"3.8"}]}}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .precheck_account_switch(&AccountSwitchPrecheckRequest::new("3"))
        .await
        .unwrap();
    assert_eq!(result[0].s_code, "4");
    assert_eq!(result[0].unmatched_info_check[0].total_asset.as_str(), "1000");
    assert_eq!(result[0].pos_tier_check[0].max_sz.as_str(), "50");
    assert!(result[0].mgn_bf.as_ref().unwrap().details.is_empty());
    assert_eq!(
        result[0].mgn_aft.as_ref().unwrap().details[0].avail_eq.as_str(),
        "900"
    );

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("acctLv=3"));
    assert!(req
        .uri
        .contains("/api/v5/account/set-account-switch-precheck?"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn preset_account_switch_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"acctLv":"3","curAcctLv":"4","lever":"10","riskOffsetType":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AccountSwitchPresetRequest::new("3").leverage("10");

    let result = client
        .account()
        .preset_account_switch(&request)
        .await
        .unwrap();
    assert_eq!(result[0].cur_acct_lv, "4");
    assert_eq!(result[0].acct_lv, "3");
    assert_eq!(result[0].lever.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req
        .uri
        .ends_with("/api/v5/account/account-level-switch-preset"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["acctLv"], "3");
    assert_eq!(sent["lever"], "10");
    assert!(sent.get("riskOffsetType").is_none());
    assert!(req.is_signed());
}
