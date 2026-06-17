//! Public reference data channel models (`instruments`, `funding-rate`, `open-interest`, etc.).
//!
//! Public channels; no authentication required.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Upcoming instrument-parameter change nested in [`InstrumentUpdate`].
    InstrumentUpcomingParamChange {
        param: String,
        new_value: String,
        eff_time: NumberString
    }
}

ws_object! {
    /// Public-data `instruments` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-instruments-channel>
    InstrumentUpdate {
        inst_type: String,
        series_id: String,
        inst_id: String,
        inst_id_code: NumberString,
        uly: String,
        inst_family: String,
        category: String,
        base_ccy: String,
        quote_ccy: String,
        settle_ccy: String,
        ct_val: NumberString,
        ct_mult: NumberString,
        ct_val_ccy: String,
        opt_type: String,
        stk: NumberString,
        list_time: NumberString,
        cont_td_sw_time: NumberString,
        pre_mkt_sw_time: NumberString,
        exp_time: NumberString,
        lever: NumberString,
        tick_sz: NumberString,
        lot_sz: NumberString,
        min_sz: NumberString,
        ct_type: String,
        alias: String,
        state: String,
        open_type: String,
        rule_type: String,
        max_lmt_sz: NumberString,
        max_lmt_amt: NumberString,
        max_mkt_sz: NumberString,
        max_mkt_amt: NumberString,
        max_twap_sz: NumberString,
        max_iceberg_sz: NumberString,
        max_trigger_sz: NumberString,
        max_stop_sz: NumberString,
        auction_end_time: NumberString,
        future_settlement: bool,
        trade_quote_ccy_list: Vec<String>,
        inst_category: String,
        pos_lmt_amt: NumberString,
        pos_lmt_pct: NumberString,
        long_pos_remaining_quota: NumberString,
        short_pos_remaining_quota: NumberString,
        max_plat_oi_lmt: NumberString,
        group_id: String,
        upc_chg: Vec<InstrumentUpcomingParamChange>
    }
}

ws_object! {
    /// `event-contract-markets` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-event-contract-markets-channel>
    EventContractMarketUpdate {
        series_id: String,
        event_id: String,
        market_id: String,
        inst_id: String,
        title: String,
        state: String,
        category: String,
        rule_type: String,
        outcome: String,
        floor_strike: NumberString,
        settle_value: NumberString,
        disputed: bool,
        tick_sz: NumberString,
        min_sz: NumberString,
        list_time: NumberString,
        exp_time: NumberString,
        fix_time: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// `open-interest` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-open-interest-channel>
    OpenInterestUpdate {
        inst_type: String,
        inst_id: String,
        oi: NumberString,
        oi_ccy: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `funding-rate` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-funding-rate-channel>
    FundingRateUpdate {
        inst_type: String,
        inst_id: String,
        funding_rate: NumberString,
        next_funding_rate: NumberString,
        funding_time: NumberString,
        next_funding_time: NumberString,
        min_funding_rate: NumberString,
        max_funding_rate: NumberString,
        method: String,
        formula_type: String,
        premium: NumberString,
        interest_rate: NumberString,
        impact_value: NumberString,
        sett_state: String,
        sett_funding_rate: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `price-limit` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-price-limit-channel>
    PriceLimitUpdate {
        inst_id: String,
        buy_lmt: NumberString,
        sell_lmt: NumberString,
        ts: NumberString,
        enabled: bool
    }
}

ws_object! {
    /// `opt-summary` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-option-summary-channel>
    OptionSummaryUpdate {
        inst_type: String,
        inst_id: String,
        uly: String,
        delta_bs: NumberString,
        delta_pa: NumberString,
        gamma_bs: NumberString,
        gamma_pa: NumberString,
        vega_bs: NumberString,
        vega_pa: NumberString,
        theta_bs: NumberString,
        theta_pa: NumberString,
        lever: NumberString,
        mark_vol: NumberString,
        bid_vol: NumberString,
        ask_vol: NumberString,
        real_vol: NumberString,
        vol_lv: NumberString,
        fwd_px: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `estimated-price` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-estimated-delivery-exercise-settlement-price-channel>
    EstimatedPriceUpdate {
        inst_type: String,
        inst_id: String,
        settle_type: String,
        #[serde(alias = "settPx")]
        settle_px: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `mark-price` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-mark-price-channel>
    MarkPriceUpdate {
        inst_type: String,
        inst_id: String,
        mark_px: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `index-tickers` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-index-tickers-channel>
    IndexTickerUpdate {
        inst_id: String,
        idx_px: NumberString,
        high24h: NumberString,
        open24h: NumberString,
        low24h: NumberString,
        sod_utc0: NumberString,
        sod_utc8: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// Detail row nested in [`LiquidationOrderUpdate`].
    LiquidationOrderDetail {
        side: String,
        pos_side: String,
        bk_px: NumberString,
        sz: NumberString,
        bk_loss: NumberString,
        ccy: String,
        ts: NumberString
    }
}

ws_object! {
    /// `liquidation-orders` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-liquidation-orders-channel>
    LiquidationOrderUpdate {
        inst_id: String,
        inst_type: String,
        uly: String,
        inst_family: String,
        details: Vec<LiquidationOrderDetail>
    }
}

ws_object! {
    /// `adl-warning` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-adl-warning-channel>
    AdlWarningUpdate {
        inst_type: String,
        inst_family: String,
        ccy: String,
        max_bal: NumberString,
        adl_rec_bal: NumberString,
        bal: NumberString,
        max_bal_ts: NumberString,
        adl_type: String,
        state: String,
        adl_bal: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `economic-calendar` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-public-data-ws-economic-calendar-channel>
    EconomicCalendarUpdate {
        calendar_id: String,
        date: String,
        time: String,
        region: String,
        category: String,
        event: String,
        ref_date: String,
        actual: String,
        previous: String,
        forecast: String,
        importance: String,
        unit: String,
        currency: String,
        ccy: String,
        date_span: String,
        prev_initial: String,
        u_time: NumberString,
        ts: NumberString
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_current_instrument_and_estimated_price_fields() {
        let instrument: InstrumentUpdate = serde_json::from_str(
            r#"{"instType":"FUTURES","instId":"BTC-USD-260626","groupId":"g1","auctionEndTime":"1","contTdSwTime":"2","maxLmtAmt":"3","upcChg":[{"param":"tickSz","newValue":"0.1","effTime":"4"}]}"#,
        )
        .unwrap();
        assert_eq!(instrument.group_id, "g1");
        assert_eq!(instrument.upc_chg[0].new_value, "0.1");

        let estimated: EstimatedPriceUpdate = serde_json::from_str(
            r#"{"instType":"OPTION","instId":"BTC-USD-260626-100000-C","settlePx":"100","settleType":"exercise","ts":"5"}"#,
        )
        .unwrap();
        assert_eq!(estimated.settle_px.as_str(), "100");
    }
}
