use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(["交易对", "最新价", "24h 涨跌"])
        .style(Style::default().fg(Color::Yellow).bold())
        .height(1);

    let rows: Vec<Row> = app
        .watchlist
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let is_active = entry.inst_id == app.config.inst_id;
            let is_selected = i == app.watchlist_cursor;

            let symbol = if is_active {
                format!("▶ {}", entry.inst_id)
            } else {
                format!("  {}", entry.inst_id)
            };

            let last = entry
                .last
                .map(|p| format!("{p:.4}"))
                .unwrap_or_else(|| "—".to_owned());

            let (change_str, change_color) = match entry.change24h {
                Some(c) if c > 0.0 => (format!("+{c:.2}%"), Color::Green),
                Some(c) if c < 0.0 => (format!("{c:.2}%"), Color::Red),
                Some(_) => ("0.00%".to_owned(), Color::DarkGray),
                None => ("—".to_owned(), Color::DarkGray),
            };

            let row = Row::new([
                Cell::from(symbol),
                Cell::from(last),
                Cell::from(change_str).style(Style::default().fg(change_color)),
            ]);

            if is_selected {
                row.style(Style::default().reversed())
            } else {
                row
            }
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(20),
            Constraint::Length(18),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" 自选列表  ↑/↓ 移动  Enter 切换行情 "),
    );

    f.render_widget(table, area);
}
