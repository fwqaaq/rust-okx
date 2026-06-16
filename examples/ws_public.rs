use std::time::Duration;

use rust_okx::api::market::Ticker;
use rust_okx::ws::channels::market;
use rust_okx::{OkxWs, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = OkxWs::public().connect().await?;
    let arg = market::tickers("BTC-USDT");
    ws.subscribe(std::slice::from_ref(&arg)).await?;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("timed out waiting for ticker push");
                break;
            }
            event = ws.next_event() => {
                match event? {
                    Some(WsEvent::Subscribed(arg)) => {
                        println!("subscribed: {}", arg.channel);
                    }
                    Some(WsEvent::Push(push)) if push.arg.channel == "tickers" => {
                        let rows: Vec<Ticker> = push.parse()?;
                        for row in rows {
                            println!("{} last={}", row.inst_id, row.last);
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
