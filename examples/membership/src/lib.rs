#[macro_use]
extern crate libcruby;

use libcruby::sys::{Qtrue,Qfalse};
use libcruby::{UncheckedValue, ToRust};

declare_types! {
    reopen class Array {
        def is_superset_of(self, needle: &[usize]) {
            if needle.is_empty() { return Qtrue }

            let haystack = self.as_ref();

            if haystack.is_empty() { return Qfalse }

            let mut needle = needle.iter();
            let mut needle_item = needle.next().unwrap();

            for item in haystack {
                if item == needle_item {
                    match needle.next() {
                        None => return Qtrue,
                        Some(next_item) => needle_item = next_item
                    }
                }
            }

            Qfalse
        }
    }
}

impl AsRef<[usize]> for Array {
    fn as_ref(&self) -> &[usize] {
        let checked = self.0.to_checked().unwrap();
        checked.to_rust()
    }
}
