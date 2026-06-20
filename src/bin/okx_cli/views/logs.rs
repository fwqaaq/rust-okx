use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::app::{App, LogLevel};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);
    let items: Vec<ListItem> = app
        .logs
        .iter()
        .map(|entry| {
            let style = match entry.level {
                LogLevel::Info => Style::default().fg(Color::Gray),
                LogLevel::Warn => Style::default().fg(Color::Yellow),
                LogLevel::Error => Style::default().fg(Color::Red),
            };
            ListItem::new(Line::styled(
                format!("{:?}: {}", entry.level, entry.message),
                style,
            ))
        })
        .collect();
    f.render_widget(
        List::new(items).block(Block::default().borders(Borders::ALL).title(" 事件日志 ")),
        chunks[0],
    );

    let help = [
        "快捷键",
        "",
        "Tab / ← / →: 切换页面",
        "1..6: 直接跳转页面",
        "r: 手动刷新 REST 数据",
        "p: 暂停/恢复行情显示",
        "/: 输入新交易对",
        "b: 切换 K 线周期",
        "",
        "Orders 页",
        "↑/↓: 选择订单",
        "c: 撤销选中订单",
        "",
        "Trade 页",
        "数字/.: 输入当前字段",
        "↑/↓: 切换数量/价格字段",
        "s/o/m: 切方向/类型/模式",
        "Enter: 提交确认",
    ]
    .join("\n");
    f.render_widget(
        Paragraph::new(help)
            .block(Block::default().borders(Borders::ALL).title(" 帮助 "))
            .wrap(Wrap { trim: false }),
        chunks[1],
    );
}
