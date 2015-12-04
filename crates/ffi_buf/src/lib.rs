#![allow(dead_code)]

use std::{slice, str};
use std::marker::PhantomData;

pub type ExtResult<T> = Result<T, Option<&'static str>>;

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Copy, Clone)]
pub struct Buf<'a> {
    ptr: *mut u8,
    len: usize,
    marker: PhantomData<&'a ()>,
}

impl<'a> Buf<'a> {
    pub fn wrap(s: &'a str) -> Buf<'a> {
        Buf {
            ptr: s.as_ptr() as *mut u8,
            len: s.len(),
            marker: PhantomData,
        }
    }

    pub fn as_slice(&self) -> &'a str {
        unsafe {
            let s = slice::from_raw_parts(self.ptr as *const u8, self.len);
            str::from_utf8_unchecked(s)
        }
    }

    pub fn as_mut_slice(&mut self) -> &'a mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }
}
