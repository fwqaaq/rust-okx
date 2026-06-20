//! Typed response rows for OKX WebSocket channels.
//!
//! Every channel exposed by [`crate::ws::channels`] has a corresponding model
//! below. OKX occasionally adds fields without a version bump, so object rows
//! keep unrecognized fields in `extra` while still exposing the documented
//! fields as first-class Rust members.
//!
//! Models are organised by OKX's own API documentation categories:
//!
//! | Submodule | Category | Auth |
//! |---|---|---|
//! | [`market`] | Market Data | Public |
//! | [`public_data`] | Public Data | Public |
//! | [`status`] | Status | Public |
//! | [`account`] | Trading Account | Private |
//! | [`trade`] | Trade | Private |
//! | [`algo`] | Algo Trading | Private |
//! | [`grid`] | Trading Bots | Private |
//! | [`block`] | Block Trading | Mixed |
//! | [`spread`] | Spread Trading | Mixed |
//! | [`funding`] | Funding | Private |

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

/// Fields added by OKX after this crate was released.
pub type ExtraFields = BTreeMap<String, Value>;

/// Generic object row for custom/future channels.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct WsRow {
    /// Complete JSON object.
    #[serde(flatten, default)]
    pub fields: ExtraFields,
}

pub mod account;
pub mod algo;
pub mod block;
pub mod funding;
pub mod grid;
pub mod market;
pub mod public_data;
pub mod spread;
pub mod status;
pub mod trade;

// Re-exports — keeping the flat `crate::ws::model::*` namespace
pub use account::{
    AccountBalanceUpdate, AccountGreeksUpdate, AccountUpdate, BalData, BalanceAndPositionUpdate,
    CloseOrderAlgo, LiquidationWarningUpdate, PosData, PositionUpdate, Trade,
};
pub use algo::{AdvancedAlgoOrderUpdate, AlgoOrderUpdate, LinkedOrder};
pub use block::{
    BlockLeg, BlockQuoteUpdate, BlockRfqUpdate, BlockTickerUpdate, PublicBlockTradeUpdate,
    PublicStructureBlockTradeUpdate, StructureBlockTradeUpdate,
};
pub use funding::{DepositInfoUpdate, WithdrawalInfoUpdate};
pub use grid::{
    CopyTradingNotification, GridOrderUpdate, GridPositionUpdate, GridSubOrderUpdate,
    RecurringBuyAllocation, RecurringBuyOrderUpdate, TradingBotUpdate,
};
pub use market::{
    BookLevel, CallAuctionDetailsUpdate, CandleUpdate, OptionTradeUpdate, OrderBookUpdate,
    TickerUpdate, TradeUpdate,
};
pub use public_data::{
    AdlWarningUpdate, EconomicCalendarUpdate, EstimatedPriceUpdate, EventContractMarketUpdate,
    FundingRateUpdate, IndexTickerUpdate, InstrumentUpcomingParamChange, InstrumentUpdate,
    LiquidationOrderDetail, LiquidationOrderUpdate, MarkPriceUpdate, OpenInterestUpdate,
    OptionSummaryUpdate, PriceLimitUpdate,
};
pub use spread::{
    SpreadAmendOrderResult, SpreadCancelOrderResult, SpreadMassCancelResult, SpreadOrderUpdate,
    SpreadPlaceOrderResult, SpreadTradeLeg, SpreadTradeUpdate,
};
pub use status::StatusUpdate;
pub use trade::{FillUpdate, MassCancelOperationResult, OrderOperationResult, OrderUpdate};

#[cfg(test)]
mod compile_checks;
