use crate::common::live_client_or_skip;

#[tokio::test]
async fn sub_account_read_only_endpoints_parse() {
    let Some(_client) = live_client_or_skip("sub_account_read_only_endpoints_parse") else {
        return;
    };
    // TODO: add live assertions here once unit tests and response types are filled in.
    // Example pattern:
    //
    // let rows = _client.sub_account().get_subaccount_list().await.expect("users/subaccount/list");
    // assert!(!rows.is_empty());
}
