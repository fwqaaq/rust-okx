use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod eth_staking;
mod dual_investment;
mod flexible_loan;
mod savings;
mod sol_staking;
mod staking_defi;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}
