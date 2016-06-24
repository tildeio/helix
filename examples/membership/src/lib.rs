#[macro_use]
extern crate helix;

declare_types! {
    reopen class Array {
        def is_superset_of(self, needle: &[usize]) -> bool {
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

// Delete me:

use helix::{UncheckedValue, ToRust};

impl AsRef<[usize]> for Array {
    fn as_ref(&self) -> &[usize] {
        let checked = self.0.to_checked().unwrap();
        checked.to_rust()
    }
}
