//! Compile-time coverage for response-only model modules.
//!
//! These rows are selected dynamically through `Push::parse`, so production
//! code does not name every concrete type. Keeping the complete list here makes
//! test targets (and rust-analyzer with `allTargets`) type-check every model.

use serde::de::DeserializeOwned;

use super::*;

fn assert_deserializable<T: DeserializeOwned>() {}

#[test]
fn all_exported_websocket_models_are_deserializable() {
    assert_deserializable::<WsRow>();

    assert_deserializable::<BookLevel>();
    assert_deserializable::<CallAuctionDetailsUpdate>();
    assert_deserializable::<CandleUpdate>();
    assert_deserializable::<OptionTradeUpdate>();
    assert_deserializable::<OrderBookUpdate>();
    assert_deserializable::<TickerUpdate>();
    assert_deserializable::<TradeUpdate>();

    assert_deserializable::<AdlWarningUpdate>();
    assert_deserializable::<EconomicCalendarUpdate>();
    assert_deserializable::<EstimatedPriceUpdate>();
    assert_deserializable::<EventContractMarketUpdate>();
    assert_deserializable::<FundingRateUpdate>();
    assert_deserializable::<IndexTickerUpdate>();
    assert_deserializable::<InstrumentUpdate>();
    assert_deserializable::<InstrumentUpcomingParamChange>();
    assert_deserializable::<LiquidationOrderDetail>();
    assert_deserializable::<LiquidationOrderUpdate>();
    assert_deserializable::<MarkPriceUpdate>();
    assert_deserializable::<OpenInterestUpdate>();
    assert_deserializable::<OptionSummaryUpdate>();
    assert_deserializable::<PriceLimitUpdate>();

    assert_deserializable::<AccountBalanceUpdate>();
    assert_deserializable::<AccountGreeksUpdate>();
    assert_deserializable::<AccountUpdate>();
    assert_deserializable::<BalData>();
    assert_deserializable::<BalanceAndPositionBalance>();
    assert_deserializable::<BalanceAndPositionPosition>();
    assert_deserializable::<BalanceAndPositionTrade>();
    assert_deserializable::<BalanceAndPositionUpdate>();
    assert_deserializable::<LiquidationWarningUpdate>();
    assert_deserializable::<PosData>();
    assert_deserializable::<PositionUpdate>();
    assert_deserializable::<Trade>();

    assert_deserializable::<FillUpdate>();
    assert_deserializable::<MassCancelOperationResult>();
    assert_deserializable::<OrderOperationResult>();
    assert_deserializable::<OrderUpdate>();
    assert_deserializable::<AdvancedAlgoOrderUpdate>();
    assert_deserializable::<AlgoOrderUpdate>();

    assert_deserializable::<CopyTradingNotification>();
    assert_deserializable::<GridOrderUpdate>();
    assert_deserializable::<GridPositionUpdate>();
    assert_deserializable::<GridSubOrderUpdate>();
    assert_deserializable::<RecurringBuyAllocation>();
    assert_deserializable::<RecurringBuyOrderUpdate>();
    assert_deserializable::<TradingBotUpdate>();

    assert_deserializable::<BlockLeg>();
    assert_deserializable::<BlockQuoteUpdate>();
    assert_deserializable::<BlockRfqUpdate>();
    assert_deserializable::<BlockTickerUpdate>();
    assert_deserializable::<PublicBlockTradeUpdate>();
    assert_deserializable::<PublicStructureBlockTradeUpdate>();
    assert_deserializable::<StructureBlockTradeUpdate>();

    assert_deserializable::<SpreadAmendOrderResult>();
    assert_deserializable::<SpreadCancelOrderResult>();
    assert_deserializable::<SpreadMassCancelResult>();
    assert_deserializable::<SpreadOrderUpdate>();
    assert_deserializable::<SpreadPlaceOrderResult>();
    assert_deserializable::<SpreadTradeLeg>();
    assert_deserializable::<SpreadTradeUpdate>();

    assert_deserializable::<DepositInfoUpdate>();
    assert_deserializable::<WithdrawalInfoUpdate>();
    assert_deserializable::<StatusUpdate>();
}
