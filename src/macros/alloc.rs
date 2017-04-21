#[macro_export]
macro_rules! codegen_allocator {
    ({
        type: class,
        name: $name:tt,
        meta: $meta:tt,
        struct: (),
        methods: $methods:tt
    }) => ();

    ({
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: [ $($method:tt)* ]
    }) => (
        impl $cls {
            extern "C" fn __mark__(_klass: &$cls) {}
            extern "C" fn __free__(_klass: Option<Box<$cls>>) {}

            #[inline]
            fn __alloc_with__(rust_self: Option<Box<$cls>>) -> $crate::sys::VALUE {
                use ::std::mem::transmute;

                unsafe {
                    let instance = $crate::sys::Data_Wrap_Struct(
                        transmute($cls),
                        transmute($cls::__mark__ as usize),
                        transmute($cls::__free__ as usize),
                        transmute(rust_self)
                    );

                    instance
                }
            }
        }
    )
}
