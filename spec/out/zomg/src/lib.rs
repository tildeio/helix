extern crate libcruby;

use libcruby::consts::*;
use libcruby::types::*;

extern "C" fn zomg(receiver: Class) -> NilClass {
    let name = receiver.send::<String>("name");
    println!("zomg, I'm a {}", &name[..]);
    Qnil
}

#[no_mangle]
pub extern "C" fn init() {
    Class.define_method::<NilClass>("zomg", zomg);
    Object.send::<NilClass>("zomg");

    let class_name = Object.send::<String>("name");

    println!("Object.name: {:?}", class_name);

    class_name.send::<NilClass>("zomg"); // undefined method `zomg' for "Object":String (NoMethodError)
}
