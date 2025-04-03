export interface ConstitutionalCase {
  case_code: string;

  decision_date: string;

  case_title: string;

  case_nickname: string | null;

  bench_type: "지정재판부" | "전원재판부";

  record_type: "결정문" | "공보" | "판례집";

  bulletin_code: string;

  judgement_note: string;
}

/** 헌법재판소 사건유형 */
export enum ConstitutionalCaseType {
  /** 위헌법률심판사건 (헌가) */
  ConstitutionalStatutes = 1,

  /** 탄핵심판사건 (헌나) */
  Impeachment,

  /** 정당해산심판사건 (헌다) */
  DissolutionParty,

  /** 권한쟁의사건 (헌라) */
  CompetenceDispute,

  /** 헌법재판소법 제68조 제1항에 의한 헌법소원심판사건 (헌마) */
  ConstitutionalComplaintsType1,

  /** 헌법재판소법 제68조 제2항에 의한 헌법소원심판사건 (헌바) */
  ConstitutionalComplaintsType2,

  /** 신청사건 (헌사) */
  Application,

  /** 각종 특별사건 (헌아) */
  Special,
}

/** 헌법재판소 종국결과 */
export enum ConstitutionalDecisionType {
  /** 위헌 */
  Unconstitutional = "위헌",

  /** 합헌 */
  Constitutional = "합헌",

  /** 헌법불합치 */
  Uncomformable = "헌법불합치",

  /** 한정위헌 */
  ConditionallyUnconstitutional = "한정위헌",

  /** 한정합헌 */
  ConditionallyConstitutional = "한정합헌",

  /** 인용 */
  Upholding = "인용",

  /** 기각 */
  Rejected = "기각",

  /** 각하 */
  Dismissed = "각하",

  /** 취하 */
  Withdrawn = "취하",

  /** 선정 */
  Appointed = "선정",

  /** 기타 */
  Other = "기타",
}

export enum BenchType {
  /** 지정재판부 */
  Panel = 2,

  /** 전원재판부 */
  EnBankBench = 1,
}

export type Sort =
  | "date:asc"
  | "date:desc"
  | "score:asc"
  | "score:desc"
  | "event_no_sort:asc"
  | "event_no_score:desc";
