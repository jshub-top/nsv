// use futures_util::StreamExt;
// use tokio::{fs::File, io::AsyncWriteExt};

// pub async fn download_files(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = File::create(path).await?;
//     let mut stream = reqwest::get(url).await?.bytes_stream();
//     while let Some(chunk_result) = stream.next().await {
//         let chunk = chunk_result?;
//         file.write_all(&chunk).await?;
//     }
//     file.flush().await?;
//     println!("Downloaded {}", url);
//     Ok(())
// }
