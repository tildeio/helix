extern crate libc;

mod buf;
use buf::Buf;

trait RubyString {
    fn trb_is_blank(&self) -> bool;
}

impl<'a> RubyString for &'a str {
    fn trb_is_blank(&self) -> bool {
        self.chars().all(|c| c.is_whitespace())
    }
}

#[no_mangle]
pub extern "C" fn trb_string_is_blank(buf: &Buf) -> bool {
    buf.as_slice().trb_is_blank()
}
