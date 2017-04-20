#![recursion_limit="1024"]

type Metadata = *mut std::os::raw::c_void;

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
        meta: «Meta»,
        struct: ‹() | { «Field»,+ }›
        methods: [ «Method»,+ ]
    }

  Meta :
    {
        pub: «bool»,
        reopen: «bool»,
        name: «ident»,
    }

  Field :
    «ident» : «ty»

  Method :
    {
        type: «MethodType»,
        name: «ident»,
        self: ‹() | «MethodSelf»›,
        args: «MethodArgs»,
        ret: «ty»,
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
        codegen! { $ast };
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
#[macro_use]
mod codegen_ast {
    #[macro_export]
    macro_rules! codegen {
        { [ $($ast:tt)* ] } => {
            // println!("{}\n\n======\n\n", stringify!($($ast)*));
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
                $($classes:tt)*
            ],
            buffer: [
                {
                    type: class,
                    meta: { pub: $pub:tt, reopen: $reopen:tt, name: $name:tt  },
                    struct: $struct:tt,
                    methods: $methods:tt
                }
                $(, $rest:tt)*
            ]
        } => {
            codegen! {
                type: class,
                classes: [
                    {
                        name: $name,
                        struct: { codegen_struct! { pub: $pub, name: $name, struct: $struct } },
                        methods: []
                    }
                    $($classes)*
                ],
                buffer: [ $methods $($rest),* ]
            }
        };

        {
            type: top,
            classes: $classes:tt,
            buffer: [ $next:tt $(, $rest:tt)* ]
        } => {
            println!("UNIMPLEMENTED IN BUFFER: {}", stringify!($next));
        };

        {
            type: class,
            classes: [
                {
                    name: $name:tt,
                    struct: $struct:tt,
                    methods: []
                }
                $($classes:tt)*
            ],
            buffer: [ [ $($method:tt),* ] $($rest:tt),* ]
        } => {
            codegen! {
                type: top,
                classes: [
                    {
                        name: $name,
                        struct: $struct,
                        methods: [ $( codegen_method! { $method } )* ]
                    }
                    $($classes)*
                ],
                buffer: [ $($rest),* ]
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

        { $($any:tt)* } => {
            println!("unimplemented; AST = {}", stringify!($($any)*));
        };
    }

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
            $($pub)* struct $name {
                helix: $crate::Metadata,
                $($struct)*
            }
        }
    }

    macro_rules! codegen_method {
        {
            {
                type: initializer,
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
            pub fn $name($($ownership)* $self : $crate::Metadata, $($args)*) -> $($ret)* $body
        };

        {
            $($any:tt)*
        } => {}
    }
}

/**
[
    {
        type : class ,
        meta : { pub : false , reopen : false , name : Multiplier } ,
        struct : { lhs : f64 } ,
        methods : [
            {
                type : initializer ,
                name : initialize ,
                self : {
                    ownership : {  } ,
                    name : helix
                } ,
                args : [ value : f64 ] ,
                ret : Multiplier ,
                body : { Multiplier{helix, value,} }
            } ,
            {
                type : instance_method ,
                name : call ,
                self : {
                    ownership : { & } ,
                    name : self
                } ,
                args : [ rhs : f64 ] ,
                ret : f64 ,
                body : { self.lhs * rhs }
            }
        ]
    }
]
*/

#[doc(hidden)]
#[macro_export]
macro_rules! item {
    ($it: item) => { $it }
}

mod print {
    macro_rules! print_ast {
        ([ $($ast:tt),* ]) => {
            $(
                print_ast!($ast);
            )*
        };

        ({
            type: class,
            meta: { pub: $pub:tt, reopen: $reopen:tt, name: $name:tt },
            struct: $struct:tt,
            methods: $methods:tt
        }) => {
            println!("\nCLASS\n=====\nclass {} (pub:{} reopen:{})", stringify!($name), stringify!($pub), stringify!($reopen));
            println!("{}", format_ast_class_struct!({ name: $name, tuple: $struct }));
            println!("\nMETHODS\n=======\n{}\n\n", format_ast_methods!($methods));
        };

        ($($ast:tt)*) => {
            println!("Unimplemented: {}", stringify!($($ast)*));
        };
    }

    macro_rules! format_ast_class_struct {
        ({
            name: $name:tt,
            tuple: { $( $field:tt : $fieldty:ty ),* }
        }) => {
            {
                let mut s = String::new();
                $(
                    s.push_str(&format!("struct {} {{ {}: {} }}", stringify!($name), stringify!($field), stringify!($fieldty)));
                )*
                s
            }
        };

        ({
            name: $name:tt,
            tuple: ()
        }) => {
            "No Struct"
        };
    }

    macro_rules! format_ast_methods {
        ([
            $($ast:tt),*
        ]) => {
            {
                let mut v = Vec::new();
                $(
                    v.push(format_ast_methods!($ast));
                )*
                v.join("\n")
            }
        };

        ({
            type: instance_method,
            name: $name:tt,
            self: $self:tt,
            args: $args:tt,
            ret: $ret:tt,
            body: $body:tt
        }) => {
            {
                let mut s = String::new();
                s.push_str(&format!("def {}", stringify!($name)));
                s.push_str(&format!("({})", format_ast_args!($args)));
                s.push_str(&format_ret!($ret));
                s.push_str(" ");
                s.push_str(&stringify!($body));
                s
            }
        };

        ({
            type: class_method,
            name: $name:tt,
            self: $self:tt,
            args: $args:tt,
            ret: $ret:ty,
            body: $body:tt
        }) => {
            {
                let mut s = String::new();
                s.push_str(&format!("def self.{}", stringify!($name)));
                s.push_str(&format!("({})", format_ast_args!($args)));
                s.push_str(&format_ret!($ret));
                s.push_str(" ");
                s.push_str(&stringify!($body));
                s
            }
        };


        ({
            type: initializer,
            name: $name:tt,
            self: $self:tt,
            args: $args:tt,
            ret: $ret:ty,
            body: $body:tt
        }) => {
            {
                let mut s = String::new();
                s.push_str(&format!("def initialize({})", format_ast_args!($args)));
                s.push_str(&format_ret!($ret));
                s.push_str(" ");
                s.push_str(&stringify!($body));
                s
            }
        };
    }

    macro_rules! format_ret {
        ($ret:ty) => { format!(" -> {}", stringify!($ret)) };
    }

    macro_rules! format_ast_args {
        ([ $($args:tt)* ]) => {
            format!("{}", stringify!($($args)*))
        }
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
                Adder { helix, lhs: value }
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
                Multiplier { helix, lhs: value }
            }

            def call(&self, rhs: f64) -> f64 {
                self.lhs * rhs
            }
        }
    }
}
