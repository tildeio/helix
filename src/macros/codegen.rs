#[macro_export]
macro_rules! codegen {
    { [ $($ast:tt)* ] } => {
        codegen! {
            type: top,
            classes: [],
            buffer: [ $($ast)* ]
        }

        codegen_init! { [ $($ast)* ] }
    };

    {
        type: top,
        classes: [ $($class:tt)* ],
        buffer: []
    } => {
        codegen! {
            type: done,
            classes: [ $($class)* ]
        }
    };

    {
        type: top,
        classes: [
            $($class:tt)*
        ],
        buffer: [
            {
                type: class,
                name: $name:tt,
                meta: { pub: $pub:tt, reopen: $reopen:tt },
                struct: $struct:tt,
                methods: [ $($method:tt)* ]
            }
            $($rest:tt)*
        ]
    } => {
        codegen! {
            type: top,
            classes: [
                $($class)*
                {
                    name: $name,
                    struct: { codegen_struct! { pub: $pub, name: $name, struct: $struct } },
                    methods: [ $( codegen_method! { $method } )* ]
                }
            ],
            buffer: [ $($rest)* ]
        }

        codegen_extra_impls!({
            type: class,
            name: $name,
            meta: { pub: $pub, reopen: $reopen },
            struct: $struct,
            methods: [ $($method)* ]
        });
    };

    {
        type: done,
        classes: [ $(
            {
                name: $name:tt,
                struct: { $($struct:tt)* },
                methods: [ $($method:tt)* ]
            }
        )* ]
    } => {
        $(
            $($struct)*

            impl $name {
                $($method)*
            }
        )*
    };
}

#[macro_export]
macro_rules! codegen_pub_classes {
    {
            $({
                type: class,
                name: $name:tt,
                meta: { pub: $pub:tt, reopen: $reopen:tt },
                struct: $struct:tt,
                methods: [ $($method:tt)* ]
            })*
    } => {
        $(
            codegen_pub_classes! {
                type: class,
                name: $name,
                pub: $pub
            }
        )*
    };

    {
        type: class,
        name: $name:tt,
        pub: false
    } => {};

    {
        type: class,
        name: $name:tt,
        pub: true
    } => {
        pub use self::init_native::$name;
    };
}

#[macro_export]
macro_rules! codegen_struct {
    { pub: false, name: $name:tt, struct: () } => {
        codegen_struct! { pub: {}, name: $name, struct: {} }
    };

    { pub: true, name: $name:tt, struct: () } => {
        codegen_struct! { pub: { pub }, name: $name, struct: {} }
    };

    { pub: false, name: $name:tt, struct: { $($rest:tt)* } } => {
        codegen_struct! { pub: {}, name: $name, struct: { $($rest)* } }
    };

    { pub: true, name: $name:tt, struct: { $($rest:tt)* } } => {
        codegen_struct! { pub: { pub }, name: $name, struct: { $($rest)* } }
    };

    {
        pub: { $($pub:tt)* },
        name: $name:tt,
        struct: { $($struct:tt)* }
    } => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        $($pub)* struct $name {
            helix: $crate::Metadata,
            $($struct)*
        }

        #[allow(non_upper_case_globals)]
        static mut $name: usize = 0;
    }
}

#[macro_export]
macro_rules! codegen_method {
    {
        {
            type: initializer,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            self: {
                ownership: {},
                name: $self:tt
            },
            args: [ $($arg:tt : $argty:ty),* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $rust_name($self : $crate::Metadata, $($arg : $argty),*) -> $($ret)* $body
    };

    {
        {
            type: class_method,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            self: (),
            args: [ $($args:tt)* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $rust_name($($args)*) -> $($ret)* $body
    };

    {
        {
            type: instance_method,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            self: {
                ownership: { $($ownership:tt)* },
                name: $self:tt
            },
            args: [ $($args:tt)* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $rust_name($($ownership)* $self, $($args)*) -> $($ret)* $body
    };
}

#[macro_export]
macro_rules! codegen_extra_impls {
    ($class:tt) => (
        codegen_allocator!($class);
        codegen_coercions!($class);
    )
}
