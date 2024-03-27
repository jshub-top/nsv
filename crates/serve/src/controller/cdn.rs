use std::sync::Arc;

use axum::extract::State;
use hyper::Uri;

use crate::app::ShareState;

pub async fn url_by_cache(State(_state): State<Arc<ShareState>>, uri: Uri) -> String {
    format!("uri is: {}", uri)
}
