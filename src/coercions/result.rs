use super::{ToRuby, ToRubyResult, ToError};

impl<T, U> ToRuby for Result<T, U> where T: ToRuby, U: ToError {
    fn to_ruby(self) -> ToRubyResult {
        match self {
            Ok(value) => value.to_ruby(),
            Err(message) => raise!(message)
        }
    }
}
