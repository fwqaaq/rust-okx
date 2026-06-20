use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::app::{App, Tab};
use crate::views;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);

    let selected = match app.tab {
        Tab::Account => 0,
        Tab::Chart => 1,
        Tab::Feed => 2,
    };
    let tabs = Tabs::new(vec!["[1] 账户余额", "[2] K 线图", "[3] 实时行情"])
        .block(Block::default().borders(Borders::ALL).title(" OKX CLI "))
        .highlight_style(Style::default().fg(Color::Yellow).bold())
        .select(selected);
    f.render_widget(tabs, chunks[0]);

    match app.tab {
        Tab::Account => views::account::render(f, chunks[1], app),
        Tab::Chart => views::chart::render(f, chunks[1], app),
        Tab::Feed => views::feed::render(f, chunks[1], app),
    }

    let footer =
        Paragraph::new("  q/Esc: 退出    Tab/→: 下一标签    ←: 上一标签    1/2/3: 直接跳转")
            .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
