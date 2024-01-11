use futures_util::StreamExt;
use std::path::Path;
use tokio::{fs::create_dir_all, io::AsyncWriteExt};
use tokio::fs::File;

pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path).await?;
    let mut stream = reqwest::get(url).await?.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(())
}

pub async fn unzip_file(
    zip_file_dir: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(output_dir.parent().unwrap()).await.unwrap();

    #[cfg(target_os = "windows")]
    {
        sevenz_rust::decompress_file(zip_file_dir, output_dir).unwrap();
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let mut unzip_file = File::open(zip_file_dir).await.unwrap();
        let mut unzip_file_buf = Vec::new();
        unzip_file.read_to_end(&mut unzip_file_buf).await.unwrap();
        let xz = XzDecoder::new(&unzip_file_buf[..]);
        let mut archive = Archive::new(xz);
        archive.unpack(output_dir).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use super::download_file;

    #[tokio::test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    async fn test_download_file() {
        let file_url = "http://127.0.0.1:3000/dist/v20.10.0/node-v20.10.0-linux-x64.tar.xz";
        let ret = download_file(file_url, &PathBuf::from("node-v20.10.0-linux-x64.tar.xz")).await;
        assert!(matches!(ret, Ok(())));
    }
    #[tokio::test]
    #[cfg(target_os = "windows")]
    async fn test_download_file() {
        let file_url = "http://127.0.0.1:3000/dist/v20.10.0/node-v20.10.0-win-x64.7z";
        let ret = download_file(file_url, &PathBuf::from("node-v20.10.0-win-x64.7z")).await;
        assert!(matches!(ret, Ok(())));
    }
}
