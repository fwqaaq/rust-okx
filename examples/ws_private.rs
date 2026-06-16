use std::env;
use std::time::Duration;

use rust_okx::{Arg, Credentials, OkxWs, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let mut ws = OkxWs::private(live_credentials()?).connect().await?;

    let args = [Arg::new("account"), Arg::new("orders").inst_type("ANY")];
    ws.subscribe(&args).await?;

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
                    Some(WsEvent::Push(push)) => println!("push: {}", push.arg.channel),
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
