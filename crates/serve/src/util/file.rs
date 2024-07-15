

use std::path::Path;


use reqwest::Client;
use tokio::{fs::{create_dir_all, File}, io::AsyncWriteExt};



pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut res = client.get(url).send().await.unwrap();

    let file_dir = path.parent().unwrap();
    create_dir_all(file_dir).await.unwrap();
    let mut file = File::create(path).await.unwrap();

    while let Some(chunk) = res.chunk().await.unwrap() {
        file.write_all(&chunk).await.unwrap();
    }
    Ok(())
}
