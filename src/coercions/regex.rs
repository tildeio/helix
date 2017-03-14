use libc;
use std;
use sys;
use regex::Regex;
use sys::{VALUE};
use std::ffi::CString;

use super::{FromRuby, CheckResult, CheckedValue, ToRuby, ToRubyResult};

impl FromRuby for Regex {
    type Checked = CheckedValue<Regex>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<Regex>> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_REGEXP) } {
            Ok(unsafe { CheckedValue::<Regex>::new(value) })
        } else {
            type_error!(value, "a Regex")
        }
    }

    fn from_checked(checked: CheckedValue<Regex>) -> Regex {
        unsafe {
            let ruby_str = sys::rb_funcall(checked.inner, sys::rb_intern(CString::new("to_s").unwrap().as_ptr()), 0);
            let size = sys::RSTRING_LEN(ruby_str);
            let ptr = sys::RSTRING_PTR(ruby_str);
            let slice = std::slice::from_raw_parts(ptr as *const u8, size as usize);
            Regex::new(std::str::from_utf8(slice).unwrap()).unwrap()
        }
    }
}

impl ToRuby for Regex {
    fn to_ruby(self) -> ToRubyResult {
        let string = self.to_string();
        let ptr = string.as_ptr();
        let len = string.len();
        let ruby_str = unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) };
        let class_id = unsafe { sys::rb_intern(CString::new("Regexp").unwrap().as_ptr()) };
        let klass = unsafe { sys::rb_const_get(sys::rb_cObject, class_id) };
        let args = [ruby_str];
        Ok(unsafe { sys::rb_class_new_instance(args.len() as isize, args.as_ptr(), klass) })
    }
}
