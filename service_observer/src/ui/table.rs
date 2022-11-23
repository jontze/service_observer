use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::app::App;

pub(crate) fn table<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &mut App) {
    let normal_style = Style::default().bg(Color::DarkGray);
    let selected_style = Style::default().add_modifier(Modifier::UNDERLINED);
    let header = Row::new(vec!["IP", "Amount of Logs"])
        .height(1)
        .bottom_margin(1)
        .style(normal_style);
    let rows: Vec<Row> = app
        .ssh_logs
        .iter()
        .map(|item| {
            let test = vec![item.0.to_string(), item.2.to_string()];
            Row::new(test)
        })
        .collect();
    let ip_table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .title("SSH Logs By IP")
                .borders(Borders::ALL),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    frame.render_stateful_widget(ip_table, area, &mut app.ssh_table_state);
}
