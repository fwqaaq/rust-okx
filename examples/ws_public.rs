use std::env;
use std::time::Duration;

use rust_okx::ws::channels::market;
use rust_okx::ws::model::TickerUpdate;
use rust_okx::{OkxWs, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let inst_id = env::var("OKX_WS_INST_ID").unwrap_or_else(|_| "BTC-USDT".to_owned());
    let arg = market::tickers(inst_id);
    let mut ws = OkxWs::public().connect().await?;

    ws.subscribe(std::slice::from_ref(&arg)).await?;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("timed out waiting for ticker push");
                break;
            }
            event = ws.next_event() => match event? {
                Some(WsEvent::Subscribed(arg)) => {
                    println!("subscribed: {}", arg.channel);
                }
                Some(WsEvent::Push(push)) if push.arg.channel == "tickers" => {
                    let rows: Vec<TickerUpdate> = push.parse()?;
                    for row in rows {
                        println!(
                            "{} last={} bid={} ask={} ts={}",
                            row.inst_id, row.last, row.bid_px, row.ask_px, row.ts
                        );
                    }
                    break;
                }
                Some(WsEvent::Error { code, msg }) => {
                    return Err(format!("OKX WS error {code}: {msg}").into());
                }
                Some(WsEvent::Disconnected) | None => break,
                Some(_) => {}
            }
        }
    }

    ws.unsubscribe(&[arg]).await?;
    ws.close().await?;
    Ok(())
}
