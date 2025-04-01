use crate::rss::traits::ConstitutionalPrecedentDBWrapper;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use court_api::CaseProvider;
use rss::{ChannelBuilder, Item};
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct QueryParam {
    length: Option<u8>,

    #[serde(default)]
    case_provider: CaseProvider,
}

pub async fn ccourt_bulletin(
    Query(param): Query<QueryParam>,
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, &'static str)> {
    let length = param.length.unwrap_or(40) as i32;
    let db_resp = db::constitutional_court::select(&pool, length).await;
    let result = db_resp
        .map_err(|err| {
            println!("{err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })?
        .into_iter()
        .map(|prec| ConstitutionalPrecedentDBWrapper {
            body: prec,
            case_provider: param.case_provider,
        })
        .collect::<Vec<ConstitutionalPrecedentDBWrapper>>();

    if result.len() == 0 {
        return Err((StatusCode::NOT_FOUND, "Not Found"));
    }

    let rss = ChannelBuilder::default()
        .title("헌법재판소 판례공보")
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
