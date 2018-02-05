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

#![cfg_attr(all(feature = "cargo-clippy", feature = "pedantic"), warn(clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", warn(use_self))]
#![deny(warnings, missing_debug_implementations)]

extern crate chrono;

pub use humantime::HumanTime;

mod humantime;

/// Present the object in human friendly text form
pub trait Humanize {
    /// Emits `String` that represents current object in human friendly form
    fn humanize(&self) -> String;
}
