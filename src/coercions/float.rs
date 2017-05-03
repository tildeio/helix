use sys::{self, VALUE, T_FLOAT, T_FIXNUM, T_BIGNUM};

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<f64> for VALUE {
    fn to_checked(self) -> CheckResult<f64> {
        if unsafe { sys::RB_TYPE_P(self, T_FLOAT) || sys::RB_TYPE_P(self, T_FIXNUM) || sys::RB_TYPE_P(self, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into Rust f64", val.to_rust()))
        }
    }
}

impl ToRust<f64> for CheckedValue<f64> {
    fn to_rust(self) -> f64 {
        unsafe { sys::NUM2F64(self.inner) }
    }
}

impl ToRuby for f64 {
    fn to_ruby(&self) -> VALUE {
        unsafe { sys::F642NUM(*self) }
    }
}
