use super::signed_client;
use crate::test_util::MockTransport;

use super::super::{
    CreateSubAccountApiKeyRequest, CreateSubAccountRequest, DeleteSubAccountApiKeyRequest,
    ModifySubAccountApiKeyRequest, SetTransferOutRequest, SubAccountApiKeysRequest,
    SubAccountListRequest,
};

#[tokio::test]
async fn get_subaccount_list_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"canTransOut":false,"enable":true,"frozenFunc":[],"gAuth":false,"label":"D456DDDLx","mobile":"","subAcct":"D456DDDL","ts":"1659334756000","type":"1","uid":"3400***7413","subAcctLv":"1","firstLvSubAcct":"D456DDDL","ifDma":false}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_list(&SubAccountListRequest::new())
        .await
        .unwrap();
    assert_eq!(rows[0].sub_acct, "D456DDDL");
    assert_eq!(rows[0].label, "D456DDDLx");
    assert!(rows[0].enable); // enable is true

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/users/subaccount/list"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn create_subaccount_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"label":"123456 ","subAcct":"subAccount002","ts":"1744875304520","uid":"698827017768230914"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CreateSubAccountRequest::new("subAccount002").label("123456");

    let rows = client
        .sub_account()
        .create_subaccount(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].sub_acct, "subAccount002");
    assert_eq!(rows[0].uid, "698827017768230914");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/users/subaccount/create-subaccount")
    );
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["subAcct"], "subAccount002");
    assert_eq!(sent["label"], "123456");
}

#[tokio::test]
async fn create_subaccount_apikey_posts_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"subAcct":"test-1","label":"v5","apiKey":"******","secretKey":"******","passphrase":"******","perm":"read_only,trade","ip":"1.1.1.1,2.2.2.2","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CreateSubAccountApiKeyRequest::new("test-1", "v5", "pass123")
        .perm("read_only,trade")
        .ip("1.1.1.1,2.2.2.2");

    let rows = client
        .sub_account()
        .create_subaccount_apikey(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].sub_acct, "test-1");
    assert_eq!(rows[0].perm, "read_only,trade");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/users/subaccount/apikey"));
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["subAcct"], "test-1");
    assert_eq!(sent["label"], "v5");
}

#[tokio::test]
async fn get_subaccount_apikeys_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"label":"v5","apiKey":"******","perm":"read_only,trade","ip":"1.1.1.1,2.2.2.2","ts":"1597026383085"},{"label":"v5.1","apiKey":"******","perm":"read_only","ip":"1.1.1.1,2.2.2.2","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_subaccount_apikeys(&SubAccountApiKeysRequest::new("test-1"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].label, "v5");
    assert_eq!(rows[1].perm, "read_only");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("subAcct=test-1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn modify_subaccount_apikey_posts_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"subAcct":"yongxu","label":"v5","apiKey":"******","perm":"read,trade","ip":"1.1.1.1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ModifySubAccountApiKeyRequest::new("yongxu", "someApiKey")
        .perm("read,trade")
        .ip("1.1.1.1");

    let rows = client
        .sub_account()
        .modify_subaccount_apikey(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].sub_acct, "yongxu");
    assert_eq!(rows[0].perm, "read,trade");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/users/subaccount/modify-apikey"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn delete_subaccount_apikey_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = DeleteSubAccountApiKeyRequest::new("test-1", "someApiKey");

    let rows = client
        .sub_account()
        .delete_subaccount_apikey(&request)
        .await
        .unwrap();
    assert_eq!(rows.len(), 0);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/users/subaccount/delete-apikey"));
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["subAcct"], "test-1");
    assert_eq!(sent["apiKey"], "someApiKey");
}

#[tokio::test]
async fn set_subaccount_transfer_out_posts_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"subAcct":"Test001","canTransOut":true},{"subAcct":"Test002","canTransOut":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetTransferOutRequest::new("Test001");

    let rows = client
        .sub_account()
        .set_subaccount_transfer_out(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].sub_acct, "Test001");
    assert!(rows[0].can_trans_out);
    assert_eq!(rows[1].sub_acct, "Test002");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/users/subaccount/set-transfer-out")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_entrust_subaccount_list_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{"subAcct":"test-1"},{"subAcct":"test-2"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .sub_account()
        .get_entrust_subaccount_list()
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].sub_acct, "test-1");
    assert_eq!(rows[1].sub_acct, "test-2");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/users/entrust-subaccount-list"));
    assert!(req.is_signed());
}
