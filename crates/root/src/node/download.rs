use async_trait::async_trait;

use crate::core::NsvCore;

use super::NsvCoreError;

#[async_trait]
pub trait NodeDisposeDownload {
    async fn download_node(version: &str) -> Result<(), NsvCoreError>;
    async fn download_node(version: &str) -> Result<(), NsvCoreError>;
}

#[async_trait]
impl NodeDisposeDownload for NsvCore {
    async fn download_node(version: &str) -> Result<(), NsvCoreError> {





        Ok(())
    }


}
