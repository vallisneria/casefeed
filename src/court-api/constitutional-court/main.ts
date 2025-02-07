import { USER_AGENT } from "../main.ts";
import {
  ConstitutionalCaseType,
  ConstitutionalDecisionType,
  ConstitutionalCase,
  Sort,
  BenchType,
} from "./types.ts";

export class ConstitutionalCaseSearch {
  private readonly _idx: string;

  private readonly _research: string;

  private _page: number;

  set page(value: number) {
    this._page = ~~value;
  }

  private _keyword: string;

  set keyword(value: string) {
    this._keyword = value;
  }

  private _from: string;

  set from(value: string) {
    this._from = value;
  }

  /** 종국일자 검색 마지막 날짜 (형태: YYYYMMDD) */
  private _to: string;

  set to(value: string) {
    this._to = value;
  }

  private _limit: number;

  set limit(value: number) {
    this._limit = ~~value;
  }

  private _sort: Sort;

  set sort(value: Sort) {
    this._sort = value;
  }

  private _case_type: ConstitutionalCaseType[];

  set case_type(value: ConstitutionalCaseType[]) {
    this._case_type = [...new Set(value)];
  }

  private _decision_type: ConstitutionalDecisionType[];

  set decision_type(value: ConstitutionalDecisionType[]) {
    this._decision_type = [...new Set(value)];
  }

  private _exclusion_keyword: string[];

  set exclusion_keyword(value: string[]) {
    this._exclusion_keyword = [...new Set(value)];
  }

  private _bench_type: BenchType[];

  set bench_type(value: BenchType[]) {
    this._bench_type = [...new Set(value)];
  }

  private _record_type: ("결정문" | "공보" | "판례집")[];

  set record_type(value: ("결정문" | "공보" | "판례집")[]) {
    this._record_type = [...new Set(value)];
  }

  constructor() {
    this._idx = "00";
    this._research = `{"flag" : "false", "reKeyword" : ""}`;
    this._page = 1;
    this._keyword = "";
    this._from = "19880901";
    this._to = "";
    this._limit = 30;
    this._sort = "date:desc";
    this._case_type = [];
    this._decision_type = [];
    this._exclusion_keyword = [];
    this._bench_type = [];
    this._record_type = [];
  }

  private buildDecisionType(): string {
    const a = {
      위헌: 67,
      합헌: 68,
      헌법불합치: 69,
      한정위헌: 70,
      한정합헌: 71,
      인용: 72,
      기각: 73,
      각하: 74,
      취하: 76,
      선정: 85,
      기타: 99,
    };

    return this._decision_type
      .map((item) => a[item])
      .filter((item) => item)
      .join(",");
  }

  private buildRecordType(): string {
    const a = {
      결정문: 1,
      공보: 2,
      판례집: 3,
    };

    return this._record_type
      .map((item) => a[item])
      .filter((item) => item)
      .join(",");
  }

  private buildBody(): { [s: string]: string } {
    return {
      idx: this._idx,
      reSearch: this._research,
      offset: `${this._page}`,
      keyword: this._keyword,
      dateFrom: this._from,
      dateTo: this._to,
      limit: `${this._limit}`,
      sort: this._sort,
      eventNobCode: this._case_type.join(","),
      endRstaCode: this.buildDecisionType(),
      exclustionKeyword: this._exclusion_keyword.join(","),
      justiceDepartCode: this._bench_type.join(","),
      lev: this.buildRecordType(),
    };
  }

  async search(): Promise<ConstitutionalCase[]> {
    const url =
      "https://isearch.ccourt.go.kr/api/index/searcher/categorySearch";

    const response = await fetch(url, {
      method: "POST",
      body: new URLSearchParams(this.buildBody()),
      headers: {
        "User-Agent": USER_AGENT,
        "Content-type": "application/x-www-form-urlencoded; charset=UTF-8",
      },
    });

    const response_json = await response.json();

    return response_json["returnObject"]["resultList"].map(
      (item: { [key: string]: unknown }): ConstitutionalCase => {
        return {
          case_code: item["eventNo"],
          decision_date: `${item.date.slice(0, 4)}-${item.date.slice(4, 6)}-${item.date.slice(6, 8)}`,
          case_title: item["eventName"],
          case_nickname: item["eventNickname"] ?? null,
          bench_type: item["justiceDepart"],
          record_type: item["name"],
          judgement_note: item["judgementNote"],
        };
      },
    );
  }
}
