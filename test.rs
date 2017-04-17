/**
  rest={ <next> ...rest }
  init={}
*/

#[macro_export]
macro_rules! assert_valid_self_arg {
    (self) => {}
}

#[macro_export]
macro_rules! declare_types {
    { $($rest:tt)* } => {
        declare_types_internal! {
            state: top_level,
            program: {
                rest: { $($rest)* },
                init: {}
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_types_internal {
    // STATE: top_level

    {
        state: top_level,
        program: {
            rest: { },
            init: $init: tt
        }
    } => {
        println!("DONE {}", stringify!($init));
    };

    {
        state: top_level,
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_class,
            pub: false,
            reopen: false,
            program: $program
        }
    };

    // STATE: parse_class

    {
        state: parse_class,
        pub: false,
        reopen: false,
        program: {
          rest: { pub $($rest:tt)* },
          init: $init:tt
        }
    } => {
        declare_types_internal! {
            state: parse_class,
            pub: true,
            reopen: false,
            program: {
              rest: { $($rest)* },
              init: $init
            }
        }
    };

    {
        state: parse_class,
        pub: $pub:tt,
        reopen: false,
        program: {
          rest: { reopen $($rest:tt)* },
          init: $init:tt
        }
    } => {
        declare_types_internal! {
            state: parse_class,
            pub: $pub,
            reopen: true,
            program: {
              rest: { $($rest)* },
              init: $init
            }
        }
    };

    {
        state: parse_class,
        pub: $pub:tt,
        reopen: $reopen:tt,
        program: {
          rest: { class $name:ident { $($body:tt)* } $($rest:tt)* },
          init: $init:tt
        }
    } => {
        declare_types_internal! {
            state: class,
            class: {
                meta: { pub: $pub, reopen: $reopen, name: $name },
                body: { $($body)* }
            },
            program: {
              rest: { $($rest)* },
              init: $init
            }
        }
    };

    // STATE: class

    {
        state: class,
        class: {
            meta: $meta:tt,
            body: { def $($rest:tt)* }
        },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_method,
            class: {
                meta: $meta,
                body: { $($rest)* }
            },
            program: $program
        }
    };

    {
        state: class,
        class: {
            meta: $meta:tt,
            body: { struct $($rest:tt)* }
        },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_struct,
            class: {
                meta: $meta,
                body: { $($rest)* }
            },
            program: $program
        }
    };

    {
        state: class,
        class: {
            meta: $meta:tt,
            body: { }
        },
        program: $program:tt
    } => {
        println!("{}", stringify!(class: {
            meta: $meta
        }));

        declare_types_internal! {
            state: top_level,
            program: $program
        }
    };

    // STATE: parse_method

    {
        state: parse_method,
        class: {
            meta: $meta:tt,
            body: { initialize $($rest:tt)* }
        },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_initialize,
            class: {
                meta: $meta,
                body: { $($rest)* }
            },
            method: {
                type: initializer,
                name: initialize,
            },
            program: $program
        }
    };

    {
        state: parse_method,
        class: {
            meta: $meta:tt,
            body: { $name:ident ( $($args:tt)* ) $($rest:tt)* }
        },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_arguments_self,
            class: {
                meta: $meta,
                body: { $($rest)* }
            },
            method: {
                name: $name,
            },
            args: { $($args)* },
            program: $program
        }
    };

    // STATE: parse_arguments_self

    {
        state: parse_arguments_self,
        class: $class:tt,
        method: {
            name: $name:tt,
        },
        args: { & mut $self_arg:ident $($rest:tt)* },
        program: $program:tt
    } => {
        assert_valid_self_arg!($self_arg);

        declare_types_internal! {
            state: parse_arguments_consume_possible_comma,
            class: $class,
            method: {
                type: instance,
                name: $name,
                self: {
                    ownership: { & mut },
                    name: $self_arg
                }
            },
            args: { $($rest)* },
            program: $program
        }
    };

    {
        state: parse_arguments_self,
        class: $class:tt,
        method: {
            name: $name:tt,
        },
        args: { & $self_arg:ident $($rest:tt)* },
        program: $program:tt
    } => {
        assert_valid_self_arg!($self_arg);

        declare_types_internal! {
            state: parse_arguments_consume_possible_comma,
            class: $class,
            method: {
                type: instance,
                name: $name,
                self: {
                    ownership: { & },
                    name: $self_arg
                }
            },
            args: { $($rest)* },
            program: $program
        }
    };

    {
        state: parse_arguments_self,
        class: $class:tt,
        method: {
            name: $name:tt,
        },
        args: { $($rest:tt)* },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_arguments,
            class: $class,
            method: {
                type: class,
                name: $name,
                self: ()
            },
            args: { $($rest)* },
            program: $program
        }
    };

    // STATE: parse_arguments_consume_possible_comma

    {
        state: parse_arguments_consume_possible_comma,
        class: $class:tt,
        method: $method:tt,
        args: { , $($rest:tt)+ },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_arguments,
            class: $class,
            method: $method,
            args: { $($rest)+ },
            program: $program
        }
    };

    {
        state: parse_arguments_consume_possible_comma,
        class: $class:tt,
        method: $method:tt,
        args: { },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_arguments,
            class: $class,
            method: $method,
            args: { },
            program: $program
        }
    };

    // STATE: parse_arguments

    {
        state: parse_arguments,
        class: $class:tt,
        method: {
            type: $type:ident,
            name: $name:tt,
            self: $self:tt
        },
        args: { $($args:tt)* },
        program: $program:tt
    } => {
        declare_types_internal! {
            state: parse_return_type,
            class: $class,
            method: {
                type: $type,
                name: $name,
                self: $self,
                args: { $($args)* }
            },
            program: $program
        }
    };

    // STATE: parse_return_type

    {
        state: parse_return_type,
        class: {
            meta: $meta:tt,
            body: { -> $ret:ty $body:block $($rest:tt)* }
        },
        method: {
            type: $type:ident,
            name: $name:tt,
            self: $self:tt,
            args: $args:tt
        },
        program: $program:tt
    } => {
        println!("{}", stringify!(method: {
            type: $type,
            name: $name,
            self: $self,
            args: $args,
            ret: $ret
        }));

        declare_types_internal! {
            state: class,
            class: {
                meta: $meta,
                body: { $($rest)* }
            },
            program: $program
        }
    };

    {
        state: parse_return_type,
        class: {
            meta: $meta:tt,
            body: { $body:block $($rest:tt)* }
        },
        method: {
            type: $type:ident,
            name: $name:tt,
            self: $self:tt,
            args: $args:tt
        },
        program: $program:tt
    } => {
        println!("{}", stringify!(method: {
            type: $type,
            name: $name,
            self: $self,
            args: $args,
            ret: ()
        }));

        declare_types_internal! {
            state: class,
            class: {
                meta: $meta,
                body: { $($rest)* }
            },
            program: $program
        }
    };

    // // STATE: class_signature

    // {
    //     state: class_signature,
    //     class_tokens: { }
    //     program: {
    //         rest: {
    //             $($signature:ident)* class $name:ident { $($body:tt)* }
    //             $($rest:tt)*
    //         },
    //         init: $init:tt
    //     }
    // } => {
    //     declare_types_internal! {
    //         class_signature: {
    //           rest: { $($signature)* },
    //           body: { $(body)* },
    //           accum: { pub: false, reopen: false }
    //         },
    //         accum: { rest: { $($rest)* }, init: $init }
    //     }
    // };

    // // Step 2: parse the class signature

    // {
    //     class_signature: {
    //         rest: { pub $($rest:ident)* },
    //         body: $body:tt,
    //         accum: { pub: $pub:tt, reopen: false }
    //     },
    //     accum: $accum:tt
    // } => {
    //     declare_types_internal! {
    //         class_signature: {
    //           rest: { $($rest)* },
    //           body: $body,
    //           accum: { pub: true, reopen: false }
    //         },
    //         accum: $accum
    //     }
    // };

    // {
    //     class_signature: {
    //         rest: { reopen $($rest:ident)* },
    //         body: $body:tt,
    //         accum: { pub: $pub:tt, reopen: false }
    //     },
    //     accum: $accum:tt
    // } => {
    //     declare_types_internal! {
    //         class_signature: {
    //           rest: { $($rest)* },
    //           body: $body,
    //           accum: { pub: $pub, reopen: true }
    //         },
    //         accum: $accum
    //     }
    // };

    // {
    //     class_signature: {
    //         rest: { class $name:ident },
    //         body: $body:tt
    //         accum: { pub: $pub:tt, reopen: $reopen:tt }
    //     },
    //     accum: $accum:tt
    // } => {
    //     declare_types_internal! {
    //         class: {
    //           meta: { name: $name, pub: $pub, reopen: $reopen }
    //           rest: { $(body)* },
    //           accum: { }
    //         },
    //         accum: $accum
    //     }
    // };

    // // Step 3: take a method out of rest

    // {
    //     class: {
    //         meta: $meta:tt,
    //         rest: {
    //             def $name:ident($($args:tt)*) $(-> $ret:ty)* { $($body:tt)* }
    //             $($rest:tt)*
    //         },
    //         accum: $class_accum:tt
    //     },
    //     accum: $accum:tt
    // } => {
    //     declare_types_internal! {
    //         method: {
    //             def $name:ident($($args)*) $(-> $ret)* { $($body)* }
    //         },

    //         class: {
    //             meta:     $meta,
    //             rest:     { $($rest)* },
    //             elements: $elements
    //         },

    //         accum: $class_accum
    //     }
    // };

    // { $($all:tt)* } => {
    //     println!("ERROR: {}", stringify!($($all)*));
    // };
}

fn main() {
    declare_types! {
        class Foo {
            def multiply(&self, one: f64, two: f64) -> f64 {
                one * two
            }
        }
    }
}
