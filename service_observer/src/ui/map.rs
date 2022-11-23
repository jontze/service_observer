use tui::{
    backend::Backend,
    layout::Rect,
    style::Color,
    widgets::{
        canvas::{Canvas, Map, MapResolution, Points},
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
                resolution: MapResolution::Low,
            });
            ctx.layer();
            ctx.draw(&Points {
                coords: &[(13.0, 52.0), (0.0, 0.0)],
                color: Color::Red,
            });
        });
    frame.render_widget(canvas, area);
}
