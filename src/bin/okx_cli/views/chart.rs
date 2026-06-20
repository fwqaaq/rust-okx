use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
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
        .map(|(_, _, l, _)| *l)
        .fold(f64::MAX, f64::min);
    let max_p = candles
        .iter()
        .map(|(_, h, _, _)| *h)
        .fold(f64::MIN, f64::max);
    let range = (max_p - min_p).max(1.0);
    let pad = range * 0.05;

    let live_tag = if app.candles.back().map(|c| !c.confirm).unwrap_or(false) {
        " [live]"
    } else {
        ""
    };
    let title = format!(
        " {} {}{}  q:退出  Tab:切换 ",
        app.inst_id, app.bar, live_tag
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

                // Wick: high to low
                ctx.draw(&Line {
                    x1: x,
                    y1: *low,
                    x2: x,
                    y2: *high,
                    color: Color::DarkGray,
                });

                // Body: three parallel vertical lines for visible width
                for dx in [-0.25_f64, 0.0, 0.25] {
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
