use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap};

use crate::app::{App, BAR_OPTIONS, StreamState, Tab};
use crate::views;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);

    let title = format!(
        " OKX CLI  {}  {} {}  trade:{} ",
        app.config.mode_label(),
        app.config.inst_id,
        app.config.bar,
        if app.config.trade_enabled {
            "enabled"
        } else {
            "read-only"
        }
    );
    let labels: Vec<Line> = Tab::ALL
        .into_iter()
        .map(|tab| Line::from(tab.label()))
        .collect();
    let tabs = Tabs::new(labels)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().fg(Color::Yellow).bold())
        .select(app.tab.index());
    f.render_widget(tabs, chunks[0]);

    match app.tab {
        Tab::Dashboard => views::dashboard::render(f, chunks[1], app),
        Tab::Market => views::market::render(f, chunks[1], app),
        Tab::Account => views::account::render(f, chunks[1], app),
        Tab::Orders => views::orders::render(f, chunks[1], app),
        Tab::Trade => views::trade::render(f, chunks[1], app),
        Tab::Logs => views::logs::render(f, chunks[1], app),
        Tab::Watchlist => views::watchlist::render(f, chunks[1], app),
    }

    let footer = if app.watchlist_editing {
        format!(
            "  添加自选交易对: {}  Enter: 保存  Esc: 取消",
            app.watchlist_input
        )
    } else if app.symbol_editing {
        format!("  输入交易对: {}  Enter: 应用  Esc: 取消", app.symbol_input)
    } else {
        "  q/Esc 退出  Tab/←/→ 切页  1-7 跳转  r 刷新  p 暂停  / 切交易对  b 切周期  7: 自选列表  a: 添加自选"
            .to_owned()
    };
    f.render_widget(
        Paragraph::new(footer).style(Style::default().fg(Color::DarkGray)),
        chunks[2],
    );

    if app.bar_picking {
        let popup = centered_rect(38, 36, area);
        f.render_widget(Clear, popup);
        let current = &app.config.bar;
        let lines: String = BAR_OPTIONS
            .iter()
            .enumerate()
            .map(|(i, bar)| {
                let marker = if *bar == current.as_str() { "▶" } else { " " };
                format!("  {}) {marker}{bar}", i + 1)
            })
            .collect::<Vec<_>>()
            .join("\n");
        let text = format!("{lines}\n\n  Esc 取消");
        f.render_widget(
            Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" 选择 K 线周期 "),
                )
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::White)),
            popup,
        );
    }

    if let Some(confirm) = &app.confirmation {
        let popup = centered_rect(62, 34, area);
        f.render_widget(Clear, popup);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 确认操作 ")
            .border_style(Style::default().fg(if app.config.demo {
                Color::Yellow
            } else {
                Color::Red
            }));
        let text = confirm.lines.join("\n");
        f.render_widget(
            Paragraph::new(text)
                .block(block)
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::White)),
            popup,
        );
    }
}

pub fn status_style(state: &StreamState) -> Style {
    match state {
        StreamState::Idle => Style::default().fg(Color::DarkGray),
        StreamState::Connecting | StreamState::Reconnecting => Style::default().fg(Color::Yellow),
        StreamState::Subscribed => Style::default().fg(Color::Green),
        StreamState::Error => Style::default().fg(Color::Red),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);
    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1])[1]
}
