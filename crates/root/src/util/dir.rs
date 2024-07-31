use std::path::Path;

use tokio::fs::create_dir_all;

pub async fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    create_dir_all(path).await
}




#[cfg(windows)]
pub async fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    tokio::fs::remove_dir(path).await?;
    Ok(())
}

#[cfg(unix)]
pub async fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    tokio::fs::remove_file(path).await?;
    Ok(())
}
