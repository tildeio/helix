use sys::{VALUE, Qnil};
use super::{FromRuby, CheckResult, ToRuby, ToRubyResult};

impl<T: FromRuby> FromRuby for Option<T> {
    type Checked = Option<T::Checked>;

    fn from_ruby(value: VALUE) -> CheckResult<Option<T::Checked>> {
        if unsafe { value == Qnil } {
            Ok(None)
        } else {
            T::from_ruby(value).map(|c| Some(c))
        }
    }

    fn from_checked(checked: Option<T::Checked>) -> Option<T> {
        checked.map(T::from_checked)
    }
}

impl<T> ToRuby for Option<T> where T: ToRuby {
    fn to_ruby(self) -> ToRubyResult {
        match self {
            Some(value) => value.to_ruby(),
            None => Ok(unsafe { Qnil })
        }
    }
}
