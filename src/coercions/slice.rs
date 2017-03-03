use std;
use sys;
use sys::{VALUE};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust};

// VALUE -> to_coercible_rust<String> -> CheckResult<String> -> unwrap() -> Coercible<String> -> to_rust() -> String

impl<'a> UncheckedValue<&'a[usize]> for VALUE {
    fn to_checked(self) -> CheckResult<&'a[usize]> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_ARRAY) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            Err(CString::new(format!("No implicit conversion of {} into String", "?")).unwrap())
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
