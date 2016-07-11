extern crate chrono;

mod humantime;
pub use humantime::HumanTime;

pub trait Humanize {
    fn humanize(&self) -> String;
}
