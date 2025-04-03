use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default)]
pub enum SortType {
    #[serde(rename = "date:asc")]
    DateAsc,

    #[default]
    #[serde(rename = "date:desc")]
    DateDesc,

    #[serde(rename = "score:asc")]
    ScoreAsc,

    #[serde(rename = "score:desc")]
    ScoreDesc,

    #[serde(rename = "event_no_sort:asc")]
    EventNumberAsc,

    #[serde(rename = "event_no_sort:desc")]
    EventNumberDesc,
}
