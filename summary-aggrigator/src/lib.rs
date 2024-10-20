use court_api::CourtType;
use serde::Deserialize;
use worker::{console_error, console_log, event, query, Env, ScheduleContext, ScheduledEvent};

#[event(scheduled)]
async fn fetch(_req: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();
    let database = env.d1("DB").unwrap();

    let list = query!(
        &database,
        "SELECT court_case.glaw_id, court_case.case_code, court_case.court_name
        FROM bulletin_case
        INNER JOIN court_case
        ON (bulletin_case.case_code = court_case.case_code)
            AND (bulletin_case.court_name = court_case.court_name)
        LEFT OUTER JOIN case_summary
        ON (bulletin_case.case_code = case_summary.case_code)
            AND (bulletin_case.court_name = case_summary.court_name)
        WHERE case_summary.case_code IS NULL
        LIMIT 20"
    )
    .run()
    .await
    .unwrap()
    .results::<CasefeedDatabaseResponse>()
    .unwrap();

    for prec in list {
        let summary = court_api::glaw::get_case_summary(prec.glaw_id)
            .await
            .unwrap();

        let main_issue: String = format!("{:?}", summary.main_issue);
        let summary_of_decision = format!("{:?}", summary.summary);

        let result = query!(
            &database,
            "INSERT INTO case_summary (case_code, court_name, main_issue, summary_of_decision)
            VALUES (?1, ?2, json(?3), json(?4))",
            &prec.case_code,
            &prec.court_name,
            &main_issue,
            &summary_of_decision
        )
        .unwrap()
        .run()
        .await;

        match result {
            Ok(_) => console_log!("'{}' is successfully inserted.", prec.case_code),
            Err(e) => console_error!("Error: {}", e),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CasefeedDatabaseResponse {
    glaw_id: u32,
    case_code: String,
    court_name: CourtType,
}
