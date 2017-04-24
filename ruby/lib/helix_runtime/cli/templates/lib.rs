#[macro_use]
extern crate helix;

ruby! {
    class <%= class_name %> {
        def hello() {
            println!("Hello from <%= app_name %>!");
        }
    }
}
