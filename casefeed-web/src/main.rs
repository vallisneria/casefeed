mod router;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(router::root))
        // /대법원/전원합의체
        .route(
            "/%EB%8C%80%EB%B2%95%EC%9B%90/%EC%A0%84%EC%9B%90%ED%95%A9%EC%9D%98%EC%B2%B4",
            get(router::court_en_banc),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
