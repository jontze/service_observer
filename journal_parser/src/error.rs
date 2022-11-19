use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum JournalError {
    #[error("Failed to parse output to response")]
    Parsing(#[from] serde_json::Error),
    #[error("Failed to parse stdout")]
    OutputParsing(#[from] FromUtf8Error),
    #[error("Failed to execute the journal command")]
    Exec(#[from] std::io::Error),
}
