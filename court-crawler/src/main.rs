use court_api::{CourtPrecedentSearchParam, CourtType};
use db;
use std::error::Error as StdErr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdErr>> {
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
        let name = std::env::var("DB_NAME").expect("`DB_NAME` 환경변수가 없음.");

        format!("postgresql://{username}:{password}@{host}:{port}/{name}")
    };
    let pool = db::db_init(&db_url).await?;

    // 법원 판결 검색
    let precs = CourtPrecedentSearchParam::default()
        .set_page(1)
        .set_size(100)
        // 헌법재판소를 제외하고 검색
        .set_court_type(vec![
            CourtType::SupremeCourt,
            CourtType::HighCourt,
            CourtType::LowerCourt,
        ])
        .search()
        .await;

    if let Ok(data) = precs {
        // 법원 판결을 데이터베이스에 입력
        for i in data.iter() {
            print!("{} {}", i.court_name, i.case_code);

            match db::court::insert(&pool, i).await {
                Ok(_) => println!(" 입력 성공"),
                Err(err) => println!(" 입력 실패: {err}"),
            }
        }

        // 이 중 판례공보 번호가 있는 판례를 분류해서
        // 판례공보 데이터베이스에 입력
        for i in data.iter().filter(|item| item.bulletin_code.is_some()) {
            print!("{:?} ", i.bulletin_code);

            match db::court::insert_bulletin(&pool, i).await {
                Ok(_) => println!("입력 성공"),
                Err(err) => println!("입력 실패: {err}"),
            }
        }
    }

    Ok(())
}
