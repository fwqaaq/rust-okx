use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for Nitro Spread endpoints.
///
/// Obtain one via [`OkxClient::spread`](crate::OkxClient::spread).
pub struct SpreadTrading<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> SpreadTrading<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve listed spreads.
    ///
    /// `GET /api/v5/sprd/spreads`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn get_spreads(&self, request: &SpreadsRequest) -> Result<Vec<Spread>, Error> {
        self.client.get(SPREADS, request, false).await
    }

    /// Retrieve a spread order book.
    ///
    /// `GET /api/v5/sprd/books`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn get_order_book(
        &self,
        request: &SpreadBooksRequest,
    ) -> Result<Vec<SpreadOrderBook>, Error> {
        self.client.get(BOOKS, request, false).await
    }

    /// Retrieve recent public spread trades.
    ///
    /// `GET /api/v5/sprd/public-trades`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn get_public_trades(
        &self,
        request: &SpreadPublicTradesRequest,
    ) -> Result<Vec<SpreadPublicTrade>, Error> {
        self.client.get(PUBLIC_TRADES, request, false).await
    }

    /// Place a spread order.
    ///
    /// `POST /api/v5/sprd/order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn place_order(
        &self,
        request: &SpreadOrderRequest,
    ) -> Result<Vec<SpreadOrderResult>, Error> {
        self.client.post(ORDER, request, true).await
    }

    /// Cancel one spread order.
    ///
    /// `POST /api/v5/sprd/cancel-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_order(
        &self,
        request: &SpreadOrderIdRequest,
    ) -> Result<Vec<SpreadOrderResult>, Error> {
        self.client.post(CANCEL_ORDER, request, true).await
    }

    /// Cancel all pending spread orders, optionally for one spread.
    ///
    /// `POST /api/v5/sprd/mass-cancel`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn mass_cancel(
        &self,
        request: &SpreadMassCancelRequest,
    ) -> Result<Vec<SpreadBooleanResult>, Error> {
        self.client.post(MASS_CANCEL, request, true).await
    }

    /// Amend one incomplete spread order.
    ///
    /// `POST /api/v5/sprd/amend-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_order(
        &self,
        request: &SpreadAmendOrderRequest,
    ) -> Result<Vec<SpreadAmendOrderResult>, Error> {
        self.client.post(AMEND_ORDER, request, true).await
    }

    /// Retrieve one spread order.
    ///
    /// `GET /api/v5/sprd/order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_order(
        &self,
        request: &SpreadOrderIdRequest,
    ) -> Result<Vec<SpreadOrder>, Error> {
        self.client.get(ORDER, request, true).await
    }

    /// Retrieve active spread orders.
    ///
    /// `GET /api/v5/sprd/orders-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_pending_orders(
        &self,
        request: &SpreadPendingOrdersRequest,
    ) -> Result<Vec<SpreadOrder>, Error> {
        self.client.get(ORDERS_PENDING, request, true).await
    }

    /// Retrieve spread orders from the last 21 days.
    ///
    /// `GET /api/v5/sprd/orders-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_orders_history(
        &self,
        request: &SpreadOrdersHistoryRequest,
    ) -> Result<Vec<SpreadOrder>, Error> {
        self.client.get(ORDERS_HISTORY, request, true).await
    }

    /// Retrieve archived spread orders from the last three months.
    ///
    /// `GET /api/v5/sprd/orders-history-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_orders_history_archive(
        &self,
        request: &SpreadOrdersArchiveRequest,
    ) -> Result<Vec<SpreadOrder>, Error> {
        self.client
            .get(ORDERS_HISTORY_ARCHIVE, request, true)
            .await
    }

    /// Retrieve private spread trades from the last seven days.
    ///
    /// `GET /api/v5/sprd/trades`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_trades(
        &self,
        request: &SpreadTradesRequest,
    ) -> Result<Vec<SpreadTrade>, Error> {
        self.client.get(TRADES, request, true).await
    }

    /// Configure cancel-all-after protection for spread orders.
    ///
    /// `POST /api/v5/sprd/cancel-all-after`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_all_after(
        &self,
        request: &SpreadCancelAllAfterRequest,
    ) -> Result<Vec<SpreadCancelAllAfter>, Error> {
        self.client.post(CANCEL_ALL_AFTER, request, true).await
    }
}
