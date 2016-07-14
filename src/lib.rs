//! Representation for chrono objects in human languages
//!
//! # Quick Start
//!
//! `HumanTime` objects are created from chrono objects, such as `chrono::DateTime`
//! and `chrono::Duration`
//!
//! # Examples
//!
//! Convert current time taken as `Local::now()` to `HumanTime`
//!
//! ```
//! extern crate chrono;
//! extern crate chrono_humanize;
//!
//! let dt = chrono::Local::now();
//! let ht = chrono_humanize::HumanTime::from(dt);
//!
//! assert_eq!("now", format!("{}", ht));
//! ```
//!
//!
//! ```
//! extern crate chrono;
//! extern crate chrono_humanize;
//!
//! let dt = chrono::Local::now() - chrono::Duration::minutes(58);
//! let ht = chrono_humanize::HumanTime::from(dt);
//!
//! assert_eq!("an hour ago", format!("{}", ht));
//! ```
//!

extern crate chrono;

mod humantime;
pub use humantime::HumanTime;

pub trait Humanize {
    fn humanize(&self) -> String;
}
