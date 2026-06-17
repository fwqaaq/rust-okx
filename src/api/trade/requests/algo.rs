use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, at_least_one, decimal_string_range, exactly_one,
    max_length, non_empty, non_negative_decimal_string, one_of, optional_non_empty,
    optional_one_of, optional_unsigned_integer_string, positive_decimal_string,
    positive_unsigned_integer_string, range_u64, reject_when_present, require_when,
    validate_client_request_id,
};

const PLACE_ALGO_ORDER_TYPES: &[&str] = &[
    "conditional",
    "oco",
    "chase",
    "trigger",
    "move_order_stop",
    "twap",
    "smart_iceberg",
];
const QUERY_ALGO_ORDER_TYPES: &[&str] = &[
    "conditional",
    "oco",
    "chase",
    "trigger",
    "move_order_stop",
    "iceberg",
    "twap",
    "smart_iceberg",
];
const PRICE_TYPES: &[&str] = &["last", "index", "mark"];

/// One attached take-profit/stop-loss definition for an algo order.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachedAlgoOrderRequest {
    #[serde(rename = "attachAlgoClOrdId", skip_serializing_if = "Option::is_none")]
    attach_algo_cl_ord_id: Option<String>,
    #[serde(rename = "tpTriggerPx", skip_serializing_if = "Option::is_none")]
    tp_trigger_px: Option<String>,
    #[serde(rename = "tpTriggerRatio", skip_serializing_if = "Option::is_none")]
    tp_trigger_ratio: Option<String>,
    #[serde(rename = "tpTriggerPxType", skip_serializing_if = "Option::is_none")]
    tp_trigger_px_type: Option<String>,
    #[serde(rename = "tpOrdPx", skip_serializing_if = "Option::is_none")]
    tp_ord_px: Option<String>,
    #[serde(rename = "slTriggerPx", skip_serializing_if = "Option::is_none")]
    sl_trigger_px: Option<String>,
    #[serde(rename = "slTriggerRatio", skip_serializing_if = "Option::is_none")]
    sl_trigger_ratio: Option<String>,
    #[serde(rename = "slTriggerPxType", skip_serializing_if = "Option::is_none")]
    sl_trigger_px_type: Option<String>,
    #[serde(rename = "slOrdPx", skip_serializing_if = "Option::is_none")]
    sl_ord_px: Option<String>,
}

impl AttachedAlgoOrderRequest {
    /// Create an empty attached TP/SL definition.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the client-supplied attached algo ID.
    pub fn client_algo_order_id(mut self, value: impl Into<String>) -> Self {
        self.attach_algo_cl_ord_id = Some(value.into());
        self
    }

    /// Configure take profit using a trigger price.
    pub fn take_profit(
        mut self,
        trigger_px: impl Into<String>,
        order_px: impl Into<String>,
    ) -> Self {
        self.tp_trigger_px = Some(trigger_px.into());
        self.tp_ord_px = Some(order_px.into());
        self
    }

    /// Configure take profit using a trigger ratio.
    pub fn take_profit_ratio(
        mut self,
        trigger_ratio: impl Into<String>,
        order_px: impl Into<String>,
    ) -> Self {
        self.tp_trigger_ratio = Some(trigger_ratio.into());
        self.tp_ord_px = Some(order_px.into());
        self
    }

    /// Set the take-profit trigger price source.
    pub fn take_profit_price_type(mut self, value: impl Into<String>) -> Self {
        self.tp_trigger_px_type = Some(value.into());
        self
    }

    /// Configure stop loss using a trigger price.
    pub fn stop_loss(mut self, trigger_px: impl Into<String>, order_px: impl Into<String>) -> Self {
        self.sl_trigger_px = Some(trigger_px.into());
        self.sl_ord_px = Some(order_px.into());
        self
    }

    /// Configure stop loss using a trigger ratio.
    pub fn stop_loss_ratio(
        mut self,
        trigger_ratio: impl Into<String>,
        order_px: impl Into<String>,
    ) -> Self {
        self.sl_trigger_ratio = Some(trigger_ratio.into());
        self.sl_ord_px = Some(order_px.into());
        self
    }

    /// Set the stop-loss trigger price source.
    pub fn stop_loss_price_type(mut self, value: impl Into<String>) -> Self {
        self.sl_trigger_px_type = Some(value.into());
        self
    }
}

impl ValidateRequest for AttachedAlgoOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        validate_client_request_id("attachAlgoClOrdId", self.attach_algo_cl_ord_id.as_deref())?;
        optional_non_empty("tpTriggerPx", self.tp_trigger_px.as_deref())?;
        if let Some(value) = self.tp_trigger_ratio.as_deref() {
            signed_decimal_string("tpTriggerRatio", value)?;
        }
        optional_one_of(
            "tpTriggerPxType",
            self.tp_trigger_px_type.as_deref(),
            PRICE_TYPES,
            "last, index, or mark",
        )?;
        optional_non_empty("tpOrdPx", self.tp_ord_px.as_deref())?;
        optional_non_empty("slTriggerPx", self.sl_trigger_px.as_deref())?;
        if let Some(value) = self.sl_trigger_ratio.as_deref() {
            signed_decimal_string("slTriggerRatio", value)?;
        }
        optional_one_of(
            "slTriggerPxType",
            self.sl_trigger_px_type.as_deref(),
            PRICE_TYPES,
            "last, index, or mark",
        )?;
        optional_non_empty("slOrdPx", self.sl_ord_px.as_deref())?;

        if self.tp_trigger_px.is_some() && self.tp_trigger_ratio.is_some() {
            return Err(RequestValidationError::MutuallyExclusive {
                fields: "tpTriggerPx, tpTriggerRatio",
            });
        }
        if self.sl_trigger_px.is_some() && self.sl_trigger_ratio.is_some() {
            return Err(RequestValidationError::MutuallyExclusive {
                fields: "slTriggerPx, slTriggerRatio",
            });
        }

        let has_tp_trigger = self.tp_trigger_px.is_some() || self.tp_trigger_ratio.is_some();
        let has_sl_trigger = self.sl_trigger_px.is_some() || self.sl_trigger_ratio.is_some();
        at_least_one(
            "take-profit fields, stop-loss fields",
            &[has_tp_trigger, has_sl_trigger],
        )?;
        if has_tp_trigger && self.tp_ord_px.is_none() {
            return Err(RequestValidationError::RequiredWhen {
                field: "tpOrdPx",
                condition: "an attached take-profit trigger is present",
            });
        }
        if has_sl_trigger && self.sl_ord_px.is_none() {
            return Err(RequestValidationError::RequiredWhen {
                field: "slOrdPx",
                condition: "an attached stop-loss trigger is present",
            });
        }
        Ok(())
    }
}

/// One trigger definition used by a `smart_iceberg` request.
#[derive(Debug, Clone, Serialize)]
pub struct SmartIcebergTriggerRequest {
    #[serde(rename = "triggerAction")]
    trigger_action: String,
    #[serde(rename = "triggerStrategy")]
    trigger_strategy: String,
    #[serde(rename = "triggerPx", skip_serializing_if = "Option::is_none")]
    trigger_px: Option<String>,
    #[serde(rename = "triggerCond", skip_serializing_if = "Option::is_none")]
    trigger_cond: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeframe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thold: Option<String>,
    #[serde(rename = "timePeriod", skip_serializing_if = "Option::is_none")]
    time_period: Option<String>,
}

impl SmartIcebergTriggerRequest {
    /// Create a start trigger with strategy `instant`, `price`, or `rsi`.
    pub fn new(trigger_strategy: impl Into<String>) -> Self {
        Self {
            trigger_action: "start".to_owned(),
            trigger_strategy: trigger_strategy.into(),
            trigger_px: None,
            trigger_cond: None,
            timeframe: None,
            thold: None,
            time_period: None,
        }
    }

    /// Set a price trigger and optional crossing condition.
    pub fn price(mut self, trigger_px: impl Into<String>) -> Self {
        self.trigger_px = Some(trigger_px.into());
        self
    }

    /// Set the trigger condition.
    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.trigger_cond = Some(value.into());
        self
    }

    /// Set the RSI candle timeframe.
    pub fn timeframe(mut self, value: impl Into<String>) -> Self {
        self.timeframe = Some(value.into());
        self
    }

    /// Set the RSI threshold from 1 through 100.
    pub fn threshold(mut self, value: impl Into<String>) -> Self {
        self.thold = Some(value.into());
        self
    }

    /// Set the RSI period. OKX currently fixes this value at `14`.
    pub fn time_period(mut self, value: impl Into<String>) -> Self {
        self.time_period = Some(value.into());
        self
    }
}

impl ValidateRequest for SmartIcebergTriggerRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        one_of("triggerAction", &self.trigger_action, &["start"], "start")?;
        one_of(
            "triggerStrategy",
            &self.trigger_strategy,
            &["instant", "price", "rsi"],
            "instant, price, or rsi",
        )?;
        optional_non_empty("triggerPx", self.trigger_px.as_deref())?;
        optional_one_of(
            "triggerCond",
            self.trigger_cond.as_deref(),
            &["cross_up", "cross_down", "above", "below", "cross"],
            "cross_up, cross_down, above, below, or cross",
        )?;
        optional_one_of(
            "timeframe",
            self.timeframe.as_deref(),
            &["3m", "5m", "15m", "30m", "1H", "4H", "1D"],
            "3m, 5m, 15m, 30m, 1H, 4H, or 1D",
        )?;

        if let Some(threshold) = self.thold.as_deref() {
            positive_unsigned_integer_string("thold", threshold)?;
            let parsed =
                threshold
                    .parse::<u64>()
                    .map_err(|_| RequestValidationError::InvalidFormat {
                        field: "thold",
                        expected: "an integer from 1 through 100",
                    })?;
            range_u64("thold", parsed, 1, 100)?;
        }
        if let Some(period) = self.time_period.as_deref() {
            one_of("timePeriod", period, &["14"], "14")?;
        }

        match self.trigger_strategy.as_str() {
            "instant" => {
                reject_when_present(
                    "triggerPx",
                    self.trigger_px.as_ref(),
                    "triggerStrategy is instant",
                )?;
                reject_when_present(
                    "triggerCond",
                    self.trigger_cond.as_ref(),
                    "triggerStrategy is instant",
                )?;
                reject_when_present(
                    "timeframe",
                    self.timeframe.as_ref(),
                    "triggerStrategy is instant",
                )?;
                reject_when_present("thold", self.thold.as_ref(), "triggerStrategy is instant")?;
                reject_when_present(
                    "timePeriod",
                    self.time_period.as_ref(),
                    "triggerStrategy is instant",
                )?;
            }
            "price" => {
                require_when(
                    "triggerPx",
                    self.trigger_px.as_deref(),
                    "triggerStrategy is price",
                )?;
                reject_when_present(
                    "timeframe",
                    self.timeframe.as_ref(),
                    "triggerStrategy is price",
                )?;
                reject_when_present("thold", self.thold.as_ref(), "triggerStrategy is price")?;
                reject_when_present(
                    "timePeriod",
                    self.time_period.as_ref(),
                    "triggerStrategy is price",
                )?;
            }
            "rsi" => {
                reject_when_present(
                    "triggerPx",
                    self.trigger_px.as_ref(),
                    "triggerStrategy is rsi",
                )?;
            }
            _ => {}
        }
        Ok(())
    }
}

/// Request body for `POST /api/v5/trade/order-algo`.
#[derive(Debug, Clone, Serialize)]
pub struct AlgoOrderRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "tdMode")]
    td_mode: String,
    side: String,
    #[serde(rename = "ordType")]
    ord_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<String>,
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    tgt_ccy: Option<String>,
    #[serde(rename = "closeFraction", skip_serializing_if = "Option::is_none")]
    close_fraction: Option<String>,
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
    #[serde(rename = "triggerPx", skip_serializing_if = "Option::is_none")]
    trigger_px: Option<String>,
    #[serde(rename = "orderPx", skip_serializing_if = "Option::is_none")]
    order_px: Option<String>,
    #[serde(rename = "advanceOrdType", skip_serializing_if = "Option::is_none")]
    advance_ord_type: Option<String>,
    #[serde(rename = "triggerPxType", skip_serializing_if = "Option::is_none")]
    trigger_px_type: Option<String>,
    #[serde(rename = "tpTriggerPx", skip_serializing_if = "Option::is_none")]
    tp_trigger_px: Option<String>,
    #[serde(rename = "tpTriggerPxType", skip_serializing_if = "Option::is_none")]
    tp_trigger_px_type: Option<String>,
    #[serde(rename = "tpOrdPx", skip_serializing_if = "Option::is_none")]
    tp_ord_px: Option<String>,
    #[serde(rename = "tpOrdKind", skip_serializing_if = "Option::is_none")]
    tp_ord_kind: Option<String>,
    #[serde(rename = "slTriggerPx", skip_serializing_if = "Option::is_none")]
    sl_trigger_px: Option<String>,
    #[serde(rename = "slTriggerPxType", skip_serializing_if = "Option::is_none")]
    sl_trigger_px_type: Option<String>,
    #[serde(rename = "slOrdPx", skip_serializing_if = "Option::is_none")]
    sl_ord_px: Option<String>,
    #[serde(rename = "cxlOnClosePos", skip_serializing_if = "Option::is_none")]
    cxl_on_close_pos: Option<bool>,
    #[serde(rename = "attachAlgoOrds", skip_serializing_if = "Option::is_none")]
    attach_algo_ords: Option<Vec<AttachedAlgoOrderRequest>>,
    #[serde(rename = "callbackRatio", skip_serializing_if = "Option::is_none")]
    callback_ratio: Option<String>,
    #[serde(rename = "callbackSpread", skip_serializing_if = "Option::is_none")]
    callback_spread: Option<String>,
    #[serde(rename = "activePx", skip_serializing_if = "Option::is_none")]
    active_px: Option<String>,
    #[serde(rename = "chaseType", skip_serializing_if = "Option::is_none")]
    chase_type: Option<String>,
    #[serde(rename = "chaseVal", skip_serializing_if = "Option::is_none")]
    chase_val: Option<String>,
    #[serde(rename = "maxChaseType", skip_serializing_if = "Option::is_none")]
    max_chase_type: Option<String>,
    #[serde(rename = "maxChaseVal", skip_serializing_if = "Option::is_none")]
    max_chase_val: Option<String>,
    #[serde(rename = "pxVar", skip_serializing_if = "Option::is_none")]
    px_var: Option<String>,
    #[serde(rename = "pxSpread", skip_serializing_if = "Option::is_none")]
    px_spread: Option<String>,
    #[serde(rename = "szLimit", skip_serializing_if = "Option::is_none")]
    sz_limit: Option<String>,
    #[serde(rename = "pxLimit", skip_serializing_if = "Option::is_none")]
    px_limit: Option<String>,
    #[serde(rename = "timeInterval", skip_serializing_if = "Option::is_none")]
    time_interval: Option<String>,
    #[serde(rename = "lmtOrderNumber", skip_serializing_if = "Option::is_none")]
    lmt_order_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggressiveness: Option<String>,
    #[serde(rename = "triggerParams", skip_serializing_if = "Option::is_none")]
    trigger_params: Option<Vec<SmartIcebergTriggerRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

impl AlgoOrderRequest {
    /// Create an algo-order request with a quantity.
    pub fn new(
        inst_id: impl Into<String>,
        td_mode: impl Into<String>,
        side: impl Into<String>,
        ord_type: impl Into<String>,
        sz: impl Into<String>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode: td_mode.into(),
            side: side.into(),
            ord_type: ord_type.into(),
            sz: Some(sz.into()),
            ccy: None,
            pos_side: None,
            reduce_only: None,
            algo_cl_ord_id: None,
            tgt_ccy: None,
            close_fraction: None,
            trade_quote_ccy: None,
            trigger_px: None,
            order_px: None,
            advance_ord_type: None,
            trigger_px_type: None,
            tp_trigger_px: None,
            tp_trigger_px_type: None,
            tp_ord_px: None,
            tp_ord_kind: None,
            sl_trigger_px: None,
            sl_trigger_px_type: None,
            sl_ord_px: None,
            cxl_on_close_pos: None,
            attach_algo_ords: None,
            callback_ratio: None,
            callback_spread: None,
            active_px: None,
            chase_type: None,
            chase_val: None,
            max_chase_type: None,
            max_chase_val: None,
            px_var: None,
            px_spread: None,
            sz_limit: None,
            px_limit: None,
            time_interval: None,
            lmt_order_number: None,
            aggressiveness: None,
            trigger_params: None,
            tag: None,
        }
    }

    /// Replace `sz` with the only currently supported full-close fraction, `1`.
    pub fn full_close(mut self) -> Self {
        self.sz = None;
        self.close_fraction = Some("1".to_owned());
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Set the position side (`long`, `short`, or `net`).
    pub fn position_side(mut self, value: impl Into<String>) -> Self {
        self.pos_side = Some(value.into());
        self
    }

    /// Set the reduce-only flag.
    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }

    /// Set the client-supplied algo ID.
    pub fn client_algo_order_id(mut self, value: impl Into<String>) -> Self {
        self.algo_cl_ord_id = Some(value.into());
        self
    }

    /// Set the Spot quantity unit (`base_ccy` or `quote_ccy`).
    pub fn target_currency(mut self, value: impl Into<String>) -> Self {
        self.tgt_ccy = Some(value.into());
        self
    }

    /// Set the quote currency used for Spot trading.
    pub fn trade_quote_currency(mut self, value: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(value.into());
        self
    }

    /// Configure a trigger order's trigger and execution prices.
    pub fn trigger(mut self, px: impl Into<String>, order_px: impl Into<String>) -> Self {
        self.trigger_px = Some(px.into());
        self.order_px = Some(order_px.into());
        self
    }

    /// Set trigger execution type (`fok` or `ioc`).
    pub fn advance_order_type(mut self, value: impl Into<String>) -> Self {
        self.advance_ord_type = Some(value.into());
        self
    }

    /// Set trigger price type (`last`, `index`, or `mark`).
    pub fn trigger_price_type(mut self, value: impl Into<String>) -> Self {
        self.trigger_px_type = Some(value.into());
        self
    }

    /// Configure take-profit trigger and order prices.
    pub fn take_profit(
        mut self,
        trigger_px: impl Into<String>,
        order_px: impl Into<String>,
    ) -> Self {
        self.tp_trigger_px = Some(trigger_px.into());
        self.tp_ord_px = Some(order_px.into());
        self
    }

    /// Configure a limit take-profit order that does not require a trigger price.
    pub fn limit_take_profit(mut self, order_px: impl Into<String>) -> Self {
        self.tp_ord_kind = Some("limit".to_owned());
        self.tp_ord_px = Some(order_px.into());
        self
    }

    /// Set the take-profit trigger source.
    pub fn take_profit_price_type(mut self, value: impl Into<String>) -> Self {
        self.tp_trigger_px_type = Some(value.into());
        self
    }

    /// Configure stop-loss trigger and order prices.
    pub fn stop_loss(mut self, trigger_px: impl Into<String>, order_px: impl Into<String>) -> Self {
        self.sl_trigger_px = Some(trigger_px.into());
        self.sl_ord_px = Some(order_px.into());
        self
    }

    /// Set the stop-loss trigger source.
    pub fn stop_loss_price_type(mut self, value: impl Into<String>) -> Self {
        self.sl_trigger_px_type = Some(value.into());
        self
    }

    /// Associate TP/SL cancellation with a fully closed position.
    pub fn cancel_on_close_position(mut self, value: bool) -> Self {
        self.cxl_on_close_pos = Some(value);
        self
    }

    /// Attach TP/SL definitions to the triggered order.
    pub fn attached_algo_orders(mut self, values: Vec<AttachedAlgoOrderRequest>) -> Self {
        self.attach_algo_ords = Some(values);
        self
    }

    /// Set a trailing-order callback ratio.
    pub fn callback_ratio(mut self, value: impl Into<String>) -> Self {
        self.callback_ratio = Some(value.into());
        self
    }

    /// Set a trailing-order callback spread.
    pub fn callback_spread(mut self, value: impl Into<String>) -> Self {
        self.callback_spread = Some(value.into());
        self
    }

    /// Set a trailing-order activation price.
    pub fn active_price(mut self, value: impl Into<String>) -> Self {
        self.active_px = Some(value.into());
        self
    }

    /// Configure chase type and value.
    pub fn chase(mut self, chase_type: impl Into<String>, chase_val: impl Into<String>) -> Self {
        self.chase_type = Some(chase_type.into());
        self.chase_val = Some(chase_val.into());
        self
    }

    /// Configure the optional maximum chase type and value.
    pub fn maximum_chase(
        mut self,
        chase_type: impl Into<String>,
        chase_val: impl Into<String>,
    ) -> Self {
        self.max_chase_type = Some(chase_type.into());
        self.max_chase_val = Some(chase_val.into());
        self
    }

    /// Configure TWAP strategy fields using a percentage variance.
    pub fn twap_by_variance(
        mut self,
        px_var: impl Into<String>,
        sz_limit: impl Into<String>,
        px_limit: impl Into<String>,
        time_interval: impl Into<String>,
    ) -> Self {
        self.px_var = Some(px_var.into());
        self.px_spread = None;
        self.sz_limit = Some(sz_limit.into());
        self.px_limit = Some(px_limit.into());
        self.time_interval = Some(time_interval.into());
        self
    }

    /// Configure TWAP strategy fields using an absolute price spread.
    pub fn twap_by_spread(
        mut self,
        px_spread: impl Into<String>,
        sz_limit: impl Into<String>,
        px_limit: impl Into<String>,
        time_interval: impl Into<String>,
    ) -> Self {
        self.px_var = None;
        self.px_spread = Some(px_spread.into());
        self.sz_limit = Some(sz_limit.into());
        self.px_limit = Some(px_limit.into());
        self.time_interval = Some(time_interval.into());
        self
    }

    /// Configure required Smart Iceberg execution fields.
    pub fn smart_iceberg(
        mut self,
        sz_limit: impl Into<String>,
        lmt_order_number: impl Into<String>,
        aggressiveness: impl Into<String>,
    ) -> Self {
        self.sz_limit = Some(sz_limit.into());
        self.lmt_order_number = Some(lmt_order_number.into());
        self.aggressiveness = Some(aggressiveness.into());
        self
    }

    /// Set an optional Smart Iceberg price limit.
    pub fn price_limit(mut self, value: impl Into<String>) -> Self {
        self.px_limit = Some(value.into());
        self
    }

    /// Set Smart Iceberg trigger parameters.
    pub fn smart_iceberg_triggers(mut self, values: Vec<SmartIcebergTriggerRequest>) -> Self {
        self.trigger_params = Some(values);
        self
    }

    /// Set an order tag of at most 16 ASCII alphanumeric characters.
    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }
}

impl ValidateRequest for AlgoOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("instId", &self.inst_id)?;
        one_of(
            "tdMode",
            &self.td_mode,
            &["cash", "cross", "isolated", "spot_isolated"],
            "cash, cross, isolated, or spot_isolated",
        )?;
        one_of("side", &self.side, &["buy", "sell"], "buy or sell")?;
        one_of(
            "ordType",
            &self.ord_type,
            PLACE_ALGO_ORDER_TYPES,
            "conditional, oco, chase, trigger, move_order_stop, twap, or smart_iceberg",
        )?;
        optional_non_empty("sz", self.sz.as_deref())?;
        if let Some(sz) = self.sz.as_deref() {
            positive_decimal_string("sz", sz)?;
        }
        optional_non_empty("closeFraction", self.close_fraction.as_deref())?;
        exactly_one(
            "sz, closeFraction",
            &[self.sz.is_some(), self.close_fraction.is_some()],
        )?;
        if let Some(value) = self.close_fraction.as_deref() {
            one_of("closeFraction", value, &["1"], "1")?;
            if !matches!(self.ord_type.as_str(), "conditional" | "oco") {
                return Err(RequestValidationError::NotApplicable {
                    field: "closeFraction",
                    condition: "ordType is not conditional or oco",
                });
            }
            if self.pos_side.as_deref() == Some("net") && self.reduce_only != Some(true) {
                return Err(RequestValidationError::RequiredWhen {
                    field: "reduceOnly=true",
                    condition: "closeFraction is used with posSide=net",
                });
            }
        }
        optional_non_empty("ccy", self.ccy.as_deref())?;
        if self.ccy.is_some() && matches!(self.td_mode.as_str(), "cash" | "spot_isolated") {
            return Err(RequestValidationError::NotApplicable {
                field: "ccy",
                condition: "tdMode is cash or spot_isolated",
            });
        }
        optional_one_of(
            "posSide",
            self.pos_side.as_deref(),
            &["long", "short", "net"],
            "long, short, or net",
        )?;
        if self.pos_side.is_some() && matches!(self.td_mode.as_str(), "cash" | "spot_isolated") {
            return Err(RequestValidationError::NotApplicable {
                field: "posSide",
                condition: "tdMode is cash or spot_isolated",
            });
        }
        if self.reduce_only.is_some() && matches!(self.td_mode.as_str(), "cash" | "spot_isolated") {
            return Err(RequestValidationError::NotApplicable {
                field: "reduceOnly",
                condition: "tdMode is cash or spot_isolated",
            });
        }
        validate_client_request_id("algoClOrdId", self.algo_cl_ord_id.as_deref())?;
        optional_one_of(
            "tgtCcy",
            self.tgt_ccy.as_deref(),
            &["base_ccy", "quote_ccy"],
            "base_ccy or quote_ccy",
        )?;
        if self.tgt_ccy.is_some()
            && (self.ord_type != "conditional"
                || self.side != "buy"
                || !matches!(self.td_mode.as_str(), "cash" | "spot_isolated"))
        {
            return Err(RequestValidationError::NotApplicable {
                field: "tgtCcy",
                condition: "the request is not a SPOT market-buy conditional order",
            });
        }
        optional_non_empty("tradeQuoteCcy", self.trade_quote_ccy.as_deref())?;
        if self.trade_quote_ccy.is_some()
            && !matches!(self.td_mode.as_str(), "cash" | "spot_isolated")
        {
            return Err(RequestValidationError::NotApplicable {
                field: "tradeQuoteCcy",
                condition: "tdMode is not cash or spot_isolated",
            });
        }
        optional_one_of(
            "triggerPxType",
            self.trigger_px_type.as_deref(),
            PRICE_TYPES,
            "last, index, or mark",
        )?;
        if self.trigger_px_type.is_some()
            && matches!(self.td_mode.as_str(), "cash" | "spot_isolated")
            && self.trigger_px_type.as_deref() != Some("last")
        {
            return Err(RequestValidationError::InvalidFormat {
                field: "triggerPxType",
                expected: "last for SPOT instruments",
            });
        }
        optional_one_of(
            "advanceOrdType",
            self.advance_ord_type.as_deref(),
            &["fok", "ioc"],
            "fok or ioc",
        )?;
        optional_one_of(
            "tpTriggerPxType",
            self.tp_trigger_px_type.as_deref(),
            PRICE_TYPES,
            "last, index, or mark",
        )?;
        optional_one_of(
            "tpOrdKind",
            self.tp_ord_kind.as_deref(),
            &["condition", "limit"],
            "condition or limit",
        )?;
        optional_one_of(
            "slTriggerPxType",
            self.sl_trigger_px_type.as_deref(),
            PRICE_TYPES,
            "last, index, or mark",
        )?;
        validate_take_profit(
            self.tp_trigger_px.as_deref(),
            self.tp_ord_px.as_deref(),
            self.tp_ord_kind.as_deref(),
        )?;
        validate_price_pair(
            "slTriggerPx",
            self.sl_trigger_px.as_deref(),
            "slOrdPx",
            self.sl_ord_px.as_deref(),
        )?;
        if self.cxl_on_close_pos == Some(true) && self.reduce_only != Some(true) {
            return Err(RequestValidationError::RequiredWhen {
                field: "reduceOnly=true",
                condition: "cxlOnClosePos is true",
            });
        }
        if let Some(values) = self.attach_algo_ords.as_deref() {
            if values.is_empty() {
                return Err(RequestValidationError::LengthOutOfRange {
                    field: "attachAlgoOrds",
                    min: 1,
                    max: usize::MAX,
                });
            }
            for value in values {
                value.validate()?;
                validate_attached_ratio_direction(&self.side, value)?;
            }
        }
        if let Some(tag) = self.tag.as_deref() {
            non_empty("tag", tag)?;
            max_length("tag", tag, 16)?;
            if !tag.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
                return Err(RequestValidationError::InvalidFormat {
                    field: "tag",
                    expected: "1-16 ASCII alphanumeric characters",
                });
            }
        }

        match self.ord_type.as_str() {
            "trigger" => {
                require_pair(
                    "triggerPx",
                    self.trigger_px.as_deref(),
                    "orderPx",
                    self.order_px.as_deref(),
                    "ordType is trigger",
                )?;
            }
            "conditional" => {
                at_least_one(
                    "take-profit fields, stop-loss fields",
                    &[
                        self.tp_trigger_px.is_some() || self.tp_ord_px.is_some(),
                        self.sl_trigger_px.is_some(),
                    ],
                )?;
            }
            "oco" => {
                require_take_profit(
                    self.tp_trigger_px.as_deref(),
                    self.tp_ord_px.as_deref(),
                    self.tp_ord_kind.as_deref(),
                    "ordType is oco",
                )?;
                require_pair(
                    "slTriggerPx",
                    self.sl_trigger_px.as_deref(),
                    "slOrdPx",
                    self.sl_ord_px.as_deref(),
                    "ordType is oco",
                )?;
            }
            "move_order_stop" => {
                exactly_one(
                    "callbackRatio, callbackSpread",
                    &[
                        self.callback_ratio.is_some(),
                        self.callback_spread.is_some(),
                    ],
                )?;
                if let Some(value) = self.callback_ratio.as_deref() {
                    positive_decimal_string("callbackRatio", value)?;
                }
                if let Some(value) = self.callback_spread.as_deref() {
                    positive_decimal_string("callbackSpread", value)?;
                }
                if let Some(value) = self.active_px.as_deref() {
                    positive_decimal_string("activePx", value)?;
                }
            }
            "chase" => {
                optional_one_of(
                    "chaseType",
                    self.chase_type.as_deref(),
                    &["distance", "ratio"],
                    "distance or ratio",
                )?;
                if let Some(value) = self.chase_val.as_deref() {
                    non_negative_decimal_string("chaseVal", value)?;
                }
                optional_one_of(
                    "maxChaseType",
                    self.max_chase_type.as_deref(),
                    &["distance", "ratio"],
                    "distance or ratio",
                )?;
                if let Some(value) = self.max_chase_val.as_deref() {
                    non_negative_decimal_string("maxChaseVal", value)?;
                }
                if self.max_chase_type.is_some() != self.max_chase_val.is_some() {
                    return Err(RequestValidationError::RequiredWhen {
                        field: if self.max_chase_type.is_some() {
                            "maxChaseVal"
                        } else {
                            "maxChaseType"
                        },
                        condition: "the other maximum-chase field is present",
                    });
                }
            }
            "twap" => {
                exactly_one(
                    "pxVar, pxSpread",
                    &[self.px_var.is_some(), self.px_spread.is_some()],
                )?;
                if let Some(value) = self.px_var.as_deref() {
                    decimal_string_range("pxVar", value, 0.0001, 0.01, "0.0001", "0.01")?;
                }
                if let Some(value) = self.px_spread.as_deref() {
                    non_negative_decimal_string("pxSpread", value)?;
                }
                let sz_limit =
                    require_value("szLimit", self.sz_limit.as_deref(), "ordType is twap")?;
                positive_decimal_string("szLimit", sz_limit)?;
                let px_limit =
                    require_value("pxLimit", self.px_limit.as_deref(), "ordType is twap")?;
                non_negative_decimal_string("pxLimit", px_limit)?;
                let interval = require_value(
                    "timeInterval",
                    self.time_interval.as_deref(),
                    "ordType is twap",
                )?;
                positive_unsigned_integer_string("timeInterval", interval)?;
            }
            "smart_iceberg" => {
                let sz_limit = require_value(
                    "szLimit",
                    self.sz_limit.as_deref(),
                    "ordType is smart_iceberg",
                )?;
                positive_decimal_string("szLimit", sz_limit)?;
                let split_count = require_value(
                    "lmtOrderNumber",
                    self.lmt_order_number.as_deref(),
                    "ordType is smart_iceberg",
                )?;
                positive_unsigned_integer_string("lmtOrderNumber", split_count)?;
                let aggressiveness = require_value(
                    "aggressiveness",
                    self.aggressiveness.as_deref(),
                    "ordType is smart_iceberg",
                )?;
                one_of(
                    "aggressiveness",
                    aggressiveness,
                    &["radical", "mid", "conservative"],
                    "radical, mid, or conservative",
                )?;
                if let Some(value) = self.px_limit.as_deref() {
                    non_negative_decimal_string("pxLimit", value)?;
                }
                if let Some(values) = self.trigger_params.as_deref() {
                    for value in values {
                        value.validate()?;
                    }
                }
            }
            _ => {}
        }

        validate_full_close_execution(self)?;
        validate_target_currency_execution(self)?;
        validate_strategy_applicability(self)
    }
}

fn signed_decimal_string(field: &'static str, value: &str) -> Result<f64, RequestValidationError> {
    non_empty(field, value)?;
    if value.starts_with('+')
        || value.contains(['e', 'E'])
        || value == "-"
        || value.starts_with('.')
        || value.ends_with('.')
        || value.matches('-').count() > 1
        || (value.contains('-') && !value.starts_with('-'))
    {
        return Err(RequestValidationError::InvalidFormat {
            field,
            expected: "a finite decimal string without scientific notation",
        });
    }
    let unsigned = value.strip_prefix('-').unwrap_or(value);
    if unsigned.is_empty()
        || !unsigned
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
        || unsigned.matches('.').count() > 1
    {
        return Err(RequestValidationError::InvalidFormat {
            field,
            expected: "a finite decimal string without scientific notation",
        });
    }
    let parsed = value
        .parse::<f64>()
        .map_err(|_| RequestValidationError::InvalidFormat {
            field,
            expected: "a finite decimal string without scientific notation",
        })?;
    if !parsed.is_finite() {
        return Err(RequestValidationError::InvalidFormat {
            field,
            expected: "a finite decimal string without scientific notation",
        });
    }
    Ok(parsed)
}

fn validate_attached_ratio_direction(
    side: &str,
    request: &AttachedAlgoOrderRequest,
) -> Result<(), RequestValidationError> {
    if let Some(value) = request.tp_trigger_ratio.as_deref() {
        let parsed = signed_decimal_string("tpTriggerRatio", value)?;
        let valid = if side == "buy" {
            parsed > 0.0
        } else {
            parsed > -1.0 && parsed < 0.0
        };
        if !valid {
            return Err(RequestValidationError::InvalidFormat {
                field: "tpTriggerRatio",
                expected: "greater than 0 for buy orders, or between -1 and 0 for sell orders",
            });
        }
    }
    if let Some(value) = request.sl_trigger_ratio.as_deref() {
        let parsed = signed_decimal_string("slTriggerRatio", value)?;
        let valid = if side == "buy" {
            parsed > 0.0 && parsed < 1.0
        } else {
            parsed > 0.0
        };
        if !valid {
            return Err(RequestValidationError::InvalidFormat {
                field: "slTriggerRatio",
                expected: "between 0 and 1 for buy orders, or greater than 0 for sell orders",
            });
        }
    }
    Ok(())
}

fn validate_full_close_execution(request: &AlgoOrderRequest) -> Result<(), RequestValidationError> {
    if request.close_fraction.is_none() {
        return Ok(());
    }
    for (field, value) in [
        ("tpOrdPx", request.tp_ord_px.as_deref()),
        ("slOrdPx", request.sl_ord_px.as_deref()),
    ] {
        if value.is_some_and(|value| value != "-1") {
            return Err(RequestValidationError::InvalidFormat {
                field,
                expected: "-1 when closeFraction is used",
            });
        }
    }
    Ok(())
}

fn validate_target_currency_execution(
    request: &AlgoOrderRequest,
) -> Result<(), RequestValidationError> {
    if request.tgt_ccy.is_none() {
        return Ok(());
    }
    if request.tp_ord_px.as_deref() != Some("-1") && request.sl_ord_px.as_deref() != Some("-1") {
        return Err(RequestValidationError::RequiredWhen {
            field: "tpOrdPx or slOrdPx set to -1",
            condition: "tgtCcy is present",
        });
    }
    Ok(())
}

fn require_value<'a>(
    field: &'static str,
    value: Option<&'a str>,
    condition: &'static str,
) -> Result<&'a str, RequestValidationError> {
    value.ok_or(RequestValidationError::RequiredWhen { field, condition })
}

fn validate_take_profit(
    trigger_px: Option<&str>,
    order_px: Option<&str>,
    order_kind: Option<&str>,
) -> Result<(), RequestValidationError> {
    optional_non_empty("tpTriggerPx", trigger_px)?;
    optional_non_empty("tpOrdPx", order_px)?;
    if order_kind == Some("limit") {
        if order_px.is_none() {
            return Err(RequestValidationError::RequiredWhen {
                field: "tpOrdPx",
                condition: "tpOrdKind is limit",
            });
        }
        return Ok(());
    }
    if trigger_px.is_some() != order_px.is_some() {
        return Err(RequestValidationError::RequiredWhen {
            field: if trigger_px.is_some() {
                "tpOrdPx"
            } else {
                "tpTriggerPx"
            },
            condition: "the paired take-profit price field is present",
        });
    }
    Ok(())
}

fn require_take_profit(
    trigger_px: Option<&str>,
    order_px: Option<&str>,
    order_kind: Option<&str>,
    condition: &'static str,
) -> Result<(), RequestValidationError> {
    if order_kind == Some("limit") {
        require_when("tpOrdPx", order_px, condition)?;
    } else {
        require_pair("tpTriggerPx", trigger_px, "tpOrdPx", order_px, condition)?;
    }
    validate_take_profit(trigger_px, order_px, order_kind)
}

fn validate_price_pair(
    left_field: &'static str,
    left: Option<&str>,
    right_field: &'static str,
    right: Option<&str>,
) -> Result<(), RequestValidationError> {
    optional_non_empty(left_field, left)?;
    optional_non_empty(right_field, right)?;
    if left.is_some() != right.is_some() {
        return Err(RequestValidationError::RequiredWhen {
            field: if left.is_some() {
                right_field
            } else {
                left_field
            },
            condition: "the paired algo price field is present",
        });
    }
    Ok(())
}

fn require_pair(
    left_field: &'static str,
    left: Option<&str>,
    right_field: &'static str,
    right: Option<&str>,
    condition: &'static str,
) -> Result<(), RequestValidationError> {
    require_when(left_field, left, condition)?;
    require_when(right_field, right, condition)?;
    validate_price_pair(left_field, left, right_field, right)
}

fn validate_strategy_applicability(
    request: &AlgoOrderRequest,
) -> Result<(), RequestValidationError> {
    let ord_type = request.ord_type.as_str();
    if ord_type != "trigger" {
        reject_when_present(
            "triggerPx",
            request.trigger_px.as_ref(),
            "ordType is not trigger",
        )?;
        reject_when_present(
            "orderPx",
            request.order_px.as_ref(),
            "ordType is not trigger",
        )?;
        reject_when_present(
            "advanceOrdType",
            request.advance_ord_type.as_ref(),
            "ordType is not trigger",
        )?;
        reject_when_present(
            "attachAlgoOrds",
            request.attach_algo_ords.as_ref(),
            "ordType is not trigger",
        )?;
    }
    if ord_type != "move_order_stop" {
        reject_when_present(
            "callbackRatio",
            request.callback_ratio.as_ref(),
            "ordType is not move_order_stop",
        )?;
        reject_when_present(
            "callbackSpread",
            request.callback_spread.as_ref(),
            "ordType is not move_order_stop",
        )?;
        reject_when_present(
            "activePx",
            request.active_px.as_ref(),
            "ordType is not move_order_stop",
        )?;
    }
    if ord_type != "chase" {
        reject_when_present(
            "chaseType",
            request.chase_type.as_ref(),
            "ordType is not chase",
        )?;
        reject_when_present(
            "chaseVal",
            request.chase_val.as_ref(),
            "ordType is not chase",
        )?;
        reject_when_present(
            "maxChaseType",
            request.max_chase_type.as_ref(),
            "ordType is not chase",
        )?;
        reject_when_present(
            "maxChaseVal",
            request.max_chase_val.as_ref(),
            "ordType is not chase",
        )?;
    }
    if ord_type != "twap" {
        reject_when_present("pxVar", request.px_var.as_ref(), "ordType is not twap")?;
        reject_when_present(
            "pxSpread",
            request.px_spread.as_ref(),
            "ordType is not twap",
        )?;
        reject_when_present(
            "timeInterval",
            request.time_interval.as_ref(),
            "ordType is not twap",
        )?;
    }
    if !matches!(ord_type, "twap" | "smart_iceberg") {
        reject_when_present(
            "szLimit",
            request.sz_limit.as_ref(),
            "ordType is not twap or smart_iceberg",
        )?;
        reject_when_present(
            "pxLimit",
            request.px_limit.as_ref(),
            "ordType is not twap or smart_iceberg",
        )?;
    }
    if ord_type != "smart_iceberg" {
        reject_when_present(
            "lmtOrderNumber",
            request.lmt_order_number.as_ref(),
            "ordType is not smart_iceberg",
        )?;
        reject_when_present(
            "aggressiveness",
            request.aggressiveness.as_ref(),
            "ordType is not smart_iceberg",
        )?;
        reject_when_present(
            "triggerParams",
            request.trigger_params.as_ref(),
            "ordType is not smart_iceberg",
        )?;
    }
    if !matches!(ord_type, "conditional" | "oco") {
        reject_when_present(
            "tgtCcy",
            request.tgt_ccy.as_ref(),
            "ordType is not conditional or oco",
        )?;
        reject_when_present(
            "cxlOnClosePos",
            request.cxl_on_close_pos.as_ref(),
            "ordType is not conditional or oco",
        )?;
    }
    Ok(())
}

/// Request body for one entry in `POST /api/v5/trade/cancel-algos`.
#[derive(Debug, Clone, Serialize)]
pub struct CancelAlgoOrderRequest {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<String>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<String>,
    #[serde(rename = "instId")]
    inst_id: String,
}

impl CancelAlgoOrderRequest {
    /// Create a cancellation using an OKX algo ID.
    pub fn new(algo_id: impl Into<String>, inst_id: impl Into<String>) -> Self {
        Self {
            algo_id: Some(algo_id.into()),
            algo_cl_ord_id: None,
            inst_id: inst_id.into(),
        }
    }

    /// Create a cancellation using a client-supplied algo ID.
    pub fn by_client_algo_order_id(
        algo_cl_ord_id: impl Into<String>,
        inst_id: impl Into<String>,
    ) -> Self {
        Self {
            algo_id: None,
            algo_cl_ord_id: Some(algo_cl_ord_id.into()),
            inst_id: inst_id.into(),
        }
    }
}

impl ValidateRequest for CancelAlgoOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("instId", &self.inst_id)?;
        optional_non_empty("algoId", self.algo_id.as_deref())?;
        validate_client_request_id("algoClOrdId", self.algo_cl_ord_id.as_deref())?;
        at_least_one(
            "algoId, algoClOrdId",
            &[self.algo_id.is_some(), self.algo_cl_ord_id.is_some()],
        )
    }
}

/// Request body for `POST /api/v5/trade/amend-algos`.
#[derive(Debug, Clone, Serialize)]
pub struct AmendAlgoOrderRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<String>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<String>,
    #[serde(rename = "reqId", skip_serializing_if = "Option::is_none")]
    req_id: Option<String>,
    #[serde(rename = "newSz", skip_serializing_if = "Option::is_none")]
    new_sz: Option<String>,
    #[serde(rename = "newTpTriggerPx", skip_serializing_if = "Option::is_none")]
    new_tp_trigger_px: Option<String>,
    #[serde(rename = "newTpOrdPx", skip_serializing_if = "Option::is_none")]
    new_tp_ord_px: Option<String>,
    #[serde(rename = "newTpTriggerPxType", skip_serializing_if = "Option::is_none")]
    new_tp_trigger_px_type: Option<String>,
    #[serde(rename = "newSlTriggerPx", skip_serializing_if = "Option::is_none")]
    new_sl_trigger_px: Option<String>,
    #[serde(rename = "newSlOrdPx", skip_serializing_if = "Option::is_none")]
    new_sl_ord_px: Option<String>,
    #[serde(rename = "newSlTriggerPxType", skip_serializing_if = "Option::is_none")]
    new_sl_trigger_px_type: Option<String>,
    #[serde(rename = "cxlOnFail", skip_serializing_if = "Option::is_none")]
    cancel_on_fail: Option<bool>,
}

impl AmendAlgoOrderRequest {
    /// Create an amendment for an instrument; add an algo identifier next.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            algo_id: None,
            algo_cl_ord_id: None,
            req_id: None,
            new_sz: None,
            new_tp_trigger_px: None,
            new_tp_ord_px: None,
            new_tp_trigger_px_type: None,
            new_sl_trigger_px: None,
            new_sl_ord_px: None,
            new_sl_trigger_px_type: None,
            cancel_on_fail: None,
        }
    }

    /// Identify the order by OKX algo ID.
    pub fn algo_id(mut self, value: impl Into<String>) -> Self {
        self.algo_id = Some(value.into());
        self
    }

    /// Identify the order by client-supplied algo ID.
    pub fn client_algo_order_id(mut self, value: impl Into<String>) -> Self {
        self.algo_cl_ord_id = Some(value.into());
        self
    }

    /// Set a client request ID of up to 32 ASCII alphanumeric characters.
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.req_id = Some(value.into());
        self
    }

    /// Amend the order size.
    pub fn new_size(mut self, value: impl Into<String>) -> Self {
        self.new_sz = Some(value.into());
        self
    }

    /// Amend take-profit prices.
    pub fn take_profit(
        mut self,
        trigger_px: impl Into<String>,
        order_px: impl Into<String>,
    ) -> Self {
        self.new_tp_trigger_px = Some(trigger_px.into());
        self.new_tp_ord_px = Some(order_px.into());
        self
    }

    /// Set the amended take-profit trigger price source.
    pub fn take_profit_price_type(mut self, value: impl Into<String>) -> Self {
        self.new_tp_trigger_px_type = Some(value.into());
        self
    }

    /// Delete the take-profit definition by sending the documented `0` sentinel.
    pub fn delete_take_profit(mut self) -> Self {
        self.new_tp_trigger_px = Some("0".to_owned());
        self.new_tp_ord_px = None;
        self.new_tp_trigger_px_type = None;
        self
    }

    /// Amend stop-loss prices.
    pub fn stop_loss(mut self, trigger_px: impl Into<String>, order_px: impl Into<String>) -> Self {
        self.new_sl_trigger_px = Some(trigger_px.into());
        self.new_sl_ord_px = Some(order_px.into());
        self
    }

    /// Set the amended stop-loss trigger price source.
    pub fn stop_loss_price_type(mut self, value: impl Into<String>) -> Self {
        self.new_sl_trigger_px_type = Some(value.into());
        self
    }

    /// Delete the stop-loss definition by sending the documented `0` sentinel.
    pub fn delete_stop_loss(mut self) -> Self {
        self.new_sl_trigger_px = Some("0".to_owned());
        self.new_sl_ord_px = None;
        self.new_sl_trigger_px_type = None;
        self
    }

    /// Cancel the original order automatically when amendment fails.
    pub fn cancel_on_fail(mut self, value: bool) -> Self {
        self.cancel_on_fail = Some(value);
        self
    }
}

impl ValidateRequest for AmendAlgoOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("instId", &self.inst_id)?;
        optional_non_empty("algoId", self.algo_id.as_deref())?;
        validate_client_request_id("algoClOrdId", self.algo_cl_ord_id.as_deref())?;
        validate_client_request_id("reqId", self.req_id.as_deref())?;
        at_least_one(
            "algoId, algoClOrdId",
            &[self.algo_id.is_some(), self.algo_cl_ord_id.is_some()],
        )?;
        at_least_one(
            "newSz, take-profit amendment fields, stop-loss amendment fields",
            &[
                self.new_sz.is_some(),
                self.new_tp_trigger_px.is_some()
                    || self.new_tp_ord_px.is_some()
                    || self.new_tp_trigger_px_type.is_some(),
                self.new_sl_trigger_px.is_some()
                    || self.new_sl_ord_px.is_some()
                    || self.new_sl_trigger_px_type.is_some(),
            ],
        )?;
        if let Some(new_sz) = self.new_sz.as_deref() {
            positive_decimal_string("newSz", new_sz)?;
        }
        validate_amended_tp_sl(
            "newTpTriggerPx",
            self.new_tp_trigger_px.as_deref(),
            "newTpOrdPx",
            self.new_tp_ord_px.as_deref(),
            "newTpTriggerPxType",
            self.new_tp_trigger_px_type.as_deref(),
        )?;
        validate_amended_tp_sl(
            "newSlTriggerPx",
            self.new_sl_trigger_px.as_deref(),
            "newSlOrdPx",
            self.new_sl_ord_px.as_deref(),
            "newSlTriggerPxType",
            self.new_sl_trigger_px_type.as_deref(),
        )
    }
}

fn validate_amended_tp_sl(
    trigger_field: &'static str,
    trigger_px: Option<&str>,
    order_field: &'static str,
    order_px: Option<&str>,
    type_field: &'static str,
    trigger_type: Option<&str>,
) -> Result<(), RequestValidationError> {
    optional_non_empty(trigger_field, trigger_px)?;
    optional_non_empty(order_field, order_px)?;
    optional_one_of(
        type_field,
        trigger_type,
        PRICE_TYPES,
        "last, index, or mark",
    )?;

    let deleting = trigger_px == Some("0") || order_px == Some("0");
    if deleting {
        return Ok(());
    }

    if trigger_px.is_some() != order_px.is_some() {
        return Err(RequestValidationError::RequiredWhen {
            field: if trigger_px.is_some() {
                order_field
            } else {
                trigger_field
            },
            condition: "the paired amended TP/SL price field is present",
        });
    }
    if trigger_px.is_some() && trigger_type.is_none() {
        return Err(RequestValidationError::RequiredWhen {
            field: type_field,
            condition: "a TP/SL trigger is added or amended",
        });
    }
    Ok(())
}

/// Query parameters shared by pending and historical algo orders.
#[derive(Debug, Clone, Serialize)]
pub struct AlgoOrderListRequest {
    #[serde(rename = "ordType")]
    ord_type: String,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<String>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl AlgoOrderListRequest {
    /// Create a query for one documented algo order type.
    pub fn new(ord_type: impl Into<String>) -> Self {
        Self {
            ord_type: ord_type.into(),
            algo_id: None,
            inst_type: None,
            inst_id: None,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Filter by algo ID.
    pub fn algo_id(mut self, value: impl Into<String>) -> Self {
        self.algo_id = Some(value.into());
        self
    }

    /// Filter by instrument type.
    pub fn inst_type(mut self, value: impl Into<String>) -> Self {
        self.inst_type = Some(value.into());
        self
    }

    /// Filter by instrument ID.
    pub fn inst_id(mut self, value: impl Into<String>) -> Self {
        self.inst_id = Some(value.into());
        self
    }

    /// Return records before this algo-ID cursor.
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records after this algo-ID cursor.
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the number of rows, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

fn validate_query_algo_order_type(value: &str) -> Result<(), RequestValidationError> {
    if matches!(value, "conditional,oco" | "oco,conditional") {
        return Ok(());
    }

    one_of(
        "ordType",
        value,
        QUERY_ALGO_ORDER_TYPES,
        "conditional, oco, chase, trigger, move_order_stop, iceberg, twap, smart_iceberg, or conditional,oco",
    )
}

impl ValidateRequest for AlgoOrderListRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        validate_query_algo_order_type(&self.ord_type)?;
        optional_non_empty("algoId", self.algo_id.as_deref())?;
        optional_one_of(
            "instType",
            self.inst_type.as_deref(),
            &["SPOT", "MARGIN", "SWAP", "FUTURES"],
            "SPOT, MARGIN, SWAP, or FUTURES",
        )?;
        optional_non_empty("instId", self.inst_id.as_deref())?;
        optional_unsigned_integer_string("after", self.after.as_deref())?;
        optional_unsigned_integer_string("before", self.before.as_deref())?;
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}

/// Query parameters for historical algo orders.
#[derive(Debug, Clone, Serialize)]
pub struct AlgoOrderHistoryRequest {
    #[serde(flatten)]
    common: AlgoOrderListRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl AlgoOrderHistoryRequest {
    /// Create a history query for one documented algo order type.
    pub fn new(ord_type: impl Into<String>) -> Self {
        Self {
            common: AlgoOrderListRequest::new(ord_type),
            state: None,
        }
    }

    /// Filter by historical state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Filter by OKX algo order ID.
    pub fn algo_id(mut self, value: impl Into<String>) -> Self {
        self.common = self.common.algo_id(value);
        self
    }

    /// Filter by instrument type.
    pub fn inst_type(mut self, value: impl Into<String>) -> Self {
        self.common = self.common.inst_type(value);
        self
    }

    /// Filter by instrument ID.
    pub fn inst_id(mut self, value: impl Into<String>) -> Self {
        self.common = self.common.inst_id(value);
        self
    }

    /// Return records before this cursor.
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.common = self.common.after(value);
        self
    }

    /// Return records after this cursor.
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.common = self.common.before(value);
        self
    }

    /// Set the number of rows, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.common = self.common.limit(value);
        self
    }
}

impl ValidateRequest for AlgoOrderHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        self.common.validate()?;
        optional_one_of(
            "state",
            self.state.as_deref(),
            &["effective", "canceled", "order_failed"],
            "effective, canceled, or order_failed",
        )?;
        at_least_one(
            "state, algoId",
            &[self.state.is_some(), self.common.algo_id.is_some()],
        )
    }
}

/// Query parameters for `GET /api/v5/trade/order-algo`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AlgoOrderDetailsRequest {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<String>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<String>,
}

impl AlgoOrderDetailsRequest {
    /// Query by OKX algo ID.
    pub fn by_algo_id(value: impl Into<String>) -> Self {
        Self {
            algo_id: Some(value.into()),
            algo_cl_ord_id: None,
        }
    }

    /// Query by client-supplied algo ID.
    pub fn by_client_algo_order_id(value: impl Into<String>) -> Self {
        Self {
            algo_id: None,
            algo_cl_ord_id: Some(value.into()),
        }
    }
}

impl ValidateRequest for AlgoOrderDetailsRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("algoId", self.algo_id.as_deref())?;
        validate_client_request_id("algoClOrdId", self.algo_cl_ord_id.as_deref())?;
        at_least_one(
            "algoId, algoClOrdId",
            &[self.algo_id.is_some(), self.algo_cl_ord_id.is_some()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_order_requires_trigger_prices() {
        let request = AlgoOrderRequest::new("BTC-USDT", "cash", "buy", "trigger", "1");
        assert!(request.validate().is_err());
    }

    #[test]
    fn pending_query_validates_limit() {
        let request = AlgoOrderListRequest::new("conditional").limit(101);
        assert!(request.validate().is_err());
    }

    #[test]
    fn pending_query_accepts_combined_conditional_and_oco_type() {
        let request = AlgoOrderListRequest::new("conditional,oco");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn history_query_requires_state_or_algo_id() {
        assert!(
            AlgoOrderHistoryRequest::new("conditional")
                .validate()
                .is_err()
        );
        assert!(
            AlgoOrderHistoryRequest::new("conditional")
                .state("effective")
                .validate()
                .is_ok()
        );
        assert!(
            AlgoOrderHistoryRequest::new("conditional")
                .algo_id("123")
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn twap_official_example_validates_and_serializes() {
        let request = AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "buy", "twap", "10")
            .position_side("net")
            .twap_by_spread("10", "10", "100", "10");

        request.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "instId": "BTC-USDT-SWAP",
                "tdMode": "cross",
                "side": "buy",
                "ordType": "twap",
                "sz": "10",
                "posSide": "net",
                "pxSpread": "10",
                "szLimit": "10",
                "pxLimit": "100",
                "timeInterval": "10"
            })
        );
    }

    #[test]
    fn smart_iceberg_official_example_validates_and_serializes() {
        let trigger = SmartIcebergTriggerRequest::new("price")
            .price("90000")
            .condition("cross_down");
        let request = AlgoOrderRequest::new("BTC-USDT", "cash", "buy", "smart_iceberg", "100")
            .smart_iceberg("10", "5", "conservative")
            .price_limit("95000")
            .smart_iceberg_triggers(vec![trigger]);

        request.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "instId": "BTC-USDT",
                "tdMode": "cash",
                "side": "buy",
                "ordType": "smart_iceberg",
                "sz": "100",
                "szLimit": "10",
                "pxLimit": "95000",
                "lmtOrderNumber": "5",
                "aggressiveness": "conservative",
                "triggerParams": [{
                    "triggerAction": "start",
                    "triggerStrategy": "price",
                    "triggerPx": "90000",
                    "triggerCond": "cross_down"
                }]
            })
        );
    }

    #[test]
    fn full_close_requires_market_execution_prices() {
        let invalid = AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "sell", "conditional", "1")
            .full_close()
            .position_side("net")
            .reduce_only(true)
            .stop_loss("50000", "49900");
        assert!(invalid.validate().is_err());

        let valid = AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "sell", "conditional", "1")
            .full_close()
            .position_side("net")
            .reduce_only(true)
            .stop_loss("50000", "-1");
        assert!(valid.validate().is_ok());
    }

    #[test]
    fn attached_ratios_follow_order_side_ranges() {
        let sell_take_profit = AttachedAlgoOrderRequest::new().take_profit_ratio("-0.3", "-1");
        let valid = AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "sell", "trigger", "1")
            .trigger("50000", "-1")
            .attached_algo_orders(vec![sell_take_profit]);
        assert!(valid.validate().is_ok());

        let invalid_ratio = AttachedAlgoOrderRequest::new().take_profit_ratio("0.3", "-1");
        let invalid = AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "sell", "trigger", "1")
            .trigger("50000", "-1")
            .attached_algo_orders(vec![invalid_ratio]);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn amend_allows_both_identifiers_and_documented_delete_sentinel() {
        let request = AmendAlgoOrderRequest::new("BTC-USDT")
            .algo_id("1")
            .client_algo_order_id("client1")
            .delete_take_profit();
        request.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "instId": "BTC-USDT",
                "algoId": "1",
                "algoClOrdId": "client1",
                "newTpTriggerPx": "0"
            })
        );
    }

    #[test]
    fn smart_iceberg_instant_trigger_rejects_strategy_fields() {
        let trigger = SmartIcebergTriggerRequest::new("instant").price("90000");
        assert!(trigger.validate().is_err());
    }

    #[test]
    fn algo_details_requires_identifier() {
        assert!(AlgoOrderDetailsRequest::default().validate().is_err());
    }
}
