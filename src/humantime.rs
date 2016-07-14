use std::fmt;
use std::convert::From;
use std::cmp::max;
use chrono;

#[derive(Debug)]
enum TimePeriod {
    Now,
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Weeks(i64),
    Months(i64),
    Years(i64),
    Eternity,
}

impl From<TimePeriod> for String {
    fn from(period: TimePeriod) -> String {
        use self::TimePeriod::*;
        match period {
            Now => String::from("now"),
            Seconds(n) => format!("{} seconds", n),
            Minutes(1) => String::from("a minute"),
            Minutes(n) => format!("{} minutes", n),
            Hours(1) => String::from("an hour"),
            Hours(n) => format!("{} hours", n),
            Days(1) => String::from("a day"),
            Days(n) => format!("{} days", n),
            Weeks(1) => String::from("a week"),
            Weeks(n) => format!("{} weeks", n),
            Months(1) => String::from("a month"),
            Months(n) => format!("{} months", n),
            Years(1) => String::from("a year"),
            Years(n) => format!("{} years", n),
            Eternity => String::from("eternity"),
        }
    }
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
    fn humanize(&self, precise: bool) -> (Vec<TimePeriod>, Tense) {
        // use std::i64::{MIN, MAX};
        //
        // let tense = match self.0.num_seconds() {
        //     MIN...-10 => Tense::Past,
        //     10...MAX => Tense::Future,
        //     _ => Tense::Present,
        // };
        let tense = if self.0 < chrono::Duration::seconds(-10) {
            Tense::Past
        } else if self.0 < chrono::Duration::seconds(10) {
            Tense::Present
        } else {
            Tense::Future
        };

        if precise {
            (vec![], tense)
        } else {
            let period = HumanTime::rough_period(self.0);
            (vec![period], tense)
        }
    }

    fn rough_period(duration: chrono::Duration) -> TimePeriod {
        use self::TimePeriod::*;
        use std::i64::MAX;

        match duration.num_seconds().abs() {
            0...10 => Now,
            n @ 11...44 => Seconds(n),
            45...90 => Minutes(1),
            n @ 91...2700 => Minutes(max(n / 60, 2)),
            2700...5400 => Hours(1),
            n @ 5400...79200 => Hours(max(n / 3600, 2)),
            79200...129600 => Days(1),
            n @ 129600...561600 => Days(max(n / 86400, 2)),
            561600...907200 => Weeks(1),
            n @ 907200...2505600 => Weeks(max(n / 604800, 2)),
            2505600...3888000 => Months(1),
            n @ 3888000...29808000 => Months(max(n / 2592000, 2)),
            29808000...47260800 => Years(1),
            n @ 47260800...MAX => Years(max(n / 31536000, 2)),
            _ => Eternity,
        }
    }

    fn locale_en(&self, precise: bool) -> String {
        let (periods, tense) = self.humanize(precise);
        let mut time = String::new();
        for period in periods {
            time = time + &String::from(period)
        }

        match tense {
            Tense::Past => format!("{} ago", time),
            Tense::Future => format!("in {}", time),
            Tense::Present => time,
        }
    }
}

impl fmt::Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let precise = f.alternate();
        f.pad(&self.locale_en(precise))
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

// impl<T> From<T> for HumanTime
//     where T: chrono::Datelike + chrono::Timelike
// {
//     fn from(dt: T) -> Self {
//         HumanTime::from(chrono::UTC::now() - dt.with_timezone(&chrono::UTC))
//     }
// }
