use std::borrow::Cow;

use serde::Serialize;

use crate::model::{InstType, OrderSide, OrderType, PositionSide, TradeMode};

/// Request body for `POST /api/v5/trade/mass-cancel`.
///
/// OKX currently supports this endpoint only for option instruments, so the
/// instrument type is fixed to `OPTION` by [`MassCancelRequest::option`].
#[derive(Debug, Clone, Serialize)]
pub struct MassCancelRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "instFamily")]
    inst_family: Cow<'a, str>,
    #[serde(rename = "lockInterval", skip_serializing_if = "Option::is_none")]
    lock_interval: Option<Cow<'a, str>>,
}

impl<'a> MassCancelRequest<'a> {
    /// Create an option mass-cancel request for an instrument family.
    pub fn option(inst_family: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_type: InstType::Option,
            inst_family: inst_family.into(),
            lock_interval: None,
        }
    }

    /// Set the lock interval in milliseconds (`0` through `10000`).
    pub fn lock_interval(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.lock_interval = Some(value.into());
        self
    }
}

/// Request body for `POST /api/v5/trade/cancel-all-after`.
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllAfterRequest<'a> {
    #[serde(rename = "timeOut")]
    time_out: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
}

impl<'a> CancelAllAfterRequest<'a> {
    /// Create a request with timeout `0` (disabled) or `10` through `120` seconds.
    pub fn new(time_out: impl Into<Cow<'a, str>>) -> Self {
        Self {
            time_out: time_out.into(),
            tag: None,
        }
    }

    /// Set the optional order tag (up to 16 alphanumeric characters).
    pub fn tag(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(value.into());
        self
    }
}

/// An attached algo order in an [`OrderPrecheckRequest`].
///
/// Only fields documented for `POST /api/v5/trade/order-precheck` are exposed.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderPrecheckAttachedAlgoOrderRequest<'a> {
    #[serde(rename = "attachAlgoClOrdId", skip_serializing_if = "Option::is_none")]
    attach_algo_cl_ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "tpTriggerPx", skip_serializing_if = "Option::is_none")]
    tp_trigger_px: Option<Cow<'a, str>>,
    #[serde(rename = "tpOrdPx", skip_serializing_if = "Option::is_none")]
    tp_ord_px: Option<Cow<'a, str>>,
    #[serde(rename = "tpOrdKind", skip_serializing_if = "Option::is_none")]
    tp_ord_kind: Option<Cow<'a, str>>,
    #[serde(rename = "slTriggerPx", skip_serializing_if = "Option::is_none")]
    sl_trigger_px: Option<Cow<'a, str>>,
    #[serde(rename = "slOrdPx", skip_serializing_if = "Option::is_none")]
    sl_ord_px: Option<Cow<'a, str>>,
    #[serde(rename = "tpTriggerPxType", skip_serializing_if = "Option::is_none")]
    tp_trigger_px_type: Option<Cow<'a, str>>,
    #[serde(rename = "slTriggerPxType", skip_serializing_if = "Option::is_none")]
    sl_trigger_px_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sz: Option<Cow<'a, str>>,
    #[serde(rename = "callbackRatio", skip_serializing_if = "Option::is_none")]
    callback_ratio: Option<Cow<'a, str>>,
    #[serde(rename = "callbackSpread", skip_serializing_if = "Option::is_none")]
    callback_spread: Option<Cow<'a, str>>,
    #[serde(rename = "activePx", skip_serializing_if = "Option::is_none")]
    active_px: Option<Cow<'a, str>>,
}

impl<'a> OrderPrecheckAttachedAlgoOrderRequest<'a> {
    /// Create an empty attached algo definition.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the client-supplied attached algo order ID.
    pub fn client_algo_order_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.attach_algo_cl_ord_id = Some(value.into());
        self
    }

    /// Configure a take-profit trigger and order price.
    pub fn take_profit(
        mut self,
        trigger_px: impl Into<Cow<'a, str>>,
        order_px: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.tp_trigger_px = Some(trigger_px.into());
        self.tp_ord_px = Some(order_px.into());
        self
    }

    /// Set the take-profit order kind (`condition` or `limit`).
    pub fn take_profit_order_kind(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.tp_ord_kind = Some(value.into());
        self
    }

    /// Configure a stop-loss trigger and order price.
    pub fn stop_loss(
        mut self,
        trigger_px: impl Into<Cow<'a, str>>,
        order_px: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.sl_trigger_px = Some(trigger_px.into());
        self.sl_ord_px = Some(order_px.into());
        self
    }

    /// Set the take-profit trigger price source (`last`, `index`, or `mark`).
    pub fn take_profit_price_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.tp_trigger_px_type = Some(value.into());
        self
    }

    /// Set the stop-loss trigger price source (`last`, `index`, or `mark`).
    pub fn stop_loss_price_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.sl_trigger_px_type = Some(value.into());
        self
    }

    /// Set the size for one split take-profit order.
    pub fn size(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.sz = Some(value.into());
        self
    }

    /// Set the callback ratio for a trailing stop.
    pub fn callback_ratio(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.callback_ratio = Some(value.into());
        self
    }

    /// Set the callback spread for a trailing stop.
    pub fn callback_spread(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.callback_spread = Some(value.into());
        self
    }

    /// Set the activation price for a trailing stop.
    pub fn active_price(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.active_px = Some(value.into());
        self
    }
}

/// Request body for `POST /api/v5/trade/order-precheck`.
///
/// The endpoint is available in multi-currency margin and portfolio margin
/// account modes.
#[derive(Debug, Clone, Serialize)]
pub struct OrderPrecheckRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "tdMode")]
    td_mode: TradeMode,
    side: OrderSide,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
    #[serde(rename = "ordType")]
    ord_type: OrderType,
    sz: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    px: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outcome: Option<Cow<'a, str>>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    tgt_ccy: Option<Cow<'a, str>>,
    #[serde(rename = "attachAlgoOrds", skip_serializing_if = "Option::is_none")]
    attach_algo_ords: Option<Vec<OrderPrecheckAttachedAlgoOrderRequest<'a>>>,
}

impl<'a> OrderPrecheckRequest<'a> {
    /// Create an order precheck request with its required fields.
    pub fn new(
        inst_id: impl Into<Cow<'a, str>>,
        td_mode: TradeMode,
        side: OrderSide,
        ord_type: OrderType,
        sz: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            side,
            pos_side: None,
            ord_type,
            sz: sz.into(),
            px: None,
            outcome: None,
            reduce_only: None,
            tgt_ccy: None,
            attach_algo_ords: None,
        }
    }

    /// Set the position side (`net`, `long`, or `short`).
    pub fn position_side(mut self, value: PositionSide) -> Self {
        self.pos_side = Some(value);
        self
    }

    /// Set the order price.
    pub fn price(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.px = Some(value.into());
        self
    }

    /// Set the event-contract outcome (`yes` or `no`).
    pub fn outcome(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.outcome = Some(value.into());
        self
    }

    /// Set the reduce-only flag.
    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }

    /// Set the spot market-order quantity unit (`base_ccy` or `quote_ccy`).
    pub fn target_ccy(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.tgt_ccy = Some(value.into());
        self
    }

    /// Set attached take-profit, stop-loss, or trailing-stop definitions.
    pub fn attached_algo_orders<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = OrderPrecheckAttachedAlgoOrderRequest<'a>>,
    {
        self.attach_algo_ords = Some(values.into_iter().collect());
        self
    }
}
