use sys;
use sys::{VALUE};

use super::{CheckResult, FromRuby, ToRuby, ToRubyResult};
use super::super::{inspect};

impl<T: FromRuby> FromRuby for Vec<T> {
    type Checked = Vec<T::Checked>;

    fn from_ruby(value: VALUE) -> CheckResult<Self::Checked> {
        if unsafe { sys::RB_TYPE_P(value, sys::T_ARRAY) } {
            // Make sure we can actually do the conversions for the values.
            let len = unsafe { sys::RARRAY_LEN(value) };
            let mut checked = Vec::with_capacity(len as usize);

            for i in 0..len {
                let val = unsafe { sys::rb_ary_entry(value, i) };
                match T::from_ruby(val) {
                    Ok(v) => checked.push(v),
                    Err(e) => type_error!(format!("Failed to convert {}, element {} has the wrong type: {}", inspect(value), i, e)),
                }
            }

            Ok(checked)
        } else {
            type_error!(value, "an array")
        }
    }

    fn from_checked(checked: Self::Checked) -> Self {
        checked.into_iter().map(T::from_checked).collect()
    }
}

impl<T: ToRuby> ToRuby for Vec<T> {
    fn to_ruby(self) -> ToRubyResult {
        let ary = unsafe { sys::rb_ary_new_capa(self.len() as isize) };
        for item in self {
            unsafe { sys::rb_ary_push(ary, item.to_ruby()?); }
        }
        Ok(ary)
    }
}
