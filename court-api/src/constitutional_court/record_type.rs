use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::DecisionDocument => "결정문",
            Self::Bulletin => "공보",
            Self::Casebook => "판례집",
        };

        write!(f, "{}", result)
    }
}
