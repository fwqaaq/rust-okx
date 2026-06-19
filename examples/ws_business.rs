use std::env;
use std::time::Duration;

use rust_okx::ws::channels::market;
use rust_okx::ws::model::CandleUpdate;
use rust_okx::{OkxWs, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let inst_id = env::var("OKX_WS_INST_ID").unwrap_or_else(|_| "BTC-USDT".to_owned());
    let channel = env::var("OKX_WS_CANDLE_CHANNEL").unwrap_or_else(|_| "candle1m".to_owned());
    let arg = market::candlesticks(channel, inst_id);
    let mut ws = OkxWs::business().connect().await?;

    ws.subscribe(std::slice::from_ref(&arg)).await?;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("timed out waiting for business candle push");
                break;
            }
            event = ws.next_event() => match event? {
                Some(WsEvent::Subscribed(arg)) => {
                    println!("subscribed: {}", arg.channel);
                }
                Some(WsEvent::Push(push)) if push.arg.channel.starts_with("candle") => {
                    let rows: Vec<CandleUpdate> = push.parse()?;
                    for row in rows {
                        println!(
                            "candle ts={} open={} high={} low={} close={} confirm={}",
                            row.ts, row.o, row.h, row.l, row.c, row.confirm
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
