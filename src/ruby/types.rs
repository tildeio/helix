use sys;
use ToRuby;
use super::Value;

pub enum Type {
    String,
    Array,
    True,
    False,
    Fixnum,
    Bignum,
    Float,
    Unknown(isize)
}

impl Type {
    pub fn from(ty: isize) -> Type {
        if ty == unsafe { sys::T_STRING } {
            Type::String
        } else if ty == unsafe { sys::T_ARRAY } {
            Type::Array
        } else if ty == unsafe { sys::T_TRUE } {
            Type::True
        } else if ty == unsafe { sys::T_FALSE } {
            Type::False
        } else if ty == unsafe { sys::T_FIXNUM } {
            Type::Fixnum
        } else if ty == unsafe { sys::T_BIGNUM } {
            Type::Bignum
        } else if ty == unsafe { sys::T_FLOAT } {
            Type::Float
        } else {
            Type::Unknown(ty)
        }
    }

    pub fn of(val: &Value) -> Type {
        Type::from(unsafe { sys::TYPE(val.to_ruby()) })
    }

    pub fn to_ruby(&self) -> isize {
        match *self {
            Type::String => unsafe { sys::T_STRING },
            Type::Array => unsafe { sys::T_ARRAY },
            Type::True => unsafe { sys::T_TRUE },
            Type::False => unsafe { sys::T_FALSE },
            Type::Fixnum => unsafe { sys::T_FIXNUM },
            Type::Bignum => unsafe { sys::T_BIGNUM },
            Type::Float => unsafe { sys::T_FLOAT },
            Type::Unknown(val) => val
        }
    }

    pub fn matches(&self, value: &Value) -> bool {
        unsafe { sys::RB_TYPE_P(value.to_ruby(), self.to_ruby()) }
    }
}
