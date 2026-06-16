use std::env;

use rust_okx::api::trade::PlaceOrderRequest;
use rust_okx::model::{OrderSide, OrderType, TradeMode};
use rust_okx::{Credentials, OkxClient};

const CONFIRM_VALUE: &str = "I_UNDERSTAND";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    if env::var("OKX_LIVE_TRADING_CONFIRM").as_deref() != Ok(CONFIRM_VALUE) {
        println!("set OKX_LIVE_TRADING_CONFIRM={CONFIRM_VALUE} to place a live order");
        return Ok(());
    }

    let client = OkxClient::builder()
        .credentials(live_credentials()?)
        .build();

    let inst_id = env::var("OKX_TRADE_INST_ID")?;
    let side = parse_side(&env::var("OKX_TRADE_SIDE")?)?;
    let size = env::var("OKX_TRADE_SIZE")?;
    let price = env::var("OKX_TRADE_PRICE")?;

    let request =
        PlaceOrderRequest::new(inst_id, TradeMode::Cash, side, OrderType::Limit, size).price(price);

    let placed = client.trade().place_order(&request).await?;
    for row in placed {
        println!(
            "ord_id={} s_code={} s_msg={}",
            row.ord_id, row.s_code, row.s_msg
        );
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

fn parse_side(side: &str) -> Result<OrderSide, Box<dyn std::error::Error>> {
    match side {
        "buy" => Ok(OrderSide::Buy),
        "sell" => Ok(OrderSide::Sell),
        other => Err(format!("OKX_TRADE_SIDE must be buy or sell, got {other}").into()),
    }
}
