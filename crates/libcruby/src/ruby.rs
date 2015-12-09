#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::*;

pub type void_ptr = *const c_void;
pub type c_string = *const c_char;

pub type ID = void_ptr;
pub type VALUE = void_ptr;

extern {
    #[link_name = "rb_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "rb_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "rb_Qnil"]
    pub static Qnil: VALUE;

    pub static rb_cBasicObject: VALUE;
    pub static rb_cObject: VALUE;
    pub static rb_cModule: VALUE;
    pub static rb_cClass: VALUE;
    pub static rb_mKernel: VALUE;

    #[link_name = "rb_rstring_len"]
    pub fn RSTRING_LEN(str: VALUE) -> c_long;

    #[link_name = "rb_rstring_ptr"]
    pub fn RSTRING_PTR(str: VALUE) -> c_string;


    pub fn rb_define_method(module: VALUE, name: c_string, func: extern "C" fn(receiver: VALUE, ...) -> VALUE, argc: c_int);
    pub fn rb_define_method_id(module: VALUE, name: ID, func: extern "C" fn(receiver: VALUE, ...) -> VALUE, argc: c_int);
    pub fn rb_funcallv(receiver: VALUE, name: ID, argc: c_int, args: *const VALUE) -> VALUE;
    pub fn rb_id2str(id: ID) -> VALUE;
    pub fn rb_inspect(value: VALUE) -> VALUE;
    pub fn rb_intern(string: c_string) -> ID;
}