use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub enum DecisionType {
    #[default]
    #[serde(rename = "판결", alias = "전원합의체 판결")]
    Judgement,

    #[serde(rename = "결정", alias = "전원합의체 결정")]
    Decision,

    #[serde(rename = "명령", alias = "전원합의체 명령")]
    Order,
}

impl Display for DecisionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::Judgement => "판결",
            Self::Decision => "결정",
            Self::Order => "명령",
        };

        write!(f, "{}", result)
    }
}
