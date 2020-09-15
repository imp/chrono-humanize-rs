use std::borrow::Cow;
use std::cmp::max;
use std::fmt;
use std::time::SystemTime;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::Humanize;

/// Indicates the time of the period in relation to the time of the utterance
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tense {
    Past,
    Present,
    Future,
}

/// The accuracy of the representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accuracy {
    /// Rough approximation, easy to grasp, but not necessarily accurate
    Rough,
    /// Concise expression, accurate, but not necessarily easy to grasp
    Precise,
}

impl Accuracy {
    /// Returns whether this accuracy is precise
    #[must_use]
    pub fn is_precise(self) -> bool {
        self == Self::Precise
    }

    /// Returns whether this accuracy is rough
    #[must_use]
    pub fn is_rough(self) -> bool {
        self == Self::Rough
    }
}

impl From<bool> for Accuracy {
    fn from(precise: bool) -> Self {
        if precise {
            Self::Precise
        } else {
            Self::Rough
        }
    }
}

// Number of seconds in various time periods
const MINUTE: i64 = 60;
const HOUR: i64 = MINUTE * 60;
const DAY: i64 = HOUR * 24;
const WEEK: i64 = DAY * 7;
const MONTH: i64 = DAY * 30;
const YEAR: i64 = DAY * 365;

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
    fn to_text_precise(&self) -> Cow<'static, str> {
        match *self {
            Self::Now => "now".into(),
            Self::Seconds(1) => "1 second".into(),
            Self::Seconds(n) => format!("{} seconds", n).into(),
            Self::Minutes(1) => "1 minute".into(),
            Self::Minutes(n) => format!("{} minutes", n).into(),
            Self::Hours(1) => "1 hour".into(),
            Self::Hours(n) => format!("{} hours", n).into(),
            Self::Days(1) => "1 day".into(),
            Self::Days(n) => format!("{} days", n).into(),
            Self::Weeks(1) => "1 week".into(),
            Self::Weeks(n) => format!("{} weeks", n).into(),
            Self::Months(1) => "1 month".into(),
            Self::Months(n) => format!("{} months", n).into(),
            Self::Years(1) => "1 year".into(),
            Self::Years(n) => format!("{} years", n).into(),
            Self::Eternity => "eternity".into(),
        }
    }

    fn to_text_rough(&self) -> Cow<'static, str> {
        match *self {
            Self::Now => "now".into(),
            Self::Seconds(n) => format!("{} seconds", n).into(),
            Self::Minutes(1) => "a minute".into(),
            Self::Minutes(n) => format!("{} minutes", n).into(),
            Self::Hours(1) => "an hour".into(),
            Self::Hours(n) => format!("{} hours", n).into(),
            Self::Days(1) => "a day".into(),
            Self::Days(n) => format!("{} days", n).into(),
            Self::Weeks(1) => "a week".into(),
            Self::Weeks(n) => format!("{} weeks", n).into(),
            Self::Months(1) => "a month".into(),
            Self::Months(n) => format!("{} months", n).into(),
            Self::Years(1) => "a year".into(),
            Self::Years(n) => format!("{} years", n).into(),
            Self::Eternity => "eternity".into(),
        }
    }

    fn to_text(&self, accuracy: Accuracy) -> Cow<'static, str> {
        match accuracy {
            Accuracy::Rough => self.to_text_rough(),
            Accuracy::Precise => self.to_text_precise(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HumanTime(Duration);

impl HumanTime {
    #[must_use]
    pub fn to_text_en(&self, accuracy: Accuracy, tense: Tense) -> String {
        let mut periods = match accuracy {
            Accuracy::Rough => self.rough_period(),
            Accuracy::Precise => self.precise_period(),
        };

        let first = periods.remove(0).to_text(accuracy);
        let last = periods.pop().map(|last| last.to_text(accuracy));

        // let append = |acc, p| format!("{}, {}", acc, p.to_text(accuracy));
        let mut text = periods.into_iter().fold(first, |acc, p| {
            format!("{}, {}", acc, p.to_text(accuracy)).into()
        });

        if let Some(last) = last {
            text = format!("{} and {}", text, last).into();
        }

        match tense {
            Tense::Past => format!("{} ago", text),
            Tense::Future => format!("in {}", text),
            Tense::Present => text.into_owned(),
        }
    }

    fn tense(&self, accuracy: Accuracy) -> Tense {
        match self.0.num_seconds() {
            -10..=10 if accuracy.is_rough() => Tense::Present,
            seconds if seconds.is_negative() => Tense::Past,
            seconds if seconds.is_positive() => Tense::Future,
            _ => Tense::Present,
        }
    }

    fn rough_period(&self) -> Vec<TimePeriod> {
        let period = match self.0.num_seconds().abs() {
            n if n > 547 * DAY => TimePeriod::Years(max(n / YEAR, 2)),
            n if n > 345 * DAY => TimePeriod::Years(1),
            n if n > 45 * DAY => TimePeriod::Months(max(n / MONTH, 2)),
            n if n > 29 * DAY => TimePeriod::Months(1),
            n if n > 10 * DAY + 12 * HOUR => TimePeriod::Weeks(max(n / WEEK, 2)),
            n if n > 6 * DAY + 12 * HOUR => TimePeriod::Weeks(1),
            n if n > 36 * HOUR => TimePeriod::Days(max(n / DAY, 2)),
            n if n > 22 * HOUR => TimePeriod::Days(1),
            n if n > 90 * MINUTE => TimePeriod::Hours(max(n / HOUR, 2)),
            n if n > 45 * MINUTE => TimePeriod::Hours(1),
            n if n > 90 => TimePeriod::Minutes(max(n / MINUTE, 2)),
            n if n > 45 => TimePeriod::Minutes(1),
            n if n > 10 => TimePeriod::Seconds(n),
            0..=10 => TimePeriod::Now,
            _ => TimePeriod::Eternity,
        };

        vec![period]
    }

    fn precise_period(&self) -> Vec<TimePeriod> {
        let zero = Duration::zero().num_seconds();

        let mut duration = self.0.num_seconds().abs();
        let mut periods = Vec::<TimePeriod>::new();

        if duration >= YEAR {
            periods.push(TimePeriod::Years(duration / YEAR));
            duration %= YEAR;
        }

        if duration >= MONTH {
            periods.push(TimePeriod::Months(duration / MONTH));
            duration %= MONTH;
        }

        if duration >= WEEK {
            periods.push(TimePeriod::Weeks(duration / WEEK));
            duration %= WEEK;
        }

        if duration >= DAY {
            periods.push(TimePeriod::Days(duration / DAY));
            duration %= DAY;
        }

        if duration >= HOUR {
            periods.push(TimePeriod::Hours(duration / HOUR));
            duration %= HOUR;
        }

        if duration >= MINUTE {
            periods.push(TimePeriod::Minutes(duration / MINUTE));
            duration %= MINUTE;
        }

        if duration > zero || periods.is_empty() {
            periods.push(TimePeriod::Seconds(duration));
        }

        periods
    }

    fn locale_en(&self, accuracy: Accuracy) -> String {
        let tense = self.tense(accuracy);
        self.to_text_en(accuracy, tense)
    }
}

impl fmt::Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accuracy = f.alternate().into();
        f.pad(&self.locale_en(accuracy))
    }
}

impl From<Duration> for HumanTime {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl<TZ> From<DateTime<TZ>> for HumanTime
where
    TZ: TimeZone,
{
    fn from(dt: DateTime<TZ>) -> Self {
        dt.signed_duration_since(Utc::now()).into()
    }
}

impl From<SystemTime> for HumanTime {
    fn from(st: SystemTime) -> Self {
        DateTime::<Utc>::from(st).into()
    }
}

impl Humanize for Duration {
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(*self))
    }
}

impl<TZ> Humanize for DateTime<TZ>
where
    TZ: TimeZone,
{
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(self.clone()))
    }
}

impl Humanize for SystemTime {
    fn humanize(&self) -> String {
        HumanTime::from(*self).to_string()
    }
}
