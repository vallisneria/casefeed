import { Context } from "@hono/hono";
import { ConstitutionalCaseSearch } from "../court-api/constitutional-court/main.ts";
import { buildRss, RssChannelConfig, RssItem } from "../rss.ts";
import { ConstitutionalCase } from "../court-api/constitutional-court/types.ts";

export async function ccourt_enbank(_c: Context) {
  const search = new ConstitutionalCaseSearch();
  search.record_type = ["공보", "판례집"];
  const item = (await search.search()).map(
    (item: ConstitutionalCase): RssItem => {
      return {
        title:
          `${item.case_title} ${item.case_nickname ? "〈" + item.case_nickname + "〉 " : ""}` +
          `(헌법재판소 ${item.decision_date.replaceAll("-", ". ")}. 자 ${item.case_code} ${item.bench_type} 결정)`,
        link: `https://casenote.kr/헌법재판소/${item.case_code.replace(/등$/gi, "")}`,
        author: "헌법재판소",
        pub_date: item.decision_date,
        guid: { is_permalink: false, value: item.case_code },
        description: `<h2>판시사항</h2>${item.judgement_note}`,
      };
    },
  );

  const option: RssChannelConfig = {
    title: "헌법재판소 판례공보",
    link: "https://ccourt.go.kr",
    language: "ko-kr",
  };

  return new Response(buildRss(option, item), {
    status: 200,
    headers: {
      "Content-type": "application/xml; charset=utf-8",
    },
  });
}
