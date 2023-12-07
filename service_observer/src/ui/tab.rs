use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::App;

use super::{map::map, table::table};

pub(crate) fn tabs<B: Backend>(
    frame: &mut Frame<B>,
    tab_area: Rect,
    body_area: Rect,
    app: &mut App,
) {
    let titels = app
        .tab_titles
        .iter()
        .map(|title_str| {
            let (first, rest) = title_str.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(app.settings.ui.accent_color)),
                Span::styled(rest, Style::default().fg(app.settings.ui.primary_color)),
            ])
        })
        .collect();
    let tab_style = Style::default().fg(app.settings.ui.primary_color);
    let tab_highlight = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(app.settings.ui.secondary_color);
    let tabs = Tabs::new(titels)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .select(app.tab_index)
        .style(tab_style)
        .highlight_style(tab_highlight);
    frame.render_widget(tabs, tab_area);
    match app.tab_index {
        0 => {
            table(frame, body_area, app);
        }
        1 => {
            map(frame, body_area, app);
        }
        _ => unreachable!(),
    };
}
