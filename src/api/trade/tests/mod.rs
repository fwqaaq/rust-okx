use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod advanced;
mod algo;
mod orders;
mod queries;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}
