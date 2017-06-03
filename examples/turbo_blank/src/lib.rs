#[macro_use]
extern crate helix;

ruby! {
    reopen class RubyString {
        #[ruby_name = "blank?"]
        def is_blank(&self) -> bool {
          // self.chars().all(|c| c.is_whitespace())
          self.to_string().chars().all(|c| c.is_whitespace())
        }
    }
}

// Delete me:

use helix::{UncheckedValue, ToRust, ruby};
use helix::coercions::CallFrame;

impl ToString for RubyString {
    fn to_string(&self) -> String {
        let lt = &();
        let val = unsafe { ruby::Value::new(self.helix, CallFrame::new(lt)) };
        let checked = UncheckedValue::<String>::to_checked(val).unwrap();
        ToRust::<String>::to_rust(checked)
    }
}
