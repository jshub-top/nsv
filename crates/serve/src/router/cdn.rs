use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{app::ShareState, controller::cdn::cache_by_url};

pub fn router(state: Arc<ShareState>) -> Router {
    Router::new().route("/*all", get(cache_by_url)).with_state(state)
}

