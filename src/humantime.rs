use std::fmt;
use std::convert::From;
use std::cmp::max;
use chrono::{DateTime, Duration, TimeZone, UTC};
use super::Humanize;

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

impl TimePeriod {
    fn to_string_precise(&self) -> String {
        use self::TimePeriod::*;
        match *self {
            Now => String::from("now"),
            Seconds(n) => format!("{} seconds", n),
            Minutes(1) => String::from("1 minute"),
            Minutes(n) => format!("{} minutes", n),
            Hours(1) => String::from("1 hour"),
            Hours(n) => format!("{} hours", n),
            Days(1) => String::from("1 day"),
            Days(n) => format!("{} days", n),
            Weeks(1) => String::from("1 week"),
            Weeks(n) => format!("{} weeks", n),
            Months(1) => String::from("1 month"),
            Months(n) => format!("{} months", n),
            Years(1) => String::from("1 year"),
            Years(n) => format!("{} years", n),
            Eternity => String::from("eternity"),
        }
    }

    fn to_string_rough(&self) -> String {
        use self::TimePeriod::*;
        match *self {
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

    pub fn to_string(&self, precise: bool) -> String {
        if precise {
            self.to_string_precise()
        } else {
            self.to_string_rough()
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
pub struct HumanTime(Duration);

impl HumanTime {
    fn humanize(&self, precise: bool) -> (Vec<TimePeriod>, Tense) {
        use std::i64::{MIN, MAX};

        let tense = match self.0.num_seconds() {
            -10...10 if !precise => Tense::Present,
            MIN...-1 => Tense::Past,
            1...MAX => Tense::Future,
            _ => Tense::Present,
        };

        let periods = if precise {
            HumanTime::precise_period(self.0)
        } else {
            vec![HumanTime::rough_period(self.0)]
        };
        (periods, tense)
    }

    fn rough_period(duration: Duration) -> TimePeriod {
        use self::TimePeriod::*;
        // use std::i64::MAX;

        match duration.num_seconds().abs() {
            n if n > 47260800 => Years(max(n / 31536000, 2)),
            n if n > 29808000 => Years(1),
            n if n > 3888000 => Months(max(n / 2592000, 2)),
            n if n > 2505600 => Months(1),
            n if n > 907200 => Weeks(max(n / 604800, 2)),
            n if n > 561600 => Weeks(1),
            n if n > 129600 => Days(max(n / 86400, 2)),
            n if n > 79200 => Days(1),
            n if n > 5400 => Hours(max(n / 3600, 2)),
            n if n > 2700 => Hours(1),
            n if n > 90 => Minutes(max(n / 60, 2)),
            n if n > 45 => Minutes(1),
            n if n > 10 => Seconds(n),
            0...10 => Now,
            _ => Eternity,
        }

        // match duration.num_seconds().abs() {
        //     0...10 => Now,
        //     n @ 11...44 => Seconds(n),
        //     45...90 => Minutes(1),
        //     n @ 91...2700 => Minutes(max(n / 60, 2)),
        //     2700...5400 => Hours(1),
        //     n @ 5400...79200 => Hours(max(n / 3600, 2)),
        //     79200...129600 => Days(1),
        //     n @ 129600...561600 => Days(max(n / 86400, 2)),
        //     561600...907200 => Weeks(1),
        //     n @ 907200...2505600 => Weeks(max(n / 604800, 2)),
        //     2505600...3888000 => Months(1),
        //     n @ 3888000...29808000 => Months(max(n / 2592000, 2)),
        //     29808000...47260800 => Years(1),
        //     n @ 47260800...MAX => Years(max(n / 31536000, 2)),
        //     _ => Eternity,
        // }
    }

    fn precise_period(duration: Duration) -> Vec<TimePeriod> {
        use self::TimePeriod::*;

        let year = Duration::days(365).num_seconds();
        let month = Duration::days(30).num_seconds();
        let week = Duration::weeks(1).num_seconds();
        let day = Duration::days(1).num_seconds();
        let hour = Duration::hours(1).num_seconds();
        let minute = Duration::minutes(1).num_seconds();
        let zero = Duration::zero().num_seconds();

        let mut duration = duration.num_seconds().abs();
        let mut periods = Vec::<TimePeriod>::new();

        if duration >= year {
            periods.push(Years(duration / year));
            duration %= year;
        }

        if duration >= month {
            periods.push(Months(duration / month));
            duration %= month;
        }

        if duration >= week {
            periods.push(Weeks(duration / week));
            duration %= week;
        }

        if duration >= day {
            periods.push(Days(duration / day));
            duration %= day;
        }

        if duration >= hour {
            periods.push(Hours(duration / hour));
            duration %= hour;
        }

        if duration >= minute {
            periods.push(Minutes(duration / minute));
            duration %= minute;
        }

        if duration > zero || periods.is_empty() {
            periods.push(Seconds(duration));
        }

        periods
    }

    fn locale_en(&self, precise: bool) -> String {
        let (mut periods, tense) = self.humanize(precise);
        let mut time = periods.remove(0).to_string(precise);
        let last = periods.pop().map(|p| p.to_string(precise));
        for period in periods {
            time = time + ", " + &period.to_string(precise);
        }

        if let Some(last) = last {
            time = time + " and " + &last;
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

impl From<Duration> for HumanTime {
    fn from(duration: Duration) -> Self {
        HumanTime(duration)
    }
}

impl<TZ> From<DateTime<TZ>> for HumanTime
    where TZ: TimeZone
{
    fn from(dt: DateTime<TZ>) -> Self {
        HumanTime::from(dt.with_timezone(&UTC) - UTC::now())
    }
}

impl Humanize for Duration {
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(self.clone()))
    }
}

impl<TZ> Humanize for DateTime<TZ> where TZ: TimeZone {
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use test::Bencher;

    #[bench]
    fn one_week(b: &mut Bencher) {
        let ht = HumanTime::from(Duration::weeks(1));
        b.iter(|| format!("{}", ht));
    }

    #[bench]
    fn two_weeks(b: &mut Bencher) {
        let ht = HumanTime::from(Duration::weeks(2));
        b.iter(|| format!("{}", ht));
    }
}
