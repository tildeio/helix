use sys::{self, VALUE, Qtrue, Qfalse};
use super::{UncheckedValue, FromRuby, CheckResult, CheckedValue, ToRust, ToRuby, ToRubyResult};

impl UncheckedValue<bool> for VALUE {
    fn to_checked(self) -> CheckResult<bool> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_TRUE) || sys::RB_TYPE_P(self, sys::T_FALSE) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            type_error!(self, "a boolean")
        }
    }
}

impl FromRuby for bool {
    fn from_ruby(value: VALUE) -> CheckResult<bool> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_TRUE) || sys::RB_TYPE_P(value, sys::T_FALSE) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a boolean")
        }
    }
}

impl ToRust<bool> for CheckedValue<bool> {
    fn to_rust(self) -> bool {
        self.inner == unsafe { Qtrue }
    }
}

impl ToRuby for bool {
    fn to_ruby(self) -> ToRubyResult {
        if self {
            Ok(unsafe { Qtrue })
        } else {
            Ok(unsafe { Qfalse })
        }
    }
}
