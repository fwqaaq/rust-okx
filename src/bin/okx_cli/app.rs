use std::collections::VecDeque;

use rust_okx::api::account::{AccountBalance, BalanceDetail};
use rust_okx::ws::model::{CandleUpdate, TickerUpdate};

#[derive(Clone)]
pub struct OhlcBar {
    pub ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
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
            confirm: c.confirm.as_str() == "1",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Account,
    Chart,
    Feed,
}

pub enum AppMsg {
    Candle(CandleUpdate),
    Ticker(TickerUpdate),
}

pub struct App {
    pub tab: Tab,
    pub balances: Vec<AccountBalance>,
    pub perm: String,
    pub inst_id: String,
    pub bar: String,
    pub candles: VecDeque<OhlcBar>,
    pub tickers: VecDeque<TickerUpdate>,
}

impl App {
    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Account => Tab::Chart,
            Tab::Chart => Tab::Feed,
            Tab::Feed => Tab::Account,
        };
    }

    pub fn prev_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Account => Tab::Feed,
            Tab::Chart => Tab::Account,
            Tab::Feed => Tab::Chart,
        };
    }

    pub fn push_candle(&mut self, cu: CandleUpdate) {
        let bar = OhlcBar::from(cu);
        if let Some(last) = self.candles.back_mut()
            && last.ts == bar.ts
        {
            *last = bar;
            return;
        }
        self.candles.push_back(bar);
        if self.candles.len() > 100 {
            self.candles.pop_front();
        }
    }

    pub fn push_ticker(&mut self, tu: TickerUpdate) {
        self.tickers.push_front(tu);
        if self.tickers.len() > 50 {
            self.tickers.pop_back();
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
}
