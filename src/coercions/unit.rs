use sys::{self};
use super::{ToRuby, ToRubyResult};

impl ToRuby for () {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { sys::Qnil })
    }
}
