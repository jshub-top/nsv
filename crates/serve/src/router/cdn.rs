use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{app::ShareState, controller::cdn::url_by_cache};


pub fn router(state: Arc<ShareState>) -> Router {
    Router::new().route("/*all", get(url_by_cache)).with_state(state)
}
