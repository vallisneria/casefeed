use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum CourtType {
    /// 대법원
    #[serde(rename(serialize = "01"))]
    SupremeCourt,

    /// 고등법원
    #[serde(rename(serialize = "03"))]
    HighCourt,

    /// 하급심
    #[serde(rename(serialize = "04"))]
    LowerCourt,
}
