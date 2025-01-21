use serde::{Deserialize, Deserializer};

use super::ConstitutionDecisionType;

pub(crate) fn des_decision_date<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;

    Ok(format!(
        "{:0>4}-{:0>2}-{:0>2}",
        &s[0..=3],
        &s[4..=5],
        &s[6..=7]
    ))
}

pub(crate) fn des_decision_type<'de, D>(d: D) -> Result<Vec<ConstitutionDecisionType>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    let result = s.split("\n").map(|item| item.try_into().unwrap()).collect();

    Ok(result)
}

pub(crate) fn des_pdf_file_path<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    let result = format!("https://isearch.ccourt.go.kr{s}");

    Ok(result)
}
