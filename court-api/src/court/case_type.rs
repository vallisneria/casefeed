use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum CaseType {
    #[serde(rename = "민사", alias = "01")]
    Civil,

    #[serde(rename = "형사", alias = "02")]
    Criminal,

    #[serde(rename = "일반행정", alias = "03")]
    Adminstration,

    #[serde(rename = "조세", alias = "04")]
    Tax,

    #[serde(rename = "가사", alias = "05")]
    Family,

    #[serde(rename = "특허", alias = "06")]
    Patent,
}
