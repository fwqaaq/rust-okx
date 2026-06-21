use std::collections::VecDeque;

use rust_okx::api::account::{AccountBalance, BalanceDetail, Position};
use rust_okx::api::trade::Order;
use rust_okx::model::{OrderSide, OrderType, TradeMode};
use rust_okx::ws::model::{BookLevel, CandleUpdate, OrderBookUpdate, TickerUpdate, TradeUpdate};

use crate::config::RuntimeConfig;

const MAX_CANDLES: usize = 160;
const MAX_TICKS: usize = 80;
const MAX_TRADES: usize = 80;
const MAX_LOGS: usize = 200;

#[derive(Debug, Clone)]
pub struct OhlcBar {
    pub ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub confirm: bool,
}

impl From<rust_okx::api::market::Candle> for OhlcBar {
    fn from(c: rust_okx::api::market::Candle) -> Self {
        Self {
            ts: c.ts.as_str().parse().unwrap_or(0),
            open: c.open.as_str().parse().unwrap_or(0.0),
            high: c.high.as_str().parse().unwrap_or(0.0),
            low: c.low.as_str().parse().unwrap_or(0.0),
            close: c.close.as_str().parse().unwrap_or(0.0),
            volume: c.vol.as_str().parse().unwrap_or(0.0),
            confirm: c.confirm.as_str() == "1",
        }
    }
}

impl From<CandleUpdate> for OhlcBar {
    fn from(c: CandleUpdate) -> Self {
        Self {
            ts: c.ts.as_str().parse().unwrap_or(0),
            open: c.o.as_str().parse().unwrap_or(0.0),
            high: c.h.as_str().parse().unwrap_or(0.0),
            low: c.l.as_str().parse().unwrap_or(0.0),
            close: c.c.as_str().parse().unwrap_or(0.0),
            volume: c.vol.as_str().parse().unwrap_or(0.0),
            confirm: c.confirm.as_str() == "1",
        }
    }
}

pub const DEFAULT_WATCHLIST: [&str; 6] = [
    "BTC-USDT",
    "ETH-USDT",
    "SOL-USDT",
    "XRP-USDT",
    "DOGE-USDT",
    "BNB-USDT",
];

pub const BAR_OPTIONS: [&str; 9] = ["1m", "3m", "5m", "15m", "30m", "1H", "4H", "1D", "1W"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Market,
    Account,
    Orders,
    Trade,
    Logs,
    Watchlist,
}

impl Tab {
    pub const ALL: [Tab; 7] = [
        Tab::Dashboard,
        Tab::Market,
        Tab::Account,
        Tab::Orders,
        Tab::Trade,
        Tab::Logs,
        Tab::Watchlist,
    ];

    pub fn index(self) -> usize {
        match self {
            Tab::Dashboard => 0,
            Tab::Market => 1,
            Tab::Account => 2,
            Tab::Orders => 3,
            Tab::Trade => 4,
            Tab::Logs => 5,
            Tab::Watchlist => 6,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tab::Dashboard => "[1] 总览",
            Tab::Market => "[2] 行情",
            Tab::Account => "[3] 账户",
            Tab::Orders => "[4] 订单",
            Tab::Trade => "[5] 交易",
            Tab::Logs => "[6] 日志",
            Tab::Watchlist => "[7] 自选",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WatchlistEntry {
    pub inst_id: String,
    pub last: Option<f64>,
    pub change24h: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamKind {
    Rest,
    Market,
    Candles,
    Private,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamState {
    Idle,
    Connecting,
    Subscribed,
    Reconnecting,
    Error,
}

#[derive(Clone, Debug)]
pub struct ConnectionStatus {
    pub state: StreamState,
    pub detail: String,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self {
            state: StreamState::Idle,
            detail: "idle".to_owned(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ConnectionStatuses {
    pub rest: ConnectionStatus,
    pub market: ConnectionStatus,
    pub candles: ConnectionStatus,
    pub private: ConnectionStatus,
}

impl ConnectionStatuses {
    pub fn get_mut(&mut self, kind: StreamKind) -> &mut ConnectionStatus {
        match kind {
            StreamKind::Rest => &mut self.rest,
            StreamKind::Market => &mut self.market,
            StreamKind::Candles => &mut self.candles,
            StreamKind::Private => &mut self.private,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub enum AppMsg {
    RestSnapshot {
        generation: u64,
        snapshot: RestSnapshot,
    },
    Candle(CandleUpdate),
    Ticker(TickerUpdate),
    WatchlistTicker(TickerUpdate),
    Trade(TradeUpdate),
    Book(OrderBookUpdate),
    Status(StreamKind, StreamState, String),
    Log(LogLevel, String),
}

#[derive(Debug)]
pub struct RestSnapshot {
    pub balances: Vec<AccountBalance>,
    pub positions: Vec<Position>,
    pub orders: Vec<Order>,
    pub candles: Vec<OhlcBar>,
    pub perm: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeSideInput {
    Buy,
    Sell,
}

impl TradeSideInput {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Buy => Self::Sell,
            Self::Sell => Self::Buy,
        };
    }

    pub fn as_order_side(self) -> OrderSide {
        match self {
            Self::Buy => OrderSide::Buy,
            Self::Sell => OrderSide::Sell,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeTypeInput {
    Limit,
    Market,
}

impl TradeTypeInput {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Limit => Self::Market,
            Self::Market => Self::Limit,
        };
    }

    pub fn as_order_type(self) -> OrderType {
        match self {
            Self::Limit => OrderType::Limit,
            Self::Market => OrderType::Market,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Limit => "limit",
            Self::Market => "market",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeModeInput {
    Cash,
    Cross,
    Isolated,
}

impl TradeModeInput {
    pub fn cycle(&mut self) {
        *self = match self {
            Self::Cash => Self::Cross,
            Self::Cross => Self::Isolated,
            Self::Isolated => Self::Cash,
        };
    }

    pub fn as_trade_mode(self) -> TradeMode {
        match self {
            Self::Cash => TradeMode::Cash,
            Self::Cross => TradeMode::Cross,
            Self::Isolated => TradeMode::Isolated,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::Cross => "cross",
            Self::Isolated => "isolated",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeField {
    Size,
    Price,
}

#[derive(Clone, Debug)]
pub struct TradeForm {
    pub side: TradeSideInput,
    pub order_type: TradeTypeInput,
    pub trade_mode: TradeModeInput,
    pub size: String,
    pub price: String,
    pub focused: TradeField,
    pub message: String,
}

impl Default for TradeForm {
    fn default() -> Self {
        Self {
            side: TradeSideInput::Buy,
            order_type: TradeTypeInput::Limit,
            trade_mode: TradeModeInput::Cash,
            size: String::new(),
            price: String::new(),
            focused: TradeField::Size,
            message: "Enter 提交，s 切方向，o 切订单类型，m 切交易模式".to_owned(),
        }
    }
}

impl TradeForm {
    pub fn validate(&self) -> Result<(), String> {
        validate_positive("size", &self.size)?;
        if self.order_type == TradeTypeInput::Limit {
            validate_positive("price", &self.price)?;
        }
        Ok(())
    }

    pub fn push_char(&mut self, c: char) {
        if !(c.is_ascii_digit() || c == '.') {
            return;
        }
        let target = self.focused_value_mut();
        if c == '.' && target.contains('.') {
            return;
        }
        target.push(c);
    }

    pub fn backspace(&mut self) {
        self.focused_value_mut().pop();
    }

    pub fn next_field(&mut self) {
        self.focused = match self.focused {
            TradeField::Size => TradeField::Price,
            TradeField::Price => TradeField::Size,
        };
    }

    fn focused_value_mut(&mut self) -> &mut String {
        match self.focused {
            TradeField::Size => &mut self.size,
            TradeField::Price => &mut self.price,
        }
    }
}

fn validate_positive(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{name} is required"));
    }
    let parsed: f64 = value
        .parse()
        .map_err(|_| format!("{name} must be a number"))?;
    if parsed <= 0.0 {
        return Err(format!("{name} must be positive"));
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub enum PendingAction {
    PlaceOrder,
    CancelOrder { ord_id: String },
}

#[derive(Clone, Debug)]
pub struct Confirmation {
    pub action: PendingAction,
    pub lines: Vec<String>,
}

pub struct App {
    pub tab: Tab,
    pub config: RuntimeConfig,
    pub balances: Vec<AccountBalance>,
    pub positions: Vec<Position>,
    pub orders: Vec<Order>,
    pub perm: String,
    pub candles: VecDeque<OhlcBar>,
    pub tickers: VecDeque<TickerUpdate>,
    pub trades: VecDeque<TradeUpdate>,
    pub book: Option<OrderBookUpdate>,
    pub statuses: ConnectionStatuses,
    pub logs: VecDeque<LogEntry>,
    pub paused: bool,
    pub symbol_editing: bool,
    pub symbol_input: String,
    pub bar_picking: bool,
    pub watchlist_editing: bool,
    pub watchlist_input: String,
    pub rest_generation: u64,
    pub selected_order: usize,
    pub trade: TradeForm,
    pub confirmation: Option<Confirmation>,
    pub watchlist: Vec<WatchlistEntry>,
    pub watchlist_cursor: usize,
}

impl App {
    pub fn new(config: RuntimeConfig, watchlist: Vec<String>) -> Self {
        let watchlist = normalized_watchlist_entries(watchlist);
        Self {
            tab: Tab::Dashboard,
            symbol_input: config.inst_id.clone(),
            config,
            balances: Vec::new(),
            positions: Vec::new(),
            orders: Vec::new(),
            perm: String::new(),
            candles: VecDeque::new(),
            tickers: VecDeque::new(),
            trades: VecDeque::new(),
            book: None,
            statuses: ConnectionStatuses::default(),
            logs: VecDeque::new(),
            paused: false,
            symbol_editing: false,
            bar_picking: false,
            watchlist_editing: false,
            watchlist_input: String::new(),
            rest_generation: 0,
            selected_order: 0,
            trade: TradeForm::default(),
            confirmation: None,
            watchlist,
            watchlist_cursor: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.tab = Tab::ALL[(self.tab.index() + 1) % Tab::ALL.len()];
    }

    pub fn prev_tab(&mut self) {
        self.tab = Tab::ALL[(self.tab.index() + Tab::ALL.len() - 1) % Tab::ALL.len()];
    }

    pub fn set_tab_by_number(&mut self, n: usize) {
        if let Some(tab) = Tab::ALL.get(n.saturating_sub(1)) {
            self.tab = *tab;
        }
    }

    pub fn apply_msg(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::RestSnapshot {
                generation,
                snapshot,
            } => self.apply_rest_snapshot_for_generation(generation, snapshot),
            AppMsg::Candle(update) if !self.paused => self.push_candle(update),
            AppMsg::Ticker(update) if !self.paused => self.push_ticker(update),
            AppMsg::Trade(update) if !self.paused => self.push_trade(update),
            AppMsg::Book(update) if !self.paused => self.book = Some(update),
            AppMsg::WatchlistTicker(update) => self.apply_watchlist_ticker(update),
            AppMsg::Status(kind, state, detail) => self.set_status(kind, state, detail),
            AppMsg::Log(level, message) => self.log(level, message),
            AppMsg::Candle(_) | AppMsg::Ticker(_) | AppMsg::Trade(_) | AppMsg::Book(_) => {}
        }
    }

    fn apply_rest_snapshot_for_generation(&mut self, generation: u64, snapshot: RestSnapshot) {
        if generation == self.rest_generation {
            self.apply_rest_snapshot(snapshot);
        }
    }

    pub fn apply_rest_snapshot(&mut self, snapshot: RestSnapshot) {
        self.balances = snapshot.balances;
        self.positions = snapshot.positions;
        self.orders = snapshot.orders;
        self.perm = snapshot.perm;
        self.candles = snapshot.candles.into();
        self.selected_order = self.selected_order.min(self.orders.len().saturating_sub(1));
        self.set_status(
            StreamKind::Rest,
            StreamState::Subscribed,
            "refreshed".to_owned(),
        );
    }

    pub fn set_market(&mut self, inst_id: String, candles: Vec<OhlcBar>) {
        self.config.inst_id = inst_id.clone();
        self.symbol_input = inst_id;
        self.candles = candles.into();
        self.tickers.clear();
        self.trades.clear();
        self.book = None;
    }

    pub fn set_bar(&mut self, bar: String, candles: Vec<OhlcBar>) {
        self.config.bar = bar;
        self.candles = candles.into();
    }

    pub fn push_candle(&mut self, update: CandleUpdate) {
        let bar = OhlcBar::from(update);
        if let Some(last) = self.candles.back_mut()
            && last.ts == bar.ts
        {
            *last = bar;
            return;
        }
        self.candles.push_back(bar);
        while self.candles.len() > MAX_CANDLES {
            self.candles.pop_front();
        }
    }

    pub fn push_ticker(&mut self, update: TickerUpdate) {
        self.tickers.push_front(update);
        while self.tickers.len() > MAX_TICKS {
            self.tickers.pop_back();
        }
    }

    pub fn push_trade(&mut self, update: TradeUpdate) {
        self.trades.push_front(update);
        while self.trades.len() > MAX_TRADES {
            self.trades.pop_back();
        }
    }

    pub fn set_status(&mut self, kind: StreamKind, state: StreamState, detail: String) {
        *self.statuses.get_mut(kind) = ConnectionStatus { state, detail };
    }

    pub fn log(&mut self, level: LogLevel, message: String) {
        self.logs.push_front(LogEntry { level, message });
        while self.logs.len() > MAX_LOGS {
            self.logs.pop_back();
        }
    }

    pub fn balance_details(&self) -> impl Iterator<Item = &BalanceDetail> {
        self.balances
            .iter()
            .flat_map(|b| b.details.iter())
            .filter(|d| {
                let eq: f64 = d.eq.as_str().parse().unwrap_or(0.0);
                eq != 0.0
            })
    }

    pub fn latest_ticker(&self) -> Option<&TickerUpdate> {
        self.tickers.front()
    }

    pub fn selected_order(&self) -> Option<&Order> {
        self.orders.get(self.selected_order)
    }

    pub fn select_next_order(&mut self) {
        if !self.orders.is_empty() {
            self.selected_order = (self.selected_order + 1).min(self.orders.len() - 1);
        }
    }

    pub fn select_prev_order(&mut self) {
        self.selected_order = self.selected_order.saturating_sub(1);
    }

    pub fn apply_watchlist_ticker(&mut self, update: TickerUpdate) {
        if let Some(entry) = self
            .watchlist
            .iter_mut()
            .find(|e| e.inst_id == update.inst_id)
        {
            let last: f64 = update.last.as_str().parse().unwrap_or(0.0);
            let open24h: f64 = update.open24h.as_str().parse().unwrap_or(0.0);
            entry.last = Some(last);
            entry.change24h = if open24h != 0.0 {
                Some((last - open24h) / open24h * 100.0)
            } else {
                None
            };
        }
    }

    pub fn select_next_watchlist(&mut self) {
        if !self.watchlist.is_empty() {
            self.watchlist_cursor = (self.watchlist_cursor + 1).min(self.watchlist.len() - 1);
        }
    }

    pub fn select_prev_watchlist(&mut self) {
        self.watchlist_cursor = self.watchlist_cursor.saturating_sub(1);
    }

    pub fn active_watchlist_inst(&self) -> Option<String> {
        self.watchlist
            .get(self.watchlist_cursor)
            .map(|e| e.inst_id.clone())
    }

    pub fn begin_rest_generation(&mut self, detail: String) -> u64 {
        self.rest_generation = self.rest_generation.wrapping_add(1);
        self.set_status(StreamKind::Rest, StreamState::Connecting, detail);
        self.rest_generation
    }

    pub fn watchlist_instruments(&self) -> Vec<String> {
        self.watchlist
            .iter()
            .map(|entry| entry.inst_id.clone())
            .collect()
    }

    pub fn add_watchlist_inst(&mut self, inst_id: String) -> bool {
        let inst_id = inst_id.trim().to_ascii_uppercase();
        if inst_id.is_empty() {
            return false;
        }
        if let Some(index) = self
            .watchlist
            .iter()
            .position(|entry| entry.inst_id == inst_id)
        {
            self.watchlist_cursor = index;
            self.log(LogLevel::Info, format!("{inst_id} 已在自选中"));
            return false;
        }

        self.watchlist.push(WatchlistEntry {
            inst_id: inst_id.clone(),
            last: None,
            change24h: None,
        });
        self.watchlist_cursor = self.watchlist.len().saturating_sub(1);
        self.log(LogLevel::Info, format!("已添加自选 {inst_id}"));
        true
    }

    pub fn best_bid_ask(&self) -> (Option<&BookLevel>, Option<&BookLevel>) {
        let bid = self.book.as_ref().and_then(|book| book.bids.first());
        let ask = self.book.as_ref().and_then(|book| book.asks.first());
        (bid, ask)
    }

    pub fn last_price(&self) -> Option<f64> {
        self.latest_ticker()
            .and_then(|ticker| ticker.last.as_str().parse().ok())
            .or_else(|| self.candles.back().map(|bar| bar.close))
    }

    pub fn daily_change_pct(&self) -> Option<f64> {
        let ticker = self.latest_ticker()?;
        let last: f64 = ticker.last.as_str().parse().ok()?;
        let open: f64 = ticker.open24h.as_str().parse().ok()?;
        if open == 0.0 {
            None
        } else {
            Some((last - open) / open * 100.0)
        }
    }

    pub fn build_place_confirmation(&mut self) -> bool {
        if !self.config.trade_enabled {
            self.trade.message = "交易未启用：启动时添加 --trade-enabled".to_owned();
            self.log(LogLevel::Warn, self.trade.message.clone());
            return false;
        }
        if let Err(error) = self.trade.validate() {
            self.trade.message = error;
            return false;
        }
        let mut lines = vec![
            format!("环境: {}", if self.config.demo { "DEMO" } else { "LIVE" }),
            format!("交易对: {}", self.config.inst_id),
            format!("方向: {}", self.trade.side.label()),
            format!("类型: {}", self.trade.order_type.label()),
            format!("模式: {}", self.trade.trade_mode.label()),
            format!("数量: {}", self.trade.size),
        ];
        if self.trade.order_type == TradeTypeInput::Limit {
            lines.push(format!("价格: {}", self.trade.price));
        }
        lines.push("按 y 确认，按 n/Esc 取消".to_owned());
        self.confirmation = Some(Confirmation {
            action: PendingAction::PlaceOrder,
            lines,
        });
        true
    }

    pub fn build_cancel_confirmation(&mut self) -> bool {
        if !self.config.trade_enabled {
            self.trade.message = "交易未启用：启动时添加 --trade-enabled".to_owned();
            self.log(LogLevel::Warn, self.trade.message.clone());
            return false;
        }
        let Some(order) = self.selected_order() else {
            self.trade.message = "没有可撤销订单".to_owned();
            return false;
        };
        self.confirmation = Some(Confirmation {
            action: PendingAction::CancelOrder {
                ord_id: order.ord_id.clone(),
            },
            lines: vec![
                format!("环境: {}", if self.config.demo { "DEMO" } else { "LIVE" }),
                format!("撤单: {}", order.inst_id),
                format!("ordId: {}", order.ord_id),
                format!("side/type: {} {}", order.side, order.ord_type),
                "按 y 确认，按 n/Esc 取消".to_owned(),
            ],
        });
        true
    }
}

fn normalized_watchlist_entries(watchlist: Vec<String>) -> Vec<WatchlistEntry> {
    let mut entries = Vec::new();
    let source = if watchlist.is_empty() {
        DEFAULT_WATCHLIST
            .iter()
            .map(|id| (*id).to_owned())
            .collect::<Vec<_>>()
    } else {
        watchlist
    };

    for inst_id in source {
        let inst_id = inst_id.trim().to_ascii_uppercase();
        if inst_id.is_empty()
            || entries
                .iter()
                .any(|entry: &WatchlistEntry| entry.inst_id == inst_id)
        {
            continue;
        }
        entries.push(WatchlistEntry {
            inst_id,
            last: None,
            change24h: None,
        });
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        App::new(
            RuntimeConfig::default(),
            DEFAULT_WATCHLIST
                .iter()
                .map(|id| (*id).to_owned())
                .collect(),
        )
    }

    fn candle(ts: &str, close: &str) -> CandleUpdate {
        vec![
            ts.into(),
            "1".into(),
            close.into(),
            "1".into(),
            close.into(),
            "1".into(),
            "1".into(),
            "1".into(),
            "0".into(),
        ]
        .into()
    }

    #[test]
    fn candle_update_replaces_same_timestamp() {
        let mut app = app();
        app.push_candle(candle("1", "2"));
        app.push_candle(candle("1", "3"));

        assert_eq!(app.candles.len(), 1);
        assert_eq!(app.candles[0].close, 3.0);
    }

    #[test]
    fn candle_update_appends_new_timestamp() {
        let mut app = app();
        app.push_candle(candle("1", "2"));
        app.push_candle(candle("2", "3"));

        assert_eq!(app.candles.len(), 2);
    }

    #[test]
    fn trade_form_requires_limit_price() {
        let form = TradeForm {
            size: "1".to_owned(),
            order_type: TradeTypeInput::Limit,
            ..TradeForm::default()
        };

        assert!(form.validate().is_err());
    }

    #[test]
    fn trade_confirmation_requires_enabled_flag() {
        let mut app = app();
        app.trade.size = "1".to_owned();
        app.trade.price = "10".to_owned();

        assert!(!app.build_place_confirmation());
        assert!(app.confirmation.is_none());
    }

    #[test]
    fn app_uses_default_watchlist_when_empty() {
        let app = App::new(RuntimeConfig::default(), Vec::new());

        assert_eq!(
            app.watchlist_instruments(),
            DEFAULT_WATCHLIST
                .iter()
                .map(|id| (*id).to_owned())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn add_watchlist_inst_uppercases_and_dedupes() {
        let mut app = App::new(RuntimeConfig::default(), vec!["BTC-USDT".to_owned()]);

        assert!(app.add_watchlist_inst(" eth-usdt ".to_owned()));
        assert_eq!(app.watchlist_cursor, 1);
        assert_eq!(
            app.watchlist_instruments(),
            vec!["BTC-USDT".to_owned(), "ETH-USDT".to_owned()]
        );

        assert!(!app.add_watchlist_inst("ETH-USDT".to_owned()));
        assert_eq!(app.watchlist_cursor, 1);
        assert_eq!(app.watchlist.len(), 2);
    }

    #[test]
    fn stale_rest_snapshot_is_ignored() {
        let mut app = app();
        let stale_generation = app.rest_generation;
        let current_generation = app.begin_rest_generation("switch".to_owned());
        assert_ne!(stale_generation, current_generation);

        app.apply_msg(AppMsg::RestSnapshot {
            generation: stale_generation,
            snapshot: RestSnapshot {
                balances: Vec::new(),
                positions: Vec::new(),
                orders: Vec::new(),
                candles: vec![OhlcBar {
                    ts: 1,
                    open: 1.0,
                    high: 1.0,
                    low: 1.0,
                    close: 9.0,
                    volume: 1.0,
                    confirm: false,
                }],
                perm: "stale".to_owned(),
            },
        });

        assert!(app.candles.is_empty());
        assert!(app.perm.is_empty());
    }
}
