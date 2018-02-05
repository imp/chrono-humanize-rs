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
//! For full control over the text representation use `HumanTime::to_text_en()`
//!
//! ```
//! extern crate chrono;
//! extern crate chrono_humanize;
//!
//! use chrono::Duration;
//! use chrono_humanize::{Accuracy, HumanTime, Tense};
//!
//! # fn main() {
//! let dt = Duration::days(45);
//! let ht = HumanTime::from(dt);
//!
//! assert_eq!("a month", ht.to_text_en(Accuracy::Rough, Tense::Present));
//! assert_eq!("1 month, 2 weeks and 1 day", ht.to_text_en(Accuracy::Precise, Tense::Present));
//! # }
//! ```

#![cfg_attr(all(feature = "cargo-clippy", feature = "pedantic"), warn(clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", warn(use_self))]
#![deny(warnings, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/chrono-humanize/0.0.11")]

extern crate chrono;

pub use humantime::{Accuracy, HumanTime, Tense};

mod humantime;

/// Present the object in human friendly text form
pub trait Humanize {
    /// Emits `String` that represents current object in human friendly form
    fn humanize(&self) -> String;
}
