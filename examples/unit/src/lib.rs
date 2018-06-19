#[macro_use]
extern crate helix;

ruby! {
    #[doc(hidden)]
    #[ruby_name="AttributesTest"]
    #[no_mangle]
    #[derive(Clone, Debug)]
    #[cfg(not(foo="bar"))]
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
