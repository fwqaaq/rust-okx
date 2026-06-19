//! Public reference data channel models (`instruments`, `funding-rate`, `open-interest`, etc.).
//!
//! Public channels; no authentication required.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

/// Upcoming instrument-parameter change nested in [`InstrumentUpdate`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InstrumentUpcomingParamChange {
    /// Name of the parameter that will change, e.g., `tickSz`, `lotSz`.
    #[serde(default)]
    pub param: String,
    /// The new value the parameter will take after `eff_time`.
    #[serde(default)]
    pub new_value: String,
    /// Time the new value takes effect (Unix milliseconds).
    #[serde(default)]
    pub eff_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Public-data `instruments` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-instruments-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InstrumentUpdate {
    /// Instrument type, e.g., `SPOT`, `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Series ID (event contracts only).
    #[serde(default)]
    pub series_id: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Internal numeric code for the instrument.
    #[serde(default)]
    pub inst_id_code: NumberString,
    /// Underlying index, e.g., `BTC-USD` (derivatives only).
    #[serde(default)]
    pub uly: String,
    /// Instrument family, e.g., `BTC-USD` (derivatives only).
    #[serde(default)]
    pub inst_family: String,
    /// Fee schedule category.
    #[serde(default)]
    pub category: String,
    /// Base currency, e.g., `BTC` (SPOT/MARGIN only).
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency, e.g., `USDT` (SPOT/MARGIN only).
    #[serde(default)]
    pub quote_ccy: String,
    /// Settlement currency (derivatives only).
    #[serde(default)]
    pub settle_ccy: String,
    /// Contract value (derivatives only).
    #[serde(default)]
    pub ct_val: NumberString,
    /// Contract multiplier (derivatives only).
    #[serde(default)]
    pub ct_mult: NumberString,
    /// Currency of the contract value (derivatives only).
    #[serde(default)]
    pub ct_val_ccy: String,
    /// Option type: `C` (call) or `P` (put). Empty for non-option instruments.
    #[serde(default)]
    pub opt_type: String,
    /// Strike price (options only).
    #[serde(default)]
    pub stk: NumberString,
    /// Listing time (Unix milliseconds).
    #[serde(default)]
    pub list_time: NumberString,
    /// Time at which the instrument transitions to continuous trading (Unix milliseconds).
    #[serde(default)]
    pub cont_td_sw_time: NumberString,
    /// Pre-market trading start time (Unix milliseconds).
    #[serde(default)]
    pub pre_mkt_sw_time: NumberString,
    /// Expiry/delivery time (Unix milliseconds); empty for perpetuals.
    #[serde(default)]
    pub exp_time: NumberString,
    /// Maximum leverage available.
    #[serde(default)]
    pub lever: NumberString,
    /// Tick size (minimum price increment).
    #[serde(default)]
    pub tick_sz: NumberString,
    /// Lot size (minimum order size increment).
    #[serde(default)]
    pub lot_sz: NumberString,
    /// Minimum order size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Contract type: `linear` or `inverse` (derivatives only).
    #[serde(default)]
    pub ct_type: String,
    /// Delivery alias: `this_week`, `next_week`, `quarter`, `next_quarter`.
    #[serde(default)]
    pub alias: String,
    /// Instrument state.
    ///
    /// Documented values: `live`, `suspend`, `preopen`, `settlement`.
    #[serde(default)]
    pub state: String,
    /// Open interest limit type.
    ///
    /// Documented values: `normal` (no limit), `limited`.
    #[serde(default)]
    pub open_type: String,
    /// Applicable trading rules: `normal`, `pre_market`.
    #[serde(default)]
    pub rule_type: String,
    /// Maximum limit order size.
    #[serde(default)]
    pub max_lmt_sz: NumberString,
    /// Maximum limit order amount in USD.
    #[serde(default)]
    pub max_lmt_amt: NumberString,
    /// Maximum market order size.
    #[serde(default)]
    pub max_mkt_sz: NumberString,
    /// Maximum market order amount in USD.
    #[serde(default)]
    pub max_mkt_amt: NumberString,
    /// Maximum TWAP order size.
    #[serde(default)]
    pub max_twap_sz: NumberString,
    /// Maximum iceberg order size.
    #[serde(default)]
    pub max_iceberg_sz: NumberString,
    /// Maximum trigger order size.
    #[serde(default)]
    pub max_trigger_sz: NumberString,
    /// Maximum stop order size.
    #[serde(default)]
    pub max_stop_sz: NumberString,
    /// Call-auction end time (Unix milliseconds); empty when not in auction.
    #[serde(default)]
    pub auction_end_time: NumberString,
    /// Whether futures settlement is enabled for this instrument.
    #[serde(default)]
    pub future_settlement: bool,
    /// List of supported quote currencies for this instrument.
    #[serde(default)]
    pub trade_quote_ccy_list: Vec<String>,
    /// Instrument sub-category.
    #[serde(default)]
    pub inst_category: String,
    /// Maximum total position size (USD amount).
    #[serde(default)]
    pub pos_lmt_amt: NumberString,
    /// Maximum position size as a percentage of total open interest.
    #[serde(default)]
    pub pos_lmt_pct: NumberString,
    /// Remaining long-position quota for this instrument.
    #[serde(default)]
    pub long_pos_remaining_quota: NumberString,
    /// Remaining short-position quota for this instrument.
    #[serde(default)]
    pub short_pos_remaining_quota: NumberString,
    /// Maximum platform-wide open-interest limit.
    #[serde(default)]
    pub max_plat_oi_lmt: NumberString,
    /// Instrument group ID.
    #[serde(default)]
    pub group_id: String,
    /// Upcoming parameter changes scheduled for this instrument.
    #[serde(default)]
    pub upc_chg: Vec<InstrumentUpcomingParamChange>,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `event-contract-markets` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-event-contract-markets-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EventContractMarketUpdate {
    /// Series ID of the event.
    #[serde(default)]
    pub series_id: String,
    /// Event ID.
    #[serde(default)]
    pub event_id: String,
    /// Market ID within the event.
    #[serde(default)]
    pub market_id: String,
    /// Instrument ID, e.g., `EVENT-BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Human-readable title of the market.
    #[serde(default)]
    pub title: String,
    /// Market state, e.g., `live`, `suspend`, `settlement`.
    #[serde(default)]
    pub state: String,
    /// Event category.
    #[serde(default)]
    pub category: String,
    /// Settlement rule type.
    #[serde(default)]
    pub rule_type: String,
    /// Settlement outcome of the market (after settlement).
    #[serde(default)]
    pub outcome: String,
    /// Floor (minimum) strike price.
    #[serde(default)]
    pub floor_strike: NumberString,
    /// Settlement value (after settlement).
    #[serde(default)]
    pub settle_value: NumberString,
    /// Whether the outcome is disputed.
    #[serde(default)]
    pub disputed: bool,
    /// Tick size (minimum price increment).
    #[serde(default)]
    pub tick_sz: NumberString,
    /// Minimum order size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Listing time (Unix milliseconds).
    #[serde(default)]
    pub list_time: NumberString,
    /// Expiry time (Unix milliseconds).
    #[serde(default)]
    pub exp_time: NumberString,
    /// Settlement fixing time (Unix milliseconds).
    #[serde(default)]
    pub fix_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `open-interest` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-open-interest-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OpenInterestUpdate {
    /// Instrument type, e.g., `FUTURES`, `SWAP`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USD-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Open interest in contracts.
    #[serde(default)]
    pub oi: NumberString,
    /// Open interest in coin/currency units.
    #[serde(default)]
    pub oi_ccy: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `funding-rate` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-funding-rate-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRateUpdate {
    /// Instrument type, always `SWAP`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USD-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Current funding rate.
    #[serde(default)]
    pub funding_rate: NumberString,
    /// Next estimated funding rate.
    #[serde(default)]
    pub next_funding_rate: NumberString,
    /// Current funding settlement time (Unix milliseconds).
    #[serde(default)]
    pub funding_time: NumberString,
    /// Next funding settlement time (Unix milliseconds).
    #[serde(default)]
    pub next_funding_time: NumberString,
    /// Minimum funding rate cap.
    #[serde(default)]
    pub min_funding_rate: NumberString,
    /// Maximum funding rate cap.
    #[serde(default)]
    pub max_funding_rate: NumberString,
    /// Funding rate calculation method, e.g., `next_period`, `current_period`.
    #[serde(default)]
    pub method: String,
    /// Funding formula type: `noRate` or `withRate`.
    #[serde(default)]
    pub formula_type: String,
    /// Premium index (difference between mark price and index price).
    #[serde(default)]
    pub premium: NumberString,
    /// Interest rate component of the funding rate.
    #[serde(default)]
    pub interest_rate: NumberString,
    /// Impact bid/ask size used in the mark-price calculation.
    #[serde(default)]
    pub impact_value: NumberString,
    /// Settlement state: `processing` during settlement, empty otherwise.
    #[serde(default)]
    pub sett_state: String,
    /// Actual settlement funding rate (available after settlement completes).
    #[serde(default)]
    pub sett_funding_rate: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `price-limit` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-price-limit-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PriceLimitUpdate {
    /// Instrument ID, e.g., `BTC-USD-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Highest buy price limit.
    #[serde(default)]
    pub buy_lmt: NumberString,
    /// Lowest sell price limit.
    #[serde(default)]
    pub sell_lmt: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Whether price-limit restrictions are currently in effect.
    #[serde(default)]
    pub enabled: bool,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `opt-summary` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-option-summary-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OptionSummaryUpdate {
    /// Instrument type, always `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USD-240329-40000-C`.
    #[serde(default)]
    pub inst_id: String,
    /// Underlying index, e.g., `BTC-USD`.
    #[serde(default)]
    pub uly: String,
    /// Black-Scholes delta.
    #[serde(default)]
    pub delta_bs: NumberString,
    /// Portfolio-adjusted delta.
    #[serde(default)]
    pub delta_pa: NumberString,
    /// Black-Scholes gamma.
    #[serde(default)]
    pub gamma_bs: NumberString,
    /// Portfolio-adjusted gamma.
    #[serde(default)]
    pub gamma_pa: NumberString,
    /// Black-Scholes vega.
    #[serde(default)]
    pub vega_bs: NumberString,
    /// Portfolio-adjusted vega.
    #[serde(default)]
    pub vega_pa: NumberString,
    /// Black-Scholes theta.
    #[serde(default)]
    pub theta_bs: NumberString,
    /// Portfolio-adjusted theta.
    #[serde(default)]
    pub theta_pa: NumberString,
    /// Leverage (delta / price).
    #[serde(default)]
    pub lever: NumberString,
    /// Mark implied volatility.
    #[serde(default)]
    pub mark_vol: NumberString,
    /// Best bid implied volatility.
    #[serde(default)]
    pub bid_vol: NumberString,
    /// Best ask implied volatility.
    #[serde(default)]
    pub ask_vol: NumberString,
    /// Realized volatility (30-day).
    #[serde(default)]
    pub real_vol: NumberString,
    /// Volatility for a given delta level.
    #[serde(default)]
    pub vol_lv: NumberString,
    /// Forward price used in the Black-Scholes calculation.
    #[serde(default)]
    pub fwd_px: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `estimated-price` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-estimated-delivery-exercise-settlement-price-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EstimatedPriceUpdate {
    /// Instrument type, e.g., `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USD-240329-40000-C`.
    #[serde(default)]
    pub inst_id: String,
    /// Settlement type: `delivery` or `exercise`.
    #[serde(default)]
    pub settle_type: String,
    /// Estimated delivery/exercise/settlement price.
    ///
    /// OKX uses the JSON key `settlePx`; the alias handles both spellings.
    #[serde(default)]
    #[serde(alias = "settPx")]
    pub settle_px: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `mark-price` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-mark-price-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MarkPriceUpdate {
    /// Instrument type, e.g., `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USD-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `index-tickers` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-index-tickers-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IndexTickerUpdate {
    /// Index instrument ID, e.g., `BTC-USD`.
    #[serde(default)]
    pub inst_id: String,
    /// Current index price.
    #[serde(default)]
    pub idx_px: NumberString,
    /// Highest index price over the last 24 hours.
    #[serde(default)]
    pub high24h: NumberString,
    /// Open index price over the last 24 hours.
    #[serde(default)]
    pub open24h: NumberString,
    /// Lowest index price over the last 24 hours.
    #[serde(default)]
    pub low24h: NumberString,
    /// Open price at start of day (00:00 UTC).
    #[serde(default)]
    pub sod_utc0: NumberString,
    /// Open price at start of day (08:00 UTC+8).
    #[serde(default)]
    pub sod_utc8: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Detail row nested in [`LiquidationOrderUpdate`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LiquidationOrderDetail {
    /// Side of the liquidated position: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Position side of the liquidation: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Bankruptcy price of the position.
    #[serde(default)]
    pub bk_px: NumberString,
    /// Liquidated size (number of contracts).
    #[serde(default)]
    pub sz: NumberString,
    /// Bankruptcy loss incurred.
    #[serde(default)]
    pub bk_loss: NumberString,
    /// Margin currency of the position.
    #[serde(default)]
    pub ccy: String,
    /// Liquidation timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `liquidation-orders` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-liquidation-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LiquidationOrderUpdate {
    /// Instrument ID, e.g., `BTC-USD-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type, e.g., `SWAP`, `FUTURES`, `OPTION`, `MARGIN`.
    #[serde(default)]
    pub inst_type: String,
    /// Underlying index, e.g., `BTC-USD` (derivatives only).
    #[serde(default)]
    pub uly: String,
    /// Instrument family, e.g., `BTC-USD` (derivatives only).
    #[serde(default)]
    pub inst_family: String,
    /// Per-position liquidation details.
    #[serde(default)]
    pub details: Vec<LiquidationOrderDetail>,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `adl-warning` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-adl-warning-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AdlWarningUpdate {
    /// Instrument type, e.g., `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument family, e.g., `BTC-USD`.
    #[serde(default)]
    pub inst_family: String,
    /// Settlement currency, e.g., `BTC` (inverse) or `USDT` (linear).
    #[serde(default)]
    pub ccy: String,
    /// Account balance at the time when the ADL risk was at its maximum.
    #[serde(default)]
    pub max_bal: NumberString,
    /// Recommended balance to reduce ADL risk.
    #[serde(default)]
    pub adl_rec_bal: NumberString,
    /// Current account balance.
    #[serde(default)]
    pub bal: NumberString,
    /// Time at which `max_bal` was recorded (Unix milliseconds).
    #[serde(default)]
    pub max_bal_ts: NumberString,
    /// ADL type.
    ///
    /// Documented values: `position` (position-level ADL), `account` (account-level ADL).
    #[serde(default)]
    pub adl_type: String,
    /// ADL risk state.
    ///
    /// Documented values: `1` (ADL will be triggered soon), `2` (ADL is triggered).
    #[serde(default)]
    pub state: String,
    /// Balance at which ADL would be triggered.
    #[serde(default)]
    pub adl_bal: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `economic-calendar` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-economic-calendar-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EconomicCalendarUpdate {
    /// Unique calendar event ID.
    #[serde(default)]
    pub calendar_id: String,
    /// Event date in `YYYY-MM-DD` format.
    #[serde(default)]
    pub date: String,
    /// Event time in `HH:MM` format (local to the reporting region).
    #[serde(default)]
    pub time: String,
    /// Reporting region or country, e.g., `United States`.
    #[serde(default)]
    pub region: String,
    /// Event category, e.g., `Employment`.
    #[serde(default)]
    pub category: String,
    /// Event name, e.g., `Non-Farm Payrolls`.
    #[serde(default)]
    pub event: String,
    /// Reference date for the data point (may differ from the release date).
    #[serde(default)]
    pub ref_date: String,
    /// Actual reported value (empty until the event is released).
    #[serde(default)]
    pub actual: String,
    /// Previously reported value.
    #[serde(default)]
    pub previous: String,
    /// Consensus forecast value.
    #[serde(default)]
    pub forecast: String,
    /// Market impact importance: `1` low, `2` medium, `3` high.
    #[serde(default)]
    pub importance: String,
    /// Unit of the reported value, e.g., `%`, `K`.
    #[serde(default)]
    pub unit: String,
    /// Reporting currency for the event (may differ from `ccy`).
    #[serde(default)]
    pub currency: String,
    /// Crypto-market relevant currency associated with the event, e.g., `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Whether the event spans multiple days: `1` yes, `0` no.
    #[serde(default)]
    pub date_span: String,
    /// Preliminary/initial value before revision (empty if not revised).
    #[serde(default)]
    pub prev_initial: String,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_current_instrument_and_estimated_price_fields() {
        let instrument: InstrumentUpdate = serde_json::from_str(
            r#"{"instType":"FUTURES","instId":"BTC-USD-260626","groupId":"g1","auctionEndTime":"1","contTdSwTime":"2","maxLmtAmt":"3","upcChg":[{"param":"tickSz","newValue":"0.1","effTime":"4"}]}"#,
        )
        .unwrap();
        assert_eq!(instrument.group_id, "g1");
        assert_eq!(instrument.upc_chg[0].new_value, "0.1");

        let estimated: EstimatedPriceUpdate = serde_json::from_str(
            r#"{"instType":"OPTION","instId":"BTC-USD-260626-100000-C","settlePx":"100","settleType":"exercise","ts":"5"}"#,
        )
        .unwrap();
        assert_eq!(estimated.settle_px.as_str(), "100");
    }
}
