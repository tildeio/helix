mod slice;
mod string;
mod unit;
mod bool;
mod integers;
mod float;
mod option;

use ruby;
use sys::{VALUE};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CallFrame<'a> {
    lifetime: &'a ()
}

impl<'a> CallFrame<'a> {
    pub unsafe fn new<'lt>(lifetime: &'lt ()) -> CallFrame<'lt> {
        CallFrame { lifetime }
    }
}

pub struct CheckedValue<'a, T> {
    pub inner: ruby::Value<'a>,
    target: PhantomData<T>
}

impl<'a, T> CheckedValue<'a, T> {
    // This is unsafe because it's the primary way that the coercion
    // protocol asserts that subsequent operations are safe
    pub unsafe fn new<'lt>(inner: ruby::Value<'lt>) -> CheckedValue<'lt, T> {
        CheckedValue { inner, target: PhantomData }
    }

    pub unsafe fn from_value<'lt>(inner: VALUE, frame: CallFrame<'lt>) -> CheckedValue<'lt, T> {
        CheckedValue { inner: ruby::Value::new(inner, frame), target: PhantomData }
    }
}

pub type CheckResult<CHECKED> = Result<CHECKED, String>;

pub trait UncheckedValue<TARGET> {
    type ToRust: ToRust<TARGET>;

    fn to_checked(self) -> CheckResult<Self::ToRust>;
}

pub trait ToRust<T> {
    fn to_rust(self) -> T;
}

pub trait ToRuby {
    fn to_ruby(self) -> VALUE;
}

impl ToRuby for VALUE {
    fn to_ruby(self) -> VALUE {
        self
    }
}
