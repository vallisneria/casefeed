use court_api::glaw::PrecedentListItem;
use worker::{console_error, console_log, event, query, Env, ScheduleContext, ScheduledEvent};

#[event(scheduled)]
async fn scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let database = env.d1("DB").unwrap();

    let list = court_api::glaw::get_all_case_list(40).await.unwrap();
    let bulletin_list = list
        .to_vec()
        .into_iter()
        .filter(|prec| prec.bulletin_code.is_some())
        .collect::<Vec<PrecedentListItem>>();

    for prec in list {
        let query = query!(
            &database,
            "INSERT INTO court_case (glaw_id, case_title, case_subtitle, case_code, court_name, case_type, decision_date, en_bank, decision_type)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
            prec.glaw_id, prec.title, prec.subtitle, prec.case_code, prec.court_name,
            prec.case_type, prec.decision_date, prec.en_bank, prec.decision_type
        ).unwrap();

        let result = query.run().await;

        match result {
            Ok(_) => console_log!("'{}' upload", prec.case_code),
            Err(e) => console_error!(
                "Failed upload '{code}': {reason}",
                code = prec.case_code,
                reason = e
            ),
        }
    }

    for prec in bulletin_list {
        let query = query!(
            &database,
            "INSERT INTO bulletin_case (case_code, court_name, bulletin_code)
            VALUES (?1, ?2, ?3)",
            prec.case_code,
            prec.court_name,
            prec.bulletin_code,
        )
        .unwrap();

        let result = query.run().await;

        match result {
            Ok(_) => console_log!("'{}' upload", prec.case_code),
            Err(e) => console_error!(
                "Failed upload '{code}': {reason}",
                code = prec.case_code,
                reason = e
            ),
        }
    }
}
