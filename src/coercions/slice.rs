use std;
use sys;
use sys::{VALUE};

use ::inspect;
use coercions::*;
use ruby::Value;
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust};

impl<'a> UncheckedValue<&'a[usize]> for Value<'a> {
    type ToRust = CheckedValue<'a, &'a[usize]>;

    fn to_checked(self) -> CheckResult<&'a[usize]> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_ARRAY) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            Err(format!("No implicit conversion of {} into Slice", inspect(self)))
        }
    }
}

impl<'a> ToRust<&'a[usize]> for CheckedValue<'a, &'a[usize]> {
    fn to_rust(self) -> &'a[usize] {
        let size = unsafe { sys::RARRAY_LEN(self.inner) };
        let ptr = unsafe { sys::RARRAY_PTR(self.inner) };
        unsafe { std::slice::from_raw_parts(ptr as *const usize, size as usize) }
    }
}
