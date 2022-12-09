use crate::app::App;
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

pub(crate) fn map<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &mut App) {
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
            ctx.layer();
            ctx.draw(&Points {
                coords: &app.map_locations,
                color: Color::Red,
            });
        });
    frame.render_widget(canvas, area);
}
