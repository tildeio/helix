#![recursion_limit="1024"]

#[allow(unused_imports)]
#[macro_use]
extern crate cstr_macro;

#[doc(hidden)]
pub use cstr_macro::*;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

use std::ffi::CStr;
use sys::VALUE;

#[macro_export]
macro_rules! raise {
    ($msg:expr) => { return Err($crate::ToError::to_error($msg)); };

    ($class:expr, $msg:expr) => {
        return Err($crate::ToError::to_error($msg).with_class($class));
    };
}

#[macro_export]
macro_rules! raise_panic {
    ($msg:expr) => { panic!($crate::ToError::to_error($msg)); };

    ($class:expr, $msg:expr) => {
        panic!($crate::ToError::to_error($msg).with_class($class));
    };
}

#[macro_export]
macro_rules! type_error {
    ($message:expr) => { raise!(unsafe { $crate::Class::from_value($crate::sys::rb_eTypeError) }, $message); };

    ($actual:expr, $expected:expr) => {
        {
            let actual = unsafe { CheckedValue::<String>::new(sys::rb_inspect($actual)) };
            let message = format!("Expected {}, got {}", $expected, actual.to_rust());
            type_error!(message);
        }
    };
}

mod macros;
mod class_definition;
mod coercions;
mod errors;

pub use coercions::*;
pub use errors::*;

pub use class_definition::{ClassDefinition, MethodDefinition};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Class(VALUE);

pub trait RubyMethod {
    fn install(self, class: VALUE, name: &CStr);
}

impl RubyMethod for extern "C" fn(VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &CStr) {
        unsafe {
            sys::rb_define_method(
                class,
                name.as_ptr(),
                self as *const libc::c_void,
                0
            );
        }
    }
}

impl RubyMethod for extern "C" fn(VALUE, VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &CStr) {
        unsafe {
            sys::rb_define_method(
                class,
                name.as_ptr(),
                self as *const libc::c_void,
                1
            );
        }
    }
}

#[allow(non_snake_case)]
#[inline]
fn ObjectClass() -> Class {
    Class(unsafe { sys::rb_cObject })
}

impl Class {
    pub unsafe fn from_value(value: VALUE) -> Class {
        Class(value)
    }

    pub fn to_value(&self) -> VALUE {
        self.0
    }

    pub fn new(name: &CStr) -> Class {
        ObjectClass().subclass(name)
    }

    pub fn subclass(&self, name: &CStr) -> Class {
        unsafe {
            Class(sys::rb_define_class(name.as_ptr(), self.0))
        }
    }

    pub fn define_method<T: RubyMethod>(&self, name: &CStr, method: T) {
        method.install(self.0, name);
    }
}

pub fn inspect(val: VALUE) -> String {
    unsafe { CheckedValue::<String>::new(sys::rb_inspect(val)).to_rust() }
}

pub unsafe fn as_usize(value: ::VALUE) -> usize {
    std::mem::transmute(value)
}

pub type Metadata = ::VALUE;
