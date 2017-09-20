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

#[macro_export]
macro_rules! throw {
    ($msg:expr) => {
        panic!($crate::ExceptionInfo::with_message(String::from($msg)))
    }
}
