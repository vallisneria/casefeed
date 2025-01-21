use super::util;
use crate::error::CourtApiError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstitutionalCase {
    /// 사건번호
    #[serde(alias = "eventNo")]
    case_code: String,

    /// 종국일자. (형식: YYYY-MM-DD)
    #[serde(alias = "date", deserialize_with = "util::des_decision_date")]
    decision_date: String,

    /// 사건명
    #[serde(alias = "eventName")]
    case_title: String,

    /// 사건별명
    #[serde(alias = "eventNickname")]
    case_nickname: Option<String>,

    /// 재판부
    #[serde(alias = "justiceDepart")]
    bench_type: BenchType,

    /// 수록정보
    #[serde(alias = "name")]
    record_type: RecordType,

    /// 종국결과
    #[serde(alias = "endRsta", deserialize_with = "util::des_decision_type")]
    decision_type: Vec<ConstitutionDecisionType>,

    /// PDF 파일 url
    #[serde(alias = "pdfFilePath", deserialize_with = "util::des_pdf_file_path")]
    pdf_file: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ConstitutionCaseType {
    /// 위헌법률심판사건
    #[serde(alias = "헌가")]
    ConstitutionalStatutes,

    /// 탄핵심판사건
    #[serde(alias = "헌나")]
    Impeachment,

    /// 정당해산심판사건
    #[serde(alias = "헌다")]
    DissolutionParty,

    /// 권한쟁의사건
    #[serde(alias = "헌라")]
    CompetenceDispute,

    /// 헌법재판소법 제68조 제1항에 의한 헌법소원심판사건
    #[serde(alias = "헌마")]
    ConstitutionalComplaintsType1,

    /// 헌법재판소법 제68조 제2항에 의한 헌법소원심판사건
    #[serde(alias = "헌바")]
    ConstitutionalComplaintsType2,

    /// 각종 신청사건 (국선대리인선임신청, 가처분신청 등)
    #[serde(alias = "헌사")]
    Application,

    /// 각종 특별사건
    #[serde(alias = "헌아")]
    Special,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ConstitutionDecisionType {
    /// 위헌
    #[serde(rename = "위헌")]
    Unconstitutional,

    /// 합헌
    #[serde(rename = "합헌")]
    Constitutional,

    /// 헌법불합치
    #[serde(rename = "헌법불합치")]
    Uncomformable,

    /// 한정위헌
    #[serde(rename = "한정위헌")]
    ConditionallyUnconstitutional,

    /// 한정합헌
    #[serde(rename = "한정합헌")]
    ConditionallyConstitutional,

    /// 인용
    #[serde(rename = "인용")]
    Upholding,

    /// 기각
    #[serde(rename = "기각")]
    Rejected,

    /// 각하
    #[serde(rename = "각하")]
    Dismissed,

    /// 취하
    #[serde(rename = "취하")]
    Withdrawn,

    /// 선정
    #[serde(rename = "선정")]
    Appointed,

    /// 기타
    #[serde(rename = "기타")]
    Other,
}

impl TryFrom<&str> for ConstitutionDecisionType {
    type Error = CourtApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match &value[0..=5] {
            "위헌" => ConstitutionDecisionType::Unconstitutional,
            "합헌" => ConstitutionDecisionType::Constitutional,
            "헌법" => ConstitutionDecisionType::Uncomformable,
            "한정" => match &value[6..=11] {
                "위헌" => Self::ConditionallyUnconstitutional,
                "합헌" => Self::ConditionallyConstitutional,
                _ => return Err(CourtApiError::UnexpectedValue),
            },
            "인용" => ConstitutionDecisionType::Upholding,
            "기각" => ConstitutionDecisionType::Rejected,
            "각하" => ConstitutionDecisionType::Dismissed,
            "취하" => ConstitutionDecisionType::Withdrawn,
            "선정" => ConstitutionDecisionType::Appointed,
            "기타" => ConstitutionDecisionType::Other,
            _ => return Err(CourtApiError::UnexpectedValue),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
pub enum Sort {
    #[serde(rename = "date:asc")]
    DateAsc,

    #[default]
    #[serde(rename = "date:desc")]
    DateDesc,

    #[serde(rename = "score:asc")]
    ScoreAsc,

    #[serde(rename = "score:desc")]
    ScoreDesc,

    #[serde(rename = "event_no_sort:asc")]
    EventNumberAsc,

    #[serde(rename = "event_no_sort:desc")]
    EventNumberDesc,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum BenchType {
    /// 지정재판부
    #[serde(rename = "지정재판부")]
    Panel,

    /// 전원재판부
    #[serde(rename = "전원재판부")]
    EnBancBench,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum RecordType {
    /// 결정문
    #[serde(rename = "결정문")]
    DecisionDocument,

    /// 공보
    #[serde(rename = "공보")]
    Bulletin,

    /// 판레집
    #[serde(rename = "판례집")]
    Casebook,
}
