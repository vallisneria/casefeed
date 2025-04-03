use serde::{Deserialize, Serialize};

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
