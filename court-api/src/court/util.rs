use crate::util::{remove_bracket, replace_middle_dot};
use serde::{Deserialize, Deserializer};

pub(super) fn title<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(replace_middle_dot(&s))
}

pub(super) fn subtitle<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s.as_str() == "" {
        return Ok(None);
    }

    let result = remove_bracket(&s);
    Ok(Some(result))
}

pub(super) fn bulletin_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "[공보불게재]" | "" => Ok(None),
        _ => Ok(Some(remove_bracket(&s))),
    }
}

pub(super) fn integer_date_to_iso8601<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let integer_date: u32 = s.parse::<u32>().unwrap();

    let year = integer_date / 10000;
    let month = (integer_date / 100) % 100;
    let day = integer_date % 100;

    Ok(format!("{year:0>4}-{month:0>2}-{day:0>2}"))
}

pub(super) fn is_enbank<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(s == "111")
}
