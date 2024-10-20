pub mod glaw;
mod util;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

const USER_AGENT: &str = "Mozilla/5.0";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CourtType {
    /// 헌법재판소
    #[serde(rename = "헌법재판소")]
    ConstitutionalCourt,

    #[serde(rename = "대법원")]
    SupremeCourt,

    #[serde(untagged)]
    Court(String),
}

impl Display for CourtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CourtType::ConstitutionalCourt => write!(f, "헌법재판소"),
            CourtType::SupremeCourt => write!(f, "대법원"),
            CourtType::Court(court_name) => write!(f, "{}", court_name),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DecisionType {
    /// 판결
    #[serde(rename = "판결")]
    Judgement,

    /// 결정
    #[serde(rename = "결정")]
    Decision,

    /// 명령
    #[serde(rename = "명령")]
    Order,
}

impl Display for DecisionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionType::Judgement => write!(f, "판결"),
            DecisionType::Decision => write!(f, "결정"),
            DecisionType::Order => write!(f, "명령"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CourtCaseType {
    /// 민사
    #[serde(rename = "민사", alias = "01")]
    Civil,

    /// 형사
    #[serde(rename = "형사", alias = "02")]
    Criminal,

    /// 일반행정
    #[serde(rename = "일반행정", alias = "06")]
    Administration,

    /// 조세
    #[serde(rename = "조세", alias = "세무", alias = "05")]
    Tax,

    /// 가사
    #[serde(rename = "가사", alias = "03")]
    Family,

    /// 특허
    #[serde(rename = "특허", alias = "04")]
    Patent,

    Etc,
}

impl Display for CourtCaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Civil => "민사",
            Self::Criminal => "형사",
            Self::Family => "가사",
            Self::Administration => "일반행정",
            Self::Patent => "특허",
            Self::Tax => "조세",
            _ => "기타",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum ConstitutionCaseType {
    /// 위헌법률심판
    #[serde(rename = "위헌법률심판", alias = "헌가")]
    ConstitutionalStatutes,

    /// 탄핵
    #[serde(rename = "탄핵심판", alias = "헌나")]
    Impeachment,

    /// 정당해산심판
    #[serde(rename = "정당해산심판", alias = "헌다")]
    DissolutionParty,

    /// 권한쟁의
    #[serde(rename = "권한쟁의", alias = "헌라")]
    CompetenceDispute,

    /// 헌법소원
    #[serde(rename = "헌법소원", alias = "헌마", alias = "헌바")]
    ConstitutionalComplaints,

    #[default]
    #[serde(rename = "기타")]
    Etc,
}
