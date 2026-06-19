use std::env;
use std::time::Duration;

use rust_okx::ws::channels::account;
use rust_okx::ws::model::BalanceAndPositionUpdate;
use rust_okx::{Credentials, OkxWs, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let arg = account::balance_and_position();
    let mut ws = OkxWs::private(live_credentials()?).connect().await?;

    ws.subscribe(std::slice::from_ref(&arg)).await?;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("timed out waiting for private balance_and_position push");
                break;
            }
            event = ws.next_event() => match event? {
                Some(WsEvent::Login) => println!("logged in"),
                Some(WsEvent::Subscribed(arg)) => println!("subscribed: {}", arg.channel),
                Some(WsEvent::Push(push)) if push.arg.channel == "balance_and_position" => {
                    let rows: Vec<BalanceAndPositionUpdate> = push.parse()?;
                    for update in rows {
                        println!(
                            "event={} pTime={} balances={} positions={} trades={}",
                            update.event_type,
                            update.p_time,
                            update.bal_data.len(),
                            update.pos_data.len(),
                            update.trades.len()
                        );
                        for balance in update.bal_data {
                            println!("  balance ccy={} cashBal={}", balance.ccy, balance.cash_bal);
                        }
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

fn live_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    Ok(Credentials::new(
        env::var("OKX_API_KEY")?,
        env::var("OKX_API_SECRET")?,
        env::var("OKX_PASSPHRASE")?,
    ))
}
