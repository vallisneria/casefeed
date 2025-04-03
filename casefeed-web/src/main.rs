mod router;
mod rss;

use axum::{routing::get, Router};
use db;

#[tokio::main]
async fn main() {
    let db_url = {
        let username = std::env::var("DB_USER").expect("`DB_USER` 환경변수가 없음.");
        let password = std::env::var("DB_PASSWORD").expect("`DB_PASSWORD` 환경 변수가 없음");
        let host = std::env::var("DB_HOST").expect("`DB_HOST` 환경 변수가 없음");
        let port: u32 = std::env::var("DB_PORT")
            .map(|port| {
                port.parse()
                    .expect("`DB_PORT` 환경 변수를 숫자로 변환할 수 없음")
            })
            .unwrap_or(5432);
        let name = std::env::var("DB_NAME").unwrap_or(String::new());
        format!("postgresql://{username}:{password}@{host}:{port}/{name}")
    };
    let pool = db::db_init(&db_url).await.unwrap();

    let app = Router::new()
        .route("/", get(router::index::index))
        .route("/favicon.ico", get(router::index::favicon))
        // /대법원/판례공보
        .route(
            "/%EB%8C%80%EB%B2%95%EC%9B%90/%ED%8C%90%EB%A1%80%EA%B3%B5%EB%B3%B4",
            get(router::scourt_bulletin::court_bulletin),
        )
        // /헌법재판소/판례공보
        .route(
            "/%ED%97%8C%EB%B2%95%EC%9E%AC%ED%8C%90%EC%86%8C/%ED%8C%90%EB%A1%80%EA%B3%B5%EB%B3%B4",
            get(router::ccourt_bulletin::ccourt_bulletin),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
