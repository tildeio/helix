#[macro_use]
extern crate helix;

ruby! {
    class Attributes {
        #[doc(hidden)]
        #[ruby_name="foo"]
        #[inline]
        #[cfg(not(foo="bar"))]
        def bar() {
            println!("Hello from bar!");
        }
    }
}
