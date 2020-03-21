extern crate chrono;
use chrono::prelude::*;

pub fn formatting_and_parsing() {
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

}
