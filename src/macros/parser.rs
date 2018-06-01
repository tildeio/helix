/**
  # AST DEFINITION

  ## Symbols

  ⁞literal⁞ means the token must be a literal, but it also has additional
  mandatory hygiene metadata that must be carried along (i.e. the token
  cannot be synthesized in the macro).

  «bool» one of the literal true or false
  «ident» a Rust identifier
  «ty» a Rust type
  «block» a Rust block
  «CamelCased» is the name of a production defined in the grammar

  ‹production1 | production2 (| productionN)*› is a union of several
  productions, all of which are valid.

  All other literals are literal Rust tokens.

  ## Grammar

  Class :
    {
        type: class,
        rust_name: «ident»,
        ruby_name: { string },
        meta: «Meta»,
        struct: ‹() | { «Field»,* }›
        methods: [ «Method»,* ]
    }

  Meta :
    {
        pub: «bool»,
        reopen: «bool»
    }

  Field :
    «ident» : «ty»

  Method :
    {
        type: «MethodType»,
        rust_name: «ident»,
        ruby_name: { string },
        self: ‹() | «MethodSelf»›,
        args: «MethodArgs»,
        ret: { «ty» },
        body: «block»
    }

  MethodType : ‹initializer | instance_method | class_method›

  MethodSelf :
    {
        ownership: { «Ownership» },
        name: ‹⁞helix⁞ | ⁞self⁞›
    }

  MethodArgs :
    [ «MethodArg»,* ]

  MethodArg :
    «Name» : «ty»

  Name : ‹_ | «ident»›
*/

#[doc(hidden)]
#[macro_export]
macro_rules! parse {

    // STATE: top_level

    {
        state: top_level,
        buffer: {},
        stack: { ast: $ast:tt }
    } => {
        codegen! { $ast }
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
                ruby_name: uninitialized,
                pub: false,
                reopen: false,
                $($stack)*
            }
        }
    };

    // STATE: parse_class

    {
        state: parse_class,
        buffer: { #[ruby_name = $ruby_name:tt] $($rest:tt)* },
        stack: {
            ruby_name: uninitialized,
            pub: false,
            reopen: false,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class,
            buffer: { $($rest)* },
            stack: {
                ruby_name: { $ruby_name },
                pub: false,
                reopen: false,
                $($stack)*
            }
        }
    };

    {
        state: parse_class,
        buffer: { pub $($rest:tt)* },
        stack: {
            ruby_name: $ruby_name:tt,
            pub: false,
            reopen: false,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class,
            buffer: { $($rest)* },
            stack: {
                ruby_name: $ruby_name,
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
            ruby_name: $ruby_name:tt,
            pub: $pub:tt,
            reopen: false,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class,
            buffer: { $($rest)* },
            stack: {
                ruby_name: $ruby_name,
                pub: $pub,
                reopen: true,
                $($stack)*
            }
        }
    };

    {
        state: parse_class,
        buffer: { class $name:tt $($rest:tt)* },
        stack: {
            ruby_name: uninitialized,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_class,
            buffer: { class $name $($rest)* },
            stack: {
                ruby_name: { stringify!($name) },
                $($stack)*
            }
        }
    };

    {
        state: parse_class,
        buffer: { class $name:tt { $($body:tt)* } $($rest:tt)* },
        stack: {
            ruby_name: $ruby_name:tt,
            pub: $pub:tt,
            reopen: $reopen:tt,

            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_struct,
            buffer: { $($body)* },
            stack: {
                class: {
                    type: class,
                    rust_name: $name,
                    ruby_name: $ruby_name,
                    meta: { pub: $pub, reopen: $reopen },
                    struct: (),
                    methods: []
                },
                program: { $($rest)* },
                $($stack)*
            }
        }
    };

    // STATE: parse_struct

    {
        state: parse_struct,
        buffer: { struct { $($struct:tt)* } $($rest:tt)* },
        stack: {
            class: {
                type: class,
                rust_name: $rust_name:tt,
                ruby_name: $ruby_name:tt,
                meta: { pub: $pub:tt, reopen: $reopen:tt },
                struct: (),
                methods: []
            },
            $($stack:tt)*
        }
    } => {
        assert_not_reopen!({ reopen: $reopen }, "Cannot define a struct in `reopen class`");

        parse! {
            state: parse_methods,
            buffer: { $($rest)* },
            stack: {
                class: {
                    type: class,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    meta: { pub: $pub, reopen: $reopen },
                    struct: { $($struct)* },
                    methods: []
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_struct,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        parse! {
            state: parse_methods,
            buffer: $buffer,
            stack: $stack
        }
    };

    // STATE: parse_methods

    {
        state: parse_methods,
        buffer: {},
        stack: {
            class: $class:tt,
            program: $program:tt,
            ast: [ $($ast:tt)* ]
        }
    } => {
        assert_has_initialize!($class, "Classes defining a struct must implement `initialize`");

        parse! {
            state: top_level,
            buffer: $program,
            stack: {
                ast: [ $($ast)* $class ]
            }
        }
    };

    {
        state: parse_methods,
        buffer: $buffer:tt,
        stack: { $($stack:tt)* }
    } => {
        parse! {
            state: parse_method_attributes,
            buffer: $buffer,
            stack: {
                rust_name: uninitialized,
                ruby_name: uninitialized,
                ruby_visibility: public,
                $($stack)*
            }
        }
    };

    // STATE: parse_method_attributes

    {
        state: parse_method_attributes,
        buffer: { #[ruby_visibility = $ruby_visibility:tt] $($rest:tt)* },
        stack: {
            rust_name: uninitialized,
            ruby_name: $ruby_name:tt,
            ruby_visibility: public,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                rust_name: uninitialized,
                ruby_name: $ruby_name,
                ruby_visibility: $ruby_visibility,
                $($stack)*
            }
        }
    };

    {
        state: parse_method_attributes,
        buffer: { #[ruby_name = $ruby_name:tt] $($rest:tt)* },
        stack: {
            rust_name: uninitialized,
            ruby_name: uninitialized,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                rust_name: uninitialized,
                ruby_name: { $ruby_name },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_attributes,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        parse! {
            state: parse_method_name,
            buffer: $buffer,
            stack: $stack
        }
    };

    // STATE: parse_method_name

    {
        state: parse_method_name,
        buffer: { def $name:tt $($rest:tt)* },
        stack: {
            rust_name: uninitialized,
            ruby_name: uninitialized,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_method,
            buffer: { $($rest)* },
            stack: {
                rust_name: $name,
                ruby_name: { stringify!($name) },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_name,
        buffer: { def $name:tt $($rest:tt)* },
        stack: {
            rust_name: uninitialized,
            ruby_name: $ruby_name:tt,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_method,
            buffer: { $($rest)* },
            stack: {
                rust_name: $name,
                ruby_name: $ruby_name,
                $($stack)*
            }
        }
    };

    // STATE: parse_method

    {
        state: parse_method,
        buffer: { ( $($args:tt)* ) $($rest:tt)* },
        stack: {
            rust_name: initialize,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class: $class:tt,
            $($stack:tt)*
        }
    } => {
        assert_not_reopen!($class, "Cannot define `initialize` in `reopen class`");
        assert_has_struct!($class, "Cannot define `initialize` without a `struct`");

        parse! {
            state: parse_arguments_initialize,
            buffer: { $($args)* },
            stack: {
                ruby_visibility: $ruby_visibility,
                class_body: { $($rest)* },
                class: $class,
                $($stack)*
            }
        }
    };

    {
        state: parse_method,
        buffer: { ( $($args:tt)* ) $($rest:tt)* },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_arguments,
            buffer: { $($args)* },
            stack: {
                rust_name: $rust_name,
                ruby_name: $ruby_name,
                ruby_visibility: $ruby_visibility,
                class_body: { $($rest)* },
                $($stack)*
            }
        }
    };

    // STATE: parse_arguments_initialize

    {
        state: parse_arguments_initialize,
        buffer: { $helix_arg:tt, $($args:tt)+ },
        stack: {
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_helix_arg!($helix_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: initializer,
                    rust_name: initialize,
                    ruby_name: { "initialize" },
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { },
                        name: $helix_arg
                    },
                    args: [ $($args)* ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments_initialize,
        buffer: { $helix_arg:tt },
        stack: {
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_helix_arg!($helix_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: initializer,
                    rust_name: initialize,
                    ruby_name: { "initialize" },
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { },
                        name: $helix_arg
                    },
                    args: [ ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    // STATE: parse_arguments

    {
        state: parse_arguments,
        buffer: { &mut $self_arg:tt, $($args:tt)+ },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: instance_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { &mut },
                        name: $self_arg
                    },
                    args: [ $($args)* ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments,
        buffer: { &mut $self_arg:tt },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: instance_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { &mut },
                        name: $self_arg
                    },
                    args: [ ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments,
        buffer: { & $self_arg:tt, $($args:tt)+ },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: instance_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { & },
                        name: $self_arg
                    },
                    args: [ $($args)* ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments,
        buffer: { & $self_arg:tt },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: instance_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { & },
                        name: $self_arg
                    },
                    args: [ ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

{
        state: parse_arguments,
        buffer: { $self_arg:tt, $($args:tt)+ },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: instance_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { },
                        name: $self_arg
                    },
                    args: [ $($args)* ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments,
        buffer: { $self_arg:tt },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        assert_valid_self_arg!($self_arg);

        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: instance_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: {
                        ownership: { },
                        name: $self_arg
                    },
                    args: [ ],
                    ret: uninitialized,
                    body: uninitialized
                },
                $($stack)*
            }
        }
    };

    {
        state: parse_arguments,
        buffer: { $($args:tt)* },
        stack: {
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            class_body: $class_body:tt,
            $($stack:tt)*
        }
    } => {
        parse! {
            state: parse_return_type,
            buffer: $class_body,
            stack: {
                method: {
                    type: class_method,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: (),
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
                rust_name: $rust_name:tt,
                ruby_name: $ruby_name:tt,
                ruby_visibility: $ruby_visibility:tt,
                self: $self:tt,
                args: $args:tt,
                ret: uninitialized,
                body: uninitialized
            },
            $($stack:tt)*
        }
    } => {
        assert_no_explict_return_for_initializer!({ type: $type }, "`def initialize` cannot have an explicit return type");

        parse! {
            state: finish_method,
            buffer: { $($rest)* },
            stack: {
                method: {
                    type: $type,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: $self,
                    args: $args,
                    ret: { $ret },
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
                rust_name: $rust_method_name:tt,
                ruby_name: $ruby_method_name:tt,
                ruby_visibility: $ruby_visibility:tt,
                self: $self:tt,
                args: $args:tt,
                ret: uninitialized,
                body: uninitialized
            },
            class: {
                type: class,
                rust_name: $rust_class_name:ident,
                ruby_name: $ruby_class_name:tt,
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
                    rust_name: $rust_method_name,
                    ruby_name: $ruby_method_name,
                    ruby_visibility: $ruby_visibility,
                    self: $self,
                    args: $args,
                    ret: { $rust_class_name },
                    body: $body
                },
                class: {
                    type: class,
                    rust_name: $rust_class_name,
                    ruby_name: $ruby_class_name,
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
                rust_name: $rust_name:tt,
                ruby_name: $ruby_name:tt,
                ruby_visibility: $ruby_visibility:tt,
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
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    ruby_visibility: $ruby_visibility,
                    self: $self,
                    args: $args,
                    ret: { () },
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
                rust_name: $rust_name:ident,
                ruby_name: $ruby_name:tt,
                meta: $meta:tt,
                struct: $struct:tt,
                methods: [ $($methods:tt)* ]
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
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    meta: $meta,
                    struct: $struct,
                    methods: [ $($methods)* $method ]
                },
                $($stack)*
            }
        }
    };

    // Catch all

    { $($state:tt)* } => {
        parse_error!(
            "Unknown parser state. ",
            "This is possibly a bug in Helix itself, please file an issue ",
            "with the following debug information:\n\n",
            format_parser_state!($($state)*)
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_error {
    ($($message:expr),*) => { compile_error!(concat!("Parse Error! ", $($message),*)); };
}

#[doc(hidden)]
#[macro_export]
macro_rules! format_parser_state {
    {
        state: $state:tt,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        concat!("{\n",
            "  state: ", stringify!($state), ",\n",
            "  buffer: ", stringify!($buffer), ",\n",
            "  stack: ", stringify!($stack), ",\n",
        "}")
    };

    { $($state:tt)* } => { concat!("(CORRUPTED)\n", stringify!($($state)*)) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_not_reopen {
    {
        {
            type: class,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            meta: { pub: $pub:tt, reopen: $reopen:tt },
            struct: $struct:tt,
            methods: $methods:tt
        },
        $($message:expr),*
    } => { assert_not_reopen!({ reopen: $reopen }, $($message),*); };

    { { reopen: true }, $($message:expr),* } => { parse_error!($($message),*); };
    { { reopen: false }, $($message:expr),* } => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_has_initialize {
    {
        {
            type: class,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            meta: $meta:tt,
            struct: $struct:tt,
            methods: $methods:tt
        },
        $($message:expr),*
    } => { assert_has_initialize!({ struct: $struct }, $methods, $($message),*); };

    { { struct: () }, $methods:tt, $($message:expr),* } => {};
    { { struct: $struct:tt }, [ ], $($message:expr),* } => { parse_error!($($message),*); };
    { { struct: $struct:tt }, [ { type: initializer, $($rest:tt)* } $($methods:tt)* ], $($message:expr),* } => {};
    { { struct: $struct:tt }, [ $method:tt $($methods:tt)* ], $($message:expr),* } => {
        assert_has_initialize!({ struct: $struct }, [ $($methods)* ], $($message),*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_has_struct {
    {
        {
            type: class,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            meta: $meta:tt,
            struct: $struct:tt,
            methods: $methods:tt
        },
        $($message:expr),*
    } => { assert_has_struct!({ struct: $struct }, $($message),*); };

    { { struct: () }, $($message:expr),* } => { parse_error!($($message),*); };
    { { struct: $struct:tt }, $($message:expr),* } => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_valid_helix_arg {
    (helix) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_valid_self_arg {
    (self) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_no_explict_return_for_initializer {
    ({ type: instance_method }, $($message:expr),*) => {};
    ({ type: class_method }, $($message:expr),*) => {};
    ({ type: initializer }, $($message:expr),*) => { parse_error!($($message),*); };
}
