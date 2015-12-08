extern crate libcruby;

use libcruby::ruby::*;
use libcruby::types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn zomg() {
    Object::from_ptr(rb_cObject).send("zomg");
}
