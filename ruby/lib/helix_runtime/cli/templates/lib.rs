#[macro_use]
extern crate helix_runtime as helix;

ruby! {
    class <%= class_name %> {
        def hello() {
            println!("Hello form <%= app_name %>!");
        }
    }
}
