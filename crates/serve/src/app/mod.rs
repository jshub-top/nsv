use std::sync::Arc;

use axum::{serve::Serve, Router};
use tokio::net::TcpListener;

use crate::{config::Config, db::Db, router};

pub struct App {
    pub serve: Serve<Router, Router>,
    pub config: Arc<Config>,
}

impl App {
    pub async fn build(config: Arc<Config>) -> App {
        let db = Db::build(config.clone()).await;

        let share_state = ShareState {
            config: config.clone(),
            db,
        };
        let share_state = Arc::new(share_state);
        let router = router::create_router(share_state);

        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
            .await
            .unwrap();
        let serve = axum::serve(listener, router);

        let app = App { serve, config };

        return app;
    }
}

pub struct ShareState {
    pub config: Arc<Config>,
    pub db: Db,
}
