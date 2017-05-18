use sys::{self, VALUE, Qtrue, Qfalse};

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<bool> for VALUE {
    fn to_checked(self) -> CheckResult<bool> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_TRUE) || sys::RB_TYPE_P(self, sys::T_FALSE) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            Err(::invalid(self, "true or false"))
        }
    }
}

impl ToRust<bool> for CheckedValue<bool> {
    fn to_rust(self) -> bool {
        self.inner == unsafe { Qtrue }
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
