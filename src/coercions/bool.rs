use sys::{self, VALUE, Qtrue, Qfalse};
use coercions::*;
use ruby::Value;
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl<'a> UncheckedValue<bool> for Value<'a> {
    type ToRust = CheckedValue<'a, bool>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), sys::T_TRUE) || sys::RB_TYPE_P(self.inner(), sys::T_FALSE) } {
            Ok(unsafe { CheckedValue::<'a, bool>::new(self) })
        } else {
            Err(format!("No implicit conversion of {} into Rust bool", "?"))
        }
    }
}

impl<'a> ToRust<bool> for CheckedValue<'a, bool> {
    fn to_rust(self) -> bool {
        unsafe { self.inner.inner() == Qtrue }
    }
}

impl ToRuby for bool {
    fn to_ruby(self) -> VALUE {
        if self {
            unsafe { Qtrue }
        } else {
            unsafe { Qfalse }
        }
    }
}
