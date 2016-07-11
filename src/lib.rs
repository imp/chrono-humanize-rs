extern crate chrono;

use std::fmt;
use std::convert::From;
use std::cmp::max;

pub trait Humanize {
    fn humanize(&self) -> String;
}

#[derive(Debug)]
enum TimePeriod {
    Now,
    Seconds(i64),
    Minute,
    Minutes(i64),
    Hour,
    Hours(i64),
    Day,
    Days(i64),
    Month,
    Months(i64),
    Year,
    Years(i64),
    Eternity,
}

#[derive(Debug)]
enum Tense {
    Past,
    Present,
    Future,
}

#[derive(Debug)]
pub struct HumanTime {
    period: TimePeriod,
    tense: Tense,
}

impl HumanTime {
    fn locale_en(&self) -> String {
        use TimePeriod::*;

        let time = match self.period {
            Now => String::from("now"),
            Seconds(n) => format!("{} seconds", n),
            Minute => String::from("a minute"),
            Minutes(n) => format!("{} minutes", n),
            Hour => String::from("an hour"),
            Hours(n) => format!("{} hours", n),
            Day => String::from("a day"),
            Days(n) => format!("{} days", n),
            Month => String::from("a month"),
            Months(n) => format!("{} months", n),
            Year => String::from("a year"),
            Years(n) => format!("{} years", n),
            Eternity => String::from("eternity"),
        };

        match self.tense {
            Tense::Past => format!("{} ago", time),
            Tense::Future => format!("in {}", time),
            Tense::Present => time,
        }
    }
}

impl fmt::Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&self.locale_en())
    }
}

impl<TZ> From<chrono::DateTime<TZ>> for HumanTime
    where TZ: chrono::TimeZone
{
    fn from(ts: chrono::DateTime<TZ>) -> Self {
        use TimePeriod::*;
        use std::i64::{MIN, MAX};

        let now = chrono::UTC::now().timestamp();
        let diff = now - ts.timestamp();
        let tense = match diff {
            MIN...-10 => Tense::Future,
            10...MAX => Tense::Past,
            _ => Tense::Present,
        };

        let diff = match diff.abs() {
            0...10 => Now,
            n @ 11...44 => Seconds(n),
            45...90 => Minute,
            n @ 91...2700 => Minutes(max(n / 60, 2)),
            2700...5400 => Hour,
            n @ 5400...79200 => Hours(max(n / 3600, 2)),
            79200...129600 => Day,
            n @ 129600...2160000 => Days(max(n / 86400, 2)),
            2160000...3888000 => Month,
            n @ 3888000...29808000 => Months(max(n / 2592000, 2)),
            29808000...47260800 => Year,
            n @ 47260800...MAX => Years(max(n / 31536000, 2)),
            _ => Eternity,
        };

        HumanTime {
            period: diff,
            tense: tense,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, UTC};
    use super::*;

    #[test]
    fn now() {
        let ht = HumanTime::from(UTC::now());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn plus_5s() {
        let ht = HumanTime::from(UTC::now() + Duration::seconds(5));
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn minus_5s() {
        let ht = HumanTime::from(UTC::now() - Duration::seconds(5));
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn plus_15s() {
        let ht = HumanTime::from(UTC::now() + Duration::seconds(15));
        let english = format!("{}", ht);
        assert_eq!("in 15 seconds", english);
    }

    #[test]
    fn minus_15s() {
        let ht = HumanTime::from(UTC::now() - Duration::seconds(15));
        let english = format!("{}", ht);
        assert_eq!("15 seconds ago", english);
    }

    #[test]
    fn plus_95s() {
        let ht = HumanTime::from(UTC::now() + Duration::seconds(95));
        let english = format!("{}", ht);
        assert_eq!("in 2 minutes", english);
    }

    #[test]
    fn minus_95s() {
        let ht = HumanTime::from(UTC::now() - Duration::seconds(95));
        let english = format!("{}", ht);
        assert_eq!("2 minutes ago", english);
    }

    #[test]
    fn plus_125s() {
        let ht = HumanTime::from(UTC::now() + Duration::seconds(125));
        let english = format!("{}", ht);
        assert_eq!("in 2 minutes", english);
    }

    #[test]
    fn minus_125s() {
        let ht = HumanTime::from(UTC::now() - Duration::seconds(125));
        let english = format!("{}", ht);
        assert_eq!("2 minutes ago", english);
    }

    #[test]
    fn plus_31m() {
        let ht = HumanTime::from(UTC::now() + Duration::minutes(31));
        let english = format!("{}", ht);
        assert_eq!("in 31 minutes", english);
    }

    #[test]
    fn minus_31m() {
        let ht = HumanTime::from(UTC::now() - Duration::minutes(31));
        let english = format!("{}", ht);
        assert_eq!("31 minutes ago", english);
    }

    #[test]
    fn plus_45m() {
        let ht = HumanTime::from(UTC::now() + Duration::minutes(45));
        let english = format!("{}", ht);
        assert_eq!("in 45 minutes", english);
    }

    #[test]
    fn minus_45m() {
        let ht = HumanTime::from(UTC::now() - Duration::minutes(45));
        let english = format!("{}", ht);
        assert_eq!("45 minutes ago", english);
    }

    #[test]
    fn plus_46m() {
        let ht = HumanTime::from(UTC::now() + Duration::minutes(46));
        let english = format!("{}", ht);
        assert_eq!("in an hour", english);
    }

    #[test]
    fn minus_46m() {
        let ht = HumanTime::from(UTC::now() - Duration::minutes(46));
        let english = format!("{}", ht);
        assert_eq!("an hour ago", english);
    }

    #[test]
    fn plus_1h() {
        let ht = HumanTime::from(UTC::now() + Duration::hours(1));
        let english = format!("{}", ht);
        assert_eq!("in an hour", english);
    }

    #[test]
    fn minus_1h() {
        let ht = HumanTime::from(UTC::now() - Duration::hours(1));
        let english = format!("{}", ht);
        assert_eq!("an hour ago", english);
    }

    #[test]
    fn plus_12h() {
        let ht = HumanTime::from(UTC::now() + Duration::hours(12));
        let english = format!("{}", ht);
        assert_eq!("in 12 hours", english);
    }

    #[test]
    fn minus_12h() {
        let ht = HumanTime::from(UTC::now() - Duration::hours(12));
        let english = format!("{}", ht);
        assert_eq!("12 hours ago", english);
    }

    #[test]
    fn plus_23h() {
        let ht = HumanTime::from(UTC::now() + Duration::hours(23));
        let english = format!("{}", ht);
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_23h() {
        let ht = HumanTime::from(UTC::now() - Duration::hours(23));
        let english = format!("{}", ht);
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_26h() {
        let ht = HumanTime::from(UTC::now() + Duration::hours(26));
        let english = format!("{}", ht);
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_26h() {
        let ht = HumanTime::from(UTC::now() - Duration::hours(26));
        let english = format!("{}", ht);
        assert_eq!("a day ago", english);
    }
}
