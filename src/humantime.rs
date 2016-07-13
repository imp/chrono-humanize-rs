use std::fmt;
use std::convert::From;
use std::cmp::max;
use chrono;

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
pub struct HumanTime(chrono::Duration);

impl HumanTime {
    fn humanize(&self, presize: bool) -> (Vec<TimePeriod>, Tense) {
        if presize {
            (vec![], Tense::Present)
        } else {
            (vec![], Tense::Present)
        }
    }

    fn period1(duration: chrono::Duration) -> TimePeriod {
        use self::TimePeriod::*;
        use std::i64::MAX;

        match duration.num_seconds().abs() {
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
        }
    }

    fn period(&self) -> (TimePeriod, Tense) {
        use std::i64::{MIN, MAX};

        let tense = match self.0.num_seconds() {
            MIN...-10 => Tense::Past,
            10...MAX => Tense::Future,
            _ => Tense::Present,
        };

        let period = HumanTime::period1(self.0);

        (period, tense)
    }

    fn locale_en(&self) -> String {
        use self::TimePeriod::*;
        let (period, tense) = self.period();
        let time = match period {
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

        match tense {
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

impl From<chrono::Duration> for HumanTime {
    fn from(duration: chrono::Duration) -> Self {
        HumanTime(duration)
    }
}

impl<TZ> From<chrono::DateTime<TZ>> for HumanTime
    where TZ: chrono::TimeZone
{
    fn from(dt: chrono::DateTime<TZ>) -> Self {
        HumanTime::from(dt.with_timezone(&chrono::UTC) - chrono::UTC::now())
    }
}
