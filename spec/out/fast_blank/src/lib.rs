extern crate libc;
extern crate ffi_buf;

use ffi_buf::Buf;

trait RubyString {
    fn trb_is_blank(&self) -> bool;
}

impl<'a> RubyString for &'a str {
    fn trb_is_blank(&self) -> bool {
        println!("{:?}", self);
        self.chars().all(|c| c.is_whitespace())
    }
}

#[no_mangle]
pub extern "C" fn trb_string_is_blank(buf: &Buf) -> bool {
    println!("{:?}", buf.len());
    buf.as_slice().trb_is_blank()
}
