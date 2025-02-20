use chrono::NaiveDate;
use serde::{de::Deserializer, Deserialize};

pub(crate) fn replace_middle_dot(before: &String) -> String {
    before.replace("ㆍ", "·")
}

pub(crate) fn remove_bracket(before: &String) -> String {
    before
        .replace("[", "")
        .replace("]", "")
        .replace("〈", "")
        .replace("〉", "")
}

pub(crate) fn integer_date_to_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let integer_date: u32 = s
        .parse::<u32>()
        .expect("Unexpected date format. `YYYYMMDD` expected.");

    let year = (integer_date / 10000) as i32;
    let month = (integer_date / 100) % 100;
    let day = integer_date % 100;

    Ok(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}
