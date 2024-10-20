pub fn iso8601_date_to_dot_date(before: &String) -> String {
    let date: Vec<&str> = before.split("-").collect();

    format!(
        "{year}. {month}. {day}.",
        year = date[0],
        month = date[1],
        day = date[2]
    )
}
