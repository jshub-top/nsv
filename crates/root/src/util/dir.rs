use std::{io, path::Path};

use tokio::fs::{create_dir_all, read_dir, remove_dir_all, remove_file};

pub async fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    create_dir_all(path).await
}

#[cfg(windows)]
pub async fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    use tokio::fs::remove_dir;

    remove_dir(path).await?;
    Ok(())
}

#[cfg(unix)]
pub async fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    remove_file(path).await?;
    Ok(())
}

pub async fn remove_dir_sub(dir_path: &Path) -> io::Result<()> {
    // 遍历目录的所有条目
    let mut dir_entries = read_dir(dir_path).await?;

    while let Some(entry) = dir_entries.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            // 如果是目录，递归删除整个目录及其内容
            remove_dir_all(&path).await?;
        } else {
            // 如果是文件，直接删除
            remove_file(&path).await?;
        }
    }

    Ok(())
}
