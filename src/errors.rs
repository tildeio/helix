use super::{Class, ToRuby};
use std::{any};
use sys::{VALUE, SPRINTF_TO_S, c_string, rb_eRuntimeError, rb_raise};

#[derive(Copy, Clone, Debug)]
pub struct Error {
    class: Class,
    message: ErrorMessage
}

#[derive(Copy, Clone, Debug)]
enum ErrorMessage {
    Static(c_string),
    Dynamic(VALUE)
}

impl Error {
    pub fn with_c_string(message: c_string) -> Error {
        Error { class: unsafe { Class(rb_eRuntimeError) }, message: ErrorMessage::Static(message) }
    }

    pub fn with_value(message: VALUE) -> Error {
        Error { class: unsafe { Class(rb_eRuntimeError) }, message: ErrorMessage::Dynamic(message) }
    }

    pub fn from_any(any: Box<any::Any>) -> Error {
        any.downcast_ref::<Error>()
            .map(|e| *e)
            .or_else(|| any.downcast_ref::<&'static str>().map(|e| e.to_error()))
            .or_else(|| any.downcast_ref::<String>().map(|e| e.as_str().to_error()))
            .unwrap_or_else(|| format!("Unknown Error (caused by `{:?}`)", any).to_error())
    }

    pub fn with_class(self, class: Class) -> Error {
        Error { class, message: self.message }
    }

    pub unsafe fn raise(self) -> ! {
        match self.message {
            ErrorMessage::Static(c_string) => rb_raise(self.class.to_value(), c_string),
            ErrorMessage::Dynamic(value) => rb_raise(self.class.to_value(), SPRINTF_TO_S, value)
        }
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

pub trait ToError {
    fn to_error(self) -> Error;
}

impl ToError for Error {
    fn to_error(self) -> Error {
        self
    }
}

impl<'a> ToError for &'a str {
    fn to_error(self) -> Error {
        Error::with_value(self.to_ruby().unwrap())
    }
}

impl ToError for String {
    fn to_error(self) -> Error {
        Error::with_value(self.to_ruby().unwrap())
    }
}
