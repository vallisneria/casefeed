use axum::Json;
use court_api::{CourtPrecedent, CourtPrecedentSearchParam, PrecedentGrade};

pub async fn root() -> &'static str {
    "Hello world!"
}

pub async fn court_en_banc() -> Json<Vec<CourtPrecedent>> {
    let search = CourtPrecedentSearchParam::default()
        .set_size(40)
        .set_page(1)
        .set_precedent_grade(Some(PrecedentGrade::EnBanc))
        .search()
        .await
        .unwrap();

    Json(search)
}
