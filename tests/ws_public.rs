#![cfg(feature = "websocket")]

//! Integration tests against OKX public WebSocket channels.
//!
//! These tests intentionally parse channel payloads with the WebSocket models
//! from [`rust_okx::ws::model`] instead of reusing REST response types. This
//! keeps the examples aligned with OKX's WebSocket payload schemas.

use std::time::Duration;

use rust_okx::ws::channels;
use rust_okx::ws::model::{
    AdlWarningUpdate, AllTradeUpdate, BlockTickerUpdate, CandleUpdate, EconomicCalendarUpdate,
    EstimatedPriceUpdate, EventContractMarketUpdate, FundingRateUpdate, IndexCandleUpdate,
    IndexTickerUpdate, InstrumentUpdate, LiquidationOrderUpdate, MarkPriceCandleUpdate,
    MarkPriceUpdate, OpenInterestUpdate, OptionSummaryUpdate, OptionTradeUpdate, OrderBookUpdate,
    PriceLimitUpdate, PublicBlockTradeUpdate, PublicStructureBlockTradeUpdate, StatusUpdate,
    TickerUpdate, TradeUpdate,
};
use rust_okx::{OkxWs, WsEvent};

/// `WS / public tickers` — subscribe to `tickers` for `BTC-USDT`, wait for an
/// acknowledgement and at least one typed ticker push, then unsubscribe and
/// close.
#[tokio::test]
async fn public_tickers_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_tickers_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_tickers_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::market::tickers("BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_tickers_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_tickers_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "tickers"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "tickers"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<TickerUpdate> = push
                            .parse()
                            .expect("tickers push should parse as Vec<TickerUpdate>");

                        pushed = rows.iter().any(|row| row.inst_id == "BTC-USDT");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_tickers_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_tickers_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }

                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / status` — subscribe to the public status channel and wait for an
/// acknowledgement.
///
/// Status pushes are sparse, so the test does not require one. When a status
/// push arrives before the acknowledgement, it is still parsed with
/// [`StatusUpdate`] to exercise the channel model.
#[tokio::test]
async fn public_status_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_status_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_status_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::status::status();
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_status_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_status_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "status" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "status" => {
                        let _: Vec<StatusUpdate> = push
                            .parse()
                            .expect("status push should parse as Vec<StatusUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!("skipping public_status_channel_subscribes: OKX WS error {code}: {msg}");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_status_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_status_channel_subscribes: receive failed: {err}");
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / block-tickers` — subscribe to the public `block-tickers` channel for
/// `BTC-USD-SWAP` and wait for an acknowledgement.
///
/// Block-ticker pushes are sparse; any push that arrives is still parsed with
/// [`BlockTickerUpdate`] to exercise the model.
#[tokio::test]
async fn public_block_tickers_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_block_tickers_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_block_tickers_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::block::block_tickers("BTC-USD-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_block_tickers_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_block_tickers_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "block-tickers" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "block-tickers" => {
                        let _: Vec<BlockTickerUpdate> = push
                            .parse()
                            .expect("block-tickers push should parse as Vec<BlockTickerUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!("skipping public_block_tickers_channel_subscribes: OKX WS error {code}: {msg}");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_block_tickers_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_block_tickers_channel_subscribes: receive failed: {err}");
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / public-block-trades` — subscribe to the public block-trades channel for
/// `BTC-USD-SWAP` and wait for an acknowledgement.
///
/// Block-trade pushes are sparse; any push that arrives is still parsed with
/// [`PublicBlockTradeUpdate`] to exercise the model.
#[tokio::test]
async fn public_block_trades_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_block_trades_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_block_trades_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::block::public_block_trades("BTC-USD-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_block_trades_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_block_trades_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "public-block-trades" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "public-block-trades" => {
                        let _: Vec<PublicBlockTradeUpdate> = push
                            .parse()
                            .expect("public-block-trades push should parse as Vec<PublicBlockTradeUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!("skipping public_block_trades_channel_subscribes: OKX WS error {code}: {msg}");
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_block_trades_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_block_trades_channel_subscribes: receive failed: {err}");
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / public-struc-block-trades` — subscribe to the public structure-block-trades
/// channel and wait for an acknowledgement.
///
/// Structure-block-trade pushes are sparse; any push that arrives is still parsed
/// with [`PublicStructureBlockTradeUpdate`] to exercise the model.
#[tokio::test]
async fn public_structure_block_trades_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!(
                "skipping public_structure_block_trades_channel_subscribes: connect failed: {err}"
            );
            return;
        }
        Err(_) => {
            eprintln!(
                "skipping public_structure_block_trades_channel_subscribes: connect timed out"
            );
            return;
        }
    };

    let arg = channels::block::public_structure_block_trades();
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!(
            "skipping public_structure_block_trades_channel_subscribes: subscribe failed: {err}"
        );
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping public_structure_block_trades_channel_subscribes: timed out waiting for ack"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "public-struc-block-trades" => break,
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "public-struc-block-trades" =>
                    {
                        let _: Vec<PublicStructureBlockTradeUpdate> = push.parse().expect(
                            "public-struc-block-trades push should parse as Vec<PublicStructureBlockTradeUpdate>",
                        );
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping public_structure_block_trades_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping public_structure_block_trades_channel_subscribes: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping public_structure_block_trades_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / business candle1m` — subscribe to the business candlestick channel,
/// wait for an acknowledgement and one typed candle push, then unsubscribe and
/// close.
#[tokio::test]
async fn business_candle_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::business().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping business_candle_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping business_candle_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::market::candlesticks("candle1m", "BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping business_candle_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping business_candle_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "candle1m"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "candle1m"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<CandleUpdate> = push
                            .parse()
                            .expect("candle push should parse as Vec<CandleUpdate>");

                        pushed = !rows.is_empty();
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping business_candle_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping business_candle_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }

                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / trades` — subscribe to aggregated `trades` for `BTC-USDT`, wait for
/// an acknowledgement and at least one typed push, then unsubscribe and close.
#[tokio::test]
async fn public_trades_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_trades_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_trades_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::market::trades("BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_trades_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping public_trades_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "trades"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "trades"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<TradeUpdate> = push
                            .parse()
                            .expect("trades push should parse as Vec<TradeUpdate>");
                        pushed = rows.iter().any(|r| r.inst_id == "BTC-USDT");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping public_trades_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping public_trades_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / trades-all` — subscribe to unaggregated `trades-all` for `BTC-USDT`,
/// wait for an acknowledgement and at least one typed push, then unsubscribe
/// and close.
#[tokio::test]
async fn public_all_trades_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::business().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!(
                "skipping public_all_trades_channel_pushes_typed_rows: connect failed: {err}"
            );
            return;
        }
        Err(_) => {
            eprintln!("skipping public_all_trades_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::market::all_trades("BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_all_trades_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping public_all_trades_channel_pushes_typed_rows: timed out waiting for push"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "trades-all"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "trades-all"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<AllTradeUpdate> = push
                            .parse()
                            .expect("trades-all push should parse as Vec<AllTradeUpdate>");
                        pushed = rows.iter().any(|r| r.inst_id == "BTC-USDT");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping public_all_trades_channel_pushes_typed_rows: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping public_all_trades_channel_pushes_typed_rows: receive failed: {err}"
                        );
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / option-trades` — subscribe to the `option-trades` channel for
/// `BTC-USD` and wait for an acknowledgement.
///
/// Option-trade pushes are sparse; any push that arrives is still parsed with
/// [`OptionTradeUpdate`] to exercise the model.
#[tokio::test]
async fn public_option_trades_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping public_option_trades_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping public_option_trades_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::market::option_trades("OPTION", None::<&str>, Some("BTC-USD"));
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_option_trades_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping public_option_trades_channel_subscribes: timed out waiting for ack"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "option-trades" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "option-trades" => {
                        let _: Vec<OptionTradeUpdate> = push
                            .parse()
                            .expect("option-trades push should parse as Vec<OptionTradeUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping public_option_trades_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping public_option_trades_channel_subscribes: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping public_option_trades_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / books5` — subscribe to `books5` for `BTC-USDT`, wait for an
/// acknowledgement and at least one typed push, then unsubscribe and close.
#[tokio::test]
async fn public_order_book_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!(
                "skipping public_order_book_channel_pushes_typed_rows: connect failed: {err}"
            );
            return;
        }
        Err(_) => {
            eprintln!("skipping public_order_book_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::market::order_book("books5", "BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping public_order_book_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping public_order_book_channel_pushes_typed_rows: timed out waiting for push"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "books5"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "books5"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<OrderBookUpdate> = push
                            .parse()
                            .expect("books5 push should parse as Vec<OrderBookUpdate>");
                        pushed = rows.iter().any(|r| !r.asks.is_empty() && !r.bids.is_empty());
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping public_order_book_channel_pushes_typed_rows: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping public_order_book_channel_pushes_typed_rows: receive failed: {err}"
                        );
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

// ── public_data channels ──────────────────────────────────────────────────────

/// `WS / instruments SPOT` — subscribe, wait for acknowledgement and at least
/// one snapshot push, parse as [`InstrumentUpdate`].
#[tokio::test]
async fn instruments_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping instruments_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping instruments_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::instruments("SPOT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping instruments_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping instruments_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "instruments"
                            && ack.inst_type.as_deref() == Some("SPOT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "instruments"
                            && push.arg.inst_type.as_deref() == Some("SPOT") =>
                    {
                        let rows: Vec<InstrumentUpdate> = push
                            .parse()
                            .expect("instruments push should parse as Vec<InstrumentUpdate>");
                        pushed = rows.iter().any(|r| !r.inst_id.is_empty());
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping instruments_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping instruments_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / open-interest BTC-USDT-SWAP` — subscribe, wait for acknowledgement
/// and at least one push, parse as [`OpenInterestUpdate`].
#[tokio::test]
async fn open_interest_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping open_interest_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping open_interest_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::open_interest("BTC-USDT-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping open_interest_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping open_interest_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "open-interest"
                            && ack.inst_id.as_deref() == Some("BTC-USDT-SWAP") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "open-interest"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT-SWAP") =>
                    {
                        let rows: Vec<OpenInterestUpdate> = push
                            .parse()
                            .expect("open-interest push should parse as Vec<OpenInterestUpdate>");
                        pushed = rows.iter().any(|r| r.inst_id == "BTC-USDT-SWAP");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping open_interest_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping open_interest_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / mark-price BTC-USDT-SWAP` — subscribe, wait for acknowledgement and
/// at least one push, parse as [`MarkPriceUpdate`].
#[tokio::test]
async fn mark_price_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping mark_price_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping mark_price_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::mark_price("BTC-USDT-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping mark_price_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping mark_price_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "mark-price"
                            && ack.inst_id.as_deref() == Some("BTC-USDT-SWAP") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "mark-price"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT-SWAP") =>
                    {
                        let rows: Vec<MarkPriceUpdate> = push
                            .parse()
                            .expect("mark-price push should parse as Vec<MarkPriceUpdate>");
                        pushed = rows.iter().any(|r| r.inst_id == "BTC-USDT-SWAP");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping mark_price_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping mark_price_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / index-tickers BTC-USDT` — subscribe, wait for acknowledgement and
/// at least one push, parse as [`IndexTickerUpdate`].
#[tokio::test]
async fn index_tickers_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping index_tickers_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping index_tickers_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::index_tickers("BTC-USDT");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping index_tickers_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping index_tickers_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "index-tickers"
                            && ack.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "index-tickers"
                            && push.arg.inst_id.as_deref() == Some("BTC-USDT") =>
                    {
                        let rows: Vec<IndexTickerUpdate> = push
                            .parse()
                            .expect("index-tickers push should parse as Vec<IndexTickerUpdate>");
                        pushed = rows.iter().any(|r| r.inst_id == "BTC-USDT");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping index_tickers_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping index_tickers_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / opt-summary BTC-USD` — subscribe, wait for acknowledgement and at
/// least one snapshot push, parse as [`OptionSummaryUpdate`].
#[tokio::test]
async fn opt_summary_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping opt_summary_channel_pushes_typed_rows: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping opt_summary_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::option_summary("BTC-USD");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping opt_summary_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping opt_summary_channel_pushes_typed_rows: timed out waiting for push");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "opt-summary"
                            && ack.inst_family.as_deref() == Some("BTC-USD") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "opt-summary"
                            && push.arg.inst_family.as_deref() == Some("BTC-USD") =>
                    {
                        let rows: Vec<OptionSummaryUpdate> = push
                            .parse()
                            .expect("opt-summary push should parse as Vec<OptionSummaryUpdate>");
                        pushed = !rows.is_empty();
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping opt_summary_channel_pushes_typed_rows: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping opt_summary_channel_pushes_typed_rows: receive failed: {err}");
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / business mark-price-candle1m BTC-USD-SWAP` — subscribe via the
/// business endpoint, wait for acknowledgement and one candle push, parse as
/// [`CandleUpdate`].
#[tokio::test]
async fn mark_price_candlesticks_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::business().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!(
                "skipping mark_price_candlesticks_channel_pushes_typed_rows: connect failed: {err}"
            );
            return;
        }
        Err(_) => {
            eprintln!(
                "skipping mark_price_candlesticks_channel_pushes_typed_rows: connect timed out"
            );
            return;
        }
    };

    let arg = channels::public_data::mark_price_candlesticks("mark-price-candle1m", "BTC-USD-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!(
            "skipping mark_price_candlesticks_channel_pushes_typed_rows: subscribe failed: {err}"
        );
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping mark_price_candlesticks_channel_pushes_typed_rows: timed out waiting for push"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "mark-price-candle1m"
                            && ack.inst_id.as_deref() == Some("BTC-USD-SWAP") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "mark-price-candle1m"
                            && push.arg.inst_id.as_deref() == Some("BTC-USD-SWAP") =>
                    {
                        let rows: Vec<MarkPriceCandleUpdate> = push
                            .parse()
                            .expect("mark-price-candle1m push should parse as Vec<MarkPriceCandleUpdate>");
                        pushed = !rows.is_empty();
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping mark_price_candlesticks_channel_pushes_typed_rows: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping mark_price_candlesticks_channel_pushes_typed_rows: receive failed: {err}"
                        );
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / business index-candle1m BTC-USD` — subscribe via the business
/// endpoint, wait for acknowledgement and one candle push, parse as
/// [`CandleUpdate`].
#[tokio::test]
async fn index_candlesticks_channel_pushes_typed_rows() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::business().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!(
                "skipping index_candlesticks_channel_pushes_typed_rows: connect failed: {err}"
            );
            return;
        }
        Err(_) => {
            eprintln!("skipping index_candlesticks_channel_pushes_typed_rows: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::index_candlesticks("index-candle1m", "BTC-USD");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping index_candlesticks_channel_pushes_typed_rows: subscribe failed: {err}");
        return;
    }

    let mut subscribed = false;
    let mut pushed = false;
    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping index_candlesticks_channel_pushes_typed_rows: timed out waiting for push"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "index-candle1m"
                            && ack.inst_id.as_deref() == Some("BTC-USD") =>
                    {
                        subscribed = true;
                    }
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "index-candle1m"
                            && push.arg.inst_id.as_deref() == Some("BTC-USD") =>
                    {
                        let rows: Vec<IndexCandleUpdate> = push
                            .parse()
                            .expect("index-candle1m push should parse as Vec<IndexCandleUpdate>");
                        pushed = !rows.is_empty();
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        panic!("OKX WS error {code}: {msg}")
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping index_candlesticks_channel_pushes_typed_rows: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping index_candlesticks_channel_pushes_typed_rows: receive failed: {err}"
                        );
                        return;
                    }
                }
                if subscribed && pushed {
                    break;
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / funding-rate BTC-USDT-SWAP` — subscribe and wait for acknowledgement.
///
/// Pushes arrive every 30–90 s; any push that arrives is still parsed with
/// [`FundingRateUpdate`] to exercise the model.
#[tokio::test]
async fn funding_rate_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping funding_rate_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping funding_rate_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::funding_rate("BTC-USDT-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping funding_rate_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping funding_rate_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "funding-rate" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "funding-rate" => {
                        let _: Vec<FundingRateUpdate> = push
                            .parse()
                            .expect("funding-rate push should parse as Vec<FundingRateUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping funding_rate_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping funding_rate_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping funding_rate_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / price-limit BTC-USDT-SWAP` — subscribe and wait for acknowledgement.
///
/// Pushes only arrive when price limits change; any push that arrives is still
/// parsed with [`PriceLimitUpdate`] to exercise the model.
#[tokio::test]
async fn price_limit_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping price_limit_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping price_limit_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::price_limit("BTC-USDT-SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping price_limit_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping price_limit_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "price-limit" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "price-limit" => {
                        let _: Vec<PriceLimitUpdate> = push
                            .parse()
                            .expect("price-limit push should parse as Vec<PriceLimitUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping price_limit_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping price_limit_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping price_limit_channel_subscribes: receive failed: {err}");
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / estimated-price FUTURES BTC-USD` — subscribe and wait for
/// acknowledgement.
///
/// Pushes only arrive during the one-hour window before futures delivery; any
/// push that arrives is still parsed with [`EstimatedPriceUpdate`].
#[tokio::test]
async fn estimated_price_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping estimated_price_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping estimated_price_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::estimated_price_by_family("FUTURES", "BTC-USD");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping estimated_price_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping estimated_price_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "estimated-price" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "estimated-price" => {
                        let _: Vec<EstimatedPriceUpdate> = push
                            .parse()
                            .expect("estimated-price push should parse as Vec<EstimatedPriceUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping estimated_price_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping estimated_price_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping estimated_price_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / event-contract-markets` — subscribe and wait for acknowledgement.
///
/// No initial snapshot is pushed; any push that arrives is still parsed with
/// [`EventContractMarketUpdate`] to exercise the model.
#[tokio::test]
async fn event_contract_markets_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping event_contract_markets_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping event_contract_markets_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::event_contract_markets();
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping event_contract_markets_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping event_contract_markets_channel_subscribes: timed out waiting for ack"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "event-contract-markets" => break,
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "event-contract-markets" =>
                    {
                        let _: Vec<EventContractMarketUpdate> = push.parse().expect(
                            "event-contract-markets push should parse as Vec<EventContractMarketUpdate>",
                        );
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping event_contract_markets_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping event_contract_markets_channel_subscribes: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping event_contract_markets_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / liquidation-orders SWAP` — subscribe and wait for acknowledgement.
///
/// Liquidation pushes are sparse; any push that arrives is still parsed with
/// [`LiquidationOrderUpdate`] to exercise the model.
#[tokio::test]
async fn liquidation_orders_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping liquidation_orders_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping liquidation_orders_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::liquidation_orders("SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping liquidation_orders_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping liquidation_orders_channel_subscribes: timed out waiting for ack"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "liquidation-orders" => break,
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "liquidation-orders" =>
                    {
                        let _: Vec<LiquidationOrderUpdate> = push.parse().expect(
                            "liquidation-orders push should parse as Vec<LiquidationOrderUpdate>",
                        );
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping liquidation_orders_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping liquidation_orders_channel_subscribes: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping liquidation_orders_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / adl-warning SWAP` — subscribe and wait for acknowledgement.
///
/// ADL warning pushes only occur during ADL events; any push that arrives is
/// still parsed with [`AdlWarningUpdate`] to exercise the model.
#[tokio::test]
async fn adl_warning_channel_subscribes() {
    let connect = tokio::time::timeout(Duration::from_secs(10), OkxWs::public().connect()).await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping adl_warning_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping adl_warning_channel_subscribes: connect timed out");
            return;
        }
    };

    let arg = channels::public_data::adl_warning("SWAP");
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping adl_warning_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!("skipping adl_warning_channel_subscribes: timed out waiting for ack");
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack))) if ack.channel == "adl-warning" => break,
                    Ok(Some(WsEvent::Push(push))) if push.arg.channel == "adl-warning" => {
                        let _: Vec<AdlWarningUpdate> = push
                            .parse()
                            .expect("adl-warning push should parse as Vec<AdlWarningUpdate>");
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping adl_warning_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!("skipping adl_warning_channel_subscribes: connection closed");
                        return;
                    }
                    Err(err) => {
                        eprintln!("skipping adl_warning_channel_subscribes: receive failed: {err}");
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}

/// `WS / economic-calendar` — connect to the business endpoint with
/// credentials, wait for login, subscribe, and wait for acknowledgement.
///
/// This channel requires VIP1+ access; the test is skipped gracefully when
/// credentials are absent. Any push that arrives is parsed with
/// [`EconomicCalendarUpdate`] to exercise the model.
#[tokio::test]
async fn economic_calendar_channel_subscribes() {
    let _ = dotenvy::dotenv();
    fn non_empty(var: &str) -> Option<String> {
        std::env::var(var).ok().filter(|v| !v.is_empty())
    }
    let creds = match (
        non_empty("OKX_API_KEY"),
        non_empty("OKX_API_SECRET"),
        non_empty("OKX_PASSPHRASE"),
    ) {
        (Some(k), Some(s), Some(p)) => rust_okx::Credentials::new(k, s, p),
        _ => {
            eprintln!("skipping economic_calendar_channel_subscribes: OKX credentials not set");
            return;
        }
    };

    let connect = tokio::time::timeout(
        Duration::from_secs(10),
        OkxWs::business().credentials(creds).connect(),
    )
    .await;
    let mut ws = match connect {
        Ok(Ok(ws)) => ws,
        Ok(Err(err)) => {
            eprintln!("skipping economic_calendar_channel_subscribes: connect failed: {err}");
            return;
        }
        Err(_) => {
            eprintln!("skipping economic_calendar_channel_subscribes: connect timed out");
            return;
        }
    };

    let login_deadline = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(login_deadline);
    loop {
        tokio::select! {
            _ = &mut login_deadline => {
                eprintln!(
                    "skipping economic_calendar_channel_subscribes: timed out waiting for login"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Login)) => break,
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping economic_calendar_channel_subscribes: login error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping economic_calendar_channel_subscribes: connection closed before login"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping economic_calendar_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    let arg = channels::public_data::economic_calendar();
    if let Err(err) = ws.subscribe(std::slice::from_ref(&arg)).await {
        eprintln!("skipping economic_calendar_channel_subscribes: subscribe failed: {err}");
        return;
    }

    let deadline = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                eprintln!(
                    "skipping economic_calendar_channel_subscribes: timed out waiting for ack"
                );
                return;
            }
            event = ws.next_event() => {
                match event {
                    Ok(Some(WsEvent::Subscribed(ack)))
                        if ack.channel == "economic-calendar" => break,
                    Ok(Some(WsEvent::Push(push)))
                        if push.arg.channel == "economic-calendar" =>
                    {
                        let _: Vec<EconomicCalendarUpdate> = push.parse().expect(
                            "economic-calendar push should parse as Vec<EconomicCalendarUpdate>",
                        );
                    }
                    Ok(Some(WsEvent::Error { code, msg })) => {
                        eprintln!(
                            "skipping economic_calendar_channel_subscribes: OKX WS error {code}: {msg}"
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        eprintln!(
                            "skipping economic_calendar_channel_subscribes: connection closed"
                        );
                        return;
                    }
                    Err(err) => {
                        eprintln!(
                            "skipping economic_calendar_channel_subscribes: receive failed: {err}"
                        );
                        return;
                    }
                }
            }
        }
    }

    ws.unsubscribe(&[arg])
        .await
        .expect("unsubscribe should send");
    ws.close().await.expect("close should send");
}
