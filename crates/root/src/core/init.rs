use crate::util::dir::ensure_dir;

use super::NsvCore;
use async_trait::async_trait;
use futures;

#[async_trait]
pub trait Init {
    async fn init(&self);
}

#[async_trait]
impl Init for NsvCore {
    async fn init(&self) {
        futures::future::join_all([
            ensure_dir(self.context.node_dir.as_path()),
            ensure_dir(self.context.node_file.as_path()),
            ensure_dir(self.context.temp.as_path()),
        ])
        .await;
    }
}
