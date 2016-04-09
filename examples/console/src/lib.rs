#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libcruby;
extern crate libc;

macro_rules! cstr {
    ( $x: expr ) => { $x.as_ptr() as *const i8 }
}

#[no_mangle]
pub extern "C" fn Init_libconsole() {
    unsafe { rb_define_global_const(cstr!(b"TRUE1\0"), Qtrue) };
}

// extern "C" fn zero() -> VALUE {
//     Qnil
// }

pub type void_ptr = *const libc::c_void;
pub type c_string = *const libc::c_char;
pub type c_func = extern "C" fn() -> VALUE;
pub type ID = void_ptr;
pub type VALUE = void_ptr;

extern "C" {
    #[link_name = "rb_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "rb_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "rb_Qnil"]
    pub static Qnil: VALUE;

    // unknown if working?
    // fn rb_define_variable(name: c_string, value: *const VALUE);
    fn rb_define_global_const(name: c_string, value: VALUE);
    fn rb_define_module(name: c_string) -> VALUE;
    fn rb_define_module_under(parent: VALUE, name: c_string) -> VALUE;
    fn rb_intern(string: c_string) -> ID;
}
