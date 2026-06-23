use std::borrow::Cow;

use serde::Serialize;

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

/// Request body for one entry in `POST /api/v5/trade/cancel-algos`.
#[derive(Debug, Clone, Serialize)]
pub struct CancelAlgoOrderRequest<'a> {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<Cow<'a, str>>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
}

impl<'a> CancelAlgoOrderRequest<'a> {
    /// Create a cancellation using an OKX algo ID.
    pub fn new(algo_id: impl Into<Cow<'a, str>>, inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            algo_id: Some(algo_id.into()),
            algo_cl_ord_id: None,
            inst_id: inst_id.into(),
        }
    }

    /// Create a cancellation using a client-supplied algo ID.
    pub fn by_client_algo_order_id(
        algo_cl_ord_id: impl Into<Cow<'a, str>>,
        inst_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            algo_id: None,
            algo_cl_ord_id: Some(algo_cl_ord_id.into()),
            inst_id: inst_id.into(),
        }
    }
}

/// Request body for `POST /api/v5/trade/amend-algos`.
#[derive(Debug, Clone, Serialize)]
pub struct AmendAlgoOrderRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<Cow<'a, str>>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "reqId", skip_serializing_if = "Option::is_none")]
    req_id: Option<Cow<'a, str>>,
    #[serde(rename = "newSz", skip_serializing_if = "Option::is_none")]
    new_sz: Option<Cow<'a, str>>,
    #[serde(rename = "newTpTriggerPx", skip_serializing_if = "Option::is_none")]
    new_tp_trigger_px: Option<Cow<'a, str>>,
    #[serde(rename = "newTpOrdPx", skip_serializing_if = "Option::is_none")]
    new_tp_ord_px: Option<Cow<'a, str>>,
    #[serde(rename = "newTpTriggerPxType", skip_serializing_if = "Option::is_none")]
    new_tp_trigger_px_type: Option<Cow<'a, str>>,
    #[serde(rename = "newSlTriggerPx", skip_serializing_if = "Option::is_none")]
    new_sl_trigger_px: Option<Cow<'a, str>>,
    #[serde(rename = "newSlOrdPx", skip_serializing_if = "Option::is_none")]
    new_sl_ord_px: Option<Cow<'a, str>>,
    #[serde(rename = "newSlTriggerPxType", skip_serializing_if = "Option::is_none")]
    new_sl_trigger_px_type: Option<Cow<'a, str>>,
    #[serde(rename = "cxlOnFail", skip_serializing_if = "Option::is_none")]
    cancel_on_fail: Option<bool>,
}

impl<'a> AmendAlgoOrderRequest<'a> {
    /// Create an amendment for an instrument; add an algo identifier next.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
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
    pub fn algo_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.algo_id = Some(value.into());
        self
    }

    /// Identify the order by client-supplied algo ID.
    pub fn client_algo_order_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.algo_cl_ord_id = Some(value.into());
        self
    }

    /// Set a client request ID of up to 32 ASCII alphanumeric characters.
    pub fn request_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.req_id = Some(value.into());
        self
    }

    /// Amend the order size.
    pub fn new_size(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.new_sz = Some(value.into());
        self
    }

    /// Amend take-profit prices.
    pub fn take_profit(
        mut self,
        trigger_px: impl Into<Cow<'a, str>>,
        order_px: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.new_tp_trigger_px = Some(trigger_px.into());
        self.new_tp_ord_px = Some(order_px.into());
        self
    }

    /// Set the amended take-profit trigger price source.
    pub fn take_profit_price_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.new_tp_trigger_px_type = Some(value.into());
        self
    }

    /// Delete the take-profit definition by sending the documented `0` sentinel.
    pub fn delete_take_profit(mut self) -> Self {
        self.new_tp_trigger_px = Some(Cow::Borrowed("0"));
        self.new_tp_ord_px = None;
        self.new_tp_trigger_px_type = None;
        self
    }

    /// Amend stop-loss prices.
    pub fn stop_loss(
        mut self,
        trigger_px: impl Into<Cow<'a, str>>,
        order_px: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.new_sl_trigger_px = Some(trigger_px.into());
        self.new_sl_ord_px = Some(order_px.into());
        self
    }

    /// Set the amended stop-loss trigger price source.
    pub fn stop_loss_price_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.new_sl_trigger_px_type = Some(value.into());
        self
    }

    /// Delete the stop-loss definition by sending the documented `0` sentinel.
    pub fn delete_stop_loss(mut self) -> Self {
        self.new_sl_trigger_px = Some(Cow::Borrowed("0"));
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

/// Query parameters shared by pending and historical algo orders.
#[derive(Debug, Clone, Serialize)]
pub struct AlgoOrderListRequest<'a> {
    #[serde(rename = "ordType")]
    ord_type: Cow<'a, str>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<Cow<'a, str>>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> AlgoOrderListRequest<'a> {
    /// Create a query for one documented algo order type.
    pub fn new(ord_type: impl Into<Cow<'a, str>>) -> Self {
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
    pub fn algo_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.algo_id = Some(value.into());
        self
    }

    /// Filter by instrument type.
    pub fn inst_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_type = Some(value.into());
        self
    }

    /// Filter by instrument ID.
    pub fn inst_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(value.into());
        self
    }

    /// Return records before this algo-ID cursor.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records after this algo-ID cursor.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the number of rows, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// Query parameters for historical algo orders.
#[derive(Debug, Clone, Serialize)]
pub struct AlgoOrderHistoryRequest<'a> {
    #[serde(flatten)]
    common: AlgoOrderListRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
}

impl<'a> AlgoOrderHistoryRequest<'a> {
    /// Create a history query for one documented algo order type.
    pub fn new(ord_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            common: AlgoOrderListRequest::new(ord_type),
            state: None,
        }
    }

    /// Filter by historical state.
    pub fn state(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Filter by OKX algo order ID.
    pub fn algo_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.common = self.common.algo_id(value);
        self
    }

    /// Filter by instrument type.
    pub fn inst_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.common = self.common.inst_type(value);
        self
    }

    /// Filter by instrument ID.
    pub fn inst_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.common = self.common.inst_id(value);
        self
    }

    /// Return records before this cursor.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.common = self.common.after(value);
        self
    }

    /// Return records after this cursor.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.common = self.common.before(value);
        self
    }

    /// Set the number of rows, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.common = self.common.limit(value);
        self
    }
}

/// Query parameters for `GET /api/v5/trade/order-algo`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AlgoOrderDetailsRequest<'a> {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<Cow<'a, str>>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    algo_cl_ord_id: Option<Cow<'a, str>>,
}

impl<'a> AlgoOrderDetailsRequest<'a> {
    /// Query by OKX algo ID.
    pub fn by_algo_id(value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            algo_id: Some(value.into()),
            algo_cl_ord_id: None,
        }
    }

    /// Query by client-supplied algo ID.
    pub fn by_client_algo_order_id(value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            algo_id: None,
            algo_cl_ord_id: Some(value.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twap_official_example_validates_and_serializes() {
        let request = AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "buy", "twap", "10")
            .position_side("net")
            .twap_by_spread("10", "10", "100", "10");

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
    fn amend_allows_both_identifiers_and_documented_delete_sentinel() {
        let request = AmendAlgoOrderRequest::new("BTC-USDT")
            .algo_id("1")
            .client_algo_order_id("client1")
            .delete_take_profit();
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
}
