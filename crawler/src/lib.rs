use async_trait::async_trait;
use entity::{geolocation, ip, prelude::*};
use ip_geolocation::IpScanner;
use migration::MigratorTrait;
use sea_orm::{prelude::*, QueryOrder};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait,
    QueryFilter, QuerySelect,
};
use std::net::Ipv4Addr;

mod error;
pub use error::CrawlerError;

pub struct Crawler {
    db: DatabaseConnection,
    scanner: IpScanner,
}

#[async_trait]
pub trait AppCrawler {
    async fn new(database_path: &str, shodan_key: &str) -> Self;
    async fn geolocation(&self, ipv4: &Ipv4Addr) -> Result<(f64, f64), CrawlerError>;
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

    async fn geolocation(&self, ipv4: &Ipv4Addr) -> Result<(f64, f64), CrawlerError> {
        // Check if IP already in DB
        let saved_ip = Ip::find()
            .filter(ip::Column::Ipv4.eq(&*ipv4.to_string()))
            .one(&self.db)
            .await?;
        if let Some(db_ip) = saved_ip {
            // Get saved locations
            let geolocations = db_ip
                .find_related(geolocation::Entity)
                .order_by_desc(geolocation::Column::Id)
                .limit(1)
                .all(&self.db)
                .await?;
            // TODO: Check age of response, and refetch location of older than 24hours / 1 week / ...
            let geolocation::Model {
                id: _,
                latitude,
                longitude,
                created: _,
                ip_id: _,
            } = geolocations.get(0).unwrap();
            // Return latest saved location
            Ok((latitude.to_owned(), longitude.to_owned()))
        } else {
            // If not saved, fetch and save ip with geolocations
            let fetched_geolocation = self.scanner.ip_geolocation(ipv4).await?;
            let ip_to_save = ip::ActiveModel {
                ipv4: ActiveValue::Set(ipv4.to_string()),
                ..Default::default()
            };
            let saved_ip = ip_to_save.insert(&self.db).await?;
            let location_to_save = geolocation::ActiveModel {
                ip_id: ActiveValue::Set(saved_ip.id),
                latitude: ActiveValue::Set(fetched_geolocation.latitude),
                longitude: ActiveValue::Set(fetched_geolocation.longitude),
                ..Default::default()
            };
            let saved_location = location_to_save.insert(&self.db).await?;
            Ok((saved_location.latitude, saved_location.longitude))
        }
    }
}
