use libc;
use std;
use sys;
use sys::{VALUE};
use super::{FromRuby, CheckResult, CheckedValue, ToRust, ToRuby, ToRubyResult};

impl FromRuby for String {
    fn from_ruby(value: VALUE) -> CheckResult<String> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_STRING) } {
            Ok(unsafe { CheckedValue::<String>::new(value) })
        } else {
            type_error!(value, "a UTF-8 String")
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
    fn to_ruby(self) -> ToRubyResult {
        let ptr = self.as_ptr();
        let len = self.len();
        Ok(unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) })
    }
}

impl<'a> ToRuby for &'a str {
    fn to_ruby(self) -> ToRubyResult {
        let ptr = self.as_ptr();
        let len = self.len();
        Ok(unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) })
    }
}
