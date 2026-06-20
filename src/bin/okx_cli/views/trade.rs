use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::{App, TradeField, TradeTypeInput};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)]).split(area);
    render_form(f, chunks[0], app);
    render_context(f, chunks[1], app);
}

fn render_form(f: &mut Frame, area: Rect, app: &App) {
    let price_focus = app.trade.focused == TradeField::Price;
    let size_focus = app.trade.focused == TradeField::Size;
    let mut lines = vec![
        Line::from(vec![
            Span::raw("  环境: "),
            Span::styled(
                app.config.mode_label(),
                Style::default().fg(if app.config.demo {
                    Color::Yellow
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(format!("  交易对: {}", app.config.inst_id)),
        Line::from(format!("  方向(s): {}", app.trade.side.label())),
        Line::from(format!("  类型(o): {}", app.trade.order_type.label())),
        Line::from(format!("  模式(m): {}", app.trade.trade_mode.label())),
        Line::from(vec![
            Span::raw("  数量: "),
            Span::styled(
                if app.trade.size.is_empty() {
                    "<输入>"
                } else {
                    app.trade.size.as_str()
                },
                focus_style(size_focus),
            ),
        ]),
    ];
    if app.trade.order_type == TradeTypeInput::Limit {
        lines.push(Line::from(vec![
            Span::raw("  价格: "),
            Span::styled(
                if app.trade.price.is_empty() {
                    "<输入>"
                } else {
                    app.trade.price.as_str()
                },
                focus_style(price_focus),
            ),
        ]));
    } else {
        lines.push(Line::styled(
            "  价格: market 不需要价格",
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::raw(""));
    lines.push(Line::styled(
        if app.config.trade_enabled {
            "  Enter: 提交确认    c: 撤销 Orders 页选中订单"
        } else {
            "  只读模式：用 --trade-enabled 启用交易"
        },
        Style::default().fg(if app.config.trade_enabled {
            Color::Green
        } else {
            Color::Yellow
        }),
    ));
    lines.push(Line::raw(""));
    lines.push(Line::from(format!("  状态: {}", app.trade.message)));

    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" 下单面板 "))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_context(f: &mut Frame, area: Rect, app: &App) {
    let last = app
        .last_price()
        .map(|v| format!("{v:.4}"))
        .unwrap_or_else(|| "-".to_owned());
    let (bid, ask) = app.best_bid_ask();
    let selected = app
        .selected_order()
        .map(|order| {
            format!(
                "{} {} {} {} @ {}",
                order.ord_id, order.side, order.ord_type, order.sz, order.px
            )
        })
        .unwrap_or_else(|| "无选中挂单".to_owned());
    let lines = vec![
        Line::from(format!("  Last: {last}")),
        Line::from(format!(
            "  Bid / Ask: {} / {}",
            bid.map(|v| v.price.as_str()).unwrap_or("-"),
            ask.map(|v| v.price.as_str()).unwrap_or("-")
        )),
        Line::from(format!("  Open orders: {}", app.orders.len())),
        Line::raw(""),
        Line::from("  选中订单:"),
        Line::styled(format!("  {selected}"), Style::default().fg(Color::Cyan)),
        Line::raw(""),
        Line::styled(
            "  安全规则：所有下单/撤单都会弹出 y 二次确认；live 模式尤其醒目。",
            Style::default().fg(Color::DarkGray),
        ),
    ];
    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" 交易上下文 "))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn focus_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    }
}
