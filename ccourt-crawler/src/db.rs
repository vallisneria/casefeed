use court_api::ConstitutionalPrecedent;
use sqlx::postgres::PgPool;
use std::error::Error as StdErr;

pub async fn insert_ccourt_case(
    pool: &PgPool,
    prec: &ConstitutionalPrecedent,
) -> Result<(), Box<dyn StdErr>> {
    let sql = sqlx::query("INSERT INTO constitutional_case (case_code, case_title, case_subtitle, decision_date, record_type, judgement_note)
        VALUES ($1, $2, $3, $4, $5, $6)")
    .bind(&prec.case_code)
    .bind(&prec.case_title)
    .bind(&prec.case_subtitle)
    .bind(&prec.decision_date)
    .bind(prec.record_type.to_string())
    .bind(&prec.judgement_note)
    .execute(pool)
    .await;

    match sql {
        Ok(_) => println!("헌법재판소 {} 입력 성공", prec.case_code),
        Err(e) => println!("헌법재판소 {} 입력 실패: {e}", prec.case_code),
    }

    Ok(())
}
