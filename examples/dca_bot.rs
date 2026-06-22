/// BTC-USDT 200-week SMA DCA bot.
///
/// Strategy:
///   - Price drops below 200W SMA → buy $100
///   - Every subsequent 3% drop (from last buy price) → buy $100
///   - Price rises more than 10% above 200W SMA → sell oldest position (FIFO)
///
/// Set OKX_DEMO=1 to run against OKX demo trading.
///
/// Required env vars: OKX_API_KEY, OKX_SECRET_KEY, OKX_PASSPHRASE
use std::collections::VecDeque;
use std::time::Duration;

use rust_okx::api::market::CandlesticksRequest;
use rust_okx::api::trade::PlaceOrderRequest;
use rust_okx::model::{OrderSide, OrderType, TradeMode};
use rust_okx::ws::channels::market;
use rust_okx::ws::model::TickerUpdate;
use rust_okx::{Credentials, OkxClient, OkxWs, WsEvent};

const INST_ID: &str = "BTC-USDT";
const BUY_USDT: &str = "100";
const DCA_DROP_PCT: f64 = 0.03;
const SELL_ABOVE_MA_PCT: f64 = 0.10;
const MA_WEEKS: usize = 200;
const MA_REFRESH_SECS: u64 = 6 * 3600;

struct Position {
    btc_amount: f64,
}

struct BotState {
    ma_200w: f64,
    below_ma: bool,
    last_buy_price: Option<f64>,
    above_sell_threshold: bool,
    open_positions: VecDeque<Position>,
}

impl BotState {
    fn new(ma_200w: f64) -> Self {
        Self {
            ma_200w,
            below_ma: false,
            last_buy_price: None,
            above_sell_threshold: false,
            open_positions: VecDeque::new(),
        }
    }
}

async fn fetch_ma(client: &OkxClient) -> Result<f64, Box<dyn std::error::Error>> {
    let batch1 = client
        .market()
        .get_history_candlesticks(&CandlesticksRequest::new(INST_ID).bar("1W").limit(100))
        .await?;

    if batch1.is_empty() {
        return Err("no weekly candle data returned".into());
    }

    // batch1 is newest-first; oldest entry is last — use its ts to fetch the prior 100 weeks
    let oldest_ts = batch1.last().unwrap().ts.as_str().to_owned();

    let batch2 = client
        .market()
        .get_history_candlesticks(
            &CandlesticksRequest::new(INST_ID)
                .bar("1W")
                .limit(100)
                .after(oldest_ts),
        )
        .await?;

    let closes: Vec<f64> = batch1
        .iter()
        .chain(batch2.iter())
        .filter_map(|c| c.close.as_str().parse::<f64>().ok())
        .take(MA_WEEKS)
        .collect();

    let n = closes.len();
    if n == 0 {
        return Err("failed to parse candle close prices".into());
    }

    let ma = closes.iter().sum::<f64>() / n as f64;
    println!("[MA] 200-week SMA = {ma:.2} (computed from {n} weekly closes)");
    Ok(ma)
}

async fn execute_buy(
    client: &OkxClient,
    price: f64,
    state: &mut BotState,
) -> Result<(), Box<dyn std::error::Error>> {
    let req = PlaceOrderRequest::new(
        INST_ID,
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Market,
        BUY_USDT,
    )
    .target_ccy("quote_ccy");

    let results = client.trade().place_order(&req).await?;
    for r in &results {
        let btc_amount = 100.0 / price;
        println!(
            "[BUY]  price={price:.2}  est_btc={btc_amount:.8}  ordId={}  sCode={}",
            r.ord_id, r.s_code
        );
        if r.s_code == "0" {
            state.open_positions.push_back(Position { btc_amount });
        }
    }
    Ok(())
}

async fn execute_sell(
    client: &OkxClient,
    pos: Position,
    price: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let btc_str = format!("{:.8}", pos.btc_amount);
    let req = PlaceOrderRequest::new(
        INST_ID,
        TradeMode::Cash,
        OrderSide::Sell,
        OrderType::Market,
        btc_str.clone(),
    );

    let results = client.trade().place_order(&req).await?;
    for r in &results {
        println!(
            "[SELL] price={price:.2}  btc={btc_str}  est_value={:.2}  ordId={}  sCode={}",
            pos.btc_amount * price,
            r.ord_id,
            r.s_code
        );
    }
    Ok(())
}

async fn on_price(
    price: f64,
    state: &mut BotState,
    client: &OkxClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let ma = state.ma_200w;

    // Buy logic
    if price < ma {
        if !state.below_ma {
            // First tick below the 200W MA
            println!("[SIGNAL] Price {price:.2} crossed below 200W MA {ma:.2}");
            state.below_ma = true;
            state.last_buy_price = Some(price);
            execute_buy(client, price, state).await?;
        } else if let Some(last) = state.last_buy_price {
            let drop = (last - price) / last;
            if drop >= DCA_DROP_PCT {
                println!(
                    "[SIGNAL] Price {price:.2} dropped {:.1}% from last buy {last:.2}",
                    drop * 100.0
                );
                state.last_buy_price = Some(price);
                execute_buy(client, price, state).await?;
            }
        }
    } else if state.below_ma {
        // Price recovered back above MA
        println!("[INFO] Price {price:.2} recovered above 200W MA {ma:.2}");
        state.below_ma = false;
    }

    // Sell logic (independent of buy logic)
    let sell_threshold = ma * (1.0 + SELL_ABOVE_MA_PCT);
    if price > sell_threshold {
        if !state.above_sell_threshold {
            state.above_sell_threshold = true;
            if let Some(pos) = state.open_positions.pop_front() {
                println!(
                    "[SIGNAL] Price {price:.2} >10% above MA {ma:.2} ({sell_threshold:.2}), selling oldest position"
                );
                execute_sell(client, pos, price).await?;
            } else {
                println!("[INFO] Sell condition met but no open positions");
            }
        }
    } else {
        state.above_sell_threshold = false;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let api_key = std::env::var("OKX_API_KEY").map_err(|_| "OKX_API_KEY not set")?;
    let secret_key = std::env::var("OKX_API_SECRET").map_err(|_| "OKX_SECRET_KEY not set")?;
    let passphrase = std::env::var("OKX_PASSPHRASE").map_err(|_| "OKX_PASSPHRASE not set")?;
    let demo = std::env::var("OKX_DEMO")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let creds = Credentials::new(api_key, secret_key, passphrase);
    let rest = OkxClient::builder()
        .credentials(creds)
        .demo_trading(demo)
        .build();

    println!("[BOT] Fetching 200-week SMA...");
    let ma = fetch_ma(&rest).await?;
    let mut state = BotState::new(ma);

    // Public WebSocket for real-time price feed (no auth required)
    let mut ws = OkxWs::public().demo_trading(demo).connect().await?;
    ws.subscribe(&[market::tickers(INST_ID)]).await?;
    println!("[BOT] Subscribed to {INST_ID} ticker  demo={demo}");

    // Periodic MA refresh — skip the first immediate tick
    let mut refresh = tokio::time::interval(Duration::from_secs(MA_REFRESH_SECS));
    refresh.tick().await;

    loop {
        tokio::select! {
            _ = refresh.tick() => {
                match fetch_ma(&rest).await {
                    Ok(new_ma) => state.ma_200w = new_ma,
                    Err(e) => eprintln!("[ERROR] MA refresh failed: {e}"),
                }
            }
            event = ws.next_event() => {
                match event? {
                    Some(WsEvent::Push(push)) if push.arg.channel == "tickers" => {
                        let rows: Vec<TickerUpdate> = push.parse()?;
                        if let Some(row) = rows.into_iter().next() {
                            if let Ok(price) = row.last.as_str().parse::<f64>() {
                                if let Err(e) = on_price(price, &mut state, &rest).await {
                                    eprintln!("[ERROR] {e}");
                                }
                            }
                        }
                    }
                    Some(WsEvent::Subscribed(arg)) => {
                        println!("[WS] Subscribed: {}", arg.channel);
                    }
                    Some(WsEvent::Error { code, msg }) => {
                        eprintln!("[WS ERROR] {code}: {msg}");
                    }
                    Some(WsEvent::Disconnected) | None => {
                        eprintln!("[WS] Disconnected");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
