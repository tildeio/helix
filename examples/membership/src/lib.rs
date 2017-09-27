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

use helix::{FromRuby, CheckedValue, CheckResult, sys};

impl<'a> FromRuby for RubyArray<'a> {
    type Checked = CheckedValue<RubyArray<'a>>;

    fn from_ruby(value: sys::VALUE) -> CheckResult<Self::Checked> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_ARRAY) } {
            Ok(unsafe { CheckedValue::new(value) })
        } else {
            type_error!(value, "an Array of unsigned pointer-sized integers")
        }
    }

    fn from_checked(checked: Self::Checked) -> RubyArray<'a> {
        let value = checked.to_value();
        let size = unsafe { sys::RARRAY_LEN(value) };
        let ptr = unsafe { sys::RARRAY_PTR(value) };
        RubyArray(unsafe { std::slice::from_raw_parts(ptr as *const usize, size as usize) })
    }
}

impl AsRef<[usize]> for Array {
    fn as_ref<'a>(&'a self) -> &'a [usize] {
        RubyArray::from_ruby_unwrap(self.helix).0
    }
}
