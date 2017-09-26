#[macro_use]
extern crate helix;

pub struct RubyArray<'a>(&'a [usize]);

ruby! {
    reopen class Array {
        def is_superset_of(&self, needle: RubyArray) -> bool {
            let needle = needle.0;

            if needle.is_empty() { return true }

            let haystack = self.as_ref();

            if haystack.is_empty() { return false }

            let mut needle = needle.iter();
            let mut needle_item = needle.next().unwrap();

            for item in haystack {
                if item == needle_item {
                    match needle.next() {
                        None => return true,
                        Some(next_item) => needle_item = next_item
                    }
                }
            }

            false
        }
    }
}

use helix::{FromRuby, CheckedValue, CheckResult, ToRust, sys};

impl<'a> FromRuby for RubyArray<'a> {
    fn from_ruby(value: sys::VALUE) -> CheckResult<RubyArray<'a>> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_ARRAY) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "an Array of unsigned pointer-sized integers")
        }
    }
}

impl<'a> ToRust<RubyArray<'a>> for CheckedValue<RubyArray<'a>> {
    fn to_rust(self) -> RubyArray<'a> {
        let size = unsafe { sys::RARRAY_LEN(self.inner) };
        let ptr = unsafe { sys::RARRAY_PTR(self.inner) };
        RubyArray(unsafe { std::slice::from_raw_parts(ptr as *const usize, size as usize) })
    }
}

impl AsRef<[usize]> for Array {
    fn as_ref<'a>(&'a self) -> &'a [usize] {
        let checked = RubyArray::from_ruby(self.helix);
        checked.unwrap().to_rust().0
    }
}
