use std::env;

use rust_okx::{
    Credentials, OkxClient, OkxRegion,
    api::{account::BalanceRequest, funding::CurrencyRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let client = authenticated_client()?;

    let account_config = client.account().get_account_config().await?;
    println!("account config rows: {}", account_config.len());

    let trading_balances = client
        .account()
        .get_balance(BalanceRequest::default())
        .await?;
    println!("trading account balance rows: {}", trading_balances.len());

    let funding_balances = client
        .funding()
        .get_balances(&CurrencyRequest::default())
        .await?;
    println!("funding account balance rows: {}", funding_balances.len());

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
        other => Err(format!("OKX_REGION must be global, us, au, eea, or eu; got {other}").into()),
    }
}

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}
