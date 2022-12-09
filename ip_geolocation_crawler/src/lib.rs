use async_trait::async_trait;

use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};

pub struct Crawler {
    db: DatabaseConnection,
}

#[async_trait]
pub trait AppCrawler {
    async fn new(database_path: &str) -> Self;
}

#[async_trait]
impl AppCrawler for Crawler {
    async fn new(database_path: &str) -> Self {
        let sqlite_connection_string = format!("sqlite://{}", database_path);
        let db = Database::connect(sqlite_connection_string).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();
        Self { db }
    }
}
