use app::App;
use crossterm::event::{self, Event, KeyCode};
use events::ObserverEvents;
use std::sync::mpsc;
use tui::backend::Backend;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod events;
mod ui;
mod util;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui::ui(frame, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Left => app.previous_tab(),
                KeyCode::Right => app.next_tab(),
                KeyCode::Down => app.down_row(),
                KeyCode::Up => app.up_row(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() {
    let stdout = util::setup_terminal().unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let (_, _) = mpsc::channel::<ObserverEvents>();

    // Create App and run
    let app = App::default();
    let res = run_app(&mut terminal, app);

    util::cleanup_terminal(&mut terminal).unwrap();

    // Print errors during exec
    if let Err(err) = res {
        println!("{:?}", err)
    }
}
