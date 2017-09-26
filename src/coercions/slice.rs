use std;
use sys;
use sys::{VALUE};

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust};

impl<'a> UncheckedValue<&'a[usize]> for VALUE {
    fn to_checked(self) -> CheckResult<&'a[usize]> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_ARRAY) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            type_error!(self, "an Array of unsigned pointer-sized integers")
        }
    }
}

impl<'a> ToRust<&'a[usize]> for CheckedValue<&'a[usize]> {
    fn to_rust(self) -> &'a[usize] {
        let size = unsafe { sys::RARRAY_LEN(self.inner) };
        let ptr = unsafe { sys::RARRAY_PTR(self.inner) };
        unsafe { std::slice::from_raw_parts(ptr as *const usize, size as usize) }
    }
}
