//! `public_data` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `instruments` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-instruments-channel>
pub fn instruments(inst_type: impl Into<String>) -> Arg {
    Arg::new("instruments").inst_type(inst_type)
}

/// Subscribe to `event-contract-markets`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-event-contract-markets-channel>
pub fn event_contract_markets() -> Arg {
    Arg::new("event-contract-markets").inst_type("EVENTS")
}

/// Subscribe to `open-interest` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-open-interest-channel>
pub fn open_interest(inst_id: impl Into<String>) -> Arg {
    Arg::new("open-interest").inst_id(inst_id)
}

/// Subscribe to `funding-rate` for one perpetual swap.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-funding-rate-channel>
pub fn funding_rate(inst_id: impl Into<String>) -> Arg {
    Arg::new("funding-rate").inst_id(inst_id)
}

/// Subscribe to `price-limit` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-price-limit-channel>
pub fn price_limit(inst_id: impl Into<String>) -> Arg {
    Arg::new("price-limit").inst_id(inst_id)
}

/// Subscribe to `opt-summary` for one option family.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-option-summary-channel>
pub fn option_summary(inst_family: impl Into<String>) -> Arg {
    Arg::new("opt-summary").inst_family(inst_family)
}

/// Subscribe to `estimated-price` for one instrument.
///
/// `inst_type` is required by OKX and must be `OPTION`, `FUTURES`, `SWAP`,
/// or `EVENTS`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-estimated-delivery-exercise-settlement-price-channel>
pub fn estimated_price(inst_type: impl Into<String>, inst_id: impl Into<String>) -> Arg {
    Arg::new("estimated-price")
        .inst_type(inst_type)
        .inst_id(inst_id)
}

/// Subscribe to `estimated-price` for one instrument family.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-estimated-delivery-exercise-settlement-price-channel>
pub fn estimated_price_by_family(
    inst_type: impl Into<String>,
    inst_family: impl Into<String>,
) -> Arg {
    Arg::new("estimated-price")
        .inst_type(inst_type)
        .inst_family(inst_family)
}

/// Subscribe to `mark-price` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-mark-price-channel>
pub fn mark_price(inst_id: impl Into<String>) -> Arg {
    Arg::new("mark-price").inst_id(inst_id)
}

/// Subscribe to `index-tickers` for one index.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-index-tickers-channel>
pub fn index_tickers(inst_id: impl Into<String>) -> Arg {
    Arg::new("index-tickers").inst_id(inst_id)
}

/// Subscribe to a `mark-price-candle*` channel.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-mark-price-candlesticks-channel>
pub fn mark_price_candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
    Arg::new(channel).inst_id(inst_id)
}

/// Subscribe to an `index-candle*` channel.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-index-candlesticks-channel>
pub fn index_candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
    Arg::new(channel).inst_id(inst_id)
}

/// Subscribe to `liquidation-orders` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-liquidation-orders-channel>
pub fn liquidation_orders(inst_type: impl Into<String>) -> Arg {
    Arg::new("liquidation-orders").inst_type(inst_type)
}

/// Subscribe to `adl-warning` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-adl-warning-channel>
pub fn adl_warning(inst_type: impl Into<String>) -> Arg {
    Arg::new("adl-warning").inst_type(inst_type)
}

/// Subscribe to `economic-calendar`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-economic-calendar-channel>
pub fn economic_calendar() -> Arg {
    Arg::new("economic-calendar")
}
