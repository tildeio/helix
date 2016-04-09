#[macro_export]
macro_rules! declare_types {
    { $(#[$attr:meta])* pub class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { $(#[$attr])* pub class $cls { $($body)* } $($rest)* }
    };

    { $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { $(#[$attr])* class $cls { $($body)* } $($rest)* }
    };

    { $(#[$attr:meta])* pub reopen class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        reopen_class! { $(#[$attr])* pub class $cls { $($body)* } $($rest)* }
    };

    { $(#[$attr:meta])* reopen class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        reopen_class! { $(#[$attr])* class $cls { $($body)* } $($rest)* }
    };

    { } => { };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_class {
    { $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        struct $cls($crate::sys::VALUE);

        class_definition! { $cls ; () ; () ; $($body)* }

        declare_types! { $($rest)* }
    };

    { $(#[$attr:meta])* pub class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        pub struct $cls($crate::sys::VALUE);

        class_definition! { $cls ; () ; (); $($body)* }

        declare_types! { $($rest)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! class_definition {
    ( $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* ) => {
        class_definition! {
            $cls ;
            ($($mimpl)* pub fn $name($self_arg, $($arg : $argty),*) -> $crate::sys::VALUE $body) ;
            ($($mdef)* {
                extern "C" fn __ruby_method__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let checked = __checked_call__(rb_self, $($arg),*);
                    match checked {
                        Ok(val) => val,
                        Err(err) => { println!("TYPE ERROR: {:?}", err); $crate::sys::Qnil }
                    }
                }

                fn __checked_call__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> Result<$crate::sys::VALUE, ::std::ffi::CString> {
                    #[allow(unused_imports)]
                    use $crate::{ToRust};

                    $(
                        let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
                    )*

                    $(
                        let $arg = $arg.to_rust();
                    )*

                    Ok(rb_self.$name($($arg),*))
                }

                let name = stringify!($name);
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::new(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    ( $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) $body:block $($rest:tt)* ) => {
        class_definition! { $cls ; ($($mimpl)*); ($($mdef)*); def $name( $self_arg, ) $body $($rest)* }
    };

    ( $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; ) => {
        item! {
            impl $cls {
                $($mimpl)*
            }
        }

        init! {
            $crate::ClassDefinition::new(stringify!($cls))$(.define_method($mdef))*;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! reopen_class {
    { $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        struct $cls($crate::sys::VALUE);

        reopen_class_definition! { $cls ; () ; () ; $($body)* }

        declare_types! { $($rest)* }
    };

    { $(#[$attr:meta])* pub class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        pub struct $cls($crate::sys::VALUE);

        reopen_class_definition! { $cls ; () ; (); $($body)* }

        declare_types! { $($rest)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! reopen_class_definition {
    ( $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* ) => {
        reopen_class_definition! {
            $cls ;
            ($($mimpl)* pub fn $name($self_arg, $($arg : $argty),*) -> $crate::sys::VALUE $body) ;
            ($($mdef)* {
                extern "C" fn __ruby_method__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let checked = __checked_call__(rb_self, $($arg),*);
                    match checked {
                        Ok(val) => val,
                        Err(err) => { println!("TYPE ERROR: {:?}", err); $crate::sys::Qnil }
                    }
                }

                fn __checked_call__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> Result<$crate::sys::VALUE, ::std::ffi::CString> {
                    #[allow(unused_imports)]
                    use $crate::{ToRust};

                    $(
                        let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
                    )*

                    $(
                        let $arg = $arg.to_rust();
                    )*

                    Ok(rb_self.$name($($arg),*))
                }

                let name = stringify!($name);
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::new(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    ( $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) $body:block $($rest:tt)* ) => {
        reopen_class_definition! { $cls ; ($($mimpl)*); ($($mdef)*); def $name( $self_arg, ) $body $($rest)* }
    };

    ( $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; ) => {
        item! {
            impl $cls {
                $($mimpl)*
            }
        }

        init! {
            $crate::ClassDefinition::reopen(stringify!($cls))$(.define_method($mdef))*;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! item {
    ($it: item) => { $it }
}

#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[doc(hidden)]
#[macro_export]
macro_rules! method_arity {
  ( $($id:pat ),* ) => {
    { 0isize $(+ replace_expr!($id 1isize))* }
  }
}