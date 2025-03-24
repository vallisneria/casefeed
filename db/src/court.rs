use crate::Error;
use chrono::{DateTime, NaiveDate, Utc};
use court_api::CourtPrecedent;
use sqlx::postgres::PgPool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CourtPrecedentDB {
    pub collected_time: DateTime<Utc>,
    pub case_title: String,
    pub case_subtitle: Option<String>,
    pub case_code: String,
    pub court_name: String,
    pub case_type: String,
    pub decision_date: NaiveDate,
    pub en_banc: bool,
    pub decision_type: String,
    pub summary: Vec<String>,
    pub bulletin_code: String,
}

pub async fn insert(pool: &PgPool, prec: &CourtPrecedent) -> Result<(), Error> {
    let query = "INSERT INTO court_case (case_code, court_name, case_title, case_subtitle, decision_date, en_banc, decision_type, summary)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8);";

    sqlx::query(query)
        .bind(&prec.case_code)
        .bind(&prec.court_name)
        .bind(&prec.case_title)
        .bind(&prec.case_subtitle)
        .bind(&prec.decision_date)
        .bind(&prec.en_bank)
        .bind(prec.decision_type.to_string())
        .bind(prec.get_summary().await?)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn insert_bulletin(pool: &PgPool, prec: &CourtPrecedent) -> Result<(), Error> {
    let query = "INSERT INTO court_bulletin (court_name, case_code, bulletin_code)
        VALUES ($1, $2, $3);";

    sqlx::query(query)
        .bind(&prec.court_name)
        .bind(&prec.case_code)
        .bind(&prec.bulletin_code)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn select(pool: &PgPool, limit: i32) -> Result<Vec<CourtPrecedentDB>, Error> {
    let query = "SELECT a.* FROM court_case AS a \
        INNER JOIN court_bulletin as b \
        ON a.court_name = b.court_name AND a.case_code = b.case_code \
        LIMIT $1";

    let db_response = sqlx::query_as::<_, CourtPrecedentDB>(query)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    Ok(db_response)
}
