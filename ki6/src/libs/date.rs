use chrono::prelude::*;

pub fn compute_last_monday(today: chrono::NaiveDate) -> chrono::NaiveDate {
    for i in 1..7 {
        let date = today - chrono::Duration::days(i);
        if date.weekday() == chrono::Weekday::Mon {
            return date;
        }
    }

    today
}

pub fn weekday_to_japanese(weekday: chrono::Weekday) -> String {
    match weekday {
        chrono::Weekday::Mon => "月",
        chrono::Weekday::Tue => "火",
        chrono::Weekday::Wed => "水",
        chrono::Weekday::Thu => "木",
        chrono::Weekday::Fri => "金",
        chrono::Weekday::Sat => "土",
        chrono::Weekday::Sun => "日",
    }
    .to_string()
}
