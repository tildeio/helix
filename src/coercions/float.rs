use sys::{self, VALUE, T_FLOAT, T_FIXNUM, T_BIGNUM};
use super::{FromRuby, CheckResult, CheckedValue, ToRuby, ToRubyResult};

impl FromRuby for f64 {
    type Checked = CheckedValue<f64>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<f64>> {
        if unsafe { sys::RB_TYPE_P(value, T_FLOAT) || sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 64-bit float")
        }
    }

    fn from_checked(checked: CheckedValue<f64>) -> f64 {
        unsafe { sys::NUM2F64(checked.to_value()) }
    }
}

impl ToRuby for f64 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::F642NUM(self) })
    }
}
