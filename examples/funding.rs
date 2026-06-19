use std::env;

use rust_okx::{Credentials, OkxClient, OkxRegion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let client = authenticated_client()?;
    let currency = env::var("OKX_EXAMPLE_FUNDING_CCY").unwrap_or_else(|_| "USDT".to_owned());

    let currencies = client.funding().get_currencies(Some(&currency)).await?;
    println!("{currency} currency metadata rows: {}", currencies.len());

    let balances = client.funding().get_balances(Some(&currency)).await?;
    for balance in &balances {
        println!(
            "funding balance: ccy={} bal={} availBal={}",
            balance.ccy, balance.bal, balance.avail_bal
        );
    }

    let deposit_addresses = client.funding().get_deposit_address(&currency).await?;
    println!("{currency} deposit address rows: {}", deposit_addresses.len());

    let valuation_ccy = env::var("OKX_EXAMPLE_VALUATION_CCY").unwrap_or_else(|_| "USD".to_owned());
    let valuation = client
        .funding()
        .get_asset_valuation(Some(&valuation_ccy))
        .await?;
    if let Some(row) = valuation.first() {
        println!("asset valuation in {valuation_ccy}: {}", row.total_bal);
    } else {
        println!("asset valuation returned no rows");
    }

    Ok(())
}

fn authenticated_client() -> Result<OkxClient, Box<dyn std::error::Error>> {
    Ok(OkxClient::builder()
        .region(example_region()?)
        .credentials(live_credentials()?)
        .demo_trading(env_flag("OKX_DEMO_TRADING"))
        .build())
}

fn live_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    Ok(Credentials::new(
        env::var("OKX_API_KEY")?,
        env::var("OKX_API_SECRET")?,
        env::var("OKX_PASSPHRASE")?,
    ))
}

fn example_region() -> Result<OkxRegion, Box<dyn std::error::Error>> {
    match env::var("OKX_REGION")
        .unwrap_or_else(|_| "global".to_owned())
        .to_ascii_lowercase()
        .as_str()
    {
        "global" => Ok(OkxRegion::Global),
        "us" | "au" => Ok(OkxRegion::Us),
        "eea" | "eu" => Ok(OkxRegion::Eea),
        other => {
            Err(format!("OKX_REGION must be global, us, au, eea, or eu; got {other}").into())
        }
    }
}

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}
