#[macro_use]
extern crate helix;

ruby! {
    reopen class Array {
        def is_superset_of(&self, needle: &[usize]) -> bool {
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

// This is incredibly terrible and illustrates an increasingly bad problem
// with the current factoring around reopen. TLDR: reopen really doesn't
// work at the moment and you shouldn't use it.

use helix::{UncheckedValue, ToRust, ruby};
use helix::coercions::CallFrame;

impl AsRef<[usize]> for Array {
    fn as_ref(&self) -> &[usize] {
        let lt: &'static () = unsafe { std::mem::transmute(&()) };
        let frame = unsafe { CallFrame::new(lt) };
        let val = unsafe { ruby::Value::new(self.helix, frame) };
        let checked = UncheckedValue::<&[usize]>::to_checked(val).unwrap();
        ToRust::<&[usize]>::to_rust(checked)
    }
}
