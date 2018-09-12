use libc;
use std;
use sys;
use sys::{VALUE};
use super::{FromRuby, CheckResult, CheckedValue, ToRuby, ToRubyResult};

impl FromRuby for String {
    type Checked = CheckedValue<String>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<String>> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_STRING) > 0 } {
            if unsafe { sys::rb_enc_get_index(value) == sys::rb_utf8_encindex() } {
                // FIXME: Bring this check back
                // if unsafe { sys::rb_str_valid_encoding_p(value) } {
                    unsafe { Ok(CheckedValue::new(value)) }
                // } else {
                //     type_error!(value, "a valid UTF-8 String")
                // }
            } else {
                // FIXME: Bring this check back
                // if unsafe { sys::rb_str_ascii_only_p(value) } {
                    unsafe { Ok(CheckedValue::new(value)) }
                // } else {
                //     type_error!(value, "an UTF-8 String")
                // }
            }
        } else {
            type_error!(value, "a String")
        }
    }

    fn from_checked(checked: CheckedValue<String>) -> String {
        let value = checked.to_value();
        let size = unsafe { sys::RSTRING_LEN(value) };
        let ptr = unsafe { sys::RSTRING_PTR(value) };
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
