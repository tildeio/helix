use sys::{self, VALUE, T_FIXNUM, T_BIGNUM};

use ruby::Value;
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl<'a> UncheckedValue<u64> for Value<'a> {
    type ToRust = CheckedValue<'a, u64>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), T_FIXNUM) || sys::RB_TYPE_P(self.inner(), T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::from_value(sys::rb_inspect(self.inner()), self.frame()) };
            Err(format!("No implicit conversion of {} into Rust u64", val.to_rust()))
        }
    }
}

impl<'a> ToRust<u64> for CheckedValue<'a, u64> {
    fn to_rust(self) -> u64 {
        unsafe { sys::NUM2U64(self.inner.inner()) }
    }
}

impl ToRuby for u64 {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::U642NUM(self) }
    }
}

impl<'a> UncheckedValue<i64> for Value<'a> {
    type ToRust = CheckedValue<'a, i64>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), sys::T_FIXNUM) || sys::RB_TYPE_P(self.inner(), sys::T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::from_value(sys::rb_inspect(self.inner()), self.frame()) };
            Err(format!("No implicit conversion of {} into Rust i64", val.to_rust()))
        }
    }
}

impl<'a> ToRust<i64> for CheckedValue<'a, i64> {
    fn to_rust(self) -> i64 {
        unsafe { sys::NUM2I64(self.inner.inner()) }
    }
}

impl ToRuby for i64 {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::I642NUM(self) }
    }
}

impl<'a> UncheckedValue<u32> for Value<'a> {
    type ToRust = CheckedValue<'a, u32>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), T_FIXNUM) || sys::RB_TYPE_P(self.inner(), T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::from_value(sys::rb_inspect(self.inner()), self.frame()) };
            Err(format!("No implicit conversion of {} into Rust u32", val.to_rust()))
        }
    }
}

impl<'a> ToRust<u32> for CheckedValue<'a, u32> {
    fn to_rust(self) -> u32 {
        unsafe { sys::NUM2U32(self.inner.inner()) }
    }
}

impl ToRuby for u32 {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::U322NUM(self) }
    }
}

impl<'a> UncheckedValue<i32> for Value<'a> {
    type ToRust = CheckedValue<'a, i32>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), sys::T_FIXNUM) || sys::RB_TYPE_P(self.inner(), sys::T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::from_value(sys::rb_inspect(self.inner()), self.frame()) };
            Err(format!("No implicit conversion of {} into Rust i32", val.to_rust()))
        }
    }
}

impl<'a> ToRust<i32> for CheckedValue<'a, i32> {
    fn to_rust(self) -> i32 {
        unsafe { sys::NUM2I32(self.inner.inner()) }
    }
}

impl ToRuby for i32 {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::I322NUM(self) }
    }
}
