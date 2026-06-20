#![cfg(feature = "websocket")]

//! Integration tests against OKX public WebSocket channels.
//!
//! These tests intentionally parse channel payloads with the WebSocket models
//! from [`rust_okx::ws::model`] instead of reusing REST response types. This
//! keeps the examples aligned with OKX's WebSocket payload schemas.

use std::time::Duration;

use rust_okx::ws::channels;
use rust_okx::ws::model::{
    AllTradeUpdate, BlockTickerUpdate, CandleUpdate, OptionTradeUpdate, OrderBookUpdate,
    PublicBlockTradeUpdate, PublicStructureBlockTradeUpdate, StatusUpdate, TickerUpdate,
    TradeUpdate,
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
