#[derive(thiserror::Error, Debug)]
pub enum IpScannerError {
    #[error("Request failed")]
    RequestError,
    #[error("Response parsing failed")]
    ResponseParsingError(#[from] serde_json::Error),
}
