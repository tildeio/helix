use super::{ToRuby, ToRubyResult};
use sys::{rb_ary_new_capa, rb_ary_push};

impl<'a, T> ToRuby for &'a [T] where &'a T: ToRuby {
    fn to_ruby(self) -> ToRubyResult {
        let ary = unsafe { rb_ary_new_capa(self.len() as isize) };
        for item in self {
            unsafe { rb_ary_push(ary, item.to_ruby()?); }
        }
        Ok(ary)
    }
}
