use libc;
use std;
use std::ops::{Deref, Index};
use sys::{self};
use super::{Value};
use coercions::*;

#[derive(Debug, Copy, Clone)]
pub struct Array<'a> {
  inner: Value<'a>
}

impl<'a> Deref for Array<'a> {
  type Target = Value<'a>;

  fn deref(&self) -> &Value<'a> {
    &self.inner
  }
}

impl<'a> Index<usize> for Array<'a> {
  type Output = Value<'a>;

  fn index(&self, offset: usize) -> &Value<'a> {
    let val = unsafe { sys::rb_ary_aref(self.to_ruby(), offset as libc::c_long) };
    unsafe { std::mem::transmute(val) }
  }
}

pub struct CheckedArray<'a> {
    inner: Value<'a>
}

impl<'a> UncheckedValue<Array<'a>> for Value<'a> {
    type ToRust = CheckedArray<'a>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
      Ok(CheckedArray { inner: self })
    }
}

impl<'a> ToRust<Array<'a>> for CheckedArray<'a> {
    fn to_rust(self) -> Array<'a> {
        Array { inner: self.inner }
    }
}
