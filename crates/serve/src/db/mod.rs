use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;



pub struct Db {
    pub db: DatabaseConnection
}

impl Db {
    pub async fn build(config: Arc<Config>) -> Self{

        let db_config = ConnectOptions::new(&config.db_url);


        let db = Database::connect(db_config).await.expect("create db error");

        Self {
            db
        }
    }
}
