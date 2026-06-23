use crate::client::OkxClient;
use crate::error::Error;
use crate::model::EmptyRequest;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the authenticated trading endpoints.
///
/// Obtain one via [`OkxClient::trade`](crate::OkxClient::trade). All methods
/// require credentials.
pub struct Trade<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Trade<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Place an order.
    ///
    /// `POST /api/v5/trade/order`. Authenticated. Build the request with
    /// [`PlaceOrderRequest::new`] plus optional setters. The returned vector
    /// contains one [`PlaceOrderResult`]; inspect its
    /// [`s_code`](PlaceOrderResult::s_code) to confirm acceptance (`"0"`).
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) without credentials, [`RestError::Okx`](crate::RestError::Okx) on a
    /// non-zero top-level OKX code, or transport/decode errors.
    pub async fn place_order(
        &self,
        request: &PlaceOrderRequest<'_>,
    ) -> Result<Vec<PlaceOrderResult>, Error> {
        self.client.post(ORDER, request, true).await
    }

    /// Place multiple orders.
    ///
    /// `POST /api/v5/trade/batch-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn place_multiple_orders(
        &self,
        requests: &[PlaceOrderRequest<'_>],
    ) -> Result<Vec<PlaceOrderResult>, Error> {
        self.client.post(BATCH_ORDERS, &requests, true).await
    }

    /// Cancel an order by its OKX order ID.
    ///
    /// `POST /api/v5/trade/cancel-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest<'_>,
    ) -> Result<Vec<CancelOrderResult>, Error> {
        self.client.post(CANCEL_ORDER, request, true).await
    }

    /// Cancel multiple orders.
    ///
    /// `POST /api/v5/trade/cancel-batch-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn cancel_multiple_orders(
        &self,
        requests: &[CancelOrderRequest<'_>],
    ) -> Result<Vec<CancelOrderResult>, Error> {
        self.client.post(CANCEL_BATCH_ORDERS, &requests, true).await
    }

    /// Amend an existing order.
    ///
    /// `POST /api/v5/trade/amend-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn amend_order(
        &self,
        request: &AmendOrderRequest<'_>,
    ) -> Result<Vec<AmendOrderResult>, Error> {
        self.client.post(AMEND_ORDER, request, true).await
    }

    /// Amend multiple existing orders.
    ///
    /// `POST /api/v5/trade/amend-batch-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn amend_multiple_orders(
        &self,
        requests: &[AmendOrderRequest<'_>],
    ) -> Result<Vec<AmendOrderResult>, Error> {
        self.client.post(AMEND_BATCH_ORDERS, &requests, true).await
    }

    /// Close positions for an instrument.
    ///
    /// `POST /api/v5/trade/close-position`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn close_positions(
        &self,
        request: &ClosePositionRequest<'_>,
    ) -> Result<Vec<ClosePositionResult>, Error> {
        self.client.post(CLOSE_POSITION, request, true).await
    }

    /// Retrieve the details of a single order by its OKX order ID.
    ///
    /// `GET /api/v5/trade/order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_order(&self, request: &GetOrderRequest<'_>) -> Result<Vec<Order>, Error> {
        self.client.get(ORDER, request, true).await
    }

    /// Retrieve pending orders.
    ///
    /// `GET /api/v5/trade/orders-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_order_list(
        &self,
        request: &OrderListRequest<'_>,
    ) -> Result<Vec<Order>, Error> {
        self.client.get(ORDERS_PENDING, request, true).await
    }

    /// Retrieve order history for the recent window.
    ///
    /// `GET /api/v5/trade/orders-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_orders_history(
        &self,
        request: &OrderHistoryRequest<'_>,
    ) -> Result<Vec<Order>, Error> {
        self.client.get(ORDERS_HISTORY, request, true).await
    }

    /// Retrieve archived order history.
    ///
    /// `GET /api/v5/trade/orders-history-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_orders_history_archive(
        &self,
        request: &OrderHistoryRequest<'_>,
    ) -> Result<Vec<Order>, Error> {
        self.client.get(ORDERS_HISTORY_ARCHIVE, request, true).await
    }

    /// Retrieve recent fills.
    ///
    /// `GET /api/v5/trade/fills`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_fills(&self, request: &FillsRequest<'_>) -> Result<Vec<Fill>, Error> {
        self.client.get(FILLS, request, true).await
    }

    /// Retrieve historical fills.
    ///
    /// `GET /api/v5/trade/fills-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_fills_history(
        &self,
        request: &FillHistoryRequest<'_>,
    ) -> Result<Vec<FillHistory>, Error> {
        self.client.get(FILLS_HISTORY, request, true).await
    }

    /// Place an algo order.
    ///
    /// `POST /api/v5/trade/order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn place_algo_order(
        &self,
        request: &AlgoOrderRequest,
    ) -> Result<Vec<AlgoOrderResult>, Error> {
        self.client.post(ORDER_ALGO, request, true).await
    }

    /// Cancel algo orders.
    ///
    /// `POST /api/v5/trade/cancel-algos`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn cancel_algo_orders(
        &self,
        requests: &[CancelAlgoOrderRequest<'_>],
    ) -> Result<Vec<AlgoOrderResult>, Error> {
        self.client.post(CANCEL_ALGOS, &requests, true).await
    }

    /// Amend an algo order.
    ///
    /// `POST /api/v5/trade/amend-algos`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn amend_algo_order(
        &self,
        request: &AmendAlgoOrderRequest<'_>,
    ) -> Result<Vec<AlgoOrderResult>, Error> {
        self.client.post(AMEND_ALGOS, request, true).await
    }

    /// Retrieve pending algo orders.
    ///
    /// `GET /api/v5/trade/orders-algo-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_algo_order_list(
        &self,
        request: &AlgoOrderListRequest<'_>,
    ) -> Result<Vec<AlgoOrder>, Error> {
        self.client.get(ORDERS_ALGO_PENDING, request, true).await
    }

    /// Retrieve algo order history.
    ///
    /// `GET /api/v5/trade/orders-algo-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_algo_orders_history(
        &self,
        request: &AlgoOrderHistoryRequest<'_>,
    ) -> Result<Vec<AlgoOrder>, Error> {
        self.client.get(ORDERS_ALGO_HISTORY, request, true).await
    }

    /// Retrieve details for an algo order.
    ///
    /// `GET /api/v5/trade/order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_algo_order_details(
        &self,
        request: &AlgoOrderDetailsRequest<'_>,
    ) -> Result<Vec<AlgoOrder>, Error> {
        self.client.get(ORDER_ALGO_DETAILS, request, true).await
    }

    /// Retrieve the easy-convert currency list.
    ///
    /// `GET /api/v5/trade/easy-convert-currency-list`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_easy_convert_currency_list(&self) -> Result<Vec<EasyConvertCurrency>, Error> {
        self.client
            .get(EASY_CONVERT_CURRENCY_LIST, &EmptyRequest {}, true)
            .await
    }

    /// Execute an easy-convert request.
    ///
    /// `POST /api/v5/trade/easy-convert`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn easy_convert(
        &self,
        request: &EasyConvertRequest<'_>,
    ) -> Result<Vec<EasyConvertResult>, Error> {
        self.client.post(EASY_CONVERT, request, true).await
    }

    /// Retrieve easy-convert history.
    ///
    /// `GET /api/v5/trade/easy-convert-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_easy_convert_history(
        &self,
        request: &EasyConvertHistoryRequest<'_>,
    ) -> Result<Vec<EasyConvertHistory>, Error> {
        self.client.get(EASY_CONVERT_HISTORY, request, true).await
    }

    /// Retrieve one-click-repay currency pairs.
    ///
    /// `GET /api/v5/trade/one-click-repay-currency-list`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_one_click_repay_currency_list(
        &self,
        request: &OneClickRepayCurrencyListRequest<'_>,
    ) -> Result<Vec<OneClickRepayCurrency>, Error> {
        self.client
            .get(ONE_CLICK_REPAY_CURRENCY_LIST, request, true)
            .await
    }

    /// Execute one-click repay.
    ///
    /// `POST /api/v5/trade/one-click-repay`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn one_click_repay(
        &self,
        request: &OneClickRepayRequest<'_>,
    ) -> Result<Vec<OneClickRepayResult>, Error> {
        self.client.post(ONE_CLICK_REPAY, request, true).await
    }

    /// Retrieve one-click-repay history.
    ///
    /// `GET /api/v5/trade/one-click-repay-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_one_click_repay_history(
        &self,
        request: &OneClickRepayHistoryRequest<'_>,
    ) -> Result<Vec<OneClickRepayHistory>, Error> {
        self.client
            .get(ONE_CLICK_REPAY_HISTORY, request, true)
            .await
    }

    /// Retrieve one-click-repay v2 currency pairs.
    ///
    /// `GET /api/v5/trade/one-click-repay-currency-list-v2`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_one_click_repay_currency_list_v2(
        &self,
        request: &OneClickRepayCurrencyListRequest<'_>,
    ) -> Result<Vec<OneClickRepayCurrency>, Error> {
        self.client
            .get(ONE_CLICK_REPAY_CURRENCY_LIST_V2, request, true)
            .await
    }

    /// Execute one-click repay v2.
    ///
    /// `POST /api/v5/trade/one-click-repay-v2`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn one_click_repay_v2(
        &self,
        request: &OneClickRepayRequest<'_>,
    ) -> Result<Vec<OneClickRepayResult>, Error> {
        self.client.post(ONE_CLICK_REPAY_V2, request, true).await
    }

    /// Retrieve one-click-repay v2 history.
    ///
    /// `GET /api/v5/trade/one-click-repay-history-v2`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_one_click_repay_history_v2(
        &self,
        request: &OneClickRepayHistoryRequest<'_>,
    ) -> Result<Vec<OneClickRepayHistory>, Error> {
        self.client
            .get(ONE_CLICK_REPAY_HISTORY_V2, request, true)
            .await
    }
}
