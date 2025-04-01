use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub enum CaseProvider {
    /// 공식 사이트
    /// - 법원의 경우 [사법정보포털](https://portal.scourt.go.kr)
    /// - 헌법재판소의 경우 [지능형 통합검색](https://isearch.ccourt.go.kr)
    Official,

    /// 케이스노트: https://casenote.kr
    #[default]
    #[serde(alias = "casenote", alias = "케이스노트")]
    Casenote,

    /// 엘박스: https://lbox.kr
    #[serde(alias = "lbox", alias = "엘박스")]
    Lbox,

    /// 빅케이스: https://bigcase.ai
    #[serde(alias = "bigcase", alias = "빅케이스")]
    Bigcase,
}
