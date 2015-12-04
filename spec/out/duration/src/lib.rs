#![allow(dead_code, unused_imports)]

extern crate libc;
extern crate ffi_buf;
extern crate va_list;

use std::ops::{Add, Neg, Sub, Deref};
use std::convert::Into;
use calculations::{Unit, Part, CalculateTime};

mod calculations;
mod ruby;
mod capi;

enum Sign {
    Plus,
    Minus
}

#[derive(Clone, Eq, PartialEq)]
pub struct Duration {
    value: i64,
    parts: Vec<Part>
}

impl Duration {
    pub fn new(value: i64, parts: Vec<Part>) -> Duration {
        Duration { value: value, parts: parts }
    }

    fn sum<T: CalculateTime>(&self, calculate: T) -> T {
        self.parts.iter().fold(calculate, |c, part| c.advance(*part))
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        let v = self.parts.iter()
            .chain(other.parts.iter())
            .map(|p| p.clone())
            .collect();

        Duration::new(self.value + other.value, v)
    }
}

impl Add<i64> for Duration {
    type Output = Duration;

    fn add(self, other: i64) -> Duration {
        let mut parts = self.parts.clone();
        parts.push(Part { amount: other, unit: Unit::Seconds });
        Duration::new(self.value + other, parts)
    }
}

impl Neg for Duration {
    type Output = Duration;

    fn neg(self) -> Duration {
        let parts = self.parts.iter().map(|p| Part { amount: -p.amount, unit: p.unit }).collect();
        Duration::new(-self.value, parts)
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        self + -other
    }
}

impl Sub<i64> for Duration {
    type Output = Duration;

    fn sub(self, other: i64) -> Duration {
        self + -other
    }
}

impl Into<i64> for Duration {
    fn into(self) -> i64 {
        self.value
    }
}

pub use capi::*;
pub use ruby::{RubyObject};
