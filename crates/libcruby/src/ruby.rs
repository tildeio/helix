#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc;

pub type void_ptr = *const libc::c_void;
pub type c_string = *const libc::c_char;

pub type ID = void_ptr;
pub type VALUE = void_ptr;

extern {
    pub static rb_cObject: VALUE;

    pub fn rb_intern(string: c_string) -> ID;
    pub fn rb_funcallv(receiver: VALUE, name: ID, argc: isize, args: *const VALUE) -> VALUE;
}