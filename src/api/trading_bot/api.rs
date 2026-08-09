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

    /// Access Recurring Buy endpoints.
    pub fn recurring_buy(&self) -> RecurringBuy<'_, T> {
        RecurringBuy {
            client: self.client,
        }
    }

    /// Access Signal Bot endpoints.
    pub fn signal(&self) -> SignalBot<'_, T> {
        SignalBot {
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
        self.client
            .post(GRID_ADJUST_INVESTMENT, request, true)
            .await
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

/// Accessor for Recurring Buy endpoints.
pub struct RecurringBuy<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> RecurringBuy<'_, T> {
    /// Place a recurring-buy algo order.
    ///
    /// `POST /api/v5/tradingBot/recurring/order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn place_order(
        &self,
        request: &RecurringOrderRequest,
    ) -> Result<Vec<RecurringCreateResult>, Error> {
        self.client.post(RECURRING_ORDER, request, true).await
    }

    /// Rename a recurring-buy algo order.
    ///
    /// `POST /api/v5/tradingBot/recurring/amend-order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_order(
        &self,
        request: &RecurringAmendRequest,
    ) -> Result<Vec<RecurringActionResult>, Error> {
        self.client.post(RECURRING_AMEND, request, true).await
    }

    /// Stop recurring-buy algo orders.
    ///
    /// `POST /api/v5/tradingBot/recurring/stop-order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn stop_orders(
        &self,
        request: &[RecurringAlgoIdRequest],
    ) -> Result<Vec<RecurringActionResult>, Error> {
        self.client.post(RECURRING_STOP, request, true).await
    }

    /// Retrieve active recurring-buy algo orders.
    ///
    /// `GET /api/v5/tradingBot/recurring/orders-algo-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_pending_orders(
        &self,
        request: &RecurringOrdersRequest,
    ) -> Result<Vec<RecurringOrder>, Error> {
        self.client.get(RECURRING_PENDING, request, true).await
    }

    /// Retrieve historical recurring-buy algo orders.
    ///
    /// `GET /api/v5/tradingBot/recurring/orders-algo-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_orders_history(
        &self,
        request: &RecurringOrdersRequest,
    ) -> Result<Vec<RecurringOrder>, Error> {
        self.client.get(RECURRING_HISTORY, request, true).await
    }

    /// Retrieve one recurring-buy algo order.
    ///
    /// `GET /api/v5/tradingBot/recurring/orders-algo-details`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_order_details(
        &self,
        request: &RecurringAlgoIdRequest,
    ) -> Result<Vec<RecurringOrder>, Error> {
        self.client.get(RECURRING_DETAILS, request, true).await
    }

    /// Retrieve recurring-buy sub-orders.
    ///
    /// `GET /api/v5/tradingBot/recurring/sub-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_sub_orders(
        &self,
        request: &RecurringSubOrdersRequest,
    ) -> Result<Vec<RecurringSubOrder>, Error> {
        self.client.get(RECURRING_SUB_ORDERS, request, true).await
    }

    /// Add an investment to a recurring-buy algo order.
    ///
    /// `POST /api/v5/tradingBot/recurring/add-investment`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn add_investment(
        &self,
        request: &RecurringAmountRequest,
    ) -> Result<Vec<RecurringOperationResult>, Error> {
        self.client
            .post(RECURRING_ADD_INVESTMENT, request, true)
            .await
    }

    /// Amend recurring-buy price ranges.
    ///
    /// `POST /api/v5/tradingBot/recurring/amend-price-range`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_price_range(
        &self,
        request: &RecurringAmendPriceRangeRequest,
    ) -> Result<Vec<RecurringOperationResult>, Error> {
        self.client
            .post(RECURRING_AMEND_PRICE_RANGE, request, true)
            .await
    }

    /// Amend the amount invested by a recurring-buy algo order.
    ///
    /// `POST /api/v5/tradingBot/recurring/amend-recurring-amount`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_amount(
        &self,
        request: &RecurringAmountRequest,
    ) -> Result<Vec<RecurringOperationResult>, Error> {
        self.client
            .post(RECURRING_AMEND_AMOUNT, request, true)
            .await
    }

    /// Amend the recurring-buy schedule.
    ///
    /// `POST /api/v5/tradingBot/recurring/amend-recurring-time`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_time(
        &self,
        request: &RecurringAmendTimeRequest,
    ) -> Result<Vec<RecurringOperationResult>, Error> {
        self.client
            .post(RECURRING_AMEND_TIME, request, true)
            .await
    }

    /// Pause a recurring-buy algo order.
    ///
    /// `POST /api/v5/tradingBot/recurring/pause`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn pause(
        &self,
        request: &RecurringAlgoIdRequest,
    ) -> Result<Vec<RecurringOperationResult>, Error> {
        self.client.post(RECURRING_PAUSE, request, true).await
    }

    /// Restart a recurring-buy algo order.
    ///
    /// `POST /api/v5/tradingBot/recurring/restart`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn restart(
        &self,
        request: &RecurringAlgoIdRequest,
    ) -> Result<Vec<RecurringOperationResult>, Error> {
        self.client.post(RECURRING_RESTART, request, true).await
    }
}

/// Accessor for Signal Bot endpoints.
pub struct SignalBot<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> SignalBot<'_, T> {
    /// Create a signal channel.
    ///
    /// `POST /api/v5/tradingBot/signal/create-signal`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn create_signal(
        &self,
        request: &SignalCreateRequest,
    ) -> Result<Vec<SignalChannelCreated>, Error> {
        self.client.post(SIGNAL_CREATE, request, true).await
    }

    /// Retrieve signal channels.
    ///
    /// `GET /api/v5/tradingBot/signal/signals`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_signals(
        &self,
        request: &SignalsRequest,
    ) -> Result<Vec<SignalChannel>, Error> {
        self.client.get(SIGNAL_LIST, request, true).await
    }

    /// Create a signal bot.
    ///
    /// `POST /api/v5/tradingBot/signal/order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn place_order(
        &self,
        request: &SignalOrderRequest,
    ) -> Result<Vec<SignalActionResult>, Error> {
        self.client.post(SIGNAL_ORDER, request, true).await
    }

    /// Stop up to ten signal bots.
    ///
    /// `POST /api/v5/tradingBot/signal/stop-order-algo`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn stop_orders(
        &self,
        request: &[SignalAlgoIdRequest],
    ) -> Result<Vec<SignalActionResult>, Error> {
        self.client.post(SIGNAL_STOP, request, true).await
    }

    /// Retrieve one signal-bot order.
    ///
    /// `GET /api/v5/tradingBot/signal/orders-algo-details`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_order_details(
        &self,
        request: &SignalAlgoRequest,
    ) -> Result<Vec<SignalAlgoOrder>, Error> {
        self.client.get(SIGNAL_DETAILS, request, true).await
    }

    /// Retrieve active signal bots.
    ///
    /// `GET /api/v5/tradingBot/signal/orders-algo-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_pending_orders(
        &self,
        request: &SignalOrdersRequest,
    ) -> Result<Vec<SignalAlgoOrder>, Error> {
        self.client.get(SIGNAL_PENDING, request, true).await
    }

    /// Retrieve signal-bot history.
    ///
    /// `GET /api/v5/tradingBot/signal/orders-algo-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_orders_history(
        &self,
        request: &SignalOrdersRequest,
    ) -> Result<Vec<SignalAlgoOrder>, Error> {
        self.client.get(SIGNAL_HISTORY, request, true).await
    }

    /// Retrieve signal-bot sub-orders.
    ///
    /// `GET /api/v5/tradingBot/signal/sub-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_sub_orders(
        &self,
        request: &SignalSubOrdersRequest,
    ) -> Result<Vec<SignalSubOrder>, Error> {
        self.client.get(SIGNAL_SUB_ORDERS, request, true).await
    }

    /// Cancel an incomplete signal-bot sub-order.
    ///
    /// `POST /api/v5/tradingBot/signal/cancel-sub-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_sub_order(
        &self,
        request: &SignalCancelSubOrderRequest,
    ) -> Result<Vec<SignalCancelSubOrderResult>, Error> {
        self.client.post(SIGNAL_CANCEL_SUB_ORDER, request, true).await
    }

    /// Retrieve signal-bot event history.
    ///
    /// `GET /api/v5/tradingBot/signal/event-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_event_history(
        &self,
        request: &SignalEventHistoryRequest,
    ) -> Result<Vec<SignalEvent>, Error> {
        self.client.get(SIGNAL_EVENT_HISTORY, request, true).await
    }

    /// Retrieve open signal-bot positions.
    ///
    /// `GET /api/v5/tradingBot/signal/positions`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_positions(
        &self,
        request: &SignalAlgoRequest,
    ) -> Result<Vec<SignalPosition>, Error> {
        self.client.get(SIGNAL_POSITIONS, request, true).await
    }

    /// Retrieve closed signal-bot positions.
    ///
    /// `GET /api/v5/tradingBot/signal/positions-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_positions_history(
        &self,
        request: &SignalPositionHistoryRequest,
    ) -> Result<Vec<SignalPositionHistory>, Error> {
        self.client
            .get(SIGNAL_POSITIONS_HISTORY, request, true)
            .await
    }

    /// Amend signal-bot take-profit and stop-loss settings.
    ///
    /// `POST /api/v5/tradingBot/signal/amendTPSL`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_tp_sl(
        &self,
        request: &SignalAmendTpSlRequest,
    ) -> Result<Vec<SignalAlgoResult>, Error> {
        self.client.post(SIGNAL_AMEND_TPSL, request, true).await
    }

    /// Close a signal-bot position.
    ///
    /// `POST /api/v5/tradingBot/signal/close-position`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn close_position(
        &self,
        request: &SignalClosePositionRequest,
    ) -> Result<Vec<SignalAlgoResult>, Error> {
        self.client.post(SIGNAL_CLOSE_POSITION, request, true).await
    }

    /// Adjust signal-bot margin.
    ///
    /// `POST /api/v5/tradingBot/signal/margin-balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn adjust_margin_balance(
        &self,
        request: &SignalMarginBalanceRequest,
    ) -> Result<Vec<SignalAlgoResult>, Error> {
        self.client.post(SIGNAL_MARGIN_BALANCE, request, true).await
    }

    /// Set the instruments traded by a signal bot.
    ///
    /// `POST /api/v5/tradingBot/signal/set-instruments`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn set_instruments(
        &self,
        request: &SignalSetInstrumentsRequest,
    ) -> Result<Vec<SignalAlgoResult>, Error> {
        self.client
            .post(SIGNAL_SET_INSTRUMENTS, request, true)
            .await
    }

    /// Place a signal-bot sub-order.
    ///
    /// `POST /api/v5/tradingBot/signal/sub-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn place_sub_order(
        &self,
        request: &SignalSubOrderRequest,
    ) -> Result<Vec<SignalSubOrderPlacement>, Error> {
        self.client.post(SIGNAL_SUB_ORDER, request, true).await
    }
}
