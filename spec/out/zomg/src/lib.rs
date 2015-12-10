extern crate libcruby;

use libcruby::consts::*;
use libcruby::types::*;

extern "C" fn string_is_blank(receiver: String) -> Boolean {
    if receiver[..].chars().all(|c| c.is_whitespace()) {
        Qtrue
    } else {
        Qfalse
    }
}

extern "C" fn string_is_present(receiver: String) -> Boolean {
    if receiver[..].chars().all(|c| c.is_whitespace()) {
        Qfalse
    } else {
        Qtrue
    }
}

#[no_mangle]
pub extern "C" fn init() {
    String.define_method::<String, Boolean>("blank?", string_is_blank);
    String.define_method::<String, Boolean>("present?", string_is_present);
}
