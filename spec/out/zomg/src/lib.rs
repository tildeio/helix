extern crate libcruby;

use libcruby::consts::*;
use libcruby::types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn zomg() {
    Object.send("zomg");
}
