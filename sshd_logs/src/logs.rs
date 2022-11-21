use journal_parser::models::{LogEntry, LogOutput, OutputStatus};
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, net::Ipv4Addr};

use crate::SshLogParserError;

pub trait SshdLogs {
    fn by_ips(&self) -> Result<HashMap<Ipv4Addr, Vec<&LogEntry>>, SshLogParserError>;
}

impl SshdLogs for LogOutput {
    fn by_ips(&self) -> Result<HashMap<Ipv4Addr, Vec<&LogEntry>>, SshLogParserError> {
        let ipv4_regex = Regex::new(
                 r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}",
             ).unwrap();
        if self.status == OutputStatus::FAILED {
            return Err(SshLogParserError::LogExtraction);
        };
        Ok(self
            .logs
            .par_iter()
            .fold(
                HashMap::<Ipv4Addr, Vec<&LogEntry>>::new,
                |mut ipv4_map, log| {
                    if let Some(ip_match) = ipv4_regex.find(&log.message) {
                        let extracted_ip = log.message[ip_match.start()..ip_match.end()]
                            .parse::<Ipv4Addr>()
                            .unwrap();
                        match ipv4_map.get_mut(&extracted_ip) {
                            Some(ip_logs) => {
                                ip_logs.push(log);
                            }
                            None => {
                                ipv4_map.insert(extracted_ip, vec![log]);
                            }
                        };
                        ipv4_map
                    } else {
                        ipv4_map
                    }
                },
            )
            .reduce(
                HashMap::<Ipv4Addr, Vec<&LogEntry>>::new,
                |mut map_1, map_2| {
                    println!("Map 2: {:?}", map_2);
                    map_2
                        .into_iter()
                        .for_each(|(addr, logs)| match map_1.get_mut(&addr) {
                            Some(map_1_entry) => {
                                map_1_entry.extend(logs);
                            }
                            None => {
                                map_1.insert(addr, logs);
                            }
                        });
                    map_1
                },
            ))
    }
}

#[cfg(test)]
mod tests {
    use journal_parser::journal::Journal;

    use super::*;

    #[test]
    fn read_todays_logs() {
        let output = Journal::with_service("sshd")
            .since("today")
            .build()
            .read()
            .unwrap();
        let ip_hash = output.by_ips().unwrap();
        assert!(ip_hash.len() > 0);
    }
}
