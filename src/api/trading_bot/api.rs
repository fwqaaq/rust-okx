use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for trading-bot endpoint groups.
///
/// Obtain one via [`OkxClient::trading_bot`](crate::OkxClient::trading_bot).
pub struct TradingBot<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> TradingBot<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Access Grid Bot endpoints.
    pub fn grid(&self) -> Grid<'_, T> {
        Grid {
            client: self.client,
        }
    }
}

/// Accessor for Grid Bot endpoints.
pub struct Grid<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> Grid<'_, T> {
    /// Place a grid algo order.
    ///
    /// `POST /api/v5/tradingBot/grid/order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn place_order(
        &self,
        request: &GridOrderRequest,
    ) -> Result<Vec<GridActionResult>, Error> {
        self.client.post(GRID_ORDER, request, true).await
    }

    /// Amend core grid price parameters.
    ///
    /// `POST /api/v5/tradingBot/grid/amend-algo-basic-param`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_basic_parameters(
        &self,
        request: &GridAmendBasicRequest,
    ) -> Result<Vec<GridAmendBasicResult>, Error> {
        self.client.post(GRID_AMEND_BASIC, request, true).await
    }

    /// Amend grid stop settings or add spot investment.
    ///
    /// `POST /api/v5/tradingBot/grid/amend-order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_order(
        &self,
        request: &GridAmendRequest,
    ) -> Result<Vec<GridActionResult>, Error> {
        self.client.post(GRID_AMEND, request, true).await
    }

    /// Stop a grid algo order.
    ///
    /// `POST /api/v5/tradingBot/grid/stop-order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn stop_order(
        &self,
        request: &GridStopRequest,
    ) -> Result<Vec<GridActionResult>, Error> {
        self.client.post(GRID_STOP, request, true).await
    }

    /// Close a contract-grid position.
    ///
    /// `POST /api/v5/tradingBot/grid/close-position`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn close_position(
        &self,
        request: &GridClosePositionRequest,
    ) -> Result<Vec<GridCloseResult>, Error> {
        self.client.post(GRID_CLOSE_POSITION, request, true).await
    }

    /// Cancel a contract-grid close order.
    ///
    /// `POST /api/v5/tradingBot/grid/cancel-close-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_close_order(
        &self,
        request: &GridCancelCloseRequest,
    ) -> Result<Vec<GridCloseResult>, Error> {
        self.client.post(GRID_CANCEL_CLOSE, request, true).await
    }

    /// Trigger a grid algo immediately.
    ///
    /// `POST /api/v5/tradingBot/grid/order-instant-trigger`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn instant_trigger(
        &self,
        request: &GridInstantTriggerRequest,
    ) -> Result<Vec<GridInstantTriggerResult>, Error> {
        self.client.post(GRID_INSTANT_TRIGGER, request, true).await
    }

    /// Retrieve active grid algo orders.
    ///
    /// `GET /api/v5/tradingBot/grid/orders-algo-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_pending_orders(
        &self,
        request: &GridOrdersRequest,
    ) -> Result<Vec<GridAlgoOrder>, Error> {
        self.client.get(GRID_PENDING, request, true).await
    }

    /// Retrieve historical grid algo orders.
    ///
    /// `GET /api/v5/tradingBot/grid/orders-algo-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_orders_history(
        &self,
        request: &GridOrdersRequest,
    ) -> Result<Vec<GridAlgoOrder>, Error> {
        self.client.get(GRID_HISTORY, request, true).await
    }

    /// Retrieve one grid algo order.
    ///
    /// `GET /api/v5/tradingBot/grid/orders-algo-details`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_order_details(
        &self,
        request: &GridAlgoRequest,
    ) -> Result<Vec<GridAlgoOrder>, Error> {
        self.client.get(GRID_DETAILS, request, true).await
    }

    /// Retrieve grid sub-orders.
    ///
    /// `GET /api/v5/tradingBot/grid/sub-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_sub_orders(
        &self,
        request: &GridSubOrdersRequest,
    ) -> Result<Vec<GridSubOrder>, Error> {
        self.client.get(GRID_SUB_ORDERS, request, true).await
    }

    /// Retrieve contract-grid positions.
    ///
    /// `GET /api/v5/tradingBot/grid/positions`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_positions(
        &self,
        request: &GridAlgoRequest,
    ) -> Result<Vec<GridPosition>, Error> {
        self.client.get(GRID_POSITIONS, request, true).await
    }

    /// Withdraw spot-grid income.
    ///
    /// `POST /api/v5/tradingBot/grid/withdraw-income`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn withdraw_income(
        &self,
        request: &GridAlgoIdRequest,
    ) -> Result<Vec<GridWithdrawIncome>, Error> {
        self.client.post(GRID_WITHDRAW_INCOME, request, true).await
    }

    /// Compute a contract-grid margin adjustment.
    ///
    /// `POST /api/v5/tradingBot/grid/compute-margin-balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn compute_margin_balance(
        &self,
        request: &GridComputeMarginRequest,
    ) -> Result<Vec<GridMarginComputation>, Error> {
        self.client.post(GRID_COMPUTE_MARGIN, request, true).await
    }

    /// Adjust a contract-grid margin balance.
    ///
    /// `POST /api/v5/tradingBot/grid/margin-balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn adjust_margin_balance(
        &self,
        request: &GridMarginBalanceRequest,
    ) -> Result<Vec<GridAlgoResult>, Error> {
        self.client.post(GRID_MARGIN_BALANCE, request, true).await
    }

    /// Add investment to a grid algo.
    ///
    /// `POST /api/v5/tradingBot/grid/adjust-investment`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn adjust_investment(
        &self,
        request: &GridAdjustInvestmentRequest,
    ) -> Result<Vec<GridAlgoResult>, Error> {
        self.client.post(GRID_ADJUST_INVESTMENT, request, true).await
    }

    /// Retrieve public Grid AI parameters.
    ///
    /// `GET /api/v5/tradingBot/grid/ai-param`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn get_ai_parameters(
        &self,
        request: &GridAiParamRequest,
    ) -> Result<Vec<GridAiParameter>, Error> {
        self.client.get(GRID_AI_PARAM, request, false).await
    }

    /// Compute the minimum investment for a grid configuration.
    ///
    /// `POST /api/v5/tradingBot/grid/min-investment`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn compute_min_investment(
        &self,
        request: &GridMinInvestmentRequest,
    ) -> Result<Vec<GridMinInvestment>, Error> {
        self.client.post(GRID_MIN_INVESTMENT, request, false).await
    }

    /// Run a public RSI trigger backtest.
    ///
    /// `GET /api/v5/tradingBot/public/rsi-back-testing`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn rsi_back_testing(
        &self,
        request: &GridRsiBackTestingRequest,
    ) -> Result<Vec<GridRsiBackTesting>, Error> {
        self.client.get(GRID_RSI_BACK_TESTING, request, false).await
    }

    /// Retrieve the maximum grid quantity for a configuration.
    ///
    /// `GET /api/v5/tradingBot/grid/grid-quantity`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn get_grid_quantity(
        &self,
        request: &GridQuantityRequest,
    ) -> Result<Vec<GridQuantity>, Error> {
        self.client.get(GRID_QUANTITY, request, false).await
    }

    /// Copy a lead grid algo order.
    ///
    /// `POST /api/v5/tradingBot/grid/copy-order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn copy_order(
        &self,
        request: &GridCopyOrderRequest,
    ) -> Result<Vec<GridActionResult>, Error> {
        self.client.post(GRID_COPY_ORDER, request, true).await
    }
}
