use journal_parser::models::LogOutput;

pub trait SshdLogs {}

impl SshdLogs for LogOutput {}

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
        for log in output.logs {
            println!("{log:?}");
        }
    }
}
