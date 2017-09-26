mod string;
mod unit;
mod bool;
mod integers;
mod float;
mod option;
mod result;

use sys::{VALUE};
use super::{Error, ToError};
use std::marker::{PhantomData, Sized};

pub struct CheckedValue<T> {
    pub inner: VALUE,
    marker: PhantomData<T>,
}

impl<T> CheckedValue<T> {
    pub unsafe fn new(inner: VALUE) -> CheckedValue<T> {
        CheckedValue { inner: inner, marker: PhantomData }
    }
}

pub type CheckResult<T> = Result<CheckedValue<T>, Error>;

pub trait FromRuby : Sized {
    fn from_ruby(value: VALUE) -> CheckResult<Self>;
}

pub trait ToRust<T> {
    fn to_rust(self) -> T;
}

pub type ToRubyResult = Result<VALUE, Error>;

pub trait ToRuby {
    fn to_ruby(self) -> ToRubyResult;
}

impl ToRuby for VALUE {
    fn to_ruby(self) -> ToRubyResult {
        Ok(self)
    }
}
