use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    let html = include_str!("../../static/index.html");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));

    (StatusCode::OK, headers, html)
}

pub async fn favicon() -> impl IntoResponse {
    let favicon_data = include_bytes!("../../static/favicon.ico");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/x-icon"));

    (StatusCode::OK, headers, favicon_data.to_vec())
}
