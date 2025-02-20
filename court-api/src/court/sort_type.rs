use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, Default)]
pub enum SortType {
    /// 정확도
    #[serde(rename(serialize = "정확도"))]
    Accuracy,

    /// 인기도
    #[serde(rename(serialize = "인기도"))]
    Popularity,

    /// 법원 심급 내림차순
    #[serde(rename(serialize = "법원심급별내림차순"))]
    CourtLevelDesc,

    /// 법원 심급 오름차순
    #[serde(rename(serialize = "법원심급별오름차순"))]
    CourtLevelAsc,

    /// 선고일자 내림차순
    #[default]
    #[serde(rename(serialize = "선고일자내림차순"))]
    DecisionDateDesc,

    /// 선고일자 오름차순
    #[serde(rename(serialize = "선고일자오름차순"))]
    DecisionDateAsc,
}
