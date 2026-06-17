//! WebSocket channel argument helpers.
//!
//! Each helper links to the matching OKX channel definition so the required
//! filters and the response model can be checked against the upstream schema.

use super::Arg;

/// Public and business market-data channels.
pub mod market {
    use super::Arg;

    /// Subscribe to `tickers` for one instrument.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-tickers-channel>
    pub fn tickers(inst_id: impl Into<String>) -> Arg {
        Arg::new("tickers").inst_id(inst_id)
    }

    /// Subscribe to a `candle*` channel such as `candle1m`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-candlesticks-channel>
    pub fn candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
        Arg::new(channel).inst_id(inst_id)
    }

    /// Subscribe to aggregated `trades`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-trades-channel>
    pub fn trades(inst_id: impl Into<String>) -> Arg {
        Arg::new("trades").inst_id(inst_id)
    }

    /// Subscribe to `trades-all`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-all-trades-channel>
    pub fn all_trades(inst_id: impl Into<String>) -> Arg {
        Arg::new("trades-all").inst_id(inst_id)
    }

    /// Subscribe to an order-book channel such as `books`, `books5`, or
    /// `books-l2-tbt`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-order-book-channel>
    pub fn order_book(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
        Arg::new(channel).inst_id(inst_id)
    }

    /// Subscribe to `option-trades` for one instrument family.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-option-trades-channel>
    pub fn option_trades(inst_family: impl Into<String>) -> Arg {
        Arg::new("option-trades").inst_family(inst_family)
    }

    /// Subscribe to `call-auction-details` for one instrument.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-call-auction-details-channel>
    pub fn call_auction_details(inst_id: impl Into<String>) -> Arg {
        Arg::new("call-auction-details").inst_id(inst_id)
    }
}

/// Public reference-data channels.
pub mod public_data {
    use super::Arg;

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
    pub fn estimated_price(
        inst_type: impl Into<String>,
        inst_id: impl Into<String>,
    ) -> Arg {
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
}

/// Trading-account private channels.
pub mod account {
    use super::Arg;

    /// Subscribe to the complete `account` channel.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-channel>
    pub fn account() -> Arg {
        Arg::new("account")
    }

    /// Subscribe to `account` filtered by currency.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-channel>
    pub fn account_by_currency(ccy: impl Into<String>) -> Arg {
        Arg::new("account").ccy(ccy)
    }

    /// Subscribe to `positions` for one instrument type.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-positions-channel>
    pub fn positions(inst_type: impl Into<String>) -> Arg {
        Arg::new("positions").inst_type(inst_type)
    }

    /// Subscribe to `balance_and_position`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
    pub fn balance_and_position() -> Arg {
        Arg::new("balance_and_position")
    }

    /// Subscribe to `liquidation-warning` for one instrument type.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-liquidation-warning-channel>
    pub fn liquidation_warning(inst_type: impl Into<String>) -> Arg {
        Arg::new("liquidation-warning").inst_type(inst_type)
    }

    /// Subscribe to `account-greeks` for one currency.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-greeks-channel>
    pub fn account_greeks(ccy: impl Into<String>) -> Arg {
        Arg::new("account-greeks").ccy(ccy)
    }
}

/// Order and fill private channels.
pub mod trade {
    use super::Arg;

    /// Subscribe to `orders` for one instrument type.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-order-channel>
    pub fn orders(inst_type: impl Into<String>) -> Arg {
        Arg::new("orders").inst_type(inst_type)
    }

    /// Subscribe to `fills` for one instrument type.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-fills-channel>
    pub fn fills(inst_type: impl Into<String>) -> Arg {
        Arg::new("fills").inst_type(inst_type)
    }
}

/// Algo-order private channels.
pub mod algo {
    use super::Arg;

    /// Subscribe to `orders-algo` for one instrument type.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-algo-orders-channel>
    pub fn orders_algo(inst_type: impl Into<String>) -> Arg {
        Arg::new("orders-algo").inst_type(inst_type)
    }

    /// Subscribe to `algo-advance` for one instrument type.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-advance-algo-orders-channel>
    pub fn algo_advance(inst_type: impl Into<String>) -> Arg {
        Arg::new("algo-advance").inst_type(inst_type)
    }
}

/// Trading-bot private channels.
pub mod grid {
    use super::Arg;

    /// Subscribe to `grid-orders-spot`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
    pub fn spot_grid_orders() -> Arg {
        Arg::new("grid-orders-spot").inst_type("SPOT")
    }

    /// Subscribe to `grid-orders-contract`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
    pub fn contract_grid_orders() -> Arg {
        Arg::new("grid-orders-contract").inst_type("ANY")
    }

    /// Subscribe to `grid-orders-moon`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
    pub fn moon_grid_orders() -> Arg {
        Arg::new("grid-orders-moon").inst_type("SPOT")
    }

    /// Subscribe to `grid-positions` for one algo order.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-positions-channel>
    pub fn positions(algo_id: impl Into<String>) -> Arg {
        Arg::new("grid-positions").algo_id(algo_id)
    }

    /// Subscribe to `grid-sub-orders` for one algo order.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-sub-orders-channel>
    pub fn sub_orders(algo_id: impl Into<String>) -> Arg {
        Arg::new("grid-sub-orders").algo_id(algo_id)
    }

    /// Subscribe to recurring-buy algo updates (`algo-recurring-buy`).
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-recurring-buy-orders-channel>
    pub fn recurring_buy_orders() -> Arg {
        Arg::new("algo-recurring-buy")
    }

    /// Subscribe to `copytrading-notification`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#copy-trading-websocket-copy-trading-notification-channel>
    pub fn copy_trading_notification() -> Arg {
        Arg::new("copytrading-notification")
    }
}

/// Block-trading channels.
pub mod block {
    use super::Arg;

    /// Subscribe to private block-trading `rfqs`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-rfqs-channel>
    pub fn rfqs() -> Arg {
        Arg::new("rfqs")
    }

    /// Subscribe to private block-trading `quotes`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-quotes-channel>
    pub fn quotes() -> Arg {
        Arg::new("quotes")
    }

    /// Subscribe to private `struc-block-trades`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
    pub fn structure_block_trades() -> Arg {
        Arg::new("struc-block-trades")
    }

    /// Subscribe to public `public-struc-block-trades`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
    pub fn public_structure_block_trades() -> Arg {
        Arg::new("public-struc-block-trades")
    }

    /// Subscribe to `public-block-trades` for one instrument.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-public-block-trades-channel>
    pub fn public_block_trades(inst_id: impl Into<String>) -> Arg {
        Arg::new("public-block-trades").inst_id(inst_id)
    }

    /// Subscribe to `block-tickers` for one instrument.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-block-tickers-channel>
    pub fn block_tickers(inst_id: impl Into<String>) -> Arg {
        Arg::new("block-tickers").inst_id(inst_id)
    }
}

/// Spread-trading channels.
pub mod spread {
    use super::Arg;

    /// Subscribe to all private `sprd-orders` updates.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-orders-channel>
    pub fn orders() -> Arg {
        Arg::new("sprd-orders")
    }

    /// Subscribe to private `sprd-orders` filtered by spread ID.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-orders-channel>
    pub fn orders_by_spread(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-orders").sprd_id(sprd_id)
    }

    /// Subscribe to private `sprd-trades` for one spread.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-trades-channel>
    pub fn trades(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-trades").sprd_id(sprd_id)
    }

    /// Subscribe to a spread order-book channel such as `sprd-books5`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-order-book-channel>
    pub fn order_book(channel: impl Into<String>, sprd_id: impl Into<String>) -> Arg {
        Arg::new(channel).sprd_id(sprd_id)
    }

    /// Subscribe to `sprd-public-trades` for one spread.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-public-trades-channel>
    pub fn public_trades(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-public-trades").sprd_id(sprd_id)
    }

    /// Subscribe to `sprd-tickers` for one spread.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-tickers-channel>
    pub fn tickers(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-tickers").sprd_id(sprd_id)
    }

    /// Subscribe to a `sprd-candle*` channel.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-candlesticks-channel>
    pub fn candlesticks(channel: impl Into<String>, sprd_id: impl Into<String>) -> Arg {
        Arg::new(channel).sprd_id(sprd_id)
    }
}

/// Funding-account private channels.
pub mod funding {
    use super::Arg;

    /// Subscribe to `deposit-info`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-deposit-info-channel>
    pub fn deposit_info() -> Arg {
        Arg::new("deposit-info")
    }

    /// Subscribe to `withdrawal-info`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-withdrawal-info-channel>
    pub fn withdrawal_info() -> Arg {
        Arg::new("withdrawal-info")
    }
}

/// System-status public channel.
pub mod status {
    use super::Arg;

    /// Subscribe to `status`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#status-websocket-status-channel>
    pub fn status() -> Arg {
        Arg::new("status")
    }
}
