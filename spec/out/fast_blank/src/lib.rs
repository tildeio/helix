extern crate libc;
extern crate ffi_buf;

use ffi_buf::Buf;

trait RubyString {
    fn trb_is_blank(&self) -> bool;
}

impl RubyString for str {
    fn trb_is_blank(&self) -> bool {
        self.chars().all(|c| c.is_whitespace())
    }
}

pub type ErrorDesc = Box<Buf<'static>>;

#[no_mangle]
pub extern "C" fn trb_string_is_blank(buf: Buf, ret: *mut bool) -> Option<ErrorDesc> {
    unsafe { *ret = buf.as_slice().trb_is_blank() };
    None
}
