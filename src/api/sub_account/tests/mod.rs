use crate::model::ValidateRequest;
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod balances;
mod management;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[test]
fn typed_requests_validate_documented_constraints() {
    use super::{
        DeleteSubAccountApiKeyRequest, ManagedSubAccountBillsRequest,
        ModifySubAccountApiKeyRequest, SubAccountBillsRequest, SubAccountListRequest,
        SubAccountTransferRequest,
    };

    // limit must be 1–100
    assert!(SubAccountListRequest::new().limit(0).validate().is_err());
    assert!(SubAccountListRequest::new().limit(101).validate().is_err());
    assert!(SubAccountListRequest::new().limit(1).validate().is_ok());
    assert!(SubAccountListRequest::new().limit(100).validate().is_ok());

    assert!(SubAccountBillsRequest::new().limit(0).validate().is_err());
    assert!(SubAccountBillsRequest::new().limit(101).validate().is_err());
    assert!(SubAccountBillsRequest::new().limit(50).validate().is_ok());

    assert!(
        ManagedSubAccountBillsRequest::new()
            .limit(101)
            .validate()
            .is_err()
    );
    assert!(
        ManagedSubAccountBillsRequest::new()
            .limit(100)
            .validate()
            .is_ok()
    );

    // modify-apikey requires at least one of label / perm / ip
    let base = ModifySubAccountApiKeyRequest::new("acct", "key");
    assert!(base.clone().validate().is_err());
    assert!(base.clone().label("lbl").validate().is_ok());
    assert!(base.clone().perm("read_only").validate().is_ok());
    assert!(base.clone().ip("1.1.1.1").validate().is_ok());

    // delete-apikey requires non-empty sub_acct and api_key
    assert!(
        DeleteSubAccountApiKeyRequest::new("", "key")
            .validate()
            .is_err()
    );
    assert!(
        DeleteSubAccountApiKeyRequest::new("acct", "")
            .validate()
            .is_err()
    );
    assert!(
        DeleteSubAccountApiKeyRequest::new("acct", "key")
            .validate()
            .is_ok()
    );

    // transfer: from / to must be "6" or "18"
    let bad = SubAccountTransferRequest::new("USDT", "1", "99", "6", "a", "b");
    assert!(bad.validate().is_err());
    let bad2 = SubAccountTransferRequest::new("USDT", "1", "6", "99", "a", "b");
    assert!(bad2.validate().is_err());
    let ok = SubAccountTransferRequest::new("USDT", "1", "6", "18", "a", "b");
    assert!(ok.validate().is_ok());

    // transfer: required fields must be non-empty
    let empty_ccy = SubAccountTransferRequest::new("", "1", "6", "18", "a", "b");
    assert!(empty_ccy.validate().is_err());
    let empty_from_acct = SubAccountTransferRequest::new("USDT", "1", "6", "18", "", "b");
    assert!(empty_from_acct.validate().is_err());
}
