use std::net::AddrParseError;

#[derive(Debug, thiserror::Error)]
pub enum SshLogParserError {
    #[error("Log extraction failed")]
    LogExtraction,
    #[error("Invalid ip")]
    InvalidIpAddr(#[from] AddrParseError),
}
