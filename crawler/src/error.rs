#[derive(thiserror::Error, Debug)]
pub enum CrawlerError {
    #[error("Failed to interact with shodan")]
    ScannerInteraction(#[from] ip_geolocation::IpScannerError),
}
