use crate::{error::CourtApiError, CourtPrecedent};

use super::{PrecedentGrade, SortType};
use crate::court::util::portal_request;
use chrono::{Datelike, NaiveDate};
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourtPrecedentSearchParam {
    /// 검색어
    #[serde(rename(serialize = "srchwd"))]
    search_word: String,

    sort: String,

    /// 정렬 방법
    sort_type: SortType,

    search_range: String,

    tpc_jdcpct_cs_als_yn: String,

    /// 사건번호
    #[serde(rename(serialize = "csNoLstCtt"))]
    case_code: String,

    /// 사건명
    #[serde(rename(serialize = "csNmLstCtt"))]
    case_title: String,

    /// 참조조문
    #[serde(rename(serialize = "prvsRefcCtt"))]
    referance_law: String,

    /// 검색범위
    search_scope: String,

    /// 법원종류
    #[serde(rename(serialize = "jisJdcpcInstnDvsCd"))]
    court_type: String,

    jdcpct_cdcs_cd: String,

    /// 검색일자 시작점
    #[serde(rename(serialize = "prnjdgYmdFrom"))]
    #[serde(serialize_with = "int_date")]
    start_date: Option<NaiveDate>,

    /// 검색일자 끝점
    #[serde(rename(serialize = "prnjdgYmdTo"))]
    #[serde(serialize_with = "int_date")]
    end_date: Option<NaiveDate>,

    /// 판례등급
    #[serde(rename(serialize = "grpJdcpctGrCd"))]
    #[serde(serialize_with = "precedent_grade")]
    precedent_grade: Option<PrecedentGrade>,

    /// 법원명
    #[serde(rename(serialize = "cortNm"))]
    court_name: String,

    /// 페이지
    #[serde(rename(serialize = "pageNo"))]
    #[serde(serialize_with = "u8_to_str")]
    page: u8,

    jis_jdcpc_instn_dvs_cd_grp: String,

    grp_jdcpct_gr_cd_grp: String,

    jdcpct_cdcs_cd_grp: String,

    adjd_typ_cd_grp: String,

    /// 한 번에 불러올 판례 개수
    #[serde(rename(serialize = "pageSize"))]
    #[serde(serialize_with = "u8_to_str")]
    size: u8,

    re_srch_flag: String,

    bef_srchwd: String,

    pre_srch_conditions: String,

    init_yn: String,

    total_count: String,

    jdcpct_gr_cd: String,

    category: String,
}

impl Default for CourtPrecedentSearchParam {
    fn default() -> Self {
        Self {
            search_word: String::new(),
            sort: String::from(
                "prnjdg_ymd_o desc, jis_jdcpc_instn_dvs_cd_s asc, jdcpct_gr_cd_s asc",
            ),
            sort_type: SortType::DecisionDateDesc,
            search_range: String::new(),
            tpc_jdcpct_cs_als_yn: String::new(),
            case_code: String::new(),
            case_title: String::new(),
            referance_law: String::new(),
            search_scope: String::new(),
            court_type: String::new(),
            jdcpct_cdcs_cd: String::new(),
            start_date: NaiveDate::from_ymd_opt(1901, 1, 1),
            end_date: NaiveDate::from_ymd_opt(2099, 12, 31),
            precedent_grade: Some(PrecedentGrade::EnBanc),
            court_name: String::new(),
            page: 1,
            jis_jdcpc_instn_dvs_cd_grp: String::new(),
            grp_jdcpct_gr_cd_grp: String::new(),
            jdcpct_cdcs_cd_grp: String::new(),
            adjd_typ_cd_grp: String::new(),
            size: 40,
            re_srch_flag: String::new(),
            bef_srchwd: String::new(),
            pre_srch_conditions: String::new(),
            init_yn: String::from("N"),
            total_count: String::from("392"),
            jdcpct_gr_cd: String::from("111|112|130|141|180|182|232|235|201"),
            category: String::from("jdcpct"),
        }
    }
}

// builder
impl CourtPrecedentSearchParam {
    /// 검색어를 설정하는 함수.
    ///
    /// ## 예시
    ///
    pub fn set_search_word<V: Into<String>>(mut self, value: V) -> Self {
        self.search_word = value.into();
        self
    }

    /// 판례의 정렬 방법을 설정하는 함수.
    ///
    /// ## 예시
    /// ```rs
    /// // 선고일자 내림차순으로 정렬
    /// search.set_sort_type(SortType::DecisionDateDesc);
    /// ```
    pub fn set_sort_type(mut self, value: SortType) -> Self {
        self.sort_type = value;
        self
    }

    /// 검색할 사건번호를 설정하는 함수.
    ///
    /// ## 예시
    /// ```rs
    /// search.set_case_code("2016도10912");
    /// ```
    pub fn set_case_code<V: Into<String>>(mut self, value: V) -> Self {
        self.case_code = value.into();
        self
    }

    /// 검색할 사건명을 설정하는 함수.
    /// # 예시
    /// ```rs
    /// search.set_case_title("손해배상");
    /// ```
    pub fn set_case_title<V: Into<String>>(mut self, value: V) -> Self {
        self.case_title = value.into();
        self
    }

    /// 참조조문을 설정하는 함수.
    /// ## 예시
    /// ```rs
    /// search.set_referance_law("민법 제390조");
    /// ```
    pub fn set_referance_law<V: Into<String>>(mut self, value: V) -> Self {
        self.referance_law = value.into();
        self
    }

    /// 검색어의 범위를 설정하는 함수
    pub fn set_search_scope() {
        unimplemented!()
    }

    /// 검색할 판례의 선고일자 시작점을 설정하는 함수.
    /// ## 예시
    /// ```
    /// let date = NaiveDate::from_ymd_opt(2000, 1, 1);
    /// search.set_start_date(date);
    /// ```
    pub fn set_start_date(mut self, value: Option<NaiveDate>) -> Self {
        self.start_date = value;
        self
    }

    /// 검색할 판례의 선고일자 시작점을 설정하는 함수.
    ///
    /// ## 예시
    /// ```rs
    /// search.set_start_date_ymd(2000, 1, 1);
    /// ```
    ///
    /// ## 주의사항
    /// 입력한 날짜가 [부적절](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html#method.from_ymd_opt)한
    /// 경우 자동으로 None이 입력됩니다.
    /// ```rs
    /// search.set_start_date_ymd(2024, 13, 32);
    /// assert_eq!(search.start_date, None);
    /// ```
    pub fn set_start_date_ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        self.start_date = date;
        self
    }

    /// 검색할 판례의 선고일자 끝점을 설정하는 함수.
    ///
    /// ## 예시
    /// ```rs
    /// let date = NaiveDate::from_ymd_opt(9999, 12, 31);
    /// search.set_end_date(date);
    /// ```
    ///
    pub fn set_end_date(mut self, value: Option<NaiveDate>) -> Self {
        self.end_date = value;
        self
    }

    /// 검색할 판례의 선고일자 끝점을 설정하는 함수.
    ///
    /// ## 예시
    /// ```rs
    /// search.set_end_date_ymd(9999, 12, 31);
    /// ```
    ///
    /// ## 주의사항
    /// 입력한 날짜가 [부적절](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html#method.from_ymd_opt)한
    /// 경우 자동으로 None이 입력됩니다.
    /// ```rs
    /// search.set_end_date_ymd(2024, 13, 32);
    /// assert_eq!(search.end_date, None);
    /// ```
    pub fn set_end_date_ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        self.end_date = date;
        self
    }

    /// 검색할 판례의 판례등급을 설정하는 함수.
    /// ## 예시
    /// ```rs
    /// // 전원합의체 판례 검색하기
    /// let grade = PrecedentGrade::EnBanc;
    /// search.set_precedent_grade(Some(grade));
    /// ```
    pub fn set_precedent_grade(mut self, value: Option<PrecedentGrade>) -> Self {
        self.precedent_grade = value;
        self
    }

    /// 법원명을 설정하는 함수.
    ///
    /// ## 예시
    /// ```
    /// search.set_court_name("서울중앙지방법원")
    /// ```
    pub fn set_court_name<V: Into<String>>(mut self, value: V) -> Self {
        self.court_name = value.into();
        self
    }

    /// 검색 결과의 페이지를 설정하는 함수.
    pub fn set_page<V: Into<u8>>(mut self, value: V) -> Self {
        self.page = value.into();
        self
    }

    /// 한 번에 검색할 판례의 개수를 설정하는 함수.
    /// ## 예시
    /// ```rs
    /// let result = search.set_size(40).search().await?;
    /// assert_eq!(result.len(), 40);
    /// ```
    pub fn set_size<V: Into<u8>>(mut self, value: V) -> Self {
        self.size = value.into();
        self
    }
}

impl CourtPrecedentSearchParam {
    /// 판례를 검색하도록 요청하는 함수
    /// ## 예시
    /// ```rs
    /// CourtPrecedentSearchParam::default()
    ///     .set_search_word("손해배상")
    ///     .search() // <- here
    ///     .await?;
    /// ```
    pub async fn search(&self) -> Result<Vec<CourtPrecedent>, CourtApiError> {
        let path = "/pgp/pgp1011/selectJdcpctSrchRsltLst.on";
        let result: Vec<CourtPrecedent> = portal_request(path, &self, "dlt_jdcpctRslt").await?;

        Ok(result)
    }
}

fn int_date<S>(x: &Option<NaiveDate>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let result = if let Some(date) = x {
        let year = date.year();
        let month = date.month0() + 1;
        let day = date.day0() + 1;

        format!("{year:0>4}{month:0>2}{day:0>2}")
    } else {
        String::new()
    };

    s.serialize_str(&result)
}

fn u8_to_str<S>(x: &u8, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let result = x.to_string();
    s.serialize_str(&result)
}

fn precedent_grade<S>(x: &Option<PrecedentGrade>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(data) = x {
        data.serialize(s)
    } else {
        s.serialize_str("")
    }
}
