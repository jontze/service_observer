use self::config::{ensure_app_files_exist, Settings};
use args::Args;
use clap::Parser;
use journal_parser::{journal::Journal, models::LogEntry};
use ratatui::widgets::TableState;
use sshd_logs::SshdLogs;
use std::net::Ipv4Addr;

mod args;
pub mod config;
pub mod constants;

pub(crate) struct App {
    pub args: Args,
    pub settings: Settings,
    pub tab_titles: Vec<&'static str>,
    pub tab_index: usize,
    pub ssh_table_state: TableState,
    pub ssh_logs: Vec<(Ipv4Addr, Vec<LogEntry>, usize)>,
    pub map_locations: Vec<(f64, f64)>,
}

impl Default for App {
    fn default() -> Self {
        let args = Args::parse();
        let settings = Settings::new();
        ensure_app_files_exist();
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
            settings,
            tab_titles: vec!["SSH Logs", "Map"],
            tab_index: 0,
            ssh_table_state: TableState::default(),
            ssh_logs,
            map_locations: vec![],
        }
    }
}

impl App {
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }

    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }

    pub fn down_row(&mut self) {
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

    pub fn up_row(&mut self) {
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

    pub fn mut_ssh_logs(&self) -> Vec<(Ipv4Addr, Vec<LogEntry>, usize)> {
        self.ssh_logs.to_owned()
    }

    pub fn add_geolocation(&mut self, lat: f64, lng: f64) {
        self.map_locations.push((lat, lng));
    }
}
