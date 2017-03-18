use sys::{self, VALUE, T_FIXNUM, T_BIGNUM, T_FLOAT, T_RATIONAL};
use std::ffi::CString;
use super::super::{Number, Rational64};

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<Number> for VALUE {
    fn to_checked(self) -> CheckResult<Number> {
        if unsafe { sys::RB_TYPE_P(self, T_FIXNUM) || sys::RB_TYPE_P(self, T_BIGNUM) ||
                    sys::RB_TYPE_P(self, T_FLOAT) || sys::RB_TYPE_P(self, T_RATIONAL) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Rust Num", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<Number> for CheckedValue<Number> {
    fn to_rust(self) -> Number {
        if unsafe { sys::RB_TYPE_P(self.inner, T_FLOAT) } {
            Number::Float(unsafe { CheckedValue::<f64>::new(self.inner) }.to_rust())
        } else if unsafe { sys::RB_TYPE_P(self.inner, T_RATIONAL) } {
            Number::Rational(unsafe { CheckedValue::<Rational64>::new(self.inner) }.to_rust())
        } else {
            Number::Integer(unsafe { CheckedValue::<i64>::new(self.inner) }.to_rust())
        }
    }
}

impl ToRuby for Number {
    fn to_ruby(self) -> VALUE {
        match self {
          Number::Float(v) => v.to_ruby(),
          Number::Integer(v) => v.to_ruby(),
          Number::Rational(v) => v.to_ruby()
        }
    }
}
