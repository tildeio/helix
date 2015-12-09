#![allow(improper_ctypes)]

use types::*;

extern {
    #[link_name = "rb_Qfalse"]
    pub static Qfalse: FalseClass<'static>;

    #[link_name = "rb_Qtrue"]
    pub static Qtrue: TrueClass<'static>;

    #[link_name = "rb_Qnil"]
    pub static Qnil: NilClass<'static>;

    #[link_name = "rb_cBasicObject"]
    pub static BasicObject: Class<'static>;

    #[link_name = "rb_cObject"]
    pub static Object: Class<'static>;

    #[link_name = "rb_cModule"]
    pub static Module: Class<'static>;

    #[link_name = "rb_cClass"]
    pub static Class: Class<'static>;

    #[link_name = "rb_mKernel"]
    pub static Kernel: Module<'static>;
}