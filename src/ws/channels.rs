//! WebSocket channel argument helpers.

use super::Arg;

/// Public and business market-data channels.
pub mod market {
    use super::Arg;

    /// `tickers`.
    pub fn tickers(inst_id: impl Into<String>) -> Arg {
        Arg::new("tickers").inst_id(inst_id)
    }

    /// `candle*`.
    pub fn candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
        Arg::new(channel).inst_id(inst_id)
    }

    /// `trades`.
    pub fn trades(inst_id: impl Into<String>) -> Arg {
        Arg::new("trades").inst_id(inst_id)
    }

    /// `trades-all`.
    pub fn all_trades(inst_id: impl Into<String>) -> Arg {
        Arg::new("trades-all").inst_id(inst_id)
    }

    /// Order-book channels such as `books`, `books5`, and `books-l2-tbt`.
    pub fn order_book(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
        Arg::new(channel).inst_id(inst_id)
    }

    /// `option-trades`.
    pub fn option_trades(inst_family: impl Into<String>) -> Arg {
        Arg::new("option-trades").inst_family(inst_family)
    }

    /// `call-auction-details`.
    pub fn call_auction_details(inst_id: impl Into<String>) -> Arg {
        Arg::new("call-auction-details").inst_id(inst_id)
    }
}

/// Public reference-data channels.
pub mod public_data {
    use super::Arg;

    /// `instruments`.
    pub fn instruments(inst_type: impl Into<String>) -> Arg {
        Arg::new("instruments").inst_type(inst_type)
    }

    /// `event-contract-markets`.
    pub fn event_contract_markets() -> Arg {
        Arg::new("event-contract-markets")
    }

    /// `open-interest`.
    pub fn open_interest(inst_id: impl Into<String>) -> Arg {
        Arg::new("open-interest").inst_id(inst_id)
    }

    /// `funding-rate`.
    pub fn funding_rate(inst_id: impl Into<String>) -> Arg {
        Arg::new("funding-rate").inst_id(inst_id)
    }

    /// `price-limit`.
    pub fn price_limit(inst_id: impl Into<String>) -> Arg {
        Arg::new("price-limit").inst_id(inst_id)
    }

    /// `opt-summary`.
    pub fn option_summary(inst_family: impl Into<String>) -> Arg {
        Arg::new("opt-summary").inst_family(inst_family)
    }

    /// `estimated-price`.
    pub fn estimated_price(inst_id: impl Into<String>) -> Arg {
        Arg::new("estimated-price").inst_id(inst_id)
    }

    /// `mark-price`.
    pub fn mark_price(inst_id: impl Into<String>) -> Arg {
        Arg::new("mark-price").inst_id(inst_id)
    }

    /// `index-tickers`.
    pub fn index_tickers(inst_id: impl Into<String>) -> Arg {
        Arg::new("index-tickers").inst_id(inst_id)
    }

    /// `mark-price-candle*`.
    pub fn mark_price_candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
        Arg::new(channel).inst_id(inst_id)
    }

    /// `index-candle*`.
    pub fn index_candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
        Arg::new(channel).inst_id(inst_id)
    }

    /// `liquidation-orders`.
    pub fn liquidation_orders(inst_type: impl Into<String>) -> Arg {
        Arg::new("liquidation-orders").inst_type(inst_type)
    }

    /// `adl-warning`.
    pub fn adl_warning(inst_type: impl Into<String>) -> Arg {
        Arg::new("adl-warning").inst_type(inst_type)
    }

    /// `economic-calendar`.
    pub fn economic_calendar() -> Arg {
        Arg::new("economic-calendar")
    }
}

/// Trading-account private channels.
pub mod account {
    use super::Arg;

    /// `account`.
    pub fn account() -> Arg {
        Arg::new("account")
    }

    /// `account` filtered by currency.
    pub fn account_by_currency(ccy: impl Into<String>) -> Arg {
        Arg::new("account").ccy(ccy)
    }

    /// `positions`.
    pub fn positions(inst_type: impl Into<String>) -> Arg {
        Arg::new("positions").inst_type(inst_type)
    }

    /// `balance_and_position`.
    pub fn balance_and_position() -> Arg {
        Arg::new("balance_and_position")
    }

    /// `liquidation-warning`.
    pub fn liquidation_warning(inst_type: impl Into<String>) -> Arg {
        Arg::new("liquidation-warning").inst_type(inst_type)
    }

    /// `account-greeks`.
    pub fn account_greeks(ccy: impl Into<String>) -> Arg {
        Arg::new("account-greeks").ccy(ccy)
    }
}

/// Order and fill private channels.
pub mod trade {
    use super::Arg;

    /// `orders`.
    pub fn orders(inst_type: impl Into<String>) -> Arg {
        Arg::new("orders").inst_type(inst_type)
    }

    /// `fills`.
    pub fn fills(inst_type: impl Into<String>) -> Arg {
        Arg::new("fills").inst_type(inst_type)
    }
}

/// Algo-order private channels.
pub mod algo {
    use super::Arg;

    /// `orders-algo`.
    pub fn orders_algo(inst_type: impl Into<String>) -> Arg {
        Arg::new("orders-algo").inst_type(inst_type)
    }

    /// `algo-advance`.
    pub fn algo_advance(inst_type: impl Into<String>) -> Arg {
        Arg::new("algo-advance").inst_type(inst_type)
    }
}

/// Trading-bot private channels.
pub mod grid {
    use super::Arg;

    /// `grid-orders-spot`.
    pub fn spot_grid_orders() -> Arg {
        Arg::new("grid-orders-spot")
    }

    /// `grid-orders-contract`.
    pub fn contract_grid_orders() -> Arg {
        Arg::new("grid-orders-contract")
    }

    /// `grid-positions`.
    pub fn positions(algo_id: impl Into<String>) -> Arg {
        Arg::new("grid-positions").algo_id(algo_id)
    }

    /// `grid-sub-orders`.
    pub fn sub_orders(algo_id: impl Into<String>) -> Arg {
        Arg::new("grid-sub-orders").algo_id(algo_id)
    }

    /// `recurring-buy-orders`.
    pub fn recurring_buy_orders() -> Arg {
        Arg::new("recurring-buy-orders")
    }

    /// `copytrading-notification`.
    pub fn copy_trading_notification() -> Arg {
        Arg::new("copytrading-notification")
    }
}

/// Block-trading channels.
pub mod block {
    use super::Arg;

    /// `rfqs`.
    pub fn rfqs() -> Arg {
        Arg::new("rfqs")
    }

    /// `quotes`.
    pub fn quotes() -> Arg {
        Arg::new("quotes")
    }

    /// `struc-block-trades`.
    pub fn structure_block_trades() -> Arg {
        Arg::new("struc-block-trades")
    }

    /// `public-struc-block-trades`.
    pub fn public_structure_block_trades() -> Arg {
        Arg::new("public-struc-block-trades")
    }

    /// `public-block-trades`.
    pub fn public_block_trades(inst_id: impl Into<String>) -> Arg {
        Arg::new("public-block-trades").inst_id(inst_id)
    }

    /// `block-tickers`.
    pub fn block_tickers(inst_id: impl Into<String>) -> Arg {
        Arg::new("block-tickers").inst_id(inst_id)
    }
}

/// Spread-trading channels.
pub mod spread {
    use super::Arg;

    /// `sprd-orders`.
    pub fn orders() -> Arg {
        Arg::new("sprd-orders")
    }

    /// `sprd-orders` filtered by spread ID.
    pub fn orders_by_spread(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-orders").sprd_id(sprd_id)
    }

    /// `sprd-trades`.
    pub fn trades(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-trades").sprd_id(sprd_id)
    }

    /// Spread order-book channels such as `sprd-books5`.
    pub fn order_book(channel: impl Into<String>, sprd_id: impl Into<String>) -> Arg {
        Arg::new(channel).sprd_id(sprd_id)
    }

    /// `sprd-public-trades`.
    pub fn public_trades(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-public-trades").sprd_id(sprd_id)
    }

    /// `sprd-tickers`.
    pub fn tickers(sprd_id: impl Into<String>) -> Arg {
        Arg::new("sprd-tickers").sprd_id(sprd_id)
    }

    /// `sprd-candle*`.
    pub fn candlesticks(channel: impl Into<String>, sprd_id: impl Into<String>) -> Arg {
        Arg::new(channel).sprd_id(sprd_id)
    }
}

/// Funding-account private channels.
pub mod funding {
    use super::Arg;

    /// `deposit-info`.
    pub fn deposit_info() -> Arg {
        Arg::new("deposit-info")
    }

    /// `withdrawal-info`.
    pub fn withdrawal_info() -> Arg {
        Arg::new("withdrawal-info")
    }
}

/// System-status public channel.
pub mod status {
    use super::Arg;

    /// `status`.
    pub fn status() -> Arg {
        Arg::new("status")
    }
}
