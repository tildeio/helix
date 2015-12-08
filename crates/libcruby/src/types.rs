use std::ptr;
use std::marker::PhantomData;
use ruby::*;
use helpers::{intern};

pub trait Value {
    fn from_ptr(value: VALUE) -> Self;
    fn as_ptr(&self) -> VALUE;
}

pub trait Send : Value {
    fn send(&self, method: &str) {
        unsafe { rb_funcallv(self.as_ptr(), intern(method), 0, ptr::null()); }
    }
}

macro_rules! RUBY_TYPE {
    ( $x:ident ) => {
        #[repr(C)]
        #[allow(non_snake_case)]
        pub struct $x<'a> {
            VALUE: VALUE,
            marker: PhantomData<&'a ()>
        }

        impl<'a> Value for $x<'a> {
            fn from_ptr(value: VALUE) -> Self {
                unsafe { ptr::read(&value as *const _ as *const Self) }
            }

            fn as_ptr(&self) -> VALUE {
                self.VALUE
            }
        }

        impl<'a> Send for $x<'a> {}
    }
}

RUBY_TYPE!(Object);
