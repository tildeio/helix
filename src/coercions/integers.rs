use sys::{self, VALUE, T_FIXNUM, T_BIGNUM};

use coercions::*;
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<u64> for VALUE {
    type ToRust = CheckedValue<u64>;

    fn to_checked<'a>(self, frame: CallFrame<'a>) -> CheckResult<u64> {
        if unsafe { sys::RB_TYPE_P(self, T_FIXNUM) || sys::RB_TYPE_P(self, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into Rust u64", val.to_rust()))
        }
    }
}

impl<'a> ToRust<u64> for CheckedValue<u64> {
    fn to_rust(self) -> u64 {
        unsafe { sys::NUM2U64(self.inner) }
    }
}

impl ToRuby for u64 {
    fn to_ruby(&self) -> VALUE {
        unsafe { sys::U642NUM(*self) }
    }
}

impl UncheckedValue<i64> for VALUE {
    type ToRust = CheckedValue<i64>;

    fn to_checked<'a>(self, frame: CallFrame<'a>) -> CheckResult<i64> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_FIXNUM) || sys::RB_TYPE_P(self, sys::T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into Rust i64", val.to_rust()))
        }
    }
}

impl<'a> ToRust<i64> for CheckedValue<i64> {
    fn to_rust(self) -> i64 {
        unsafe { sys::NUM2I64(self.inner) }
    }
}

impl ToRuby for i64 {
    fn to_ruby(&self) -> VALUE {
        unsafe { sys::I642NUM(*self) }
    }
}

impl UncheckedValue<u32> for VALUE {
    type ToRust = CheckedValue<u32>;

    fn to_checked<'a>(self, frame: CallFrame<'a>) -> CheckResult<u32> {
        if unsafe { sys::RB_TYPE_P(self, T_FIXNUM) || sys::RB_TYPE_P(self, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into Rust u32", val.to_rust()))
        }
    }
}

impl<'a> ToRust<u32> for CheckedValue<u32> {
    fn to_rust(self) -> u32 {
        unsafe { sys::NUM2U32(self.inner) }
    }
}

impl ToRuby for u32 {
    fn to_ruby(&self) -> VALUE {
        unsafe { sys::U322NUM(*self) }
    }
}

impl UncheckedValue<i32> for VALUE {
    type ToRust = CheckedValue<i32>;

    fn to_checked<'a>(self, frame: CallFrame<'a>) -> CheckResult<i32> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_FIXNUM) || sys::RB_TYPE_P(self, sys::T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into Rust i32", val.to_rust()))
        }
    }
}

impl<'a> ToRust<i32> for CheckedValue<i32> {
    fn to_rust(self) -> i32 {
        unsafe { sys::NUM2I32(self.inner) }
    }
}

impl ToRuby for i32 {
    fn to_ruby(&self) -> VALUE {
        unsafe { sys::I322NUM(*self) }
    }
}
