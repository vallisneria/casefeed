use super::{ConstitutionalCase, ConstitutionalCaseSearchParameter};
use reqwest;
use serde_json::{from_str, from_value, Value};
use std::error::Error as StdErr;

impl ConstitutionalCaseSearchParameter {
    pub async fn search(&self) -> Result<Vec<ConstitutionalCase>, Box<dyn StdErr>> {
        const URL: &'static str = "https://isearch.ccourt.go.kr/api/index/searcher/categorySearch";
        let client = reqwest::Client::new();
        let request = client
            .post(URL)
            .form(self)
            .header("User-Agent", "Mozilla/5.0");
        let response = request.send().await?;
        let response_text = response.text().await?;
        let response_json: Value = from_str(&response_text)?;
        let result = from_value(response_json["returnObject"]["resultList"].to_owned())?;

        Ok(result)
    }
}
