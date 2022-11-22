use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Map, MapResolution},
        Block, Borders,
    },
    Frame,
};

pub(crate) fn map<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Map");
    let canvas = Canvas::default()
        .block(block)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.print(
                0.0,
                0.0,
                Span::styled("You are here", Style::default().fg(Color::Yellow)),
            );
        });
    frame.render_widget(canvas, area);
}
