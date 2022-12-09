use app::config::get_db_path;
use app::{constants, App};
use clokwerk::{AsyncScheduler, TimeUnits};
use crawler::{AppCrawler, Crawler};
use crossterm::event::{self, Event, KeyCode};
use events::{IpGeolocation, ObserverEvents};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::time::Duration;
use tui::backend::Backend;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod events;
mod ui;
mod util;

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    event_recevier: Receiver<ObserverEvents>,
) -> std::io::Result<()> {
    loop {
        if let Ok(event_received) =
            event::poll(Duration::from_millis(constants::APP_DRAW_TICK_RATE / 2))
        {
            if event_received {
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
        }
        if let Ok(input_event) =
            event_recevier.recv_timeout(Duration::from_millis(constants::APP_DRAW_TICK_RATE / 2))
        {
            match input_event {
                ObserverEvents::Geolocation(location) => {
                    app.add_geolocation(location.lat, location.lng)
                }
            }
        }
        terminal.draw(|frame| ui::ui(frame, &mut app))?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let stdout = util::setup_terminal().unwrap();

    let mut scheduler = AsyncScheduler::new();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let (sender, receiver) = mpsc::sync_channel::<ObserverEvents>(1);

    // Create task runner thread
    let sender_arc = Arc::new(sender);

    // Create App and run
    let app = App::default();
    let crawler = Arc::new(
        Crawler::new(
            &get_db_path().into_os_string().into_string().unwrap(),
            &std::env::var("SHODAN_TOKEN").expect("Missing shodan token"),
        )
        .await,
    );
    let ssh_logs = Arc::new(app.mut_ssh_logs());
    scheduler.every(1.seconds()).run(move || {
        let ssh_logs = ssh_logs.clone();
        let thread_sender = sender_arc.clone();
        let thread_crawler = crawler.clone();
        async move {
            for (ip, _, _) in ssh_logs.iter() {
                let (lat, lng) = thread_crawler.geolocation(&ip).await;
                thread_sender
                    .send(ObserverEvents::Geolocation(IpGeolocation {
                        lat,
                        lng,
                        ipv4: ip.to_owned(),
                    }))
                    .unwrap();
            }
        }
    });
    let task_handler = tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(constants::TASK_TICK_RATE_MS)).await;
        }
    });
    let res = run_app(&mut terminal, app, receiver);

    util::cleanup_terminal(&mut terminal).unwrap();
    task_handler.abort();

    // Print errors during exec
    if let Err(err) = res {
        println!("{:?}", err)
    }
}
