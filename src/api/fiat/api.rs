use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for authenticated fiat endpoints.
pub struct Fiat<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Fiat<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve fiat deposit payment methods.
    ///
    /// `GET /api/v5/fiat/deposit-payment-methods`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_deposit_payment_methods(
        &self,
        request: &FiatCurrencyRequest<'_>,
    ) -> Result<Vec<FiatPaymentMethod>, Error> {
        self.client
            .get(DEPOSIT_PAYMENT_METHODS, request, true)
            .await
    }

    /// Retrieve fiat withdrawal payment methods.
    ///
    /// `GET /api/v5/fiat/withdrawal-payment-methods`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_withdrawal_payment_methods(
        &self,
        request: &FiatCurrencyRequest<'_>,
    ) -> Result<Vec<FiatPaymentMethod>, Error> {
        self.client
            .get(WITHDRAWAL_PAYMENT_METHODS, request, true)
            .await
    }

    /// Create a fiat withdrawal order.
    ///
    /// `POST /api/v5/fiat/create-withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn create_withdrawal(
        &self,
        request: &CreateFiatWithdrawalRequest<'_>,
    ) -> Result<Vec<FiatOrder>, Error> {
        self.client.post(CREATE_WITHDRAWAL, request, true).await
    }

    /// Cancel a fiat withdrawal order.
    ///
    /// `POST /api/v5/fiat/cancel-withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_withdrawal(
        &self,
        request: &FiatOrderIdRequest<'_>,
    ) -> Result<Vec<CancelFiatWithdrawal>, Error> {
        self.client.post(CANCEL_WITHDRAWAL, request, true).await
    }

    /// Retrieve fiat withdrawal order history.
    ///
    /// `GET /api/v5/fiat/withdrawal-order-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_withdrawal_order_history(
        &self,
        request: &FiatOrderHistoryRequest<'_>,
    ) -> Result<Vec<FiatOrder>, Error> {
        self.client
            .get(WITHDRAWAL_ORDER_HISTORY, request, true)
            .await
    }

    /// Retrieve one fiat withdrawal order.
    ///
    /// `GET /api/v5/fiat/withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_withdrawal(
        &self,
        request: &FiatOrderIdRequest<'_>,
    ) -> Result<Vec<FiatOrder>, Error> {
        self.client.get(WITHDRAWAL, request, true).await
    }

    /// Retrieve fiat deposit order history.
    ///
    /// `GET /api/v5/fiat/deposit-order-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_deposit_order_history(
        &self,
        request: &FiatOrderHistoryRequest<'_>,
    ) -> Result<Vec<FiatOrder>, Error> {
        self.client.get(DEPOSIT_ORDER_HISTORY, request, true).await
    }

    /// Retrieve one fiat deposit order.
    ///
    /// `GET /api/v5/fiat/deposit`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_deposit(
        &self,
        request: &FiatOrderIdRequest<'_>,
    ) -> Result<Vec<FiatOrder>, Error> {
        self.client.get(DEPOSIT, request, true).await
    }

    /// Retrieve fiat buy/sell currencies.
    ///
    /// `GET /api/v5/fiat/buy-sell/currencies`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_buy_sell_currencies(&self) -> Result<Vec<FiatBuySellCurrencies>, Error> {
        self.client.get(BUY_SELL_CURRENCIES, &(), true).await
    }

    /// Retrieve a fiat buy/sell currency pair.
    ///
    /// `GET /api/v5/fiat/buy-sell/currency-pair`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_buy_sell_currency_pair(
        &self,
        request: &FiatBuySellPairRequest<'_>,
    ) -> Result<Vec<FiatBuySellPair>, Error> {
        self.client
            .get(BUY_SELL_CURRENCY_PAIR, request, true)
            .await
    }

    /// Request a fiat buy/sell quote.
    ///
    /// `POST /api/v5/fiat/buy-sell/quote`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_buy_sell_quote(
        &self,
        request: &FiatBuySellQuoteRequest<'_>,
    ) -> Result<Vec<FiatBuySellQuote>, Error> {
        self.client.post(BUY_SELL_QUOTE, request, true).await
    }

    /// Execute a fiat buy/sell quote.
    ///
    /// `POST /api/v5/fiat/buy-sell/trade`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn buy_sell_trade(
        &self,
        request: &FiatBuySellTradeRequest<'_>,
    ) -> Result<Vec<FiatBuySellTrade>, Error> {
        self.client.post(BUY_SELL_TRADE, request, true).await
    }

    /// Retrieve fiat buy/sell history.
    ///
    /// `GET /api/v5/fiat/buy-sell/history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_buy_sell_history(
        &self,
        request: &FiatBuySellHistoryRequest<'_>,
    ) -> Result<Vec<FiatBuySellTrade>, Error> {
        self.client.get(BUY_SELL_HISTORY, request, true).await
    }
}
