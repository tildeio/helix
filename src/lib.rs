extern crate cslice;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

#[doc(hidden)]
pub extern crate num;

use std::ffi::CString;
use sys::VALUE;

mod macros;
mod class_definition;
mod coercions;

pub use coercions::*;

pub use class_definition::{ClassDefinition, MethodDefinition};

pub use num::complex::Complex64;
pub use num::rational::Rational64;

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



pub enum Number {
    Float(f64),
    Integer(i64), // Should we support other types?
    Rational(Rational64)
}

impl Number {
    fn to_f64(self) -> f64 {
        match self {
            Number::Float(v) => v,
            Number::Integer(v) => v as f64,
            Number::Rational(v) => v.numer().clone() as f64 / v.denom().clone() as f64
        }
    }
}

macro_rules! impl_num_op {
    ($trt:ident, $name:ident, $sym:tt) => {
        impl std::ops::$trt for Number {
            type Output = Number;

            fn $name(self, other: Number) -> Number {
                if let Number::Integer(a) = self {
                    // Integer + Integer = Integer
                    if let Number::Integer(b) = other { return Number::Integer(a $sym b); }
                } else if let Number::Rational(a) = self {
                    // Rational + Rational = Rational
                    if let Number::Rational(b) = other { return Number::Rational(a $sym b); }
                    // Rational + Integer = Rational
                    if let Number::Integer(b) = other { return Number::Rational(a $sym Rational64::new(b, 1)); }
                } else if let Number::Integer(a) = self {
                    // Integer + Rational = Rational
                    if let Number::Rational(b) = other { return Number::Rational(b $sym Rational64::new(a, 1)); }
                }
                // ? + ? = Float
                Number::Float(self.to_f64() $sym other.to_f64())
            }
        }
    }
}

impl_num_op!(Add, add, +);
impl_num_op!(Sub, sub, -);
impl_num_op!(Mul, mul, *);
impl_num_op!(Div, div, /);
