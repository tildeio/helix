use std::ptr;
use std::ops::{Index,RangeFull};
use libc::*;
use ruby::*;
use helpers::*;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! RUBY_VALUE {
      ( $x:ident ) => {
            impl<'a> ::libcruby::types::Value for $x<'a> {
                fn from_ptr(value: ::libcruby::ruby::VALUE) -> Self {
                    unsafe { ::libcruby::helpers::cast_value::<Self>(value) }
                }

                fn as_ptr(&self) -> ::libcruby::ruby::VALUE {
                    self.VALUE
                }
            }
        }
    }

    #[macro_export]
    macro_rules! RUBY_UTILS {
        ( $x:ident ) => {
            impl<'a> Send for $x<'a> {}

            impl<'a> ::std::fmt::Display for $x<'a> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "{}", ::libcruby::helpers::rstring_to_str(self.send::<::libcruby::ruby::VALUE>("to_s")))
                }
            }

            impl<'a> ::std::fmt::Debug for $x<'a> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    let inspect = unsafe { ::libcruby::ruby::rb_inspect(self.as_ptr()) };
                    write!(f, "{}", ::libcruby::helpers::rstring_to_str(inspect))
                }
            }
        }
    }

    #[macro_export]
    macro_rules! RUBY_TYPE {
        ( $x:ident ) => {
            #[repr(C)]
            #[allow(non_snake_case)]
            pub struct $x<'a> {
                VALUE: ::libcruby::ruby::VALUE,
                marker: ::std::marker::PhantomData<&'a ()>
            }

            RUBY_VALUE!($x);
            RUBY_UTILS!($x);
        }

    }

    #[macro_export]
    macro_rules! RUBY_SINGLETON_TYPE {
        ( $x:ident ) => {
            #[repr(C)]
            #[allow(non_snake_case)]
            #[derive(Copy, Clone)]
            pub struct $x<'a> {
                VALUE: ::libcruby::ruby::VALUE,
                marker: ::std::marker::PhantomData<&'a ()>
            }

            unsafe impl<'a> Sync for $x<'a> {}

            RUBY_VALUE!($x);
            RUBY_UTILS!($x);
        }
    }
}

pub trait Value {
    fn from_ptr(value: VALUE) -> Self;
    fn as_ptr(&self) -> VALUE;
}

pub trait Send : Value {
    fn send<T>(&self, method: &str) -> T {
        unsafe { cast_value::<T>(rb_funcallv(self.as_ptr(), intern(method), 0, ptr::null())) }
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
    fn const_get<T: Value>(&self, name: &str) -> T {
        unsafe { cast_value::<T>(rb_const_get_from(self.as_ptr(), intern(name))) }
    }

    fn define_class(&self, name: &str, super_class: &Class) -> Class {
        unsafe { cast_value::<Class>(rb_define_class_id_under(self.as_ptr(), intern(name), super_class.as_ptr())) }
    }

    fn define_method<T: Value, U: Value>(&self, name: &str, func: extern "C" fn(receiver: T) -> U) {
        unsafe { rb_define_method_id_2(self.as_ptr(), intern(name), func as *const c_void, 0); }
    }
}

impl<'a> Namespace for Module<'a> {}
impl<'a> Namespace for Class<'a> {}

// RUBY_SINGLETON_TYPE!(FalseClass);
// RUBY_SINGLETON_TYPE!(TrueClass);
RUBY_SINGLETON_TYPE!(Boolean);
RUBY_SINGLETON_TYPE!(NilClass);
