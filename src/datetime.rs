extern crate chrono;
use chrono::format::ParseError;
use chrono::prelude::*;

#[test]
fn formatting_and_parsing() -> Result<(), ParseError> {
    // 1. DateTime -> &str (格式化输出日期)
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
    println!("local:{}", local.format("%Y-%m-%d %H:%M:%S%z").to_string());

    // DateTime<Local> -> UTC

    //
    let this_year = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    println!("{:?}", this_year.timestamp_millis());

    let birthday = Utc.ymd(1990, 6, 29).and_hms(21, 0, 0);

    let age = Utc::now().signed_duration_since(birthday);

    println!("{:?}", age);

    println!("local rfc3339： {}", Local::now().to_rfc3339());

    // 2. &str -> DateTime
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    //RFC 3339遵循ISO 8601 DateTime格式。 唯一的区别是RFC允许我们用“空格”替换“ T”。
    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    //“ Z”：代表零时区（UTC + 0）。 等于RFC 3339中的+00：00。
    let rfc3339 = DateTime::parse_from_rfc3339("2019-10-12T07:20:50.52Z")?;
    println!("{}", rfc3339);

    //北京时间
    let rfc3339 = DateTime::parse_from_rfc3339("2024-01-21T11:22:50.52+08:00")?;
    println!("{}", rfc3339);

    //印度新德里时间
    let rfc3339 = DateTime::parse_from_rfc3339("2024-01-21T11:22:50.52+05:30")?;
    println!("{}", rfc3339);

    //美国东部时间（EST）
    let rfc3339 = DateTime::parse_from_rfc3339("2024-01-21T11:22:50.52-05:00")?;
    println!("{}", rfc3339);

    //太平洋标准时区（PST）
    let rfc3339 = DateTime::parse_from_rfc3339("2024-01-21T11:22:50.52-08:00")?;
    println!("{}", rfc3339);

    // UTC+5:30   印度新德里时间       （东5.5区时间）
    // UTC+8      北京时间             （东八区时间）
    // UTC+9      东京时间             （东九区时间）
    // UTC+10    （东10区时间）
    // UTC-5      东部时间（EST）      （西五区时间）
    // UTC-8      太平洋标准时区（PST）（西八区时间）

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    // 带时区的时间
    assert_eq!(
        NaiveDateTime::parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),
        Ok(NaiveDate::from_ymd_opt(2014, 5, 17)
            .unwrap()
            .and_hms_opt(12, 34, 56)
            .unwrap())
    );

    //1997-12-17 07:37:16-08 2004-05-03T17:30:08
    let iso8601_str = "1997-12-17T07:37:16";
    let moment: NaiveDateTime = iso8601_str.parse().unwrap();
    println!("{}", moment);
    Ok(())
}

use chrono::{DateTime, Duration, FixedOffset, Local, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

#[test]
fn cal() {
    use std::time::Instant;
    let start = Instant::now();
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

    // Local --> Utc
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);

    let new_delhi_timezone = FixedOffset::east(5 * 3600 + 1800);
    let china_timezone = FixedOffset::east(8 * 3600);
    let japan_timezone = FixedOffset::east(9 * 3600);

    let rio_timezone = FixedOffset::west(2 * 3600);
    let est_timezone = FixedOffset::west(5 * 3600);
    let pst_timezone = FixedOffset::west(8 * 3600);

    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);

    // UTC --> Local
    println!(
        "Time in Bei Jing now is {}",
        utc_time.with_timezone(&china_timezone)
    );

    println!(
        "Time in Tokyo  now is {}",
        utc_time.with_timezone(&japan_timezone)
    );

    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );

    println!(
        "Time in New Delhi now is {}",
        utc_time.with_timezone(&new_delhi_timezone)
    );

    println!(
        "Time in EST now is {}",
        utc_time.with_timezone(&est_timezone)
    );

    println!(
        "Time in PST now is {}",
        utc_time.with_timezone(&pst_timezone)
    );

    //计算时间间隔
    let dt = Utc.ymd(1990, 4, 1);
    let how_old_are_your = Utc.ymd(2023, 4, 1);
    println!(
        "I'am {} days(years?) ago 😂",
        how_old_are_your.signed_duration_since(dt).num_days()
    );

    println!("test run {} millis", start.elapsed().as_millis());

    let today = Utc.ymd(2024, 10, 1);
    let founding_date = Utc.ymd(1949, 10, 1);
    println!(
        "建国{}年",
        today.signed_duration_since(founding_date).num_days() / 365
    );
}
