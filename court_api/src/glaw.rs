use crate::{util, CourtCaseType, CourtType, DecisionType, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::error::Error as StdErr;

pub async fn get_all_case_list(length: u8) -> Result<Vec<PrecedentListItem>, Box<dyn StdErr>> {
    let request = Client::new()
        .post("https://glaw.scourt.go.kr/wsjo/panre/sjo050/panreList.do")
        .form(&[
            ("q", "*"),
            ("w", "panre"),
            ("section", "panre_tot"),
            ("subId", "2"),
            ("outmax", &format!("{length}")),
            ("sysCd", "WSJO"),
            ("hanjaYn", "N"),
            ("msort", "d:1:1"),
        ])
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/json");

    let response = request.send().await?;
    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;
    let result = serde_json::from_value(response_json["searchResultList"].to_owned())?;

    Ok(result)
}

pub async fn get_case_summary(case_id: u32) -> Result<PrecedentSummary, Box<dyn StdErr>> {
    let main_issue = get_main_issue(case_id).await?;
    let summary = get_summary(case_id).await?;

    Ok(PrecedentSummary {
        main_issue,
        summary,
    })
}

async fn get_main_issue(case_id: u32) -> Result<Vec<String>, Box<dyn StdErr>> {
    let request = Client::new()
        .post("https://glaw.scourt.go.kr/wsjo/panre/sjo080p10/selPanreYoyakDesc.ajax")
        .form(&[("panreYoyakGbnCd", "01"), ("contId", &format!("{case_id}"))])
        .header("User-Agent", USER_AGENT)
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
            util::replace_middle_dot(&item)
        })
        .collect();

    Ok(result)
}

async fn get_summary(case_id: u32) -> Result<Vec<String>, Box<dyn StdErr>> {
    let request = Client::new()
        .post("https://glaw.scourt.go.kr/wsjo/panre/sjo080p10/selPanreYoyakDesc.ajax")
        .form(&[("panreYoyakGbnCd", "02"), ("contId", &format!("{case_id}"))])
        .header("User-Agent", USER_AGENT)
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
            util::replace_middle_dot(&item)
        })
        .collect();

    Ok(result)
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrecedentListItem {
    /// 법원 법령정보시스템(https://glaw.scourt.go.kr) API 내 id
    #[serde(alias = "contId")]
    pub glaw_id: String,

    /// 사건 제목
    #[serde(alias = "saNm", deserialize_with = "title")]
    pub title: String,

    /// 사건 부제목
    #[serde(alias = "saBm", deserialize_with = "subtitle")]
    pub subtitle: Option<String>,

    /// 법원명
    #[serde(alias = "bubNm")]
    pub court_name: CourtType,

    /// 사건번호
    #[serde(alias = "saNo")]
    pub case_code: String,

    /// 사건종류
    #[serde(alias = "lawGbnCd")]
    pub case_type: CourtCaseType,

    /// 판결일자
    #[serde(alias = "sngoDay", deserialize_with = "integer_date_to_iso8601")]
    pub decision_date: String,

    /// (존재 할 경우) 판례공보 번호
    #[serde(alias = "gjaeInfo", deserialize_with = "bulletin_code")]
    pub bulletin_code: Option<String>,

    /// 전원합의체 여부
    #[serde(alias = "panreGradeCd", deserialize_with = "is_enbank")]
    pub en_bank: bool,

    /// 결정 종류
    #[serde(alias = "panTypeNm")]
    pub decision_type: DecisionType,
}

#[derive(Debug, Clone)]
pub struct PrecedentSummary {
    /// 판시사항
    pub main_issue: Vec<String>,

    /// 판결요지
    pub summary: Vec<String>,
}

fn title<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(util::replace_middle_dot(&s))
}

fn subtitle<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s.as_str() == "" {
        return Ok(None);
    }

    let result = util::remove_bracket(&s);
    Ok(Some(result))
}

fn bulletin_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s.as_str() == "[공보불게재]" {
        return Ok(None);
    }

    let result = util::remove_bracket(&s);
    Ok(Some(result))
}

fn integer_date_to_iso8601<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let integer_date: u32 = s.parse::<u32>().unwrap();

    let year = integer_date / 10000;
    let month = (integer_date / 100) % 100;
    let day = integer_date % 100;

    Ok(format!("{year:0>4}-{month:0>2}-{day:0>2}"))
}

fn is_enbank<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(s == "111")
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn glaw_case_list() {
        let list = super::get_all_case_list(10).await.unwrap();

        println!("{list:#?}");
    }

    #[tokio::test]
    async fn glaw_case_main_issue() {
        let main_issue = super::get_main_issue(3332073).await.unwrap();

        println!("{main_issue:#?}")
    }

    #[tokio::test]
    async fn glaw_case_summary() {
        let main_issue = super::get_case_summary(3332073).await.unwrap();

        println!("{main_issue:#?}")
    }
}
