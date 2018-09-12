use sys::{self, VALUE, Qtrue, Qfalse};
use super::{FromRuby, CheckResult, ToRuby, ToRubyResult};

impl FromRuby for bool {
    type Checked = bool;

    fn from_ruby(value: VALUE) -> CheckResult<bool> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_TRUE) > 0 } {
            Ok(true)
        } else if unsafe { sys::RB_TYPE_P(value, sys::T_FALSE) > 0 } {
            Ok(false)
        } else {
            type_error!(value, "a boolean")
        }
    }

    fn from_checked(checked: bool) -> bool {
        checked
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
