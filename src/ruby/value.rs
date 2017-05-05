use sys::VALUE;
use super::Type;
use coercions::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    pub unsafe fn frame(&self) -> CallFrame<'a> {
        self.frame
    }

    pub fn is_type(&self, ty: Type) -> bool {
        ty.matches(self)
    }

    pub fn ruby_type(&self) -> Type {
        Type::of(self)
    }

    pub fn to_rust<T>(&self) -> T where Value<'a>: UncheckedValue<T>, CheckedValue<'a, T>: ToRust<T> {
        self.to_checked().unwrap().to_rust()
    }
}

impl<'a> ToRuby for Value<'a> {
    fn to_ruby(self) -> VALUE {
      self.inner
    }
}

impl<'a> UncheckedValue<Value<'a>> for Value<'a> {
    type ToRust = Value<'a>;

    fn to_checked(self) -> CheckResult<Value<'a>> {
        Ok(self)
    }
}

impl<'a> ToRust<Value<'a>> for Value<'a> {
    fn to_rust(self) -> Value<'a> {
        self
    }
}
