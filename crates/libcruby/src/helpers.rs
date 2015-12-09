use types::*;
use ruby::*;
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