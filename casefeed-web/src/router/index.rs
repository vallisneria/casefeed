use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;

pub async fn index() -> &'static str {
    r"Casefeed [beta]

법원/헌법재판소 판례공보를 RSS로 제공하는 서비스입니다.
사용하시려면 아래 링크를 사용하시는 RSS 리더에 추가해주세요.

대법원 판례공보: https://casefeed.kr/대법원/판례공보
헌법재판소 전원재판부 판례: https://casefeed.kr/헌법재판소/전원재판부
"
}

pub async fn favicon() -> impl IntoResponse {
    let favicon_data = include_bytes!("../../static/favicon.ico");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/x-icon"));

    (StatusCode::OK, headers, favicon_data.to_vec())
}
