use sys::{VALUE, Qnil};
use super::{FromRuby, CheckResult, ToRuby, ToRubyResult};

impl FromRuby for () {
    type Checked = ();

    fn from_ruby(value: VALUE) -> CheckResult<()> {
        if value == unsafe { Qnil } {
            Ok(())
        } else {
            type_error!(value, "nil")
        }
    }

    fn from_checked(checked: ()) -> () {
        checked
    }
}

impl ToRuby for () {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { Qnil })
    }
}
