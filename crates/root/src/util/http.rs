use reqwest::{Client, Error, IntoUrl, Response};

pub async fn get(url: impl IntoUrl) -> Result<Response, Error> {
    Ok(Client::new()
        .get(url)
        .header("User-Agent", concat!("nsv ", env!("CARGO_PKG_VERSION")))
        .send()
        .await?)
}
