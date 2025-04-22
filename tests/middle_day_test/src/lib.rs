use chrono::Weekday;
use middle_day::*;

#[test]
fn leap_years() {
    assert!(middle_day(1892).is_none(), "1892 was a leap year!");
    assert!(middle_day(1904).is_none(), "1904 was a leap year!");
    assert!(middle_day(2012).is_none(), "2012 was a leap year!");
}

#[test]
fn weekdays() {
    assert_eq!(Weekday::Tue, middle_day(2019).unwrap());
    assert_eq!(Weekday::Wed, middle_day(1997).unwrap());
    assert_eq!(Weekday::Mon, middle_day(1663).unwrap());
    assert_eq!(Weekday::Wed, middle_day(1873).unwrap());
    assert_eq!(Weekday::Thu, middle_day(1953).unwrap());
    assert_eq!(Weekday::Wed, middle_day(1879).unwrap());
}
