import { Context } from "@hono/hono";
import { ConstitutionalCaseSearch } from "../court-api/constitutional-court/main.ts";
import { buildRss, RssChannelConfig, RssItem } from "../rss.ts";
import { ConstitutionalCase } from "../court-api/constitutional-court/types.ts";
import { get_case } from "../court-api/court/main.ts";
import { CourtCase } from "../court-api/court/types.ts";

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
    link: "https://vallisneria-casefeed.deno.dev",
    language: "ko-kr",
  };

  return new Response(buildRss(option, item), {
    status: 200,
    headers: {
      "Content-type": "application/xml; charset=utf-8",
    },
  });
}

export async function scourt_enbank(_c: Context) {
  const start = new Temporal.PlainDate(2000, 1, 1);
  const end = new Temporal.PlainDate(9999, 12, 31);
  const item = (await get_case(20, start, end, "01")).map(
    (item: CourtCase): RssItem => {
      let sngo;
      switch (item.decision_type) {
        case "판결":
          sngo = " 선고";
          break;
        case "결정":
        case "명령":
          sngo = "자";
          break;
      }

      return {
        title:
          `${item.case_title} ${item.case_nickname ? "〈" + item.case_nickname + "〉 " : ""}` +
          `(${item.court_name} ${item.decision_date.toString().replaceAll("-", ". ")}.${sngo} ${item.case_code} ${item.en_bank ? "전원합의체 " : ""}${item.decision_type})`,
        link: `https://casenote.kr/${item.court_name}/${item.case_code.replace(/등$/gi, "")}`,
        author: item.court_name,
        pub_date: `${temporal_to_rfc822(item.decision_date.toZonedDateTime("14:00:00+0900"))}`,
        guid: {
          value: `${item.court_name}_${item.case_code}`,
          is_permalink: false,
        },
        description: "",
      };
    },
  );

  const option: RssChannelConfig = {
    title: "대법원 전원합의체",
    link: "https://vallisneria-casefeed.deno.dev",
    language: "ko-kr",
  };

  return new Response(buildRss(option, item), {
    status: 200,
    headers: {
      "Content-type": "application/xml; charset=utf-8",
    },
  });
}

export async function scourt_bulletin(_c: Context) {
  const start = new Temporal.PlainDate(2000, 1, 1);
  const end = new Temporal.PlainDate(9999, 12, 31);
  const item = (await get_case(30, start, end, "01")).map(
    (item: CourtCase): RssItem => {
      let sngo;
      switch (item.decision_type) {
        case "판결":
          sngo = " 선고";
          break;
        case "결정":
        case "명령":
          sngo = "자";
          break;
      }

      return {
        title:
          `${item.case_title} ${item.case_nickname ? "〈" + item.case_nickname + "〉 " : ""}` +
          `(${item.court_name} ${item.decision_date.toString().replaceAll("-", ". ")}.${sngo} ${item.case_code} ${item.en_bank ? "전원합의체 " : ""}${item.decision_type})`,
        link: `https://casenote.kr/${item.court_name}/${item.case_code.replace(/등$/gi, "")}`,
        author: item.court_name,
        pub_date: `${temporal_to_rfc822(item.decision_date.toZonedDateTime("14:00:00+0900"))}`,
        guid: {
          value: `${item.court_name}_${item.case_code}`,
          is_permalink: false,
        },
        description: "",
      };
    },
  );

  const option: RssChannelConfig = {
    title: "대법원 판례공보",
    link: "https://vallisneria-casefeed.deno.dev",
    language: "ko-kr",
  };

  return new Response(buildRss(option, item), {
    status: 200,
    headers: {
      "Content-type": "application/xml; charset=utf-8",
    },
  });
}

function temporal_to_rfc822(datetime: Temporal.ZonedDateTime): string {
  const day_of_week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][
    datetime.dayOfWeek
  ];
  const month = [
    "",
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ][datetime.month];

  return (
    `${day_of_week}, ${datetime.day.toString().padStart(2, "0")} ${month} ${datetime.year} ` +
    `${datetime.hour.toString().padStart(2, "0")}:${datetime.minute.toString().padStart(2, "0")}:` +
    `${datetime.second.toString().padStart(2, "0")} ${datetime.timeZoneId}`
  );
}
