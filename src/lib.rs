extern crate cslice;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

use std::ffi::CString;
use sys::VALUE;

mod macros;
mod class_definition;
mod coercions;

pub use coercions::*;

pub use class_definition::{ClassDefinition, MethodDefinition};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Class(sys::VALUE);

pub trait RubyMethod {
    fn install(self, class: VALUE, name: &str);
}

impl RubyMethod for extern "C" fn(VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        unsafe {
            sys::rb_define_method(
                class,
                CString::new(name).unwrap().as_ptr(),
                self as *const libc::c_void,
                0
            );
        }
    }
}

impl RubyMethod for extern "C" fn(VALUE, VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        unsafe {
            sys::rb_define_method(
                class,
                CString::new(name).unwrap().as_ptr(),
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
    pub fn new(name: &str) -> Class {
        ObjectClass().subclass(name)
    }

    pub fn subclass(&self, name: &str) -> Class {
        unsafe {
            Class(sys::rb_define_class(CString::new(name).unwrap().as_ptr(), self.0))
        }
    }

    pub fn define_method<T: RubyMethod>(&self, name: &str, method: T) {
        method.install(self.0, name);
    }
}

pub fn inspect(val: VALUE) -> String {
    unsafe { CheckedValue::<String>::new(sys::rb_inspect(val)).to_rust() }
}

pub type Metadata = ::sys::VALUE;

#[derive(Clone)]
pub struct Exception {
    ruby_class: Class,
    message: String
}

impl Exception {
    pub fn with_message(string: String) -> Exception {
        Exception {
            ruby_class: Class(unsafe { sys::rb_eRuntimeError }),
            message: string
        }
    }

    pub fn type_error(string: String) -> Exception {
        Exception {
            ruby_class: Class(unsafe { sys::rb_eTypeError }),
            message: string
        }
    }

    pub fn from_any(any: Box<std::any::Any>) -> Exception {
        match any.downcast_ref::<Exception>() {
            Some(e) => e.clone(),
            None    => match any.downcast_ref::<&'static str>() {
                Some(e) => Exception::with_message(format!("{}", e)),
                None    => match any.downcast_ref::<String>() {
                    Some(e) => Exception::with_message(e.clone()),
                    None    => Exception::with_message(format!("Unknown Error; err={:?}", any))
                }
            }
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn raise(&self) -> sys::VALUE {
        unsafe {
            // Hopefully this doesn't leak memory!
            sys::rb_raise(self.ruby_class.0, sys::PRINT_VALUE_STR, self.message.clone().to_ruby());
            sys::Qnil // Return a Ruby nil
        }
    }
}

unsafe impl Send for Exception {}
unsafe impl Sync for Exception {}
