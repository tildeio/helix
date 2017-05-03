#[macro_use]
extern crate helix;
use helix::ruby;

ruby! {
    class Primitive {
        def is_bool(value: ruby::Value) -> bool {
            value.is_type(ruby::Type::True) || value.is_type(ruby::Type::False)
        }
    }
}
