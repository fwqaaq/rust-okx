use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::vertical([Constraint::Fill(1), Constraint::Length(10)]).split(area);
    let top =
        Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]).split(rows[0]);
    render_chart(f, top[0], app);
    render_book(f, top[1], app);
    render_trades(f, rows[1], app);
}

fn render_chart(f: &mut Frame, area: Rect, app: &App) {
    if app.candles.is_empty() {
        f.render_widget(
            Paragraph::new("无 K 线数据").block(Block::default().borders(Borders::ALL)),
            area,
        );
        return;
    }
    let candles: Vec<(f64, f64, f64, f64)> = app
        .candles
        .iter()
        .map(|c| (c.open, c.high, c.low, c.close))
        .collect();
    let n = candles.len() as f64;
    let min_p = candles
        .iter()
        .map(|(_, _, low, _)| *low)
        .fold(f64::MAX, f64::min);
    let max_p = candles
        .iter()
        .map(|(_, high, _, _)| *high)
        .fold(f64::MIN, f64::max);
    let range = (max_p - min_p).max(1.0);
    let pad = range * 0.05;
    let live_tag = if app.candles.back().map(|c| !c.confirm).unwrap_or(false) {
        " live"
    } else {
        " closed"
    };
    let last = app
        .last_price()
        .map(|v| format!("{v:.4}"))
        .unwrap_or_else(|| "-".to_owned());
    let volume = app
        .candles
        .back()
        .map(|bar| format!("{:.4}", bar.volume))
        .unwrap_or_else(|| "-".to_owned());
    let title = format!(
        " {} {} {} last={} vol={}{} ",
        app.config.inst_id,
        app.config.bar,
        if app.paused { "PAUSED" } else { "" },
        last,
        volume,
        live_tag
    );

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .x_bounds([0.0, n])
        .y_bounds([min_p - pad, max_p + pad])
        .paint(move |ctx| {
            for (i, (open, high, low, close)) in candles.iter().enumerate() {
                let x = i as f64 + 0.5;
                let color = if close >= open {
                    Color::Green
                } else {
                    Color::Red
                };
                let body_top = open.max(*close);
                let body_bot = open.min(*close);
                ctx.draw(&Line {
                    x1: x,
                    y1: *low,
                    x2: x,
                    y2: *high,
                    color: Color::DarkGray,
                });
                for dx in [-0.22_f64, 0.0, 0.22] {
                    ctx.draw(&Line {
                        x1: x + dx,
                        y1: body_bot,
                        x2: x + dx,
                        y2: body_top,
                        color,
                    });
                }
            }
        });
    f.render_widget(canvas, area);
}

fn render_book(f: &mut Frame, area: Rect, app: &App) {
    let mut rows = Vec::new();
    if let Some(book) = &app.book {
        for ask in book.asks.iter().rev().take(5) {
            rows.push(Row::new(vec![
                Cell::from("ask").style(Style::default().fg(Color::Red)),
                Cell::from(ask.price.as_str()),
                Cell::from(ask.size.as_str()),
            ]));
        }
        for bid in book.bids.iter().take(5) {
            rows.push(Row::new(vec![
                Cell::from("bid").style(Style::default().fg(Color::Green)),
                Cell::from(bid.price.as_str()),
                Cell::from(bid.size.as_str()),
            ]));
        }
    }
    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Percentage(45),
            Constraint::Percentage(50),
        ],
    )
    .header(Row::new(["侧", "价格", "数量"]).fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL).title(" 五档盘口 "));
    f.render_widget(table, area);
}

fn render_trades(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .trades
        .iter()
        .take(8)
        .map(|t| {
            let style = if t.side == "buy" {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };
            Row::new(vec![
                Cell::from(t.ts.as_str()).style(Style::default().fg(Color::DarkGray)),
                Cell::from(t.side.as_str()).style(style),
                Cell::from(t.px.as_str()),
                Cell::from(t.sz.as_str()),
                Cell::from(t.trade_id.as_str()),
            ])
        })
        .collect();
    let table = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Length(6),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .header(Row::new(["时间", "方向", "价格", "数量", "tradeId"]).fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL).title(" 实时成交 "));
    f.render_widget(table, area);
}
