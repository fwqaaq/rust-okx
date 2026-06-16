use std::time::Duration;

use rust_okx::api::market::Candle;
use rust_okx::{Arg, OkxRegion, OkxWs, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = OkxWs::business(OkxRegion::Global).connect().await?;
    let arg = Arg::new("candle1m").inst_id("BTC-USDT");
    ws.subscribe(std::slice::from_ref(&arg)).await?;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("timed out waiting for business candle push");
                break;
            }
            event = ws.next_event() => {
                match event? {
                    Some(WsEvent::Subscribed(arg)) => {
                        println!("subscribed: {}", arg.channel);
                    }
                    Some(WsEvent::Push(push)) if push.arg.channel == "candle1m" => {
                        let rows: Vec<Candle> = push.parse()?;
                        for row in rows {
                            println!("candle open={} high={} low={} close={}", row.open, row.high, row.low, row.close);
                        }
                        break;
                    }
                    Some(WsEvent::Error { code, msg }) => {
                        return Err(format!("OKX WS error {code}: {msg}").into());
                    }
                    Some(_) => {}
                    None => break,
                }
            }
        }
    }

    ws.unsubscribe(&[arg]).await?;
    ws.close().await?;
    Ok(())
}
