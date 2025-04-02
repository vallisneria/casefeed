use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;

pub async fn index() -> &'static str {
    include_str!("../../static/index.html")
}

pub async fn favicon() -> impl IntoResponse {
    let favicon_data = include_bytes!("../../static/favicon.ico");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/x-icon"));

    (StatusCode::OK, headers, favicon_data.to_vec())
}
