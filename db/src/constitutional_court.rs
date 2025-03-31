use crate::Error;
use chrono::{DateTime, Utc};
use court_api::ConstitutionalPrecedent;
use sqlx::postgres::PgPool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ConstitutionalPrecedentDB {
    pub collected_time: DateTime<Utc>,

    #[sqlx(flatten)]
    pub precedent: ConstitutionalPrecedent,
}

pub async fn insert(pool: &PgPool, prec: &ConstitutionalPrecedent) -> Result<(), Error> {
    let query = "INSERT INTO constitutional_case (case_title, case_subtitle, case_code, en_banc, record_type, decision_date, judgement_note, id) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
        ON CONFLICT (case_code) DO NOTHING";

    sqlx::query(query)
        .bind(&prec.case_title)
        .bind(&prec.case_subtitle)
        .bind(&prec.case_code)
        .bind(&prec.en_banc)
        .bind(prec.record_type.to_string())
        .bind(&prec.decision_date)
        .bind(&prec.judgement_note)
        .bind(prec.id as i64)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn insert_bulletin(pool: &PgPool, prec: &ConstitutionalPrecedent) -> Result<(), Error> {
    let query = "INSERT INTO constitutional_bulletin (case_code, bulletin_code) \
        VALUES ($1, $2) ON CONFLICT (bulletin_code) DO NOTHING";

    sqlx::query(query)
        .bind(&prec.case_code)
        .bind(&prec.bulletin_code)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn select(pool: &PgPool, limit: i32) -> Result<Vec<ConstitutionalPrecedentDB>, Error> {
    let query = "SELECT * FROM court_case LIMIT $1;";
    let db_response = sqlx::query_as(query).bind(limit).fetch_all(pool).await?;

    Ok(db_response)
}
