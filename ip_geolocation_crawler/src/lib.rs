use async_trait::async_trait;

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
        let db = Database::connect(database_path).await.unwrap();
        Self { db }
    }
}
