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
        ManagedSubAccountBillsRequest, ModifySubAccountApiKeyRequest, SubAccountBillsRequest,
        SubAccountListRequest,
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
}
