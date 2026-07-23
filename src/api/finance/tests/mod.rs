use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

mod dual_investment;
mod eth_staking;
mod flexible_loan;
mod savings;
mod sol_staking;
mod stable_rewards;
mod staking_defi;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}
