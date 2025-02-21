use court_api::CourtPrecedent;
use serde_json::to_value;
use sqlx::postgres::PgPool;
use std::error::Error as StdErr;

pub async fn insert_court_case(
    pool: &PgPool,
    prec: &CourtPrecedent,
) -> Result<(), Box<dyn StdErr>> {
    let sql = sqlx::query("INSERT INTO court_case (case_code, court_name, case_title, case_subtitle, decision_date, en_banc, decision_type, summary)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
    .bind(&prec.case_code)
    .bind(&prec.court_name)
    .bind(&prec.case_title)
    .bind(&prec.case_subtitle)
    .bind(&prec.decision_date)
    .bind(&prec.en_bank)
    .bind(prec.decision_type.to_string())
    .bind(to_value(prec.get_summary().await?)?)
    .execute(pool)
    .await;

    match sql {
        Ok(_) => println!("{} {} 입력 성공", prec.court_name, prec.case_code),
        Err(e) => println!("{} {} 입력 실패: {e}", prec.court_name, prec.case_code),
    }

    Ok(())
}
pub async fn insert_court_bulletin(
    pool: &PgPool,
    prec: &CourtPrecedent,
) -> Result<(), Box<dyn StdErr>> {
    let sql = sqlx::query(
        "INSERT INTO court_bulletin (court_name, case_code, bulletin_code) VALUES ($1, $2, $3)",
    )
    .bind(&prec.court_name)
    .bind(&prec.case_code)
    .bind(&prec.bulletin_code)
    .execute(pool)
    .await;

    match sql {
        Ok(_) => println!("[{:?}] 입력 성공", prec.bulletin_code),
        Err(e) => println!("[{:?}] 입력 실패: {e}", prec.bulletin_code),
    }

    Ok(())
}
