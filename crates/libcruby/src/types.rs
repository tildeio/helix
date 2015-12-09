use std::marker::{PhantomData, Sync};
use std::ops::{Index, RangeFull};
use std::{fmt, ptr, slice, str};
use libc::*;
use ruby::*;
use helpers::{intern};

pub trait Value {
    fn from_ptr(value: VALUE) -> Self;
    fn as_ptr(&self) -> VALUE;
}

pub trait Send : Value {
    fn send<T>(&self, method: &str) -> T {
        unsafe { ptr::read(&rb_funcallv(self.as_ptr(), intern(method), 0, ptr::null()) as *const _ as *const T) }
    }
}

fn rstring_to_str(rstring: VALUE) -> &'static str {
    unsafe {
      let ptr = RSTRING_PTR(rstring) as *const u8;
      let len = RSTRING_LEN(rstring) as usize;
      let slice = slice::from_raw_parts(ptr, len);

      str::from_utf8_unchecked(slice)
    }
}

macro_rules! RUBY_VALUE {
    ( $x:ident ) => {
        impl<'a> Value for $x<'a> {
            fn from_ptr(value: VALUE) -> Self {
                unsafe { ptr::read(&value as *const _ as *const Self) }
            }

            fn as_ptr(&self) -> VALUE {
                self.VALUE
            }
        }
    }
}

macro_rules! RUBY_UTILS {
    ( $x:ident ) => {
        impl<'a> Send for $x<'a> {}

        impl<'a> fmt::Display for $x<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", rstring_to_str(self.send::<VALUE>("to_s")))
            }
        }

        impl<'a> fmt::Debug for $x<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let inspect = unsafe { rb_inspect(self.as_ptr()) };
                write!(f, "{}", rstring_to_str(inspect))
            }
        }
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

        RUBY_VALUE!($x);
        RUBY_UTILS!($x);
    }

}

RUBY_TYPE!(Object);
RUBY_TYPE!(String);

impl<'a> Index<RangeFull> for String<'a> {
    type Output = str;

    fn index(&self, _: RangeFull) -> &str {
        rstring_to_str(self.as_ptr())
    }
}

RUBY_TYPE!(Module);
RUBY_TYPE!(Class);

extern {
    #[link_name = "rb_define_method_id"]
    fn rb_define_method_id_2(receiver: VALUE, name: ID, func: *const c_void, argc: c_int);
}

pub trait Namespace : Send {
    fn define_method<T: Value>(&self, name: &str, func: extern "C" fn(receiver: Self) -> T) {
        unsafe { rb_define_method_id_2(self.as_ptr(), intern(name), func as *const c_void, 0); }
    }
}

impl<'a> Namespace for Module<'a> {}
impl<'a> Namespace for Class<'a> {}

macro_rules! RUBY_SINGLETON_TYPE {
    ( $x:ident ) => {
        #[repr(C)]
        #[allow(non_snake_case)]
        #[derive(Copy, Clone)]
        pub struct $x<'a> {
            VALUE: VALUE,
            marker: PhantomData<&'a ()>
        }

        unsafe impl<'a> Sync for $x<'a> {}

        RUBY_VALUE!($x);
        RUBY_UTILS!($x);
    }
}

RUBY_SINGLETON_TYPE!(FalseClass);
RUBY_SINGLETON_TYPE!(TrueClass);
RUBY_SINGLETON_TYPE!(NilClass);
