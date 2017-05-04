use std;
use sys::VALUE;
use super::Type;
use coercions::*;

#[derive(Debug, Clone, Copy)]
pub struct Value<'a> {
  inner: VALUE,
  frame: CallFrame<'a>
}

impl<'a> Value<'a> {
    pub unsafe fn new<'b>(value: VALUE, frame: CallFrame<'b>) -> Value<'b> {
        Value { inner: value, frame }
    }

    pub unsafe fn inner(&self) -> VALUE {
        self.inner
    }

    pub fn is_type(&self, ty: Type) -> bool {
        ty.matches(self)
    }

    pub fn ruby_type(&self) -> Type {
        Type::of(self)
    }

    pub fn to_rust<'lt, T>(&self) -> T where Value<'lt>: UncheckedValue<T>, CheckedValue<'lt, T>: ToRust<T> {
        self.inner.to_checked(self.frame).unwrap().to_rust()
    }
}

impl<'a> ToRuby for Value<'a> {
    fn to_ruby(&self) -> VALUE {
      self.inner
    }
}

impl<'a> UncheckedValue<Value<'a>> for VALUE {
    type ToRust = Value<'a>;

    fn to_checked(self) -> CheckResult<Value<'a>> {
        Ok(unsafe { Value::new(self, frame) })
    }
}

impl<'a> ToRust<Value<'a>> for Value<'a> {
    fn to_rust(self) -> Value<'a> {
        self
    }
}
