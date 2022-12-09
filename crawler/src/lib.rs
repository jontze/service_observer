use std::net::Ipv4Addr;

use async_trait::async_trait;

use ip_geolocation::IpScanner;
use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};

pub struct Crawler {
    db: DatabaseConnection,
    scanner: IpScanner,
}

#[async_trait]
pub trait AppCrawler {
    async fn new(database_path: &str, shodan_key: &str) -> Self;
    async fn geolocation(&self, ipv4: &Ipv4Addr) -> (f64, f64);
}

#[async_trait]
impl AppCrawler for Crawler {
    async fn new(database_path: &str, shodan_key: &str) -> Self {
        let scanner = IpScanner::new(shodan_key);
        let sqlite_connection_string = format!("sqlite://{}", database_path);
        let db = Database::connect(sqlite_connection_string).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();
        Self { db, scanner }
    }

    async fn geolocation(&self, ipv4: &Ipv4Addr) -> (f64, f64) {
        // TODO: Check if ip already in db and if an geolocation exists that is younger than 24h
        // --> If Exists -> take db saved location and return
        // else
        // Fetch geolocation --> Save in db and return value
        let geolocation = self.scanner.clone().ip_geolocation(&ipv4).await.unwrap();
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{AppCrawler, Crawler};

    #[tokio::test]
    async fn test_connection() {
        let path = "/home/jontze/.local/share/service_observer/data.sqlite";
        let crawler = Crawler::new(path, "").await;
    }
}
