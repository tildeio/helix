#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libc;

use std::ffi::CStr;
use std::mem::size_of;

pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn check_version() {
    let raw_version = unsafe { CStr::from_ptr(HELIX_RUNTIME_VERSION) };
    let version = raw_version.to_str().expect("HELIX_RUNTIME_VERSION must be defined");

    if PKG_VERSION != version {
        panic!("libcsys-ruby version ({}) doesn't match helix_runtime version ({}).", PKG_VERSION, version);
    }

    if size_of::<usize>() != size_of::<u32>() && size_of::<usize>() != size_of::<u64>() {
        panic!("unsupported architecture, size_of::<usize>() = {}", size_of::<usize>());
    }
}

pub type void = libc::c_void;
pub type c_func = *const void;
pub type c_string = *const libc::c_char;
// pub type c_func = extern "C" fn(...);

#[repr(C)]
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct ID(*mut void);

unsafe impl Sync for ID {}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct VALUE(*mut void);

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct RubyException(isize);

impl RubyException {
    pub fn new() -> RubyException {
        RubyException(0)
    }

    pub fn empty() -> RubyException {
        RubyException(0)
    }

    pub fn for_tag(tag: isize) -> RubyException {
        RubyException(tag)
    }
}

pub const EMPTY_EXCEPTION: RubyException = RubyException(0);

#[repr(C)]
pub enum st_retval {
    ST_CONTINUE,
    ST_STOP,
    ST_DELETE,
    // ST_CHECK
}

#[cfg_attr(windows, link(name="helix-runtime"))]
extern "C" {
    #[link_name = "HELIX_RUNTIME_VERSION"]
    pub static HELIX_RUNTIME_VERSION: c_string;

    #[link_name = "HELIX_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "HELIX_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "HELIX_Qnil"]
    pub static Qnil: VALUE;

    #[link_name = "HELIX_PRIsVALUE"]
    pub static PRIsVALUE: c_string;

    #[link_name = "HELIX_SPRINTF_TO_S"]
    pub static SPRINTF_TO_S: c_string;

    #[link_name = "HELIX_SPRINTF_INSPECT"]
    pub static SPRINTF_INSPECT: c_string;

    #[link_name = "rb_cObject"]
    pub static rb_cObject: VALUE;

    #[link_name = "rb_cBasicObject"]
    pub static rb_cBasicObject: VALUE;

    #[link_name = "rb_eRuntimeError"]
    pub static rb_eRuntimeError: VALUE;

    #[link_name = "rb_eTypeError"]
    pub static rb_eTypeError: VALUE;

    #[link_name = "HELIX_RSTRING_LEN"]
    pub fn RSTRING_LEN(string: VALUE) -> isize;

    #[link_name = "HELIX_RSTRING_PTR"]
    pub fn RSTRING_PTR(string: VALUE) -> c_string;

    #[link_name = "HELIX_rb_utf8_str_new"]
    pub fn rb_utf8_str_new(string: c_string, len: libc::c_long) -> VALUE;

    #[link_name = "HELIX_RARRAY_LEN"]
    pub fn RARRAY_LEN(array: VALUE) -> isize;

    #[link_name = "HELIX_RARRAY_PTR"]
    pub fn RARRAY_PTR(array: VALUE) -> *mut VALUE;

    #[link_name = "HELIX_RARRAY_CONST_PTR"]
    pub fn RARRAY_CONST_PTR(array: VALUE) -> *const VALUE;

    #[link_name = "HELIX_RHASH_SIZE"]
    pub fn RHASH_SIZE(hash: VALUE) -> isize;

    #[link_name = "HELIX_RB_TYPE_P"]
    pub fn RB_TYPE_P(val: VALUE, rb_type: isize) -> bool;

    #[link_name = "HELIX_RB_NIL_P"]
    pub fn RB_NIL_P(val: VALUE) -> bool;

    #[link_name = "HELIX_RTEST"]
    pub fn RTEST(val: VALUE) -> bool;

    #[link_name = "HELIX_TYPE"]
    pub fn TYPE(val: VALUE) -> isize;

    pub fn rb_check_type(v: VALUE, rb_type: isize);

    #[link_name = "HELIX_NUM2U64"]
    pub fn NUM2U64(v: VALUE) -> u64;

    #[link_name = "HELIX_U642NUM"]
    pub fn U642NUM(num: u64) -> VALUE;

    #[link_name = "HELIX_NUM2I64"]
    pub fn NUM2I64(v: VALUE) -> i64;

    #[link_name = "HELIX_I642NUM"]
    pub fn I642NUM(num: i64) -> VALUE;

    #[link_name = "HELIX_NUM2U32"]
    pub fn NUM2U32(v: VALUE) -> u32;

    #[link_name = "HELIX_U322NUM"]
    pub fn U322NUM(num: u32) -> VALUE;

    #[link_name = "HELIX_NUM2I32"]
    pub fn NUM2I32(v: VALUE) -> i32;

    #[link_name = "HELIX_I322NUM"]
    pub fn I322NUM(num: i32) -> VALUE;

    #[link_name = "HELIX_NUM2F64"]
    pub fn NUM2F64(v: VALUE) -> f64;

    #[link_name = "HELIX_F642NUM"]
    pub fn F642NUM(num: f64) -> VALUE;

    #[link_name = "HELIX_OBJ_FROZEN"]
    pub fn OBJ_FROZEN(v: VALUE) -> bool;

    #[link_name = "HELIX_CLASS_OF"]
    pub fn CLASS_OF(v: VALUE) -> VALUE;

    #[link_name = "HELIX_T_OBJECT"]
    pub static T_OBJECT: isize;

    #[link_name = "HELIX_T_STRING"]
    pub static T_STRING: isize;

    #[link_name = "HELIX_T_ARRAY"]
    pub static T_ARRAY: isize;

    #[link_name = "HELIX_T_HASH"]
    pub static T_HASH: isize;

    #[link_name = "HELIX_T_TRUE"]
    pub static T_TRUE: isize;

    #[link_name = "HELIX_T_FALSE"]
    pub static T_FALSE: isize;

    #[link_name = "HELIX_T_SYMBOL"]
    pub static T_SYMBOL: isize;

    #[link_name = "HELIX_T_FIXNUM"]
    pub static T_FIXNUM: isize;

    #[link_name = "HELIX_T_FLOAT"]
    pub static T_FLOAT: isize;

    #[link_name = "HELIX_T_BIGNUM"]
    pub static T_BIGNUM: isize;

    #[link_name = "HELIX_T_DATA"]
    pub static T_DATA: isize;

    // unknown if working?
    // fn rb_define_variable(name: c_string, value: *const VALUE);
    pub fn rb_obj_class(obj: VALUE) -> VALUE;
    pub fn rb_obj_classname(obj: VALUE) -> c_string;
    pub fn rb_const_get(class: VALUE, name: ID) -> VALUE;
    pub fn rb_define_global_const(name: c_string, value: VALUE);
    pub fn rb_define_module(name: c_string) -> VALUE;
    pub fn rb_define_module_under(namespace: VALUE, name: c_string) -> VALUE;
    pub fn rb_define_class(name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_class_under(namespace: VALUE, name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_alloc_func(class: VALUE, func: extern "C" fn(class: VALUE) -> VALUE);
    pub fn rb_define_method(class: VALUE, name: c_string, func: c_func, arity: isize);
    pub fn rb_define_singleton_method(class: VALUE, name: c_string, func: c_func, arity: isize);
    pub fn rb_undef_method(class: VALUE, name: c_string);
    pub fn rb_enc_get_index(obj: VALUE) -> isize;
    pub fn rb_utf8_encindex() -> isize;
    pub fn rb_sprintf(specifier: c_string, ...) -> VALUE;
    pub fn rb_inspect(value: VALUE) -> VALUE;
    pub fn rb_intern(string: c_string) -> ID;
    pub fn rb_intern_str(string: VALUE) -> ID;
    pub fn rb_sym2id(symbol: VALUE) -> ID;
    pub fn rb_id2sym(id: ID) -> VALUE;
    pub fn rb_id2str(id: ID) -> VALUE;
    pub fn rb_ary_new() -> VALUE;
    pub fn rb_ary_new_capa(capa: isize) -> VALUE;
    pub fn rb_ary_entry(ary: VALUE, offset: isize) -> VALUE;
    pub fn rb_ary_push(ary: VALUE, item: VALUE) -> VALUE;
    pub fn rb_hash_new() -> VALUE;
    pub fn rb_hash_aref(hash: VALUE, key: VALUE) -> VALUE;
    pub fn rb_hash_aset(hash: VALUE, key: VALUE, value: VALUE) -> VALUE;
    pub fn rb_hash_foreach(hash: VALUE, f: extern "C" fn(key: VALUE, value: VALUE, farg: *mut void) -> st_retval, farg: *mut void);
    pub fn rb_gc_mark(value: VALUE);
    pub fn rb_funcall(value: VALUE, mid: ID, argc: libc::c_int, ...) -> VALUE;
    pub fn rb_funcallv(value: VALUE, mid: ID, argc: libc::c_int, argv: *const VALUE) -> VALUE;
    pub fn rb_scan_args(argc: libc::c_int, argv: *const VALUE, fmt: c_string, ...);
    pub fn rb_block_given_p() -> bool;
    pub fn rb_yield(value: VALUE) -> VALUE;
    pub fn rb_obj_dup(value: VALUE) -> VALUE;
    pub fn rb_obj_init_copy(value: VALUE, orig: VALUE) -> VALUE;

    pub fn rb_raise(exc: VALUE, string: c_string, ...) -> !;
    pub fn rb_jump_tag(state: RubyException) -> !;
    pub fn rb_protect(try: extern "C" fn(v: *mut void) -> VALUE,
                      arg: *mut void,
                      state: *mut RubyException)
                      -> VALUE;

    #[link_name = "HELIX_rb_str_valid_encoding_p"]
    pub fn rb_str_valid_encoding_p(string: VALUE) -> bool;

    #[link_name = "HELIX_rb_str_ascii_only_p"]
    pub fn rb_str_ascii_only_p(string: VALUE) -> bool;

    #[link_name = "HELIX_Data_Wrap_Struct"]
    pub fn Data_Wrap_Struct(klass: VALUE, mark: extern "C" fn(*mut void), free: extern "C" fn(*mut void), data: *mut void) -> VALUE;

    #[link_name = "HELIX_Data_Get_Struct_Value"]
    pub fn Data_Get_Struct_Value(obj: VALUE) -> *mut void;

    #[link_name = "HELIX_Data_Set_Struct_Value"]
    pub fn Data_Set_Struct_Value(obj: VALUE, data: *mut void);
}

#[inline]
pub unsafe fn NUM2USIZE(v: VALUE) -> usize {
    if size_of::<usize>() == size_of::<u32>() {
        NUM2U32(v) as usize
    } else {
        NUM2U64(v) as usize
    }
}

#[inline]
pub unsafe fn USIZE2NUM(num: usize) -> VALUE {
    if size_of::<usize>() == size_of::<u32>() {
        U322NUM(num as u32)
    } else {
        U642NUM(num as u64)
    }
}

#[inline]
pub unsafe fn NUM2ISIZE(v: VALUE) -> isize {
    if size_of::<isize>() == size_of::<i32>() {
        NUM2I32(v) as isize
    } else {
        NUM2I64(v) as isize
    }
}

#[inline]
pub unsafe fn ISIZE2NUM(num: isize) -> VALUE {
    if size_of::<isize>() == size_of::<i32>() {
        I322NUM(num as i32)
    } else {
        I642NUM(num as i64)
    }
}
