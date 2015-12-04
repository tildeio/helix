#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc;
use std::ptr;
use std::ffi::CString;
use std::ops::{Index, RangeFull};
use std::marker::PhantomData;
use std::fmt::{self, Formatter, Debug};
use std::mem::transmute;
use va_list::VaList;
use ffi_buf::Buf;

pub trait RubyValue : Sized {
    fn value(&self) -> VALUE;
}

macro_rules! RUBY_VALUE {
    ( $x:ident ) => {
        #[repr(C)]
        pub struct $x<'a> {
            VALUE: VALUE,
            marker: PhantomData<&'a ()>
        }

        impl<'a> RubyValue for $x<'a> {
            fn value(&self) -> VALUE {
                self.VALUE
            }
        }
    }
}

pub trait RubyKernel : RubyValue {
    fn send<T: RubyKernel>(&mut self, method: &str) -> T {
        unsafe {
            ptr::read(&rb_funcallv(self.value(), rb_intern(CString::new(method).unwrap().as_ptr()), 0, ptr::null()) as *const _ as *const T)
        }
    }

    fn inspect(&mut self) -> RubyString {
        self.send::<RubyString>("inspect")
    }
}

impl<T: RubyValue> RubyKernel for T {}

RUBY_VALUE!(RubyObject);
RUBY_VALUE!(RubyString);
RUBY_VALUE!(RubyModule);
RUBY_VALUE!(RubyClass);

pub trait RubyNamespace : RubyValue {
    fn is_const_defined(&self, name: ID) -> bool {
        unsafe { rb_const_defined_from(self.value(), name) }
    }

    fn const_get<T: RubyValue>(&self, name: ID) -> T {
        unsafe { cast_value(rb_const_get_from(self.value(), name)) }
    }

    fn define_module(&self, name: &str) -> RubyModule {
        unsafe { rb_define_module(name.to_c_str()) }
    }

    fn define_class(&self, name: &str, superclass: RubyClass) -> RubyClass {
        unsafe { rb_define_class_under(self.value(), name.to_c_str(), superclass) }
    }
}

impl<'a> RubyClass<'a> {
    pub fn define_alloc_func(&self, func: RubyAllocFunc) {
        unsafe { rb_define_alloc_func(self.value(), func) }
    }

    pub fn define_method(&self, name: &str, argc: isize, func: extern "C" fn(receiver: RubyObject) -> RubyObject) {
        unsafe { rb_define_method(self.value(), name.to_c_str(), argc, func) }
    }
}

unsafe fn cast_value<T: RubyValue>(raw: VALUE) -> T {
    ptr::read(&raw as *const _ as *const T)
}

impl<'a> RubyNamespace for RubyModule<'a> {}
impl<'a> RubyNamespace for RubyClass<'a> {}

impl<'a> Debug for RubyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self[..])
    }
}

impl<'a> Index<RangeFull> for RubyString<'a> {
    type Output = str;

    fn index(&self, _: RangeFull) -> &str {
        unsafe { trb_string_to_buf(self.VALUE) }.as_slice()
    }
}

pub trait RubySliceHelpers {
    fn intern(&self) -> ID;
    fn to_c_str(&self) -> c_string;
}

impl RubySliceHelpers for str {
    fn intern(&self) -> ID {
        unsafe { rb_intern(CString::new(self).unwrap().as_ptr()) }
    }

    fn to_c_str(&self) -> c_string {
        unsafe { CString::new(self).unwrap().as_ptr() }
    }
}

pub type void_ptr = *const libc::c_void;
pub type c_string = *const libc::c_char;
pub type ID = void_ptr;
pub type VALUE = void_ptr;

type RubyAllocFunc = extern "C" fn(RubyClass) -> RubyObject;
type RubyMarkFunc = extern "C" fn();
type RubyFreeFunc = extern "C" fn(void_ptr);

extern {
    pub static rb_cObject: RubyClass<'static>;

    pub fn rb_define_alloc_func(class: VALUE, func: RubyAllocFunc);

    pub fn rb_define_class(name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_module(name: c_string) -> RubyModule;
    pub fn rb_define_class_under(module: VALUE, name: c_string, superclass: RubyClass) -> RubyClass;
    pub fn rb_define_module_under(module: VALUE, name: c_string);

    pub fn rb_const_defined(receiver: VALUE, name: ID) -> bool;
    pub fn rb_const_defined_at(receiver: VALUE, name: ID) -> bool;
    pub fn rb_const_defined_from(receiver: VALUE, name: ID) -> bool;
    pub fn rb_const_get(receiver: VALUE, name: ID) -> VALUE;
    pub fn rb_const_get_at(receiver: VALUE, name: ID) -> VALUE;
    pub fn rb_const_get_from(receiver: VALUE, name: ID) -> VALUE;

    pub fn rb_define_method(receiver: VALUE, name: c_string, argc: isize, func: extern "C" fn(receiver: RubyObject) -> RubyObject);

    pub fn rb_intern(string: c_string) -> ID;
    pub fn rb_funcallv(receiver: VALUE, name: ID, argc: isize, args: *const VALUE) -> RubyObject;
    pub fn trb_string_to_buf(string: VALUE) -> Buf;

    pub fn trb_Data_Get_Struct(receiver: RubyObject) -> void_ptr;
    pub fn trb_Data_Wrap_Struct(klass: RubyClass, mark: RubyMarkFunc, free: RubyFreeFunc, data: void_ptr) -> RubyObject;
}
