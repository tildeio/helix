use sys::{self, VALUE, T_FLOAT, T_FIXNUM, T_BIGNUM};
use libc::c_double;
use super::{FromRuby, CheckResult, CheckedValue, ToRuby, ToRubyResult};

impl FromRuby for f64 {
    type Checked = CheckedValue<f64>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<f64>> {
        if unsafe { sys::RB_TYPE_P(value, T_FLOAT) > 0 || sys::RB_TYPE_P(value, T_FIXNUM) > 0 || sys::RB_TYPE_P(value, T_BIGNUM) > 0 } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 64-bit float")
        }
    }

    fn from_checked(checked: CheckedValue<f64>) -> f64 {
        unsafe { sys::NUM2DBL(checked.to_value()) as f64 }
    }
}

impl ToRuby for f64 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::DBL2NUM(self as c_double) })
    }
}
