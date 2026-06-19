use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod balance;
mod bills;
mod borrowing;
mod risk;
mod trading;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn missing_credentials_is_configuration_error() {
    let mock = MockTransport::new(r#"{}"#);
    let client = OkxClient::with_transport(mock).build();
    let err = client.account().get_balance(None).await.unwrap_err();
    assert!(matches!(
        err,
        crate::Error::Rest(crate::RestError::Configuration(_))
    ));
}

#[tokio::test]
async fn demo_trading_sets_simulated_header() {
    let body = r#"{"code":"0","msg":"","data":[
        {"uid":"12345","acctLv":"2","posMode":"net_mode","greeksType":"PA","autoLoan":false,
         "level":"Lv1","levelTmp":"","ctIsoMode":"automatic","mgnIsoMode":"automatic",
         "spotOffsetType":"","roleType":"0","traderInsts":[],"spotRoleType":"0","spotTraderInsts":[],
         "opAuth":"0","ip":"","perm":"read_only,trade","mainUid":"","kycLv":"1","label":"","ip":""}]}"#;
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
async fn invalid_account_request_fails_before_transport() {
    use crate::Error;
    let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
    let client = signed_client(mock.clone());
    let request = super::MaxLoanRequest::new("", crate::model::TradeMode::Cross);

    let error = client.account().get_max_loan(&request).await.unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());
}
