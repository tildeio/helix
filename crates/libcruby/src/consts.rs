#![allow(improper_ctypes)]

use types::*;

extern {
    #[link_name = "rb_cObject"]
    pub static Object: Object<'static>;
}