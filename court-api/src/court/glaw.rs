use super::CourtCase;
use crate::util::replace_middle_dot;
use reqwest::Client;
use serde::Serialize;
use serde_json::{from_str, Value};
use std::error::Error as StdErr;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlawSearchParam {
    q: &'static str,
    w: &'static str,
    section: &'static str,
    sub_id: &'static str,
    out_max: u8,
    sys_cd: &'static str,
    hanja_yn: &'static str,
    msort: &'static str,
}

pub async fn get_all_case_list(param: GlawSearchParam) -> Result<Vec<CourtCase>, Box<dyn StdErr>> {
    let request = Client::new()
        .post("https://glaw.scourt.go.kr/wsjo/panre/sjo050/panreList.do")
        .form(&param)
        .header("User-Agent", "Mozilla/5.0")
        .header("Accept", "application/json");

    let response = request.send().await?;
    let response_text = response.text().await?;
    let result: Vec<CourtCase> = from_str(&response_text)?;

    Ok(result)
}

pub(super) async fn get_main_issue(case_id: u32) -> Result<Vec<String>, Box<dyn StdErr>> {
    let request = Client::new()
        .post("https://glaw.scourt.go.kr/wsjo/panre/sjo080p10/selPanreYoyakDesc.ajax")
        .form(&[("panreYoyakGbnCd", "01"), ("contId", &format!("{case_id}"))])
        .header("User-Agent", "Mozilla/5.0")
        .header("Accept", "application/json");
    let response = request.send().await?;
    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;

    let result = response_json["selPanreYoyakDesc"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|item| {
            let item = item["panreYoyakDesc"].as_str().unwrap().trim().to_string();
            replace_middle_dot(&item)
        })
        .collect();

    Ok(result)
}

pub(super) async fn get_summary(case_id: u32) -> Result<Vec<String>, Box<dyn StdErr>> {
    let request = Client::new()
        .post("https://glaw.scourt.go.kr/wsjo/panre/sjo080p10/selPanreYoyakDesc.ajax")
        .form(&[("panreYoyakGbnCd", "02"), ("contId", &format!("{case_id}"))])
        .header("User-Agent", "Mozilla/5.0")
        .header("Accept", "application/json");
    let response = request.send().await?;
    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;

    let result = response_json["selPanreYoyakDesc"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|item| {
            let item = item["panreYoyakDesc"].as_str().unwrap().trim().to_string();
            replace_middle_dot(&item)
        })
        .collect();

    Ok(result)
}
