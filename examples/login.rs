use std::env;

use rust_okx::{Credentials, OkxClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let client = OkxClient::builder()
        .credentials(live_credentials()?)
        .build();

    let config = client.account().get_account_config().await?;
    println!("account config rows: {}", config.len());

    let balances = client.funding().get_balances(None).await?;
    println!("funding balances: {}", balances.len());

    Ok(())
}

fn live_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    Ok(Credentials::new(
        env::var("OKX_API_KEY")?,
        env::var("OKX_API_SECRET")?,
        env::var("OKX_PASSPHRASE")?,
    ))
}
