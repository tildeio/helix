#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libcruby;
extern crate libc;

use std::mem;

macro_rules! cstr {
    ( $x: expr ) => { $x.as_ptr() as *const i8 }
}

#[no_mangle]
pub extern "C" fn Init_libconsole() {
    unsafe {
        let Console = rb_define_class(cstr!(b"Console\0"), rb_cObject);
        rb_define_method(Console, cstr!(b"log\0"), log as void_ptr, 1);
    };
}

extern "C" fn log(_: VALUE, message: VALUE) -> VALUE {
    #[repr(C)]
    struct CheckTypeArgs {
        value: VALUE,
        rb_type: isize,
    }

    extern "C" fn CheckType(args: &CheckTypeArgs) -> VALUE {
        unsafe { rb_check_type(args.value, args.rb_type) };
        Qnil
    }

    let result = std::panic::catch_unwind(|| {
        with_protect(CheckType,
                     &CheckTypeArgs {
                         value: message,
                         rb_type: T_STRING,
                     });
    });

    if let Err(state) = result {
        let state = state.downcast_ref::<RubyException>().unwrap();
        unsafe { rb_jump_tag(*state) };
    } else {
        if unsafe { RB_TYPE_P(message, T_STRING) } {
            let size = unsafe { RSTRING_LEN(message) };
            let ptr = unsafe { RSTRING_PTR(message) };
            let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
            let string = unsafe { std::str::from_utf8_unchecked(slice) };
            println!("size: {}", size);
            println!("ptr: {:?}", ptr);
            println!("string: {}", string);
            Qtrue
        } else {
            Qfalse
        }
    }


}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RubyException(isize);

fn with_protect<T>(func: extern "C" fn(&T) -> VALUE, arg: &T) {
    let mut state: RubyException = RubyException(0);
    let arg: void_ptr = unsafe { mem::transmute(arg) };
    let func: extern "C" fn(void_ptr) -> VALUE = unsafe { mem::transmute(func) };

    unsafe { rb_protect(func, arg, &mut state as *mut RubyException) };

    if state == RubyException(0) {
        println!("IT WORKED");
    } else {
        panic!(state);
    }
}

pub type void_ptr = *const libc::c_void;
pub type c_string = *const libc::c_char;
// pub type c_func = extern "C" fn(...);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ID(void_ptr);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VALUE(void_ptr);

extern "C" {
    #[link_name = "helix_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "helix_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "helix_Qnil"]
    pub static Qnil: VALUE;

    #[link_name = "rb_cObject"]
    pub static rb_cObject: VALUE;

    #[link_name = "HELIX_RSTRING_LEN"]
    pub fn RSTRING_LEN(string: VALUE) -> isize;

    #[link_name = "HELIX_RSTRING_PTR"]
    pub fn RSTRING_PTR(string: VALUE) -> c_string;

    #[link_name = "HELIX_RB_TYPE_P"]
    pub fn RB_TYPE_P(val: VALUE, rb_type: isize) -> bool;

    pub fn rb_check_type(v: VALUE, rb_type: isize);

    #[link_name = "HELIX_T_STRING"]
    pub static T_STRING: isize;

    // unknown if working?
    // fn rb_define_variable(name: c_string, value: *const VALUE);
    pub fn rb_define_global_const(name: c_string, value: VALUE);
    pub fn rb_define_module(name: c_string) -> VALUE;
    pub fn rb_define_module_under(namespace: VALUE, name: c_string) -> VALUE;
    pub fn rb_define_class(name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_class_under(namespace: VALUE, name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_method(class: VALUE, name: c_string, func: void_ptr, arity: isize);
    pub fn rb_intern(string: c_string) -> ID;
    pub fn rb_jump_tag(state: RubyException) -> !;
    pub fn rb_protect(try: extern "C" fn(v: void_ptr) -> VALUE,
                      arg: void_ptr,
                      state: *mut RubyException)
                      -> VALUE;
}
