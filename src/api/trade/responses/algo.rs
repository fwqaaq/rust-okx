use crate::model::NumberString;
use serde::Deserialize;

/// Result row returned by algo-order mutation endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AlgoOrderResult {
    /// Value returned by OKX in the `algoId` field.
    #[serde(default)]
    pub algo_id: String,
    /// Value returned by OKX in the `algoClOrdId` field.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Value returned by OKX in the `sCode` field.
    #[serde(default)]
    pub s_code: String,
    /// Value returned by OKX in the `sMsg` field.
    #[serde(default)]
    pub s_msg: String,
}

/// Algo-order row returned by list, history, and details endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AlgoOrder {
    /// Value returned by OKX in the `algoId` field.
    #[serde(default)]
    pub algo_id: String,
    /// Value returned by OKX in the `algoClOrdId` field.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Value returned by OKX in the `instType` field.
    #[serde(default)]
    pub inst_type: String,
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `tdMode` field.
    #[serde(default)]
    pub td_mode: String,
    /// Value returned by OKX in the `side` field.
    #[serde(default)]
    pub side: String,
    /// Value returned by OKX in the `posSide` field.
    #[serde(default)]
    pub pos_side: String,
    /// Value returned by OKX in the `ordType` field.
    #[serde(default)]
    pub ord_type: String,
    /// Value returned by OKX in the `state` field.
    #[serde(default)]
    pub state: String,
    /// Value returned by OKX in the `sz` field.
    #[serde(default)]
    pub sz: NumberString,
    /// Value returned by OKX in the `actualSz` field.
    #[serde(default)]
    pub actual_sz: NumberString,
    /// Value returned by OKX in the `actualPx` field.
    #[serde(default)]
    pub actual_px: NumberString,
    /// Value returned by OKX in the `triggerPx` field.
    #[serde(default)]
    pub trigger_px: NumberString,
    /// Value returned by OKX in the `orderPx` field.
    #[serde(default)]
    pub order_px: NumberString,
    /// Value returned by OKX in the `triggerPxType` field.
    #[serde(default)]
    pub trigger_px_type: String,
    /// Value returned by OKX in the `tpTriggerPx` field.
    #[serde(default)]
    pub tp_trigger_px: NumberString,
    /// Value returned by OKX in the `tpOrdPx` field.
    #[serde(default)]
    pub tp_ord_px: NumberString,
    /// Value returned by OKX in the `slTriggerPx` field.
    #[serde(default)]
    pub sl_trigger_px: NumberString,
    /// Value returned by OKX in the `slOrdPx` field.
    #[serde(default)]
    pub sl_ord_px: NumberString,
    /// Value returned by OKX in the `callbackRatio` field.
    #[serde(default)]
    pub callback_ratio: NumberString,
    /// Value returned by OKX in the `callbackSpread` field.
    #[serde(default)]
    pub callback_spread: NumberString,
    /// Value returned by OKX in the `activePx` field.
    #[serde(default)]
    pub active_px: NumberString,
    /// Value returned by OKX in the `tag` field.
    #[serde(default)]
    pub tag: String,
    /// Value returned by OKX in the `cTime` field.
    #[serde(default)]
    pub c_time: NumberString,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
}
