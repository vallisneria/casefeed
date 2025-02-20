use crate::{error::CourtApiError, USER_AGENT};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use url::Url;

pub(crate) async fn portal_request<S: Serialize, D: DeserializeOwned>(
    path: &str,
    body: &S,
    key: &str,
) -> Result<D, CourtApiError> {
    let url: Url = format!("https://portal.scourt.go.kr{}", path).parse()?;
    let body = json!({"dma_searchParam": body});
    let response = Client::new()
        .post(url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json;charset=UTF-8")
        .header("User-Agent", USER_AGENT)
        .header("Origin", "https://portal.scourt.go.kr")
        .send()
        .await?;

    let response_json: Value = response.json().await?;

    Ok(serde_json::from_value(
        response_json["data"][key].to_owned(),
    )?)
}
