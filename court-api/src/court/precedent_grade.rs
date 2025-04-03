use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum PrecedentGrade {
    /// 전원합의체
    #[serde(rename(serialize = "01"))]
    EnBanc,

    /// 간행판결
    #[serde(rename(serialize = "02"))]
    Published,

    /// 미간행판결
    #[serde(rename(serialize = "03"))]
    Unpublished,

    /// 폐기 (변경)
    #[serde(rename(serialize = "04"))]
    Overruled,
}
