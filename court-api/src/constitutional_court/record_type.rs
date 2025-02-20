use serde::{Deserialize, Serialize};

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
