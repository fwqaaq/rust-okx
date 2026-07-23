use serde::Deserialize;

use crate::model::NumberString;

/// Block-trading counterparty.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqCounterparty {
    /// Display name of the trader or entity.
    #[serde(default)]
    pub trader_name: String,
    /// Publicly visible counterparty identifier.
    #[serde(default)]
    pub trader_code: String,
    /// Counterparty type.
    #[serde(default)]
    pub r#type: String,
}

/// One RFQ leg.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Trade mode.
    #[serde(default)]
    pub td_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Leg size.
    #[serde(default)]
    pub sz: NumberString,
    /// Leg direction.
    #[serde(default)]
    pub side: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Spot size currency unit.
    #[serde(default)]
    pub tgt_ccy: String,
    /// Spot trading quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
}

/// One leg allocated to an account in a group RFQ.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqAllocatedLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Allocated leg size.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade mode.
    #[serde(default)]
    pub td_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
}

/// Account allocation returned for a group RFQ.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqAccountAllocation {
    /// Allocated account name.
    #[serde(default)]
    pub acct: String,
    /// Per-account result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-account rejection message.
    #[serde(default)]
    pub s_msg: String,
    /// Allocated legs.
    #[serde(default)]
    pub legs: Vec<RfqAllocatedLeg>,
}

/// Block-trading request for quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqOrder {
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// RFQ state.
    #[serde(default)]
    pub state: String,
    /// Counterparty trader codes.
    #[serde(default)]
    pub counterparties: Vec<String>,
    /// Expiration timestamp.
    #[serde(default)]
    pub valid_until: NumberString,
    /// Client RFQ ID.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// RFQ tag.
    #[serde(default)]
    pub tag: String,
    /// RFQ flow type.
    #[serde(default)]
    pub flow_type: String,
    /// Taker trader code.
    #[serde(default)]
    pub trader_code: String,
    /// System RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Whether partial execution is allowed.
    #[serde(default)]
    pub allow_partial_execution: bool,
    /// RFQ legs.
    #[serde(default)]
    pub legs: Vec<RfqLeg>,
    /// Group RFQ ID.
    #[serde(default)]
    pub group_id: String,
    /// Account allocations.
    #[serde(default)]
    pub acct_alloc: Vec<RfqAccountAllocation>,
}

/// Result of canceling an RFQ.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqCancelResult {
    /// System RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Client RFQ ID.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// Per-item result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-item rejection message.
    #[serde(default)]
    pub s_msg: String,
}

/// Timestamp returned by a bulk cancellation or MMP reset.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqTimestamp {
    /// Successful operation timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// One requested execution leg.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqTradeLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Execution price.
    #[serde(default)]
    pub px: NumberString,
    /// Executed size.
    #[serde(default)]
    pub sz: NumberString,
    /// Direction from the taker's perspective.
    #[serde(default)]
    pub side: String,
    /// Trading fee or rebate.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Spot trading quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
}

/// Executed leg allocated to one account.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqExecutionAllocatedLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Filled size.
    #[serde(default)]
    pub sz: NumberString,
    /// Fee.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
}

/// Account allocation returned after executing a quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqExecutionAllocation {
    /// Allocated account.
    #[serde(default)]
    pub acct: String,
    /// Account-level block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Per-account result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-account rejection message.
    #[serde(default)]
    pub s_msg: String,
    /// Filled account legs.
    #[serde(default)]
    pub legs: Vec<RfqExecutionAllocatedLeg>,
}

/// Result of executing a block-trading quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqExecution {
    /// Execution timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// System RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Client RFQ ID.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// System quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Client quote ID.
    #[serde(default)]
    pub cl_quote_id: String,
    /// Block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Trade tag.
    #[serde(default)]
    pub tag: String,
    /// Taker trader code.
    #[serde(default)]
    pub t_trader_code: String,
    /// Maker trader code.
    #[serde(default)]
    pub m_trader_code: String,
    /// Executed legs.
    #[serde(default)]
    pub legs: Vec<RfqTradeLeg>,
    /// Account allocations.
    #[serde(default)]
    pub acct_alloc: Vec<RfqExecutionAllocation>,
}

/// Maker product settings for one instrument type.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MakerInstrumentSettings {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Whether all products of this type receive RFQs.
    #[serde(default)]
    pub include_all: bool,
    /// Product-level settings.
    #[serde(default)]
    pub data: Vec<MakerInstrument>,
}

/// Maker settings for one product.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MakerInstrument {
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Spot instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Maximum block size.
    #[serde(default)]
    pub max_block_sz: NumberString,
    /// Maker price band in ticks.
    #[serde(default)]
    pub maker_px_band: NumberString,
}

/// Boolean result of updating maker settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqBooleanResult {
    /// Whether the request succeeded.
    #[serde(default)]
    pub result: bool,
}

/// Block-trading market maker protection configuration.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqMmpConfig {
    /// MMP monitoring window in milliseconds.
    #[serde(default)]
    pub time_interval: NumberString,
    /// Frozen period in milliseconds.
    #[serde(default)]
    pub frozen_interval: NumberString,
    /// Execution-attempt limit.
    #[serde(default)]
    pub count_limit: NumberString,
    /// Whether MMP is currently triggered.
    #[serde(default)]
    pub mmp_frozen: bool,
    /// Timestamp when the current freeze ends.
    #[serde(default)]
    pub mmp_frozen_until: NumberString,
}

/// One maker quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqQuote {
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Quote state.
    #[serde(default)]
    pub state: String,
    /// State reason.
    #[serde(default)]
    pub reason: String,
    /// Expiration timestamp.
    #[serde(default)]
    pub valid_until: NumberString,
    /// System RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Client RFQ ID.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// System quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Client quote ID.
    #[serde(default)]
    pub cl_quote_id: String,
    /// Quote tag.
    #[serde(default)]
    pub tag: String,
    /// Maker trader code.
    #[serde(default)]
    pub trader_code: String,
    /// Top-level quote direction.
    #[serde(default)]
    pub quote_side: String,
    /// Quote legs.
    #[serde(default)]
    pub legs: Vec<RfqQuoteLeg>,
}

/// One leg of a maker quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqQuoteLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Trade mode.
    #[serde(default)]
    pub td_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Quote size.
    #[serde(default)]
    pub sz: NumberString,
    /// Quote price.
    #[serde(default)]
    pub px: NumberString,
    /// Leg direction.
    #[serde(default)]
    pub side: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Spot size currency unit.
    #[serde(default)]
    pub tgt_ccy: String,
    /// Spot trading quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
}

/// Result of canceling a quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqQuoteCancelResult {
    /// System quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Client quote ID.
    #[serde(default)]
    pub cl_quote_id: String,
    /// Per-item result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-item rejection message.
    #[serde(default)]
    pub s_msg: String,
}

/// Filled leg allocated to an account in a private block trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqTradeAllocatedLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Filled size.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Fee.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
}

/// Account allocation returned for a private block trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqTradeAllocation {
    /// Account-level block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Account-level error code.
    #[serde(default)]
    pub error_code: String,
    /// Allocated account.
    #[serde(default)]
    pub acct: String,
    /// Filled account legs.
    #[serde(default)]
    pub legs: Vec<RfqTradeAllocatedLeg>,
}

/// Private block-trading execution.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqTrade {
    /// Execution timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// System RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Client RFQ ID.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// System quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Client quote ID.
    #[serde(default)]
    pub cl_quote_id: String,
    /// Block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Trade tag.
    #[serde(default)]
    pub tag: String,
    /// Taker trader code.
    #[serde(default)]
    pub t_trader_code: String,
    /// Maker trader code.
    #[serde(default)]
    pub m_trader_code: String,
    /// Whether the trade filled successfully.
    #[serde(default)]
    pub is_successful: bool,
    /// Error code for an unsuccessful trade.
    #[serde(default)]
    pub error_code: String,
    /// Executed legs.
    #[serde(default)]
    pub legs: Vec<RfqTradeLeg>,
    /// Account allocations.
    #[serde(default)]
    pub acct_alloc: Vec<RfqTradeAllocation>,
}

/// Public multi-leg block trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicRfqTrade {
    /// Option strategy.
    #[serde(default)]
    pub strategy: String,
    /// Execution timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Group RFQ ID.
    #[serde(default)]
    pub group_id: String,
    /// Executed legs.
    #[serde(default)]
    pub legs: Vec<PublicRfqTradeLeg>,
}

/// One leg of a public multi-leg block trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicRfqTradeLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Execution price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade quantity.
    #[serde(default)]
    pub sz: NumberString,
    /// Direction from the taker's perspective.
    #[serde(default)]
    pub side: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
}

/// Response from configuring cancel-all-after.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RfqCancelAllAfter {
    /// Protection trigger timestamp.
    #[serde(default)]
    pub trigger_time: NumberString,
    /// Timestamp when the request was received.
    #[serde(default)]
    pub ts: NumberString,
}
