use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod balances;
mod management;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}
