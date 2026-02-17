use chrono::{TimeZone, Utc};

#[test]
fn test_date() {
    let start_date = Utc.with_ymd_and_hms(2011, 4, 25, 0, 0, 0).unwrap();

    assert_eq!(
        gigasecond::after(start_date),
        Utc.with_ymd_and_hms(2043, 1, 1, 1, 46, 40).unwrap()
    );
}

#[test]
fn test_another_date() {
    let start_date = Utc.with_ymd_and_hms(1977, 6, 13, 0, 0, 0).unwrap();

    assert_eq!(
        gigasecond::after(start_date),
        Utc.with_ymd_and_hms(2009, 2, 19, 1, 46, 40).unwrap()
    );
}

#[test]
fn test_third_date() {
    let start_date = Utc.with_ymd_and_hms(1959, 7, 19, 0, 0, 0).unwrap();

    assert_eq!(
        gigasecond::after(start_date),
        Utc.with_ymd_and_hms(1991, 3, 27, 1, 46, 40).unwrap()
    );
}

#[test]
fn test_datetime() {
    let start_date = Utc.with_ymd_and_hms(2015, 1, 24, 22, 0, 0).unwrap();

    assert_eq!(
        gigasecond::after(start_date),
        Utc.with_ymd_and_hms(2046, 10, 2, 23, 46, 40).unwrap()
    );
}

#[test]
fn test_another_datetime() {
    let start_date = Utc.with_ymd_and_hms(2015, 1, 24, 23, 59, 59).unwrap();

    assert_eq!(
        gigasecond::after(start_date),
        Utc.with_ymd_and_hms(2046, 10, 3, 1, 46, 39).unwrap()
    );
}
