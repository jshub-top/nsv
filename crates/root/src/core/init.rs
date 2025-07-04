use crate::util::dir::ensure_dir;

use super::NsvCore;
use anyhow::Result;
use async_trait::async_trait;
use futures;
use std::io;
use std::pin::Pin;
use std::future::Future;
use tokio::fs::read_dir;

#[async_trait]
pub trait Init {
    async fn init(&mut self) -> Result<()>;

    async fn set_local_version(&mut self) -> Result<()>;
}

#[async_trait]
impl Init for NsvCore {
    async fn init(&mut self) -> Result<()> {
        // 先复制需要的路径，避免同时借用self
        let node_dir = &self.context.node_dir;
        let node_file = &self.context.node_file;
        let temp_dir = &self.context.temp;

        // 创建目录的Future
        let dir_futures: Vec<Pin<Box<dyn Future<Output = io::Result<()>> + Send>>> = vec![
            Box::pin(ensure_dir(&node_dir)),
            Box::pin(ensure_dir(&node_file)),
            Box::pin(ensure_dir(&temp_dir)),
        ];
        let _ = futures::future::join_all(dir_futures).await;

        // 初始化之后运行
        let dir_futures: Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>> = vec![
            Box::pin(self.set_local_version()),
        ];
        futures::future::join_all(dir_futures).await?;

    }

    async fn set_local_version(&mut self) -> io::Result<()> {
        let local_version = read_dir(self.context.node_dir.as_path()).await?;
        Ok(())
    }
}
