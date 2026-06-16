use std::env;
use std::time::Duration;

use rust_okx::{Arg, Credentials, OkxWs, WsEvent, ws::model::BalanceAndPositionUpdate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let mut ws = OkxWs::private(live_credentials()?).connect().await?;

    let args = Arg::new("balance_and_position");
    ws.subscribe(std::slice::from_ref(&args)).await?;

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("timed out waiting for private events");
                break;
            }
            event = ws.next_event() => {
                match event? {
                    Some(WsEvent::Login) => println!("logged in"),
                    Some(WsEvent::Subscribed(arg)) => println!("subscribed: {}", arg.channel),
                    Some(WsEvent::Push(push)) => {
                        if push.arg.channel == "balance_and_position" {
                            let row: Vec<BalanceAndPositionUpdate> = push.parse().unwrap();
                            for subscribed_object in row {
                                subscribed_object.bal_data.iter().for_each(|item| {
                                    println!("Crypto: {}, Balance: {}", item.ccy, item.cash_bal.as_str())
                                });
                            }
                        }
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
