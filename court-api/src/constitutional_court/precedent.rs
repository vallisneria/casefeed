use super::RecordType;
use crate::{
    traits::HasUrl,
    util::{integer_date_to_naive_date, replace_middle_dot},
    CaseProvider,
};
use chrono::NaiveDate;
use serde::{de::Deserializer, Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[serde(from = "ConstitutionalPrecedentHelper")]
pub struct ConstitutionalPrecedent {
    #[serde(alias = "docId")]
    #[serde(deserialize_with = "get_id")]
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "i64"))]
    pub id: u64,

    #[serde(alias = "eventName")]
    #[serde(deserialize_with = "case_title")]
    pub case_title: String,

    #[serde(default)]
    #[serde(alias = "eventNickname")]
    #[serde(deserialize_with = "case_subtitle")]
    pub case_subtitle: Option<String>,

    #[serde(alias = "eventNo")]
    pub case_code: String,

    #[serde(alias = "date")]
    #[serde(deserialize_with = "integer_date_to_naive_date")]
    pub decision_date: NaiveDate,

    #[serde(alias = "justiceDepart")]
    #[serde(deserialize_with = "is_enbanc")]
    pub en_banc: bool,

    #[serde(alias = "name")]
    #[cfg_attr(feature = "sqlx", sqlx(try_from = "String"))]
    pub record_type: RecordType,

    pub bulletin_code: Option<String>,

    #[serde(alias = "judgementNote")]
    #[serde(deserialize_with = "judgement_note")]
    #[serde(default)]
    pub judgement_note: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalPrecedentHelper {
    #[serde(alias = "docId")]
    #[serde(deserialize_with = "get_id")]
    pub id: u64,

    #[serde(alias = "eventName")]
    #[serde(deserialize_with = "case_title")]
    pub case_title: String,

    #[serde(default)]
    #[serde(alias = "eventNickname")]
    #[serde(deserialize_with = "case_subtitle")]
    pub case_subtitle: Option<String>,

    #[serde(alias = "eventNo")]
    pub case_code: String,

    #[serde(alias = "date")]
    #[serde(deserialize_with = "integer_date_to_naive_date")]
    pub decision_date: NaiveDate,

    #[serde(alias = "justiceDepart")]
    #[serde(deserialize_with = "is_enbanc")]
    pub en_banc: bool,

    #[serde(alias = "name")]
    pub record_type: RecordType,

    pub pages: Option<String>,

    pub volume: Option<String>,

    #[serde(alias = "judgementNote")]
    #[serde(deserialize_with = "judgement_note")]
    #[serde(default)]
    pub judgement_note: Option<Vec<String>>,
}

impl From<ConstitutionalPrecedentHelper> for ConstitutionalPrecedent {
    fn from(value: ConstitutionalPrecedentHelper) -> Self {
        let bulletin_code = match (value.volume, value.pages) {
            (Some(vol), Some(pa)) => Some(format!("헌공{},{}", vol, pa)),
            _ => None,
        };

        Self {
            id: value.id,
            case_title: value.case_title,
            case_subtitle: value.case_subtitle,
            case_code: value.case_code,
            decision_date: value.decision_date,
            en_banc: value.en_banc,
            record_type: value.record_type,
            bulletin_code,
            judgement_note: value.judgement_note,
        }
    }
}

impl HasUrl for ConstitutionalPrecedent {
    fn url(&self, provider: &CaseProvider) -> url::Url {
        let case_code = self.case_code.replace("등", "");

        match provider {
            CaseProvider::Official => unimplemented!(),
            CaseProvider::Casenote => {
                format!("https://casenote.kr/헌법재판소/{}", case_code)
            }
            CaseProvider::Lbox => format!("https://lbox.kr/v2/헌법재판소/{}", case_code),
            CaseProvider::Bigcase => {
                format!("https://bigcase.ai/cases/헌법재판소/{}", case_code)
            }
        }
        .parse()
        .unwrap()
    }
}
fn get_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let result = s.split("_").collect::<Vec<_>>()[0].parse::<u64>().unwrap();
    Ok(result)
}

fn case_title<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let result = replace_middle_dot(&s);

    Ok(result)
}

fn case_subtitle<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    let result = s.map(|x| replace_middle_dot(&x));

    Ok(result)
}

fn is_enbanc<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s == "전원재판부")
}

fn judgement_note<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    if let Some(data) = s {
        let result = replace_middle_dot(&data)
            .split("\u{A0}")
            .map(|x| x.trim().to_string())
            .filter(|x| x.len() != 0)
            .collect();

        Ok(Some(result))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use crate::{BenchType, ConstitutionalPrecedentSearchParam};

    #[tokio::test]
    async fn test() {
        ConstitutionalPrecedentSearchParam::default()
            .set_bench_type(vec![BenchType::EnBancBench])
            .set_exclusion_keyword(vec!["불기소 처분", "기소유예처분", "국선대리인"])
            .set_page(1)
            .set_size(40)
            .search()
            .await
            .unwrap();
    }
}
