use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ConstitutionCaseType {
    /// 위헌법률심판사건
    #[serde(alias = "헌가")]
    ConstitutionalStatutes,

    /// 탄핵심판사건
    #[serde(alias = "헌나")]
    Impeachment,

    /// 정당해산심판사건
    #[serde(alias = "헌다")]
    DissolutionParty,

    /// 권한쟁의사건
    #[serde(alias = "헌라")]
    CompetenceDispute,

    /// 헌법재판소법 제68조 제1항에 의한 헌법소원심판사건
    #[serde(alias = "헌마")]
    ConstitutionalComplaintsType1,

    /// 헌법재판소법 제68조 제2항에 의한 헌법소원심판사건
    #[serde(alias = "헌바")]
    ConstitutionalComplaintsType2,

    /// 각종 신청사건 (국선대리인선임신청, 가처분신청 등)
    #[serde(alias = "헌사")]
    Application,

    /// 각종 특별사건
    #[serde(alias = "헌아")]
    Special,
}
