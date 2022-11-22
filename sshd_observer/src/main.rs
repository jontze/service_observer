use std::net::Ipv4Addr;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use journal_parser::{journal::Journal, models::LogEntry};
use sshd_logs::SshdLogs;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Map, MapResolution},
        Block, Borders, Row, Table, TableState, Tabs,
    },
    Frame,
};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {}

struct App {
    args: Args,
    tab_titles: Vec<&'static str>,
    tab_index: usize,
    ssh_table_state: TableState,
    ssh_logs: Vec<(Ipv4Addr, Vec<LogEntry>, usize)>,
}

impl App {
    fn with_args(args: Args) -> Self {
        let sshd_output = Journal::with_service("sshd")
            .since("yesterday")
            .no_pager()
            .build()
            .read()
            .unwrap();
        let mut ssh_logs: Vec<(Ipv4Addr, Vec<LogEntry>, usize)> = sshd_output
            .by_ips()
            .unwrap()
            .into_iter()
            .map(|(ip_key, log_value)| {
                let mut log_amount = 0;
                let logs: Vec<LogEntry> = log_value
                    .into_iter()
                    // Yep, not nice
                    .map(|e| {
                        log_amount += 1;
                        e.clone()
                    })
                    .collect();
                (ip_key, logs, log_amount)
            })
            .collect();
        ssh_logs.sort_by(|(_, _, amount_1), (_, _, amount_2)| amount_2.cmp(amount_1));
        Self {
            args,
            tab_titles: vec!["SSH Logs", "Map"],
            tab_index: 0,
            ssh_table_state: TableState::default(),
            ssh_logs,
        }
    }

    fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }

    fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }

    fn down_row(&mut self) {
        // Only on table tab
        if self.tab_index == 0 {
            let i = match self.ssh_table_state.selected() {
                Some(i) => {
                    if i >= self.ssh_logs.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.ssh_table_state.select(Some(i));
        }
    }

    fn up_row(&mut self) {
        // Only on table tab
        if self.tab_index == 0 {
            let i = match self.ssh_table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.ssh_logs.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.ssh_table_state.select(Some(i));
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());
    tabs(frame, chunks[0], chunks[1], app);
}

fn tabs<B: Backend>(frame: &mut Frame<B>, tab_area: Rect, body_area: Rect, app: &mut App) {
    let titels = app
        .tab_titles
        .iter()
        .map(|title_str| {
            let (first, rest) = title_str.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tab_style = Style::default().fg(Color::Cyan);
    let tab_highlight = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Black);
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
            map(frame, body_area);
        }
        _ => unreachable!(),
    };
}

fn table<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &mut App) {
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

fn map<B: Backend>(frame: &mut Frame<B>, area: Rect) {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

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
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App and run
    let app = App::with_args(args);
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
