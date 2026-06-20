use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap};

use crate::app::{App, LogLevel};
use crate::ui::status_style;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::vertical([
        Constraint::Length(8),
        Constraint::Fill(1),
        Constraint::Length(8),
    ])
    .split(area);
    let top = Layout::horizontal([
        Constraint::Percentage(34),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ])
    .split(rows[0]);

    render_market_card(f, top[0], app);
    render_account_card(f, top[1], app);
    render_connection_card(f, top[2], app);
    render_orders_and_positions(f, rows[1], app);
    render_recent_logs(f, rows[2], app);
}

fn render_market_card(f: &mut Frame, area: Rect, app: &App) {
    let last = app
        .last_price()
        .map(|v| format!("{v:.4}"))
        .unwrap_or_else(|| "-".to_owned());
    let change = app
        .daily_change_pct()
        .map(|v| format!("{v:+.2}%"))
        .unwrap_or_else(|| "-".to_owned());
    let (bid, ask) = app.best_bid_ask();
    let lines = vec![
        Line::from(format!("  {} {}", app.config.inst_id, app.config.bar)),
        Line::from(vec![
            Span::raw("  Last: "),
            Span::styled(last, Style::default().fg(Color::Yellow).bold()),
            Span::raw("   24h: "),
            Span::styled(change, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(format!(
            "  Bid: {}   Ask: {}",
            bid.map(|v| v.price.as_str()).unwrap_or("-"),
            ask.map(|v| v.price.as_str()).unwrap_or("-")
        )),
        Line::from(format!(
            "  Candles: {}   Trades: {}",
            app.candles.len(),
            app.trades.len()
        )),
    ];
    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" 行情摘要 "))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_account_card(f: &mut Frame, area: Rect, app: &App) {
    let total_eq: f64 = app
        .balance_details()
        .filter_map(|row| row.eq.as_str().parse::<f64>().ok())
        .sum();
    let lines = vec![
        Line::from(vec![
            Span::raw("  Mode: "),
            Span::styled(
                app.config.mode_label(),
                Style::default().fg(if app.config.demo {
                    Color::Yellow
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(format!("  Total eq*: {total_eq:.4}")),
        Line::from(format!("  Balances: {}", app.balance_details().count())),
        Line::from(format!("  Positions: {}", app.positions.len())),
        Line::styled(
            "  *按返回币种直接求和，仅作快速视图",
            Style::default().fg(Color::DarkGray),
        ),
    ];
    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" 账户摘要 ")),
        area,
    );
}

fn render_connection_card(f: &mut Frame, area: Rect, app: &App) {
    let statuses = [
        ("REST", &app.statuses.rest),
        ("Market", &app.statuses.market),
        ("Candles", &app.statuses.candles),
        ("Private", &app.statuses.private),
    ];
    let rows = statuses.into_iter().map(|(name, status)| {
        Row::new(vec![
            Cell::from(name),
            Cell::from(format!("{:?}", status.state)).style(status_style(&status.state)),
            Cell::from(status.detail.as_str()),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(14),
            Constraint::Fill(1),
        ],
    )
    .block(Block::default().borders(Borders::ALL).title(" 连接状态 "));
    f.render_widget(table, area);
}

fn render_orders_and_positions(f: &mut Frame, area: Rect, app: &App) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let orders: Vec<Row> = app
        .orders
        .iter()
        .take(8)
        .map(|o| {
            Row::new(vec![
                Cell::from(o.inst_id.as_str()),
                Cell::from(o.side.as_str()),
                Cell::from(o.ord_type.as_str()),
                Cell::from(o.px.as_str()),
                Cell::from(o.sz.as_str()),
                Cell::from(o.state.as_str()),
            ])
        })
        .collect();
    let orders = Table::new(
        orders,
        [
            Constraint::Percentage(26),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Length(12),
        ],
    )
    .header(Row::new(["合约", "方向", "类型", "价格", "数量", "状态"]).fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL).title(" 挂单 "));
    f.render_widget(orders, chunks[0]);

    let positions: Vec<Row> = app
        .positions
        .iter()
        .take(8)
        .map(|p| {
            Row::new(vec![
                Cell::from(p.inst_id.as_str()),
                Cell::from(p.pos_side.as_str()),
                Cell::from(p.pos.as_str()),
                Cell::from(p.upl.as_str()),
            ])
        })
        .collect();
    let positions = Table::new(
        positions,
        [
            Constraint::Percentage(34),
            Constraint::Length(8),
            Constraint::Percentage(28),
            Constraint::Percentage(30),
        ],
    )
    .header(Row::new(["合约", "方向", "数量", "未实现"]).fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL).title(" 持仓 "));
    f.render_widget(positions, chunks[1]);
}

fn render_recent_logs(f: &mut Frame, area: Rect, app: &App) {
    let lines: Vec<Line> = app
        .logs
        .iter()
        .take(5)
        .map(|entry| {
            let style = match entry.level {
                LogLevel::Info => Style::default().fg(Color::Gray),
                LogLevel::Warn => Style::default().fg(Color::Yellow),
                LogLevel::Error => Style::default().fg(Color::Red),
            };
            Line::styled(format!("  {:?}: {}", entry.level, entry.message), style)
        })
        .collect();
    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" 最近事件 ")),
        area,
    );
}
