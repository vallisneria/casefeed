use crate::rss::traits::CourtPrecedentDBWrapper;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use court_api::CaseProvider;
use rss::{ChannelBuilder, Item};
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize)]
pub struct QueryParam {
    length: Option<u8>,

    #[serde(default)]
    case_provider: CaseProvider,
}

pub async fn court_bulletin(
    Query(param): Query<QueryParam>,
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, &'static str)> {
    let length: i32 = param.length.unwrap_or(40) as i32;
    let db_response = db::court::select(&pool, length).await;
    let result = db_response
        .map_err(|_err| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?
        .into_iter()
        .map(|prec_db| CourtPrecedentDBWrapper {
            body: prec_db,
            case_provider: param.case_provider,
        })
        .collect::<Vec<CourtPrecedentDBWrapper>>();

    if result.len() == 0 {
        return Err((StatusCode::NOT_FOUND, "Not Found"));
    }

    let rss = ChannelBuilder::default()
        .title("대법원 판례공보")
        .language(Some("ko-kr".to_string()))
        .items(
            result
                .into_iter()
                .map(|prec| prec.into())
                .collect::<Vec<Item>>(),
        )
        .build();

    Ok(rss.to_string())
}
