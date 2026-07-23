use std::borrow::Cow;

use serde::Serialize;

/// Query for a fiat currency's deposit or withdrawal payment methods.
#[derive(Debug, Clone, Serialize)]
pub struct FiatCurrencyRequest<'a> {
    ccy: Cow<'a, str>,
}

impl<'a> FiatCurrencyRequest<'a> {
    /// Select an ISO-4217 fiat currency.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self { ccy: ccy.into() }
    }
}

/// Request body for creating a fiat withdrawal.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFiatWithdrawalRequest<'a> {
    payment_acct_id: Cow<'a, str>,
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
    payment_method: Cow<'a, str>,
    client_id: Cow<'a, str>,
}

impl<'a> CreateFiatWithdrawalRequest<'a> {
    /// Create a fiat withdrawal request using a documented payment account.
    pub fn new(
        payment_acct_id: impl Into<Cow<'a, str>>,
        ccy: impl Into<Cow<'a, str>>,
        amt: impl Into<Cow<'a, str>>,
        payment_method: impl Into<Cow<'a, str>>,
        client_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            payment_acct_id: payment_acct_id.into(),
            ccy: ccy.into(),
            amt: amt.into(),
            payment_method: payment_method.into(),
            client_id: client_id.into(),
        }
    }
}

/// Query or body identifying one fiat order.
#[derive(Debug, Clone, Serialize)]
pub struct FiatOrderIdRequest<'a> {
    #[serde(rename = "ordId")]
    ord_id: Cow<'a, str>,
}

impl<'a> FiatOrderIdRequest<'a> {
    /// Select a fiat order ID.
    pub fn new(ord_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ord_id: ord_id.into(),
        }
    }
}

/// Query for fiat deposit or withdrawal order history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatOrderHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> FiatOrderHistoryRequest<'a> {
    /// Create an unfiltered order-history query.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    pub fn payment_method(mut self, payment_method: impl Into<Cow<'a, str>>) -> Self {
        self.payment_method = Some(payment_method.into());
        self
    }

    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query for one fiat/crypto buy-sell pair.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatBuySellPairRequest<'a> {
    from_ccy: Cow<'a, str>,
    to_ccy: Cow<'a, str>,
}

impl<'a> FiatBuySellPairRequest<'a> {
    pub fn new(
        from_ccy: impl Into<Cow<'a, str>>,
        to_ccy: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            from_ccy: from_ccy.into(),
            to_ccy: to_ccy.into(),
        }
    }
}

/// Side accepted by fiat buy/sell quote and trade requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FiatBuySellSide {
    Buy,
    Sell,
}

/// Request body for a fiat buy/sell quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatBuySellQuoteRequest<'a> {
    side: FiatBuySellSide,
    from_ccy: Cow<'a, str>,
    to_ccy: Cow<'a, str>,
    rfq_amt: Cow<'a, str>,
    rfq_ccy: Cow<'a, str>,
}

impl<'a> FiatBuySellQuoteRequest<'a> {
    pub fn new(
        side: FiatBuySellSide,
        from_ccy: impl Into<Cow<'a, str>>,
        to_ccy: impl Into<Cow<'a, str>>,
        rfq_amt: impl Into<Cow<'a, str>>,
        rfq_ccy: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            side,
            from_ccy: from_ccy.into(),
            to_ccy: to_ccy.into(),
            rfq_amt: rfq_amt.into(),
            rfq_ccy: rfq_ccy.into(),
        }
    }
}

/// Request body for executing a fiat buy/sell quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatBuySellTradeRequest<'a> {
    quote_id: Cow<'a, str>,
    side: FiatBuySellSide,
    from_ccy: Cow<'a, str>,
    to_ccy: Cow<'a, str>,
    rfq_amt: Cow<'a, str>,
    rfq_ccy: Cow<'a, str>,
    payment_method: Cow<'a, str>,
    cl_ord_id: Cow<'a, str>,
}

impl<'a> FiatBuySellTradeRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        quote_id: impl Into<Cow<'a, str>>,
        side: FiatBuySellSide,
        from_ccy: impl Into<Cow<'a, str>>,
        to_ccy: impl Into<Cow<'a, str>>,
        rfq_amt: impl Into<Cow<'a, str>>,
        rfq_ccy: impl Into<Cow<'a, str>>,
        payment_method: impl Into<Cow<'a, str>>,
        cl_ord_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            quote_id: quote_id.into(),
            side,
            from_ccy: from_ccy.into(),
            to_ccy: to_ccy.into(),
            rfq_amt: rfq_amt.into(),
            rfq_ccy: rfq_ccy.into(),
            payment_method: payment_method.into(),
            cl_ord_id: cl_ord_id.into(),
        }
    }
}

/// Query for fiat buy/sell trade history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatBuySellHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> FiatBuySellHistoryRequest<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn order_id(mut self, ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }

    pub fn client_order_id(mut self, cl_ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
