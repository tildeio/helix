use sys::{self, VALUE, T_COMPLEX};
use std::ffi::CString;
use num_complex::Complex64;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<Complex64> for VALUE {
    fn to_checked(self) -> CheckResult<Complex64> {
        if unsafe { sys::RB_TYPE_P(self, T_COMPLEX) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Rust Complex64", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<Complex64> for CheckedValue<Complex64> {
    fn to_rust(self) -> Complex64 {
        let real = unsafe { sys::NUM2F64(sys::rb_funcall(self.inner, sys::rb_intern(CString::new("real").unwrap().as_ptr()), 0)) };
        let imag = unsafe { sys::NUM2F64(sys::rb_funcall(self.inner, sys::rb_intern(CString::new("imag").unwrap().as_ptr()), 0)) };
        Complex64::new(real, imag)
    }
}

impl ToRuby for Complex64 {
    fn to_ruby(self) -> VALUE {
        let class_id = unsafe { sys::rb_intern(CString::new("Complex").unwrap().as_ptr()) };
        let klass = unsafe { sys::rb_const_get(sys::rb_cObject, class_id) };
        let real = unsafe { sys::F642NUM(self.re) };
        let imag = unsafe { sys::F642NUM(self.im) };
        unsafe { sys::rb_funcall(klass, sys::rb_intern(CString::new("rectangular").unwrap().as_ptr()), 2, real, imag) }
    }
}
