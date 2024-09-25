use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};

const BASE54: &str = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ34689";
const WEEK_TICK: i64 = 11631; // Number of seconds in one tick (a 52nd of a week)

/// Returns the base time (January 1, 2024, at 00:00:00 UTC).
fn base_time() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()
}

/// Converts an integer to a string using the custom base54 alphabet.
fn base_x(mut num: i32, alphabet: &str) -> String {
    let base = alphabet.chars().count() as i32;
    let chars: Vec<char> = alphabet.chars().collect();

    if num == 0 {
        return chars[0].to_string();
    }

    let mut base_x_result = Vec::new();
    while num > 0 {
        let remainder = (num % base) as usize;
        num /= base;
        base_x_result.push(chars[remainder]);
    }
    base_x_result.reverse();
    base_x_result.iter().collect()
}

/// Generates a unique tag from a `DateTime<Utc>` object.
pub fn to_datetag(date: NaiveDateTime) -> String {
    let base_time = base_time();
    let year = date.year() - base_time.year();

    let month = date.month();
    let mut week = date.iso_week().week();

    if month == 1 && week > 51 {
        week = 0;
    }
    if month == 12 && week < 2 {
        let previous_week_date = date - Duration::weeks(1);
        week = previous_week_date.iso_week().week() + 1;
    }

    let day = date.weekday().num_days_from_monday() as i64; // Monday = 0
    let eve_est = date - Duration::days(day);

    let week_eve = NaiveDate::from_ymd_opt(eve_est.year(), eve_est.month(), eve_est.day())
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let delta = date - week_eve;
    let tick = (delta.num_seconds() / WEEK_TICK) as i32;

    format!(
        "{}{}{}",
        base_x(year, BASE54),
        base_x(week as i32, BASE54),
        base_x(tick, BASE54),
    )
}
