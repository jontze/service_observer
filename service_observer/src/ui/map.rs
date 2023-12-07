use crate::app::App;
use ratatui::{
    backend::Backend,
    layout::Rect,
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
                color: app.settings.ui.primary_color,
                resolution: MapResolution::High,
            });
            ctx.layer();
            // The point struct expect (Longitude, Latitude)
            let reversed_coords: Vec<(f64, f64)> = app
                .map_locations
                .iter()
                .map(|(lat, lng)| (*lng, *lat))
                .collect();
            ctx.draw(&Points {
                coords: &reversed_coords,
                color: app.settings.ui.accent_color,
            });
        });
    frame.render_widget(canvas, area);
}
