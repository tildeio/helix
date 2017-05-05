use sys::{self, VALUE, T_FLOAT, T_FIXNUM, T_BIGNUM};

use ruby::Value;
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl<'a> UncheckedValue<f64> for Value<'a> {
    type ToRust = CheckedValue<'a, f64>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), T_FLOAT) || sys::RB_TYPE_P(self.inner(), T_FIXNUM) || sys::RB_TYPE_P(self.inner(), T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::from_value(sys::rb_inspect(self.inner()), self.frame()) };
            Err(format!("No implicit conversion of {} into Rust f64", val.to_rust()))
        }
    }
}

impl<'a> ToRust<f64> for CheckedValue<'a, f64> {
    fn to_rust(self) -> f64 {
        unsafe { sys::NUM2F64(self.inner.inner()) }
    }
}

impl ToRuby for f64 {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::F642NUM(self) }
    }
}
