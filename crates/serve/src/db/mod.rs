use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};

use crate::config::Config;



pub struct Db {
    pub db: DatabaseConnection
}

impl Db {
    pub async fn build(config: Arc<Config>) -> Self{
        let db = Database::connect(&config.db_url).await.expect("create db error");
        Self {
            db
        }
    }
}
