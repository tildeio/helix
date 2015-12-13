use types::*;
use ruby::*;
use std::{slice,str,ptr};
use std::ffi::CString;

pub trait Name {
    fn to_id(&self) -> ID;
}

impl Name for ID {
    fn to_id(&self) -> ID {
        *self
    }
}

impl<'a> Name for &'a str {
    fn to_id(&self) -> ID {
        unsafe { rb_intern(CString::new(*self).unwrap().as_ptr()) }
    }
}

pub fn intern<T: Name>(name: T) -> ID {
    name.to_id()
}

pub fn rstring_to_str(rstring: VALUE) -> &'static str {
    unsafe {
      let ptr = RSTRING_PTR(rstring) as *const u8;
      let len = RSTRING_LEN(rstring) as usize;
      let slice = slice::from_raw_parts(ptr, len);

      str::from_utf8_unchecked(slice)
    }
}

pub unsafe fn cast_value<T>(raw: VALUE) -> T {
    ptr::read(&raw as *const _ as *const T)
}