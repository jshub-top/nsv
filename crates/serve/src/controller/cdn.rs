use crate::{app::ShareState, util::file::download_file};
use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderValue};
use std::{env, sync::Arc};
use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;

pub async fn cache_by_url(
    State(state): State<Arc<ShareState>>,
    request: Request,
) -> (HeaderMap, Body) {
    let uri = request.uri();
    let mut current_exe_path = env::current_exe().unwrap();
    current_exe_path.pop();
    uri.path().split("/").for_each(|x| current_exe_path.push(x));

    println!("file not exist: {}", current_exe_path.display());
    if fs::metadata(&current_exe_path).await.is_err() {
        println!("file not exist: {}", current_exe_path.display());
        let url = format!("{}{}", state.config.base_url, uri.path());
        download_file(&url, &current_exe_path).await.unwrap();
    }

    let file = File::open(&current_exe_path).await.unwrap();
    let file_meta = file.try_clone().await.unwrap().metadata().await.unwrap();
    let stream = ReaderStream::new(file.try_clone().await.unwrap());
    let mut header_map = HeaderMap::new();
    let name = current_exe_path.file_name().unwrap().to_str().unwrap();
    header_map.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/x-7z-compressed").unwrap(),
    );
    header_map.insert(header::CONTENT_LENGTH, HeaderValue::from(file_meta.len()));
    header_map.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename={}", name)).unwrap(),
    );
    (header_map, Body::from_stream(stream))
}
