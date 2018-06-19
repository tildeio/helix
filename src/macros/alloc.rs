#[macro_export]
macro_rules! codegen_allocator {
    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        attributes: $attributes:tt,
        meta: $meta:tt,
        struct: (),
        methods: $methods:tt
    }) => ();

    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        attributes: $attributes:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: [ $($method:tt)* ]
    }) => (
        impl $rust_name {
            extern "C" fn __mark__(_klass: &$rust_name) {}
            extern "C" fn __free__(_klass: Option<Box<$rust_name>>) {}

            #[inline]
            fn __alloc_with__(rust_self: Option<Box<$rust_name>>) -> $crate::sys::VALUE {
                use ::std::mem::transmute;

                unsafe {
                    let instance = $crate::sys::Data_Wrap_Struct(
                        transmute($rust_name),
                        transmute($rust_name::__mark__ as usize),
                        transmute($rust_name::__free__ as usize),
                        transmute(rust_self)
                    );

                    instance
                }
            }
        }
    )
}
