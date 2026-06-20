use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]).split(area);
    let rows: Vec<Row> = app
        .orders
        .iter()
        .enumerate()
        .map(|(idx, order)| {
            let prefix = if idx == app.selected_order { ">" } else { " " };
            Row::new(vec![
                Cell::from(prefix),
                Cell::from(order.inst_id.as_str()),
                Cell::from(order.ord_id.as_str()),
                Cell::from(order.side.as_str()),
                Cell::from(order.ord_type.as_str()),
                Cell::from(order.px.as_str()),
                Cell::from(order.sz.as_str()),
                Cell::from(order.acc_fill_sz.as_str()),
                Cell::from(order.state.as_str()),
            ])
        })
        .collect();
    let table = Table::new(
        rows,
        [
            Constraint::Length(2),
            Constraint::Percentage(17),
            Constraint::Percentage(18),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Length(12),
        ],
    )
    .header(
        Row::new([
            "", "合约", "ordId", "方向", "类型", "价格", "数量", "成交", "状态",
        ])
        .fg(Color::Cyan),
    )
    .block(Block::default().borders(Borders::ALL).title(" 当前挂单 "));
    f.render_widget(table, chunks[0]);

    let help = if app.config.trade_enabled {
        "  ↑/↓ 选择订单    c 撤销选中订单    r 刷新订单"
    } else {
        "  当前只读：启动时添加 --trade-enabled 后可撤销选中订单"
    };
    f.render_widget(
        Paragraph::new(help).block(Block::default().borders(Borders::ALL).title(" 操作 ")),
        chunks[1],
    );
}
