use std;
use sys;
use sys::{VALUE};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

// VALUE -> to_coercible_rust<String> -> CheckResult<String> -> unwrap() -> Coercible<String> -> to_rust() -> String

impl UncheckedValue<String> for VALUE {
    fn to_checked(self) -> CheckResult<String> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_STRING) } {
            Ok(unsafe { CheckedValue::<String>::new(self) })
        } else {
            Err(CString::new(format!("No implicit conversion from {} to String", "?")).unwrap())
        }
    }
}

impl ToRust<String> for CheckedValue<String> {
    fn to_rust(self) -> String {
        let size = unsafe { sys::RSTRING_LEN(self.inner) };
        let ptr = unsafe { sys::RSTRING_PTR(self.inner) };
        let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
        unsafe { std::str::from_utf8_unchecked(slice) }.to_string()
    }
}

impl ToRuby for String {
    fn to_ruby(self) -> VALUE {
        let cstr = CString::new(self).unwrap();
        unsafe { sys::rb_str_new_cstr(cstr.as_ptr()) }
    }
}
