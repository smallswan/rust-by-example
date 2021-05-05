extern crate chrono;
use chrono::format::ParseError;
use chrono::prelude::*;

#[test]
fn formatting_and_parsing() -> Result<(), ParseError> {
    // 1. 格式化输出日期
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2014-11-28 12:00:09"
    );
    assert_eq!(
        dt.format("%a %b %e %T %Y").to_string(),
        "Fri Nov 28 12:00:09 2014"
    );
    assert_eq!(
        dt.format("%a %b %e %T %Y").to_string(),
        dt.format("%c").to_string()
    );

    assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
    assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
    assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
    assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");

    // Note that milli/nanoseconds are only printed if they are non-zero
    let dt_nano = Utc.ymd(2014, 11, 28).and_hms_nano(12, 0, 9, 1);
    assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");

    println!("Utc timestamp:{}", Utc::now().timestamp());
    println!("Utc timestamp_millis:{}", Utc::now().timestamp_millis());
    println!("Utc timestamp_nanos:{}", Utc::now().timestamp_nanos());

    println!(
        "timestamp:{}",
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let local: DateTime<Local> = Local::now();

    println!("local:{}", local);
    println!("local timestamp_millis:{}", local.timestamp_millis());

    //
    let this_year = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    println!("{:?}", this_year.timestamp_millis());

    let birthday = Utc.ymd(1990, 6, 29).and_hms(21, 0, 0);

    let age = Utc::now().signed_duration_since(birthday);

    println!("{:?}", age);

    println!("{}", Local::now().to_rfc3339());

    // 2. &str -> DateTime
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);
    Ok(())
}

use chrono::{DateTime, Duration, FixedOffset, Local, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

#[test]
fn cal() {
    let now = Utc::now();
    println!("{}", now);

    // 1. 日期计算
    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }

    // 2. Local <--> Utc
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );
}
