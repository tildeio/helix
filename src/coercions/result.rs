use super::{ToRuby, ToRubyResult, ExceptionInfo};

impl<T, U> ToRuby for Result<T, U> where T: ToRuby, U: ToRuby {
    fn to_ruby(self) -> ToRubyResult {
        match self {
            Ok(value) => value.to_ruby(),
            Err(message) => Err(ExceptionInfo::with_message(message))
        }
    }
}
