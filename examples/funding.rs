use std::env;

use rust_okx::{Credentials, OkxClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let client = OkxClient::builder()
        .credentials(live_credentials()?)
        .build();

    let currency = env::var("OKX_EXAMPLE_FUNDING_CCY").unwrap_or_else(|_| "USDT".to_owned());

    let currencies = client.funding().get_currencies(Some(&currency)).await?;
    println!("{currency} currency rows: {}", currencies.len());

    let balances = client.funding().get_balances(Some(&currency)).await?;
    println!("{currency} funding balance rows: {}", balances.len());

    let addresses = client.funding().get_deposit_address(&currency).await?;
    println!("{currency} deposit address rows: {}", addresses.len());

    let valuation = client.funding().get_asset_valuation(Some("USD")).await?;
    if let Some(row) = valuation.first() {
        println!("asset valuation in USD: {}", row.total_bal);
    } else {
        println!("asset valuation returned no rows");
    }

    Ok(())
}

fn live_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    Ok(Credentials::new(
        env::var("OKX_API_KEY")?,
        env::var("OKX_API_SECRET")?,
        env::var("OKX_PASSPHRASE")?,
    ))
}
