use app::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tui::backend::Backend;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod events;
mod ui;
mod util;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    // To control the loop until CTRL+C
    let is_terminated = Arc::new(AtomicBool::new(false));
    let is_terminated_clone = is_terminated.clone();
    ctrlc::set_handler(move || {
        is_terminated_clone.store(true, Ordering::SeqCst);
    })
    .unwrap();

    while !is_terminated.load(Ordering::SeqCst) {
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
    Ok(())
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
