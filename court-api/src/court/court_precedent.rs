use crate::{
    court::util::portal_request,
    error::CourtApiError,
    traits::HasUrl,
    util::{integer_date_to_naive_date, remove_bracket, replace_middle_dot},
    CaseProvider, DecisionType,
};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use url::Url;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct CourtPrecedent {
    /// 사법정보포털 일련번호
    #[serde(alias = "jisCntntsSrno")]
    #[serde(deserialize_with = "string_to_u64")]
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i64"))]
    pub id: u64,

    /// 사건명
    #[serde(alias = "csNmLstCtt", deserialize_with = "title")]
    pub case_title: String,

    #[serde(alias = "jdcpctCsAlsNm", deserialize_with = "subtitle")]
    pub case_subtitle: Option<String>,

    #[serde(alias = "csNoLstCtt")]
    pub case_code: String,

    #[serde(alias = "jdcpctPublcCtt", deserialize_with = "bulletin_code")]
    pub bulletin_code: Option<String>,

    #[serde(alias = "cortNm")]
    pub court_name: String,

    #[serde(alias = "prnjdgYmd")]
    #[serde(deserialize_with = "integer_date_to_naive_date")]
    pub decision_date: NaiveDate,

    /// 전원합의체 여부
    #[serde(alias = "jdcpctGrCd", deserialize_with = "is_enbanc")]
    pub en_banc: bool,

    #[serde(alias = "adjdTypNm")]
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "String"))]
    pub decision_type: DecisionType,
}

impl CourtPrecedent {
    pub async fn get_summary(&self) -> Result<Vec<String>, CourtApiError> {
        let path = "/pgp/pgp1011/selectJdcpctSumrInf.on";
        let body = json!({ "jisCntntsSrno": self.id });
        let response: Vec<Value> = portal_request(path, &body, "dlt_sumrInf").await?;

        let result: Vec<String> = response
            .iter()
            .map(|item| item["jdcpctSumrCtt"].as_str().unwrap().trim().to_string())
            .collect();

        Ok(result)
    }
}

impl HasUrl for CourtPrecedent {
    fn url(&self, provider: &CaseProvider) -> Url {
        match provider {
            CaseProvider::Official => {
                format!(
                    "https://portal.scourt.go.kr/pgp/main.on?w2xPath=PGP1011M04&jisCntntsSrno={}",
                    self.id
                )
            }
            CaseProvider::Casenote => {
                format!("https://casenote.kr/{}/{}", self.court_name, self.case_code)
            }
            CaseProvider::Lbox => {
                format!(
                    "https://lbox.kr/v2/case/{}/{}",
                    self.court_name, self.case_code
                )
            }
            CaseProvider::Bigcase => {
                format!(
                    "https://bigcase.ai/cases/{}/{}",
                    self.court_name, self.case_code
                )
            }
        }
        .parse()
        .unwrap()
    }
}

fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let int: u64 = s.parse().unwrap();
    Ok(int)
}

fn title<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(replace_middle_dot(&s))
}

fn subtitle<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s.as_str() == "" {
        return Ok(None);
    }

    let result = remove_bracket(&s);
    Ok(Some(result))
}

fn bulletin_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "[공보불게재]" | "" => Ok(None),
        _ => Ok(Some(remove_bracket(&s))),
    }
}

fn is_enbanc<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(s == "111")
}
