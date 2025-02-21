mod db;

use court_api::{BenchType, ConstitutionalPrecedentSearchParam};
use sqlx::postgres::PgPoolOptions;
use std::error::Error as StdErr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdErr>> {
    let pool = {
        let username = std::env::var("DB_USER").expect("`DB_USER` 환경변수가 없음.");
        let password = std::env::var("DB_PASSWORD").expect("`DB_PASSWORD` 환경 변수가 없음");
        let host = std::env::var("DB_HOST").expect("`DB_HOST` 환경 변수가 없음");
        let port: u32 = std::env::var("DB_PORT")
            .map(|port| {
                port.parse()
                    .expect("`DB_PORT` 환경 변수를 숫자로 변환할 수 없음")
            })
            .unwrap_or(5432);
        let database_url = format!("postgresql://{username}:{password}@{host}:{port}");

        PgPoolOptions::new().connect(&database_url).await.unwrap()
    };

    // 헌재 전원합의체 판결 검색
    let en_banc = ConstitutionalPrecedentSearchParam::default()
        .set_bench_type(vec![BenchType::EnBancBench])
        .set_exclusion_keyword(vec!["불기소 처분", "기소유예처분", "국선대리인"])
        .set_page(1)
        .set_size(40)
        .search()
        .await;

    if let Ok(data) = en_banc {
        // 법원 전원합의체 판결을 데이터베이스에 입력
        for i in data.iter() {
            db::insert_ccourt_case(&pool, i).await?
        }
    }

    Ok(())
}
