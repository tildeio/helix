use types::*;
use ruby::*;
use std::ffi::CString;

pub fn intern(name: &str) -> ID {
    unsafe { rb_intern(CString::new(name).unwrap().as_ptr()) }
}