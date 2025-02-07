export interface CourtCase {
  case_code: string;

  decision_date: Temporal.PlainDate;

  case_title: string;

  case_nickname: string | null;

  en_bank: boolean;

  bulletin_code: string;

  court_name: string;

  decision_type: "판결" | "결정" | "명령";
}

function full_case_code(c: CourtCase): string {
  let sngo;
  switch (c.decision_type) {
    case "판결":
      sngo = " 선고";
      break;
    case "결정":
    case "명령":
      sngo = "자";
      break;
  }

  return `${c.court_name} ${c.decision_date}${sngo} ${c.case_code} ${c.en_bank ? "전원합의체 " : ""}${c.decision_type}`;
}
