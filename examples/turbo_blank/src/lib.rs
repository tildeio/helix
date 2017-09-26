#[macro_use]
extern crate helix;

ruby! {
    #[ruby_name = "String"]
    reopen class RubyString {
        #[ruby_name = "blank?"]
        def is_blank(&self) -> bool {
          // self.chars().all(|c| c.is_whitespace())
          self.to_string().chars().all(|c| c.is_whitespace())
        }
    }
}

// Delete me:

use helix::{FromRuby, ToRust};

impl ToString for RubyString {
    fn to_string(&self) -> String {
        let checked = FromRuby::from_ruby(self.helix);
        checked.unwrap().to_rust()
    }
}
