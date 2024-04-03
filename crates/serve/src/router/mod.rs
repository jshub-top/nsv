pub mod cdn;
use axum::Router;
use std::sync::Arc;

use crate::app::ShareState;

pub fn create_router(share_state: Arc<ShareState>) -> Router {
    Router::new().nest("/cdn", cdn::router(share_state))
}
