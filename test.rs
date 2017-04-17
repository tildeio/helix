#[macro_export]
macro_rules! declare_types {
    { $($rest:tt)* } => {
        parse! {
            state: top_level,
            buffer: { $($rest)* },
            stack: { ast: [] }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse {
    // STATE: top_level

    {
        state: top_level,
        buffer: {},
        stack: { ast: $ast:tt }
    } => {
        codegen!($ast);
    };

    {
        state: top_level,
        buffer: $buffer:tt,
        stack: { $($stack: tt)* }
    } => {
        parse! {
            state: parse_class,
            buffer: $buffer,
            stack: {
                pub: false,
                reopen: false,
                $($stack)*
            }
        }
    };

    // STATE: parse_class

    {
        state: parse_class,
        buffer: { pub $($rest:tt)* },
        stack: {
            pub: false,
            reopen: false,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class,
            buffer: { $($rest)* },
            stack: {
                pub: true,
                reopen: false,
                $($stack)*
            }
        }
    };

    {
        state: parse_class,
        buffer: { reopen $($rest:tt)* },
        stack: {
            pub: $pub:tt,
            reopen: false,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class,
            buffer: { $($rest)* },
            stack: {
                pub: $pub,
                reopen: true,
                $($stack)*
            }
        }
    };

    {
        state: parse_class,
        buffer: { class $name:ident { $($body:tt)* } $($rest:tt)* },
        stack: {
            pub: $pub:tt,
            reopen: $reopen:tt,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class_body,
            buffer: { $($body)* },
            stack: {
                class: {
                    type: class,
                    name: $name,
                    meta: { pub: $pub, reopen: $reopen },
                    struct: uninitialized,
                    methods: []
                },
                program: { $($rest)* },
                $($stack)*
            }
        }
    };

    // STATE: parse_class_body

    {
        state: parse_class_body,
        buffer: { struct { $($struct:tt)* } $($rest:tt)* },
        stack: {
            class: {
                type: class,
                name: $name:ident,
                meta: $meta:tt,
                struct: uninitialized,
                methods : []
            },
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_methods,
            buffer: { $($rest)* },
            stack: {
                class: {
                    type: class,
                    name: $name,
                    meta: $meta,
                    struct: { $($struct)* },
                    methods: []
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_class_body,
        buffer: $buffer:tt,
        stack: {
            class: {
                type: class,
                name: $name:ident,
                meta: $meta:tt,
                struct: uninitialized,
                methods : []
            },
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_methods,
            buffer: $buffer,
            stack: {
                class: {
                    type: class,
                    name: $name,
                    meta: $meta,
                    struct: (),
                    methods: []
                },
                $($stack)*
            }
        }
    };

    // STATE: parse_methods

    {
        state: parse_methods,
        buffer: { def $($rest:tt)* },
        stack: $stack:tt
    } => {
        parse! {
            state: parse_method,
            buffer: { $($rest)* },
            stack: $stack
        }
    };

    {
        state: parse_methods,
        buffer: {},
        stack: {
            class: $class:tt,
            program: $program:tt,
            ast: [ $($ast:tt),* ]
        }
    } => {
        parse! {
            state: top_level,
            buffer: $program,
            stack: {
                ast: [ $($ast,)* $class ]
            }
        }
    };

    // STATE: parse_method

    {
        state: parse_method,
        buffer: { initialize ( $($args:tt)* ) $($rest:tt)* },
        stack: {
            class: $class:tt,
            $($stack:tt)*
        }
    } => {
        assert_struct!(true, $class);

        parse! {
            state: parse_arguments_helix,
            buffer: { $($args)* },
            stack: {
                class_body: { $($rest)* },
                class: $class,
                $($stack)*
            }
        }
    };

    {
        state: parse_method,
        buffer: { $name:ident ( $($args:tt)* ) $($rest:tt)* },
        stack: { $($stack:tt)* }
    } => {
        parse! {
            state: parse_arguments_self,
            buffer: { $($args)* },
            stack: {
                name: $name,
                class_body: { $($rest)* },
                $($stack)*
            }
        }
    };

    // STATE: parse_arguments_helix

    {
        state: parse_arguments_helix,
        buffer: { $helix_arg:ident $($rest:tt)* },
        stack: { $($stack:tt)* }
    } => {
        parse! {
            state: parse_arguments_consume_possible_comma,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: initializer,
                    name: initialize,
                    self: {
                        ownership: { },
                        name: $helix_arg
                    },
                    args: uninitialized,
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    // STATE: parse_arguments_self

    {
        state: parse_arguments_self,
        buffer: { &mut $self_arg:ident $($rest:tt)* },
        stack: {
            name: $name:ident,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_arguments_consume_possible_comma,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: instance_method,
                    name: $name,
                    self: {
                        ownership: { &mut },
                        name: $self_arg
                    },
                    args: uninitialized,
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments_self,
        buffer: { & $self_arg:ident $($rest:tt)* },
        stack: {
            name: $name:ident,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_arguments_consume_possible_comma,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: instance_method,
                    name: $name,
                    self: {
                        ownership: { & },
                        name: $self_arg
                    },
                    args: uninitialized,
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments_self,
        buffer: $buffer:tt,
        stack: {
            name: $name:ident,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_arguments,
            buffer: $buffer,
            stack: {
                method: {
                    type: class_method,
                    name: $name,
                    self: (),
                    args: uninitialized,
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    // STATE: parse_arguments_consume_possible_comma

    {
        state: parse_arguments_consume_possible_comma,
        buffer: { , $($rest:tt)+ },
        stack: $stack:tt
    } => {
        parse! {
            state: parse_arguments,
            buffer: { $($rest)+ },
            stack: $stack
        }
    };

    {
        state: parse_arguments_consume_possible_comma,
        buffer: { },
        stack: $stack:tt
    } => {
        parse! {
            state: parse_arguments,
            buffer: $buffer,
            stack: $stack
        }
    };

    // STATE: parse_arguments

    {
        state: parse_arguments,
        buffer: { $($args:tt)* },
        stack: {
            method: {
                type: $type:tt,
                name: $name:ident,
                self: $self:tt,
                args: uninitialized,
                ret: uninitialized,
                body: uninitialized
            },
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: $type,
                    name: $name,
                    self: $self,
                    args: [ $($args)* ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    // STATE: parse_return_type

    {
        state: parse_return_type,
        buffer: { -> $ret:ty $body:block $($rest:tt)* },
        stack: {
            method: {
                type: $type:tt,
                name: $name:ident,
                self: $self:tt,
                args: $args:tt,
                ret: uninitialized,
                body: uninitialized
            },
            $($stack:tt)*
        }
    } => {
        assert_no_explict_return_for_initializer!($type, ->);

        parse! {
            state: finish_method,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: $type,
                    name: $name,
                    self: $self,
                    args: $args,
                    ret: $ret,
                    body: $body
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_return_type,
        buffer: { $body:block $($rest:tt)* },
        stack: {
            method: {
                type: initializer,
                name: initialize,
                self: $self:tt,
                args: $args:tt,
                ret: uninitialized,
                body: uninitialized
            },
            class: {
                type: class,
                name: $name:ident,
                meta: $meta:tt,
                struct: $struct:tt,
                methods: $methods:tt
            },
            $($stack:tt)*
        }
    } => {
        parse! {
            state: finish_method,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: initializer,
                    name: initialize,
                    self: $self,
                    args: $args,
                    ret: $name,
                    body: $body
                },
                class: {
                    type: class,
                    name: $name,
                    meta: $meta,
                    struct: $struct,
                    methods: $methods
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_return_type,
        buffer: { $body:block $($rest:tt)* },
        stack: {
            method: {
                type: $type:tt,
                name: $name:ident,
                self: $self:tt,
                args: $args:tt,
                ret: uninitialized,
                body: uninitialized
            },
            $($stack:tt)*
        }
    } => {
        parse! {
            state: finish_method,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: $type,
                    name: $name,
                    self: $self,
                    args: $args,
                    ret: (),
                    body: $body
                },
                $($stack)*
            }
        }
    };

    // STATE: finish_method

    {
        state: finish_method,
        buffer: $buffer:tt,
        stack: {
            method: $method:tt,
            class: {
                type: class,
                name: $name:ident,
                meta: $meta:tt,
                struct: $struct:tt,
                methods: [ $($methods:tt),* ]
            },
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_methods,
            buffer: $buffer,
            stack: {
                class: {
                    type: class,
                    name: $name,
                    meta: $meta,
                    struct: $struct,
                    methods: [ $($methods,)* $method ]
                },
                $($stack)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! codegen {
    ($($ast:tt)*) => {
        println!("TODO: codegen; AST = {}", stringify!($($ast)*));
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_struct {
    (true, {
        type: class,
        name: $name:ident,
        meta: $meta:tt,
        struct: { $($struct:tt)+ },
        methods: $methods:tt
    }) => {};

    (false, {
        type: class,
        name: $name:ident,
        meta: $meta:tt,
        struct: (),
        methods: $methods:tt
    }) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_valid_self_arg {
    (self) => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_valid_arg {
    ($arg:ident) => {};
    (_) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_no_explict_return_for_initializer {
    (instance_method, $($rest:tt)*) => {};
    (class_method, $($rest:tt)*) => {};
    (initializer, ) => {};
}

fn main() {
    declare_types! {
        class Calculator {
            def add(lhs: f64, rhs: f64) -> f64 {
                Adder::new(lhs).call(rhs)
            }

            def multiply(lhs: f64, rhs: f64) -> f64 {
                Multiplier::new(lhs).call(rhs)
            }
        }

        class Adder {
            struct {
                lhs: f64
            }

            def initialize(helix, value: f64) {
                Adder { helix, value }
            }

            def call(&self, rhs: f64) -> f64 {
                self.lhs + rhs
            }
        }

        class Multiplier {
            struct {
                lhs: f64
            }

            def initialize(helix, value: f64) {
                Multiplier { helix, value }
            }

            def call(&self, rhs: f64) -> f64 {
                self.lhs * rhs
            }
        }
    }
}
