#[macro_export]
macro_rules! codegen {
    { [ $($ast:tt)* ] } => {
        codegen! {
            type: top,
            classes: [],
            buffer: [ $($ast)* ]
        }
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
            name: $name:tt,
            self: {
                ownership: {},
                name: $self:tt
            },
            args: [ $($arg:tt : $argty:ty),* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $name($self : $crate::Metadata, $($arg : $argty),*) -> $($ret)* $body
    };

    {
        {
            type: class_method,
            name: $name:tt,
            self: (),
            args: [ $($args:tt)* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $name($($args)*) -> $($ret)* $body
    };

    {
        {
            type: instance_method,
            name: $name:tt,
            self: {
                ownership: { $($ownership:tt)* },
                name: $self:tt
            },
            args: [ $($args:tt)* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $name($($ownership)* $self, $($args)*) -> $($ret)* $body
    };
}
