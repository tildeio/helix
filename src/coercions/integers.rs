use std::mem::size_of;
use sys::{self, VALUE, T_FIXNUM, T_BIGNUM};
use super::{FromRuby, CheckResult, CheckedValue, ToRuby, ToRubyResult};

impl FromRuby for usize {
    type Checked = CheckedValue<usize>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<usize>> {
        if unsafe { sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else if size_of::<usize>() == size_of::<u32>() {
            type_error!(value, "a 32-bit unsigned integer")
        } else {
            type_error!(value, "a 64-bit unsigned integer")
        }
    }

    fn from_checked(checked: CheckedValue<usize>) -> usize {
        unsafe { sys::NUM2USIZE(checked.to_value()) }
    }
}

impl ToRuby for usize {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::USIZE2NUM(self) })
    }
}

impl FromRuby for isize {
    type Checked = CheckedValue<isize>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<isize>> {
        if unsafe { sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else if size_of::<isize>() == size_of::<i32>() {
            type_error!(value, "a 32-bit signed integer")
        } else {
            type_error!(value, "a 64-bit signed integer")
        }
    }

    fn from_checked(checked: CheckedValue<isize>) -> isize {
        unsafe { sys::NUM2ISIZE(checked.to_value()) }
    }
}

impl ToRuby for isize {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::ISIZE2NUM(self) })
    }
}

impl FromRuby for u64 {
    type Checked = CheckedValue<u64>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<u64>> {
        if unsafe { sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 64-bit unsigned integer")
        }
    }

    fn from_checked(checked: CheckedValue<u64>) -> u64 {
        unsafe { sys::NUM2U64(checked.to_value()) }
    }
}

impl ToRuby for u64 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::U642NUM(self) })
    }
}

impl FromRuby for i64 {
    type Checked = CheckedValue<i64>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<i64>> {
        if unsafe { sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 64-bit signed integer")
        }
    }

    fn from_checked(checked: CheckedValue<i64>) -> i64 {
        unsafe { sys::NUM2I64(checked.to_value()) }
    }
}

impl ToRuby for i64 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::I642NUM(self) })
    }
}

impl FromRuby for u32 {
    type Checked = CheckedValue<u32>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<u32>> {
        if unsafe { sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 32-bit unsigned integer")
        }
    }

    fn from_checked(checked: CheckedValue<u32>) -> u32 {
        unsafe { sys::NUM2U32(checked.to_value()) }
    }
}

impl ToRuby for u32 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::U322NUM(self) })
    }
}

impl FromRuby for i32 {
    type Checked = CheckedValue<i32>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<i32>> {
        if unsafe { sys::RB_TYPE_P(value, T_FIXNUM) || sys::RB_TYPE_P(value, T_BIGNUM) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "a 32-bit signed integer")
        }
    }

    fn from_checked(checked: CheckedValue<i32>) -> i32 {
        unsafe { sys::NUM2I32(checked.to_value()) }
    }
}

impl ToRuby for i32 {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::I322NUM(self) })
    }
}
