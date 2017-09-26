use sys::{self, VALUE, T_FLOAT, T_FIXNUM, T_BIGNUM};
use super::{FromRuby, CheckResult, CheckedValue, ToRust, ToRuby, ToRubyResult};

impl FromRuby for f64 {
    fn from_ruby(value: VALUE) -> CheckResult<f64> {
        if unsafe { sys::RB_TYPE_P(value, T_FLOAT) || sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 64-bit float")
        }
    }
}

impl ToRust<f64> for CheckedValue<f64> {
    fn to_rust(self) -> f64 {
        unsafe { sys::NUM2F64(self.inner) }
    }
}

impl ToRuby for f64 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::F642NUM(self) })
    }
}
