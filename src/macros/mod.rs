#[macro_use]
mod parser;

#[macro_use]
mod codegen;

#[macro_use]
mod init;

#[macro_use]
mod coercions;

#[macro_use]
mod alloc;

#[macro_export]
macro_rules! ruby {
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
macro_rules! item {
    ($it: item) => { $it }
}


#[doc(hidden)]
#[macro_export]
macro_rules! assert_struct {
    (true, {
        type: class,
        rust_name: $rust_name:ident,
        ruby_name: $ruby_name:tt,
        meta: $meta:tt,
        struct: { $($struct:tt)+ },
        methods: $methods:tt
    }) => {};

    (false, {
        type: class,
        rust_name: $rust_name:ident,
        ruby_name: $ruby_name:tt,
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


#[macro_export]
macro_rules! throw {
    ($msg:expr) => {
        panic!($crate::ExceptionInfo::with_message(String::from($msg)))
    }
}

