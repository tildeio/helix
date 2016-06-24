#[macro_use]
extern crate helix;

declare_types! {
    reopen class RubyString {
        def is_blank(self) -> bool {
          // self.chars().all(|c| c.is_whitespace())
          self.to_string().chars().all(|c| c.is_whitespace())
        }
    }
}

// Delete me:

use helix::{UncheckedValue, ToRust};

impl ToString for RubyString {
    fn to_string(&self) -> String {
        let checked = self.0.to_checked().unwrap();
        checked.to_rust()
    }
}
