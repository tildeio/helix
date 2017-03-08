use sys::{self, VALUE, T_FIXNUM, T_BIGNUM, NUM2U64, U642NUM, NUM2I64, I642NUM};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<u64> for VALUE {
    fn to_checked(self) -> CheckResult<u64> {
        if unsafe { sys::RB_TYPE_P(self, T_FIXNUM) || sys::RB_TYPE_P(self, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Rust u64", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<u64> for CheckedValue<u64> {
    fn to_rust(self) -> u64 {
        unsafe { NUM2U64(self.inner) }
    }
}

impl ToRuby for u64 {
    fn to_ruby(self) -> VALUE {
        unsafe { U642NUM(self) }
    }
}

impl UncheckedValue<i64> for VALUE {
    fn to_checked(self) -> CheckResult<i64> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_FIXNUM) || sys::RB_TYPE_P(self, sys::T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Rust i64", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<i64> for CheckedValue<i64> {
    fn to_rust(self) -> i64 {
        unsafe { NUM2I64(self.inner) }
    }
}

impl ToRuby for i64 {
    fn to_ruby(self) -> VALUE {
        unsafe { I642NUM(self) }
    }
}
