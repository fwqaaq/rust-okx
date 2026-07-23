use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod assets;
mod deposits;
mod edge;
mod transfers;
mod withdrawals;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

// Serialization-only tests that verify request builder field omission.

#[test]
fn funds_transfer_request_omits_unset_optional_fields() {
    use super::FundsTransferRequest;
    let value = serde_json::to_value(FundsTransferRequest::new("USDT", "1", "6", "18")).unwrap();

    assert_eq!(value["ccy"], "USDT");
    assert_eq!(value["amt"], "1");
    assert_eq!(value["from"], "6");
    assert_eq!(value["to"], "18");
    assert!(value.get("subAcct").is_none());
    assert!(value.get("instId").is_none());
    assert!(value.get("loanTrans").is_none());
}

#[test]
fn withdrawal_request_omits_unset_optional_fields() {
    use super::WithdrawalRequest;
    let value = serde_json::to_value(WithdrawalRequest::new("USDT", "1", "3", "example")).unwrap();

    assert_eq!(value["ccy"], "USDT");
    assert_eq!(value["amt"], "1");
    assert_eq!(value["dest"], "3");
    assert_eq!(value["toAddr"], "example");
    assert!(value.get("chain").is_none());
    assert!(value.get("areaCode").is_none());
    assert!(value.get("toAddrType").is_none());
}

#[test]
fn history_requests_omit_unset_optional_fields() {
    use super::{DepositHistoryRequest, WithdrawalHistoryRequest};

    let deposit = serde_urlencoded::to_string(DepositHistoryRequest::new().limit(5)).unwrap();
    assert_eq!(deposit, "limit=5");

    let withdrawal =
        serde_urlencoded::to_string(WithdrawalHistoryRequest::new().to_addr_type("1")).unwrap();
    assert_eq!(withdrawal, "toAddrType=1");
}

#[test]
fn deposit_withdraw_status_request_omits_unset_optional_fields() {
    use super::DepositWithdrawStatusRequest;

    let query = serde_urlencoded::to_string(
        DepositWithdrawStatusRequest::new()
            .currency("USDT")
            .chain("USDT-TRC20"),
    )
    .unwrap();

    assert_eq!(query, "ccy=USDT&chain=USDT-TRC20");
}
