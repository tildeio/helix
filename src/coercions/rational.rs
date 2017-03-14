use sys::{self, VALUE, T_RATIONAL};
use std::ffi::CString;
use num_rational::Rational64;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<Rational64> for VALUE {
    fn to_checked(self) -> CheckResult<Rational64> {
        if unsafe { sys::RB_TYPE_P(self, T_RATIONAL) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Rust Rational64", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<Rational64> for CheckedValue<Rational64> {
    fn to_rust(self) -> Rational64 {
        let numerator = unsafe { sys::NUM2I64(sys::rb_funcall(self.inner, sys::rb_intern(CString::new("numerator").unwrap().as_ptr()), 0)) };
        let denominator = unsafe { sys::NUM2I64(sys::rb_funcall(self.inner, sys::rb_intern(CString::new("denominator").unwrap().as_ptr()), 0)) };
        Rational64::new(numerator, denominator)
    }
}

impl ToRuby for Rational64 {
    fn to_ruby(self) -> VALUE {
        let numerator = unsafe { sys::I642NUM(*self.numer()) };
        let denominator = unsafe { sys::I642NUM(*self.denom()) };
        unsafe { sys::rb_funcall(sys::rb_mKernel, sys::rb_intern(CString::new("Rational").unwrap().as_ptr()), 2, numerator, denominator) }
    }
}
