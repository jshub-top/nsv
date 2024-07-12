use std::{env, sync::Arc};
use std::os::windows::fs::MetadataExt;
use axum::{
    extract::{Request},
};
use axum::body::{Body};
use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue};
use axum::response::IntoResponse;
use hyper::Uri;
use sea_orm::JsonValue::String;
use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;
use crate::{app::ShareState, util::file::download_file};
use crate::util::file::{stream_to_file};


pub async fn cache_by_url(State(state): State<Arc<ShareState>>, request: Request) -> (HeaderMap, Body) {
    let uri = request.uri();
    let mut current_exe_path = env::current_exe().unwrap();
    current_exe_path.pop();
    uri.path().split("/").for_each(|x| current_exe_path.push(x));

    println!("file not exist: {}", current_exe_path.display());
    if fs::metadata(&current_exe_path).await.is_ok() {
        // let file = File::open(&current_exe_path).await.unwrap();
        // let meta = file.metadata().await.unwrap();
        // let create_time = meta.creation_time().unwrap();
        // if create_time.as_secs() > state.config.cache_time {
        //     stream_to_file(&current_exe_path, &state.config.base_url, uri.path()).await.unwrap();
        // }
    } else {
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
    header_map.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from(file_meta.len()),
    );
    header_map.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename={}", name)).unwrap(),
    );
    (header_map, Body::from_stream(stream))
}
