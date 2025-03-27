use chrono::Datelike;
use court_api::HasUrl;
use court_api::{CaseProvider, DecisionType};
use db::{constitutional_court::ConstitutionalPrecedentDB, court::CourtPrecedentDB};
use rss::{Guid, GuidBuilder, Item, ItemBuilder};
use url::Url;

pub trait Rss {
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn link(&self) -> Url;
    fn guid(&self) -> Guid;
    fn pub_date(&self) -> String;
}

pub struct CourtPrecedentDBWrapper {
    pub body: CourtPrecedentDB,
    pub case_provider: CaseProvider,
}

impl Rss for CourtPrecedentDBWrapper {
    fn title(&self) -> String {
        let subtitle = if let Some(t) = self.body.precedent.case_subtitle.as_ref() {
            format!(" 〈{t}〉")
        } else {
            String::new()
        };

        let case_code = {
            let date = format!(
                "{year}. {month}. {day}.",
                year = self.body.precedent.decision_date.year(),
                month = self.body.precedent.decision_date.month0() + 1,
                day = self.body.precedent.decision_date.day0() + 1
            );

            let sungo = match self.body.precedent.decision_type {
                DecisionType::Decision | DecisionType::Order => "자",
                _ => " 선고",
            };

            let en_banc = match self.body.precedent.en_banc {
                true => "전원합의체 ",
                false => "",
            };

            format!(
                "{} {}{} {} {}{}",
                self.body.precedent.court_name,
                date,
                sungo,
                self.body.precedent.case_code,
                en_banc,
                self.body.precedent.decision_type
            )
        };
        format!(
            "{}{} ({})",
            self.body.precedent.case_title, subtitle, case_code
        )
    }

    fn description(&self) -> String {
        format!(
            "<h2>판결요지</h2>{}",
            self.body
                .summary
                .iter()
                .map(|s| format!("<p>{s}</p>"))
                .collect::<Vec<String>>()
                .join("")
        )
    }

    fn pub_date(&self) -> String {
        self.body.collected_time.to_rfc2822()
    }

    fn link(&self) -> Url {
        self.body.precedent.url(&self.case_provider)
    }

    fn guid(&self) -> Guid {
        let value = format!(
            "{}_{}",
            self.body.precedent.court_name, self.body.precedent.case_code
        );

        GuidBuilder::default().permalink(false).value(value).build()
    }
}

impl Into<Item> for CourtPrecedentDBWrapper {
    fn into(self) -> Item {
        ItemBuilder::default()
            .title(self.title())
            .description(self.description())
            .pub_date(self.pub_date())
            .link(Some(self.link().as_str().to_string()))
            .guid(self.guid())
            .build()
    }
}

pub struct ConstitutionalPrecedentDBWrapper {
    pub body: ConstitutionalPrecedentDB,
    pub case_provider: CaseProvider,
}

impl Rss for ConstitutionalPrecedentDBWrapper {
    fn title(&self) -> String {
        let subtitle = if let Some(subtitle) = self.body.precedent.case_subtitle.as_ref() {
            format!(" 〈{subtitle}〉")
        } else {
            String::new()
        };

        let case_code = {
            let decision_date = &self.body.precedent.decision_date;
            let decision_date = format!(
                "{year}. {month}. {day}.",
                year = decision_date.year(),
                month = decision_date.month0() + 1,
                day = decision_date.day0() + 1
            );

            format!(
                "헌법재판소 {decision_date}자 {} 결정",
                self.body.precedent.case_code
            )
        };

        format!(
            "{}{} ({})",
            self.body.precedent.case_title, subtitle, case_code
        )
    }

    fn description(&self) -> String {
        if let Some(judgement_note) = self.body.precedent.judgement_note.as_ref() {
            let judgement_note_html = judgement_note
                .iter()
                .map(|note| format!("<p>{note}</>"))
                .collect::<Vec<String>>()
                .join("");
            format!("<h2>판시사항<h2>{judgement_note_html}")
        } else {
            String::new()
        }
    }

    fn pub_date(&self) -> String {
        self.body.collected_time.to_rfc2822()
    }

    fn link(&self) -> Url {
        self.body.precedent.url(&self.case_provider)
    }

    fn guid(&self) -> Guid {
        GuidBuilder::default()
            .permalink(false)
            .value(format!("헌법재판소_{}", self.body.precedent.case_code))
            .build()
    }
}

impl Into<Item> for ConstitutionalPrecedentDBWrapper {
    fn into(self) -> Item {
        ItemBuilder::default()
            .title(self.title())
            .description(self.description())
            .pub_date(self.pub_date())
            .link(Some(self.link().as_str().to_string()))
            .guid(self.guid())
            .build()
    }
}
