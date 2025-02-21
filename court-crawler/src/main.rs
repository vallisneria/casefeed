mod db;

use court_api::{CourtPrecedentSearchParam, PrecedentGrade};
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

    // 법원 간행판결 검색
    let published = CourtPrecedentSearchParam::default()
        .set_precedent_grade(Some(PrecedentGrade::Published))
        .set_page(1)
        .set_size(40)
        .search()
        .await;

    // 법원 전원합의체 판결 검색
    let en_banc = CourtPrecedentSearchParam::default()
        .set_precedent_grade(Some(PrecedentGrade::EnBanc))
        .set_page(1)
        .set_size(40)
        .search()
        .await;

    if let Ok(data) = published {
        // 법원 판결을 데이터베이스에 입력
        for i in data.iter() {
            db::insert_court_case(&pool, i).await?
        }

        // 이 중 판례공보 번호가 있는 판례를 분류해서
        // 판례공보 데이터베이스에 입력
        for i in data.iter().filter(|item| item.bulletin_code.is_some()) {
            db::insert_court_bulletin(&pool, i).await?
        }
    }

    if let Ok(data) = en_banc {
        // 법원 전원합의체 판결을 데이터베이스에 입력
        for i in data.iter() {
            db::insert_court_case(&pool, i).await?
        }

        // 이 중 판례공보 번호가 있는 판례를 분류해서
        // 판례공보 데이터베이스에 입력
        for i in data.iter().filter(|item| item.bulletin_code.is_some()) {
            db::insert_court_bulletin(&pool, i).await?
        }
    }

    Ok(())
}
