use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let title = format!(" WS 实时行情  {} ", app.inst_id);

    let items: Vec<ListItem> = app
        .tickers
        .iter()
        .map(|t| {
            let last: f64 = t.last.as_str().parse().unwrap_or(0.0);
            let open: f64 = t.open24h.as_str().parse().unwrap_or(0.0);
            let price_color = if last >= open {
                Color::Green
            } else {
                Color::Red
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{:<16}", t.ts.as_str()),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw("last="),
                Span::styled(
                    format!("{:<14}", t.last.as_str()),
                    Style::default().fg(price_color),
                ),
                Span::raw("bid="),
                Span::raw(format!("{:<14}", t.bid_px.as_str())),
                Span::raw("ask="),
                Span::raw(format!("{:<14}", t.ask_px.as_str())),
                Span::styled(
                    format!("vol={}", t.vol24h.as_str()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}
