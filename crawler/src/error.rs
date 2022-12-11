#[derive(thiserror::Error, Debug)]
pub enum CrawlerError {
    #[error("Failed to interact with shodan")]
    ScannerInteraction(#[from] ip_geolocation::IpScannerError),
    #[error("Failed to interact with the database")]
    DbInteraction(#[from] sea_orm::DbErr),
}
