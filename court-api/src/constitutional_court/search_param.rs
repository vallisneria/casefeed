use crate::{CourtApiError, USER_AGENT};

use super::{
    BenchType, ConstitutionCaseType, ConstitutionDecisionType, ConstitutionalPrecedent, RecordType,
    SortType,
};
use chrono::{Datelike, NaiveDate};
use reqwest::Client;
use serde::{ser::Serializer, Serialize};
use serde_json::{from_str, from_value, Value};

#[derive(Debug, Clone, Serialize)]
pub struct ConstitutionalPrecedentSearchParam {
    #[serde(rename(serialize = "idx"))]
    _idx: String,

    keyword: String,

    #[serde(rename(serialize = "dateFrom"))]
    #[serde(serialize_with = "serialize_date_to_integer_date")]
    from: Option<NaiveDate>,

    #[serde(rename(serialize = "dateTo"))]
    #[serde(serialize_with = "serialize_date_to_integer_date")]
    to: Option<NaiveDate>,

    #[serde(rename(serialize = "offset"))]
    page: u8,

    #[serde(rename(serialize = "limit"))]
    size: u8,

    sort: SortType,

    #[serde(rename(serialize = "eventNo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    case_code: Option<String>,

    #[serde(rename(serialize = "eventName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    case_title: Option<String>,

    #[serde(rename(serialize = "eventNobCode"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_case_type")]
    case_type: Vec<ConstitutionCaseType>,

    #[serde(rename(serialize = "endRstCode"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_decision_type")]
    decision_type: Vec<ConstitutionDecisionType>,

    #[serde(rename(serialize = "justiceDepartCode"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_bench_type")]
    bench_type: Vec<BenchType>,

    #[serde(rename(serialize = "lev"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_record_type")]
    record_type: Vec<RecordType>,

    #[serde(rename(serialize = "exclustionKeyword"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serialize_exclusion_keyword")]
    exclusion_keyword: Vec<String>,

    #[serde(rename(serialize = "reSearch"))]
    _research: String,
}

impl Default for ConstitutionalPrecedentSearchParam {
    fn default() -> Self {
        Self {
            _idx: String::from("00"),
            keyword: String::new(),
            from: NaiveDate::from_ymd_opt(1988, 9, 1),
            to: NaiveDate::from_ymd_opt(9999, 12, 31),
            page: 1,
            size: 30,
            sort: SortType::DateDesc,
            case_code: None,
            case_title: None,
            case_type: Vec::new(),
            decision_type: Vec::new(),
            bench_type: Vec::new(),
            record_type: Vec::new(),
            exclusion_keyword: Vec::new(),
            _research: String::from(r#"{"flag" : "false", "reKeyword" : ""}"#),
        }
    }
}

fn serialize_date_to_integer_date<S>(value: &Option<NaiveDate>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let result = if let Some(date) = value {
        let year = date.year();
        let month = date.month0() + 1;
        let day = date.day0() + 1;

        format!("{year:0>4}{month:0>2}{day:0>2}")
    } else {
        String::new()
    };

    s.serialize_str(&result)
}

fn serialize_case_type<S>(value: &Vec<ConstitutionCaseType>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value: Vec<&str> = value
        .iter()
        .map(|item| match item {
            ConstitutionCaseType::ConstitutionalStatutes => "1",
            ConstitutionCaseType::Impeachment => "2",
            ConstitutionCaseType::DissolutionParty => "3",
            ConstitutionCaseType::CompetenceDispute => "4",
            ConstitutionCaseType::ConstitutionalComplaintsType1 => "5",
            ConstitutionCaseType::ConstitutionalComplaintsType2 => "6",
            ConstitutionCaseType::Application => "7",
            ConstitutionCaseType::Special => "8",
        })
        .collect();

    let result = value.join(",");
    s.serialize_str(&result)
}

fn serialize_decision_type<S>(
    value: &Vec<ConstitutionDecisionType>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value: Vec<&str> = value
        .iter()
        .map(|item| match item {
            ConstitutionDecisionType::Unconstitutional => "67",
            ConstitutionDecisionType::Uncomformable => "68",
            ConstitutionDecisionType::ConditionallyUnconstitutional => "69",
            ConstitutionDecisionType::ConditionallyConstitutional => "70",
            ConstitutionDecisionType::Upholding => "71",
            ConstitutionDecisionType::Constitutional => "72",
            ConstitutionDecisionType::Rejected => "73",
            ConstitutionDecisionType::Dismissed => "74",
            ConstitutionDecisionType::Withdrawn => "76",
            ConstitutionDecisionType::Appointed => "85",
            ConstitutionDecisionType::Other => "99",
        })
        .collect();

    let result = value.join(",");
    s.serialize_str(&result)
}

fn serialize_bench_type<S>(value: &Vec<BenchType>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value: Vec<&str> = value
        .iter()
        .map(|item| match item {
            BenchType::Panel => "2",
            BenchType::EnBancBench => "1",
        })
        .collect();

    let result = value.join(",");
    s.serialize_str(&result)
}

fn serialize_record_type<S>(value: &Vec<RecordType>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value: Vec<&str> = value
        .iter()
        .map(|item| match item {
            RecordType::DecisionDocument => "1",
            RecordType::Bulletin => "2",
            RecordType::Casebook => "3",
        })
        .collect();

    let result = value.join(",");
    s.serialize_str(&result)
}

fn serialize_exclusion_keyword<S>(value: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let result = value.join(",");
    s.serialize_str(&result)
}

// Builder
impl ConstitutionalPrecedentSearchParam {
    pub fn set_keyword<V: Into<String>>(mut self, value: V) -> Self {
        self.keyword = value.into();
        self
    }

    pub fn set_from_ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        self.from = NaiveDate::from_ymd_opt(year, month, day);
        self
    }

    pub fn set_from(mut self, value: Option<NaiveDate>) -> Self {
        self.from = value;
        self
    }

    pub fn set_to_ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        self.to = NaiveDate::from_ymd_opt(year, month, day);
        self
    }

    pub fn set_to(mut self, value: Option<NaiveDate>) -> Self {
        self.to = value;
        self
    }

    pub fn set_page(mut self, value: u8) -> Self {
        self.page = value;
        self
    }

    pub fn set_size(mut self, value: u8) -> Self {
        self.size = value;
        self
    }

    pub fn set_sort(mut self, value: SortType) -> Self {
        self.sort = value;
        self
    }

    pub fn set_case_code<V: Into<String>>(mut self, value: Option<V>) -> Self {
        self.case_code = value.map(|x| x.into());
        self
    }

    pub fn set_case_title<V: Into<String>>(mut self, value: Option<V>) -> Self {
        self.case_title = value.map(|x| x.into());
        self
    }

    pub fn set_case_type(mut self, value: Vec<ConstitutionCaseType>) -> Self {
        self.case_type = value;
        self
    }

    pub fn set_decision_type(mut self, value: Vec<ConstitutionDecisionType>) -> Self {
        self.decision_type = value;
        self
    }

    pub fn set_bench_type(mut self, value: Vec<BenchType>) -> Self {
        self.bench_type = value;
        self
    }

    pub fn set_record_type(mut self, value: Vec<RecordType>) -> Self {
        self.record_type = value;
        self
    }

    pub fn set_exclusion_keyword<V: Into<String>>(mut self, value: Vec<V>) -> Self {
        self.exclusion_keyword = value.into_iter().map(|x| x.into()).collect();
        self
    }
}

impl ConstitutionalPrecedentSearchParam {
    pub async fn search(&self) -> Result<Vec<ConstitutionalPrecedent>, CourtApiError> {
        const URL: &'static str = "https://isearch.ccourt.go.kr/api/index/searcher/categorySearch";
        let request = Client::new()
            .post(URL)
            .form(self)
            .header("User-Agent", USER_AGENT);

        let response = request.send().await?;
        let response_text = response.text().await?;
        let response_json: Value = from_str(&response_text)?;
        let result = from_value(response_json["returnObject"]["resultList"].to_owned())?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn search_test() {
        let search = ConstitutionalPrecedentSearchParam::default()
            .set_bench_type(vec![BenchType::EnBancBench])
            .set_exclusion_keyword(vec!["불기소 처분", "기소유예처분", "국선대리인"])
            .search()
            .await
            .unwrap();

        println!("{search:#?}")
    }
}
