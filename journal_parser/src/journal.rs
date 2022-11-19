use std::process::{Command, Stdio};

use crate::{
    error::JournalError,
    models::{LogOutput, Parser},
};

pub struct Journal<'a> {
    service: &'a str,
    format: &'a str,
    order: Option<&'static str>,
    no_pager: Option<&'static str>,
    lines: Option<usize>,
    quiet: Option<&'static str>,
    since: Option<&'a str>,
    until: Option<&'a str>,
}

impl<'a> Journal<'a> {
    pub fn with_service(service: &'a str) -> JournalBuilder<'a> {
        JournalBuilder {
            service,
            format: "json",
            order: None,
            no_pager: None,
            lines: None,
            quiet: None,
            since: None,
            until: None,
        }
    }

    pub fn read(&self) -> Result<LogOutput, JournalError> {
        let mut command = Command::new("journalctl");
        command
            .env("LC_ALL", "en_US.UTF-8")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());
        command.args(["-u", self.service, "-o", self.format]);
        if let Some(no_pager) = self.no_pager {
            command.arg(no_pager);
        };
        if let Some(order) = self.order {
            command.arg(order);
        };
        if let Some(lines) = self.lines {
            command.args(["-n", &lines.to_string()]);
        };
        if let Some(quiet) = self.quiet {
            command.arg(quiet);
        };
        if let Some(since) = self.since {
            command.args(["-S", since]);
        };
        if let Some(until) = self.until {
            command.args(["-U", until]);
        };
        let output = command.spawn().and_then(|child| child.wait_with_output())?;
        LogOutput::parse(String::from_utf8(output.stdout)?)
    }
}

pub struct JournalBuilder<'a> {
    service: &'a str,
    format: &'a str,
    lines: Option<usize>,
    no_pager: Option<&'static str>,
    order: Option<&'static str>,
    quiet: Option<&'static str>,
    since: Option<&'a str>,
    until: Option<&'a str>,
}

impl<'a> JournalBuilder<'a> {
    pub fn reverse(mut self) -> Self {
        self.order = Some("--reverse");
        self
    }

    pub fn no_pager(mut self) -> Self {
        self.no_pager = Some("--no-pager");
        self
    }

    pub fn output(mut self, output: &'a str) -> Self {
        self.format = output;
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = Some(lines);
        self
    }

    pub fn quiet(mut self) -> Self {
        self.quiet = Some("--quite");
        self
    }

    pub fn since(mut self, since_date: &'a str) -> Self {
        self.since = Some(since_date);
        self
    }

    pub fn until(mut self, until_date: &'a str) -> Self {
        self.until = Some(until_date);
        self
    }

    pub fn build(self) -> Journal<'a> {
        Journal {
            service: self.service,
            order: self.order,
            no_pager: self.no_pager,
            format: self.format,
            lines: self.lines,
            quiet: self.quiet,
            since: self.since,
            until: self.until,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::OutputStatus;

    use super::*;

    const LOG_AMOUNT: usize = 5;
    const LOG_SERVICE: &'static str = "sshd";

    #[test]
    fn should_read_from_journal() {
        let log_output = Journal::with_service(LOG_SERVICE)
            .reverse()
            .lines(LOG_AMOUNT)
            .no_pager()
            .build()
            .read();
        assert!(log_output.is_ok(), "Failed to parse logs")
    }

    #[test]
    fn should_read_lines_amount() {
        let output = Journal::with_service(LOG_SERVICE)
            .reverse()
            .lines(LOG_AMOUNT)
            .no_pager()
            .build()
            .read();
        let log_output = output.unwrap();
        assert_eq!(
            log_output.amount + log_output.failed_amount,
            LOG_AMOUNT,
            "Should extract given log amount"
        );
    }

    #[test]
    fn should_read_successfully() {
        let log_output = Journal::with_service(LOG_SERVICE)
            .reverse()
            .lines(LOG_AMOUNT)
            .no_pager()
            .build()
            .read();
        assert!(log_output.is_ok(), "Failed to parse logs");
        assert_eq!(
            log_output.unwrap().status,
            OutputStatus::SUCCESSFUL,
            "Should return output successfully"
        );
    }

    #[test]
    fn should_return_without_warnings() {
        let log_output = Journal::with_service(LOG_SERVICE)
            .reverse()
            .lines(LOG_AMOUNT)
            .no_pager()
            .build()
            .read();
        assert!(log_output.is_ok(), "Failed to parse logs");
        assert_ne!(
            log_output.unwrap().status,
            OutputStatus::WARNINGS,
            "Should return output without any warnings"
        );
    }
    #[test]
    fn should_return_without_failure() {
        let log_output = Journal::with_service(LOG_SERVICE)
            .reverse()
            .lines(LOG_AMOUNT)
            .no_pager()
            .build()
            .read();
        assert!(log_output.is_ok(), "Failed to parse logs");
        assert_ne!(
            log_output.unwrap().status,
            OutputStatus::FAILED,
            "Should return output without failure"
        );
    }
}
