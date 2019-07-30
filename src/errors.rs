use super::{Class, ToRuby};
use std::{any, fmt};
use sys::{VALUE, RubyException, SPRINTF_TO_S, c_string, rb_eRuntimeError, rb_raise, rb_jump_tag};

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Library { class: Class, message: ErrorMessage },
    Ruby(RubyException)
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorMessage {
    Static(c_string),
    Dynamic(VALUE)
}

impl Error {
    // Currently unused
    pub fn with_c_string(message: c_string) -> Error {
        Error::Library { class: unsafe { Class(rb_eRuntimeError) }, message: ErrorMessage::Static(message) }
    }

    pub fn with_value(message: VALUE) -> Error {
        Error::Library { class: unsafe { Class(rb_eRuntimeError) }, message: ErrorMessage::Dynamic(message) }
    }

    // TODO: Can we use a trait for this?
    pub fn from_ruby(exception: RubyException) -> Error {
        Error::Ruby(exception)
    }

    pub fn from_any(any: Box<any::Any>) -> Error {
        any.downcast::<Error>()
            .map(|e| *e)
            .or_else(|any| any.downcast::<&str>().map(|e| e.to_error()))
            .or_else(|any| any.downcast::<String>().map(|e| e.to_error()))
            .unwrap_or_else(|any| format!("Unknown Error (caused by `{:?}`)", any).to_error())
    }

    pub fn with_class(self, class: Class) -> Error {
        match self {
            Error::Library { message, .. } => Error::Library { class, message },
            _ => panic!("Only supported for Error::Library")
        }
    }

    pub unsafe fn raise(self) -> ! {
        match self {
            Error::Library { class, message } => match message {
                ErrorMessage::Static(c_string) => rb_raise(class.to_value(), c_string),
                ErrorMessage::Dynamic(value) => rb_raise(class.to_value(), SPRINTF_TO_S, value)
            },
            Error::Ruby(exception) => rb_jump_tag(exception)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Library { message, .. } => match message {
                ErrorMessage::Static(c_string) => {
                    use ::std::ffi::CStr;
                    write!(f, "{}", unsafe { CStr::from_ptr(c_string) }.to_str().unwrap())
                },
                ErrorMessage::Dynamic(value) => {
                    use super::FromRuby;
                    write!(f, "{}", String::from_ruby_unwrap(value))
                }
            },
            Error::Ruby(_exception) => {
                // FIXME: Implement properly
                write!(f, "Ruby Exception")
            }
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

impl ToError for RubyException {
    fn to_error(self) -> Error {
        Error::from_ruby(self)
    }
}
