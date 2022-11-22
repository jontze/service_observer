use app::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::Backend;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod ui;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui::ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Left => app.previous_tab(),
                KeyCode::Right => app.next_tab(),
                KeyCode::Down => app.down_row(),
                KeyCode::Up => app.up_row(),
                _ => {}
            }
        }
    }
}

fn main() {
    // Setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App and run
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // Restore Terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    // Print errors during exec
    terminal.show_cursor().unwrap();
    if let Err(err) = res {
        println!("{:?}", err)
    }
}
