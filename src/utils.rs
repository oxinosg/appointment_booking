//! Utility functions for the application

use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, Timelike};

/// Return a NaiveDateTime for the next 15 minute mark time from passed local
/// time
///
/// i.e. 18:12 => 18:15
pub fn next_15_mark(date: DateTime<Local>) -> NaiveDateTime {
    let mut minute = (date.minute() / 15) * 15 + 15;
    let mut hour = date.hour();

    if minute >= 60 {
        minute = 0;
        hour += 1;
    }

    // Get the next 15 minute mark time
    date.with_hour(hour)
        .unwrap_or_default()
        .with_minute(minute)
        .unwrap_or_default()
        .with_second(0)
        .unwrap_or_default()
        .with_nanosecond(0)
        .unwrap_or_default()
        .naive_utc()
}

/// Return a NaiveDateTime for the next 15 minute time from the current time
pub fn now_next_15_mark() -> NaiveDateTime {
    // Get the current time
    let now = Local::now();
    next_15_mark(now)
}

/// Return a NaiveDateTime for the end of the day
pub fn end_of_day() -> NaiveDateTime {
    // Get the current time
    let now = Local::now();

    // Get the end of the day
    now.with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
        .naive_utc()
}


/// Return a NaiveDateTime for the end of the day this Friday
pub fn end_of_week() -> NaiveDateTime {
    // Get the current time
    let now = Local::now();

    // Get the end of the day
    let end_of_day = now.with_hour(23).unwrap().with_minute(59).unwrap();

    // Get the number of days until the end of the week
    let end_of_week = end_of_day.weekday().num_days_from_monday();

    // Get the NaiveDateTime for the end of the day this Friday
    let end_of_weekdays = end_of_day + Duration::days(7 - end_of_week as i64);
    end_of_weekdays.naive_utc()
}
