use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub enum CaseProvider {
    /// 공식 사이트
    /// - 법원의 경우 [사법정보포털](https://portal.scourt.go.kr)
    /// - 헌법재판소의 경우 [지능형 통합검색](https://isearch.ccourt.go.kr)
    #[default]
    Official,

    /// 케이스노트: https://casenote.kr
    Casenote,

    /// 엘박스: https://lbox.kr
    Lbox,

    /// 빅케이스: https://bigcase.ai
    Bigcase,
}
