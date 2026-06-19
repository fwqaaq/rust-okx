use std::env;

use rust_okx::api::trade::PlaceOrderRequest;
use rust_okx::model::{OrderSide, OrderType, TradeMode};
use rust_okx::{Credentials, OkxClient, OkxRegion};

const CONFIRM_VALUE: &str = "I_UNDERSTAND";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    if env::var("OKX_LIVE_TRADING_CONFIRM").as_deref() != Ok(CONFIRM_VALUE) {
        println!("set OKX_LIVE_TRADING_CONFIRM={CONFIRM_VALUE} to place an order");
        println!("set OKX_DEMO_TRADING=1 to send it to OKX demo trading");
        return Ok(());
    }

    let client = authenticated_client()?;
    let request = PlaceOrderRequest::new(
        env::var("OKX_TRADE_INST_ID")?,
        parse_trade_mode(&env::var("OKX_TRADE_MODE").unwrap_or_else(|_| "cash".to_owned()))?,
        parse_side(&env::var("OKX_TRADE_SIDE")?)?,
        parse_order_type(&env::var("OKX_TRADE_ORDER_TYPE").unwrap_or_else(|_| "limit".to_owned()))?,
        env::var("OKX_TRADE_SIZE")?,
    )
    .price(env::var("OKX_TRADE_PRICE")?)
    .client_order_id(
        env::var("OKX_TRADE_CLIENT_ORDER_ID").unwrap_or_else(|_| "rust-okx-example".to_owned()),
    );

    let placed = client.trade().place_order(&request).await?;
    for row in placed {
        println!(
            "ordId={} clOrdId={} sCode={} sMsg={}",
            row.ord_id, row.cl_ord_id, row.s_code, row.s_msg
        );
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
        other => Err(format!("OKX_REGION must be global, us, au, eea, or eu; got {other}").into()),
    }
}

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

fn parse_side(side: &str) -> Result<OrderSide, Box<dyn std::error::Error>> {
    match side {
        "buy" => Ok(OrderSide::Buy),
        "sell" => Ok(OrderSide::Sell),
        other => Err(format!("OKX_TRADE_SIDE must be buy or sell, got {other}").into()),
    }
}

fn parse_trade_mode(mode: &str) -> Result<TradeMode, Box<dyn std::error::Error>> {
    match mode {
        "cash" => Ok(TradeMode::Cash),
        "cross" => Ok(TradeMode::Cross),
        "isolated" => Ok(TradeMode::Isolated),
        "spot_isolated" => Ok(TradeMode::SpotIsolated),
        other => Err(format!(
            "OKX_TRADE_MODE must be cash, cross, isolated, or spot_isolated; got {other}"
        )
        .into()),
    }
}

fn parse_order_type(order_type: &str) -> Result<OrderType, Box<dyn std::error::Error>> {
    match order_type {
        "limit" => Ok(OrderType::Limit),
        "market" => Ok(OrderType::Market),
        "post_only" => Ok(OrderType::PostOnly),
        "fok" => Ok(OrderType::Fok),
        "ioc" => Ok(OrderType::Ioc),
        "optimal_limit_ioc" => Ok(OrderType::OptimalLimitIoc),
        other => Err(format!("unsupported OKX_TRADE_ORDER_TYPE: {other}").into()),
    }
}
