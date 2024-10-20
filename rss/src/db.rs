use crate::rss::RssItem;
use crate::util;
use chrono::DateTime;
use court_api::{CourtCaseType, CourtType, DecisionType};
use serde::{Deserialize, Deserializer};
use std::error::Error as StdErr;
use worker::d1::{D1Database, D1Result};
use worker::query;

pub async fn get_scourt_bulletin_list(
    db: &D1Database,
    length: u8,
) -> Result<Vec<CasefeedDatabaseResponse>, Box<dyn StdErr>> {
    let q = "SELECT a.collected_time, a.bulletin_code, b.case_title, b.case_subtitle, b.glaw_id,
        b.case_code, b.court_name, b.case_type, b.decision_date, b.en_bank, b.decision_type,
        c.main_issue, c.summary_of_decision
    FROM bulletin_case as a
    INNER JOIN court_case as b
    ON (a.case_code = b.case_code) AND (a.court_name = b.court_name)
    INNER JOIN case_summary as c
    ON (a.case_code = c.case_code) AND (a.court_name = c.court_name)
    WHERE a.court_name = '대법원'
    ORDER BY a.collected_time DESC
    LIMIT ?1";

    let result = query!(&db, q, length)?
        .run()
        .await?
        .results::<CasefeedDatabaseResponse>()?;

    Ok(result)
}

pub async fn get_scourt_bulletin_list_since(
    db: &D1Database,
    since: &String,
) -> Result<Vec<CasefeedDatabaseResponse>, Box<dyn StdErr>> {
    let since = DateTime::parse_from_rfc2822(&since)?.timestamp();
    let q = "SELECT a.collected_time, a.bulletin_code, b.case_title, b.case_subtitle, b.glaw_id,
        b.case_code, b.court_name, b.case_type, b.decision_date, b.en_bank, b.decision_type,
        c.main_issue, c.summary_of_decision
    FROM bulletin_case as a
    INNER JOIN court_case as b
    ON (a.case_code = b.case_code) AND (a.court_name = b.court_name)
    INNER JOIN case_summary as c
    ON (a.case_code = c.case_code) AND (a.court_name = c.court_name)
    WHERE (a.court_name = '대법원') AND (a.collected_time > ?1)
    ORDER BY a.collected_time DESC";

    let result = query!(&db, q, since)?
        .run()
        .await?
        .results::<CasefeedDatabaseResponse>()?;

    Ok(result)
}

#[derive(Debug, Deserialize)]
pub struct CasefeedDatabaseResponse {
    collected_time: i64,

    court_name: CourtType,

    case_code: String,

    bulletin_code: String,

    glaw_id: u32,

    case_title: String,

    case_subtitle: Option<String>,

    case_type: CourtCaseType,

    decision_date: String,

    #[serde(deserialize_with = "sqlite_bool")]
    en_bank: bool,

    decision_type: DecisionType,

    #[serde(deserialize_with = "string_to_vec")]
    main_issue: Vec<String>,

    #[serde(deserialize_with = "string_to_vec")]
    summary_of_decision: Vec<String>,
}

fn sqlite_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u8 = Deserialize::deserialize(deserializer)?;
    Ok(s == 1)
}

fn string_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(serde_json::from_str::<Vec<String>>(&s).unwrap())
}

impl CasefeedDatabaseResponse {
    fn long_code(&self) -> String {
        let ja = match &self.decision_type {
            DecisionType::Judgement => "선고",
            _ => "자",
        };

        let en_bank = if self.en_bank { "전원합의체 " } else { "" };

        format!(
            "{court_name} {decision_date} {ja} {case_code} {en_bank}{decision_type}",
            court_name = self.court_name,
            decision_date = util::iso8601_date_to_dot_date(&self.decision_date),
            case_code = self.case_code,
            decision_type = self.decision_type
        )
    }
}

impl RssItem for CasefeedDatabaseResponse {
    fn get_title(&self) -> String {
        let subtitle = match self.case_subtitle.as_ref() {
            Some(sub) => format!("〈{}〉 ", sub),
            None => String::new(),
        };

        format!(
            "{title} {subtitle}({case_code}) [{bulletin_code}]",
            title = self.case_title,
            case_code = self.long_code(),
            bulletin_code = self.bulletin_code
        )
    }

    fn get_author(&self) -> String {
        self.court_name.to_string()
    }

    fn get_category(&self) -> String {
        self.case_type.to_string()
    }

    fn get_pubdate(&self) -> String {
        DateTime::from_timestamp(self.collected_time, 0)
            .unwrap()
            .to_rfc2822()
    }

    fn get_description(&self) -> String {
        let main_issue = self
            .main_issue
            .to_vec()
            .into_iter()
            .map(|paragraph| format!("<p>{paragraph}</p>"))
            .collect::<Vec<String>>()
            .join("");

        let summary = self
            .summary_of_decision
            .to_vec()
            .into_iter()
            .map(|paragraph| format!("<p>{paragraph}</p>"))
            .collect::<Vec<String>>()
            .join("");

        format!("<h2>판시사항</h2>{main_issue}<h2>판결요지</h2>{summary}")
    }

    fn get_link(&self) -> String {
        format!(
            "https://glaw.scourt.go.kr/wsjo/panre/sjo100.do?contId={}",
            self.glaw_id
        )
    }

    fn get_guid(&self) -> String {
        self.get_link()
    }
}
