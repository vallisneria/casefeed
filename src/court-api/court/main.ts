import { USER_AGENT } from "../main.ts";
import { CourtCase } from "./types.ts";

export async function get_case(
  length: number,
  start: Temporal.PlainDate,
  end: Temporal.PlainDate,
  record_type: "01" | "02",
) {
  const query = {
    srchwd: "",
    sort: "prnjdg_ymd_o desc, jis_jdcpc_instn_dvs_cd_s asc, jdcpct_gr_cd_s asc",
    sortType: "선고일자내림차순",
    searchRange: "",
    tpcJdcpctCsAlsYn: "",
    csNoLstCtt: "",
    csNmLstCtt: "",
    prvsRefcCtt: "",
    searchScope: "",
    jisJdcpcInstnDvsCd: "01",
    jdcpctCdcsCd: "",
    prnjdgYmdFrom: `${start.year}${start.month.toString().padStart(2, "0")}${start.day.toString().padStart(2, "0")}`,
    prnjdgYmdTo: `${end.year}${end.month.toString().padStart(2, "0")}${end.day.toString().padStart(2, "0")}`,
    grpJdcpctGrCd: record_type,
    cortNm: "",
    pageNo: "1",
    jisJdcpcInstnDvsCdGrp: "",
    grpJdcpctGrCdGrp: "",
    jdcpctCdcsCdGrp: "",
    adjdTypCdGrp: "",
    pageSize: `${~~length}`,
    reSrchFlag: "",
    befSrchwd: "",
    preSrchConditions: "",
    initYn: "N",
    totalCount: "395",
    jdcpctGrCd: "111|112|130|141|180|182|232|235|201",
    category: "jdcpct",
  };

  const resp = await fetch(
    "https://portal.scourt.go.kr/pgp/pgp1011/selectJdcpctSrchRsltLst.on",
    {
      method: "POST",
      body: JSON.stringify({ dma_searchParam: query }),
      headers: {
        "Content-Type": "application/json;charset=UTF-8",
        "User-Agent": USER_AGENT,
        Origin: "https://portal.scourt.go.kr",
      },
    },
  );

  const resp_json = await resp.json();

  if (resp_json["errors"] != null) {
    throw new Error(resp_json["errors"]["errorMessage"]);
  }

  return resp_json["data"]["dlt_jdcpctRslt"].map((item: any): CourtCase => {
    const year = item["prnjdgYmd"].substr(0, 4);
    const month = item["prnjdgYmd"].substr(4, 2);
    const day = item["prnjdgYmd"].substr(6, 2);

    return {
      court_name: item["cortNm"],
      en_bank: item["jdcpctGrCd"] === "111",
      case_code: item["csNoLstCtt"].replace(/ㆍ/gi, "·"),
      case_title: item["csNmLstCtt"].replace(/ㆍ/gi, "·"),
      case_nickname: item["jdcpctCsAlsNm"]?.replace(/〈(.+)〉/giu, "$1"),
      decision_date: new Temporal.PlainDate(year, month, day),
      bulletin_code: item["jdcpctPublcCtt"]?.replace(/\[(.+)\]/giu, "$1"),
      decision_type: item["adjdTypNm"],
    };
  });
}

Deno.test("adsf", async (): Promise<void> => {
  const start = new Temporal.PlainDate(2000, 1, 1);
  const end = new Temporal.PlainDate(9999, 12, 31);
  const result = await get_case(20, start, end, "01");

  console.log(result);
});

// 판례목록
("https://portal.scourt.go.kr/pgp/pgp1011/selectJdcpctSrchRsltLst.on");

// 판시사항
("https://portal.scourt.go.kr/pgp/pgp1011/selectJdcpctSumrInf.on");

let query = { dma_searchParam: { jisCntntsSrno: 3336565 } };
