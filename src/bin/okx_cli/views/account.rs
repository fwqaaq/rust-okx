use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).split(area);

    render_permissions(f, chunks[0], app);
    render_balances(f, chunks[1], app);
}

fn render_permissions(f: &mut Frame, area: Rect, app: &App) {
    let lines = if app.perm.is_empty() {
        // OKX did not return a perm field for this account/key type
        vec![
            Line::raw(""),
            Line::styled(
                "  权限信息不可用（API Key 未返回 perm 字段）",
                Style::default().fg(Color::DarkGray),
            ),
        ]
    } else {
        let has_trade = app.perm.contains("trade");
        let has_withdraw = app.perm.contains("withdraw");

        let mut v = vec![
            Line::from(vec![
                Span::raw("  交易: "),
                Span::styled(
                    if has_trade {
                        "✓ 已开启"
                    } else {
                        "✗ 未开启"
                    },
                    Style::default().fg(if has_trade { Color::Green } else { Color::Red }),
                ),
                Span::raw("   提现: "),
                Span::styled(
                    if has_withdraw {
                        "✓ 已开启"
                    } else {
                        "✗ 未开启"
                    },
                    Style::default().fg(if has_withdraw {
                        Color::Green
                    } else {
                        Color::Red
                    }),
                ),
            ]),
            Line::raw(""),
        ];

        if !has_trade {
            v.push(Line::styled(
                "  ⚠  该 API Key 无交易权限，无法下单",
                Style::default().fg(Color::Yellow),
            ));
        }
        if !has_withdraw {
            v.push(Line::styled(
                "  ⚠  该 API Key 无提现权限，无法提现",
                Style::default().fg(Color::Yellow),
            ));
        }
        v
    };

    let para =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" 账户权限 "));
    f.render_widget(para, area);
}

fn render_balances(f: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec![
        Cell::from("币种").style(Style::default().fg(Color::Cyan).bold()),
        Cell::from("总权益").style(Style::default().fg(Color::Cyan).bold()),
        Cell::from("可用余额").style(Style::default().fg(Color::Cyan).bold()),
        Cell::from("冻结余额").style(Style::default().fg(Color::Cyan).bold()),
    ]);

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

    let widths = [
        Constraint::Length(10),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::default().reversed())
        .block(Block::default().borders(Borders::ALL).title(" 余额明细 "));

    f.render_widget(table, area);
}
