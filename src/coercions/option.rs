use sys::{VALUE, Qnil};
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby, ToRubyResult};

impl<T> UncheckedValue<Option<T>> for VALUE where VALUE: UncheckedValue<T> {
    fn to_checked(self) -> CheckResult<Option<T>> {
        if unsafe { self == Qnil } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            UncheckedValue::<T>::to_checked(self)
                .map(|_| unsafe { CheckedValue::new(self) })
        }
    }
}

impl<T> ToRust<Option<T>> for CheckedValue<Option<T>> where CheckedValue<T>: ToRust<T> {
    fn to_rust(self) -> Option<T> {
        if unsafe { self.inner == Qnil } {
            None
        } else {
            let checked: CheckedValue<T> = unsafe { CheckedValue::new(self.inner) };
            Some(checked.to_rust())
        }
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
