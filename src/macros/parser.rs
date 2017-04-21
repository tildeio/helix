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
        name: «ident»,
        meta: «Meta»,
        struct: ‹() | { «Field»,* }›
        methods: [ «Method»,* ]
    }

  Meta :
    {
        pub: «bool»,
        reopen: «bool»,
    }

  Field :
    «ident» : «ty»

  Method :
    {
        type: «MethodType»,
        name: «ident»,
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
        codegen_init! { $ast }
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
        buffer: { class $name:tt { $($body:tt)* } $($rest:tt)* },
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
            ast: [ $($ast:tt)* ]
        }
    } => {
        parse! {
            state: top_level,
            buffer: $program,
            stack: {
                ast: [ $($ast)* $class ]
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
        buffer: { $name:tt ( $($args:tt)* ) $($rest:tt)* },
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
        buffer: { $helix_arg:tt $($rest:tt)* },
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
        buffer: { &mut $self_arg:tt $($rest:tt)* },
        stack: {
            name: $name:tt,
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
        buffer: { & $self_arg:tt $($rest:tt)* },
        stack: {
            name: $name:tt,
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
            name: $name:tt,
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
            buffer: { },
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
                name: $name:tt,
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
                name: $name:tt,
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
                    ret: { $name },
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
                type: initializer,
                name: $name:tt,
                self: $self:tt,
                args: $args:tt,
                ret: uninitialized,
                body: uninitialized
            },
            class: {
                type: class,
                meta: { pub: $pub:tt, reopen: $reopen:tt, name: $class_name:tt },
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
                    name: $name,
                    self: $self,
                    args: $args,
                    ret: { $class_name },
                    body: $body
                },
                class: {
                    type: class,
                    meta: { pub: $pub, reopen: $reopen, name: $class_name },
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
                name: $name:tt,
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
                name: $name:ident,
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
                    name: $name,
                    meta: $meta,
                    struct: $struct,
                    methods: [ $($methods)* $method ]
                },
                $($stack)*
            }
        }
    };
}
