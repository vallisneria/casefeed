use super::{BenchType, ConstitutionCaseType, ConstitutionDecisionType, RecordType, Sort};
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Serialize)]
pub struct ConstitutionalCaseSearchParameter {
    // 건드리면 안되는 값
    #[serde(rename = "idx")]
    _idx: String,

    // 건드리면 안되는 값
    #[serde(rename = "reSearch")]
    _research: String,

    /// 페이지
    #[serde(rename = "offset")]
    pub(crate) page: u32,

    /// 키워드
    pub(crate) keyword: String,

    /// 종국일자 검색 시작점
    #[serde(rename = "dateFrom")]
    pub(crate) from: String,

    /// 종국일자 검색 끝점
    #[serde(rename = "dateTo")]
    pub(crate) to: String,

    /// 검색 개수
    pub(crate) limit: u8,

    /// 정렬 방식
    pub(crate) sort: Sort,

    /// 사건번호
    #[serde(rename = "eventNo", skip_serializing_if = "Option::is_none")]
    pub(crate) case_code: Option<String>,

    /// 사건명
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub(crate) case_name: Option<String>,

    /// 사건유형
    #[serde(rename = "eventNobCode", serialize_with = "serialize_case_type")]
    pub(crate) case_type: Vec<ConstitutionCaseType>,

    /// 종국결과
    #[serde(rename = "endRstCode", serialize_with = "serialize_decision_type")]
    pub(crate) decision_type: Vec<ConstitutionDecisionType>,

    /// 재판부
    #[serde(
        rename = "justiceDepartCode",
        serialize_with = "serialize_department_type"
    )]
    pub(crate) bench_type: Vec<BenchType>,

    ///
    #[serde(rename = "lev", serialize_with = "serialize_record_type")]
    pub(crate) record_type: Vec<RecordType>,

    /// 제외 키워드
    #[serde(
        rename = "exclustionKeyword",
        serialize_with = "serialize_exclusion_keyword"
    )]
    pub(crate) exclusion_keyword: Vec<String>,
}

impl Default for ConstitutionalCaseSearchParameter {
    fn default() -> Self {
        Self {
            _idx: String::from("00"),
            _research: String::from(r#"{"flag" : "false", "reKeyword" : ""}"#),
            page: 1,
            keyword: String::new(),
            from: String::from("19880901"),
            to: String::from(""),
            limit: 30,
            sort: Sort::DateDesc,
            case_code: None,
            case_name: None,
            case_type: Vec::new(),
            decision_type: Vec::new(),
            bench_type: Vec::new(),
            record_type: Vec::new(),
            exclusion_keyword: Vec::new(),
        }
    }
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

fn serialize_department_type<S>(value: &Vec<BenchType>, s: S) -> Result<S::Ok, S::Error>
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

fn serialize_exclusion_keyword<S>(value: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
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
