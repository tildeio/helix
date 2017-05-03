use sys::VALUE;
use super::Type;

pub struct Value {
  inner: VALUE
}

impl Value {
    pub fn is_type(&self, ty: Type) -> bool {
        ty.matches(self)
    }

    pub fn ruby_type(&self) -> Type {
        Type::of(self)
    }
}

use coercions::*;

impl ToRuby for Value {
    fn to_ruby(&self) -> VALUE {
      self.inner
    }
}

impl UncheckedValue<Value> for VALUE {
    fn to_checked(self) -> CheckResult<Value> {
        Ok(unsafe { CheckedValue::new(self) })
    }
}

impl ToRust<Value> for CheckedValue<Value> {
    fn to_rust(self) -> Value {
        Value { inner: self.inner }
    }
}
