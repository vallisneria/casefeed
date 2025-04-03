use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum BenchType {
    /// 지정재판부
    #[serde(rename = "지정재판부")]
    Panel,

    /// 전원재판부
    #[serde(rename = "전원재판부")]
    EnBancBench,
}
