mod error;
mod glaw;
mod util;

use serde::{Deserialize, Serialize};
use std::error::Error as StdErr;

#[derive(Debug, Deserialize, Serialize)]
pub struct CourtCase {
    #[serde(alias = "contId")]
    glaw_id: u32,

    #[serde(alias = "saNm", deserialize_with = "util::title")]
    case_title: String,

    #[serde(alias = "saBm", deserialize_with = "util::subtitle")]
    case_subtitle: Option<String>,

    #[serde(alias = "saNo")]
    case_code: String,

    #[serde(alias = "gjaeInfo", deserialize_with = "util::bulletin_code")]
    bulletin_code: Option<String>,

    #[serde(alias = "bubNm")]
    court_name: String,

    #[serde(alias = "lawGbnCd")]
    case_type: CaseType,

    #[serde(alias = "sngoDay", deserialize_with = "util::integer_date_to_iso8601")]
    decision_date: String,

    #[serde(alias = "panreGradeCd", deserialize_with = "util::is_enbank")]
    en_bank: bool,

    #[serde(alias = "panTypeNm")]
    decision_type: DecisionType,
}

impl CourtCase {
    pub async fn main_issue(&self) -> Result<Vec<String>, Box<dyn StdErr>> {
        glaw::get_main_issue(self.glaw_id).await
    }

    pub async fn summary_of_decision(&self) -> Result<Vec<String>, Box<dyn StdErr>> {
        glaw::get_summary(self.glaw_id).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CaseType {
    #[serde(rename = "민사", alias = "01")]
    Civil,

    #[serde(rename = "형사", alias = "02")]
    Criminal,

    #[serde(rename = "일반행정", alias = "03")]
    Adminstration,

    #[serde(rename = "조세", alias = "04")]
    Tax,

    #[serde(rename = "가사", alias = "05")]
    Family,

    #[serde(rename = "특허", alias = "06")]
    Patent,

    #[serde(rename = "기타")]
    Etc,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub enum DecisionType {
    #[default]
    #[serde(rename = "판결")]
    Judgement,

    #[serde(rename = "결정")]
    Decision,

    #[serde(rename = "명령")]
    Order,
}
