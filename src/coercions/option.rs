use sys::{Qnil};
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust};
use ruby::Value;

pub struct CheckedOption<'a, T> {
    checked: Option<CheckedValue<'a, T>>
}

impl<'a, T> UncheckedValue<Option<T>> for Value<'a>
            where Value<'a>: UncheckedValue<T>,
            CheckedOption<'a, T>: ToRust<Option<T>>
{
    type ToRust = CheckedOption<'a, T>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { self.inner() } == unsafe { Qnil } {
            Ok(CheckedOption { checked: None })
        } else {
            UncheckedValue::<T>::to_checked(self)
                .map(|_| CheckedOption { checked: Some(unsafe { CheckedValue::<T>::new(self) }) })
        }
    }
}

impl<'a, T> ToRust<Option<T>> for CheckedOption<'a, T> where CheckedValue<'a, T>: ToRust<T> {
    fn to_rust(self) -> Option<T> {
        self.checked.map(|c| c.to_rust())
    }
}
