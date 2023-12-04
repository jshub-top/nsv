use std::path::Path;

use tokio::fs::create_dir_all;

pub async fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    create_dir_all(path).await
}
