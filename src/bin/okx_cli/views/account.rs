use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(7), Constraint::Fill(1)]).split(area);
    render_summary(f, chunks[0], app);
    render_tables(f, chunks[1], app);
}

fn render_summary(f: &mut Frame, area: Rect, app: &App) {
    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::raw("  权限: "),
        Span::styled(
            if app.perm.is_empty() {
                "unknown"
            } else {
                app.perm.as_str()
            },
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("    模式: "),
        Span::styled(
            app.config.mode_label(),
            Style::default().fg(if app.config.demo {
                Color::Yellow
            } else {
                Color::Red
            }),
        ),
        Span::raw("    交易: "),
        Span::styled(
            if app.config.trade_enabled {
                "enabled"
            } else {
                "read-only"
            },
            Style::default().fg(if app.config.trade_enabled {
                Color::Green
            } else {
                Color::DarkGray
            }),
        ),
    ]));
    lines.push(Line::raw(""));
    lines.push(Line::from(format!(
        "  非零余额: {}    持仓: {}    挂单: {}",
        app.balance_details().count(),
        app.positions.len(),
        app.orders.len()
    )));
    lines.push(Line::styled(
        "  提醒：Trade 页只有 --trade-enabled 时才允许真实交易动作。",
        Style::default().fg(Color::DarkGray),
    ));

    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" 账户概览 ")),
        area,
    );
}

fn render_tables(f: &mut Frame, area: Rect, app: &App) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    render_balances(f, chunks[0], app);
    render_positions(f, chunks[1], app);
}

fn render_balances(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .balance_details()
        .map(|d| {
            Row::new(vec![
                Cell::from(d.ccy.as_str()),
                Cell::from(d.eq.as_str()),
                Cell::from(d.avail_bal.as_str()),
                Cell::from(d.frozen_bal.as_str()),
            ])
        })
        .collect();
    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Percentage(32),
            Constraint::Percentage(32),
            Constraint::Percentage(28),
        ],
    )
    .header(Row::new(["币种", "权益", "可用", "冻结"]).style(Style::default().fg(Color::Cyan)))
    .block(Block::default().borders(Borders::ALL).title(" 余额 "));
    f.render_widget(table, area);
}

fn render_positions(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .positions
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.inst_id.as_str()),
                Cell::from(p.pos_side.as_str()),
                Cell::from(p.pos.as_str()),
                Cell::from(p.avg_px.as_str()),
                Cell::from(p.upl.as_str()),
            ])
        })
        .collect();
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(32),
            Constraint::Length(8),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .header(
        Row::new(["合约", "方向", "数量", "均价", "未实现"])
            .style(Style::default().fg(Color::Cyan)),
    )
    .block(Block::default().borders(Borders::ALL).title(" 持仓 "));
    f.render_widget(table, area);
}
