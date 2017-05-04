mod slice;
mod string;
mod unit;
mod bool;
mod integers;
mod float;
mod option;

use sys::{VALUE};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct CallFrame<'a> {
    lifetime: &'a ()
}

impl<'a> CallFrame<'a> {
    pub unsafe fn new<'lt>(lifetime: &'lt ()) -> CallFrame<'lt> {
        CallFrame { lifetime }
    }
}

pub struct CheckedValue<T> {
    pub inner: VALUE,
    target: PhantomData<T>
}

impl<T> CheckedValue<T> {
    // This is unsafe because it's the primary way that the coercion
    // protocol asserts that subsequent operations are safe
    pub unsafe fn new<M>(inner: VALUE) -> CheckedValue<T> {
        CheckedValue { inner, target: PhantomData }
    }
}

pub type CheckResult<CHECKED> = Result<CHECKED, String>;

pub trait UncheckedValue<T> {
    type ToRust: ToRust<T>;

    fn to_checked<'a>(self, frame: CallFrame<'a>) -> CheckResult<Self::ToRust>;
}

pub trait ToRust<T> {
    fn to_rust(self) -> T;
}

pub trait ToRuby {
    fn to_ruby(&self) -> VALUE;
}

impl ToRuby for VALUE {
    fn to_ruby(&self) -> VALUE {
        *self
    }
}
