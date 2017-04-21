#[macro_export]
macro_rules! codegen_init {
    { [ $($class:tt)* ] } => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn Init_native() {
            $crate::sys::check_version();

            $(
                codegen_class_coercions!($class);
                codegen_class_binding!($class, $class);
            )*
        }
    }
}

#[macro_export]
macro_rules! codegen_class_binding {
    { $class:tt, {
        type: class,
        name: $name:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: (),
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;
        let def = $crate::ClassDefinition::new(cstr!(stringify!($name)));

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $name = transmute(def.class) };
    });

    { $class:tt, {
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: { $($struct:tt)* },
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;

        extern "C" fn __mark__(_klass: &$cls) {}
        extern "C" fn __free__(_klass: Option<Box<$cls>>) {}

        extern "C" fn __alloc__(_klass: $crate::sys::VALUE) -> $crate::sys::VALUE {
            __alloc_with__(None)
        }

        fn __alloc_with__(rust_self: Option<Box<$cls>>) -> $crate::sys::VALUE {
            unsafe {
                let instance = $crate::sys::Data_Wrap_Struct(
                    transmute($cls),
                    transmute(__mark__ as usize),
                    transmute(__free__ as usize),
                    transmute(rust_self)
                );

                instance
            }
        }

        let def = $crate::ClassDefinition::wrapped(cstr!(stringify!($cls)), __alloc__);

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $cls = transmute(def.class) }
    });

}

#[macro_export]
macro_rules! codegen_define_method {
    ($def:tt, {
        type: class,
        name: $cls:tt,
        $($rest:tt)*
    }, {
        type: class_method,
        name: $name:tt,
        self: (),
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE, SPRINTF_TO_S, Qnil, rb_raise};

        #[repr(C)]
        struct CallResult {
            error_klass: VALUE,
            value: VALUE
        }

        extern "C" fn __ruby_method__(_: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            let result = __rust_method__($($arg),*);

            if result.error_klass == unsafe { Qnil } {
                result.value
            } else {
                unsafe { rb_raise(result.error_klass, SPRINTF_TO_S, result.value) }
            }
        }

        #[inline]
        fn __rust_method__($($arg : $crate::sys::VALUE),*) -> CallResult {
            let checked = __checked_call__($($arg),*);

            match checked {
                Ok(val) => CallResult { error_klass: unsafe { Qnil }, value: $crate::ToRuby::to_ruby(val) },
                Err(err) => CallResult { error_klass: err.exception.inner(), value: err.message }
            }
        }

        #[inline]
        fn __checked_call__($($arg : $crate::sys::VALUE),*) -> Result<$($ret)*, $crate::ExceptionInfo> {
            #[allow(unused_imports)]
            use $crate::{ToRust};

            $(
                let $arg = match $crate::UncheckedValue::<$argty>::to_checked($arg) {
                    Ok(v) => v,
                    Err(e) => return Err($crate::ExceptionInfo::type_error(e))
                };
            )*

            $(
                let $arg = $crate::ToRust::to_rust($arg);
            )*

            handle_exception! {
                $cls::$name($($arg),*)
            }
        }

        let name = cstr!(stringify!($name));
        let method = __ruby_method__ as *const $crate::libc::c_void;
        let arity = method_arity!($($arg)*);

        $def.define_method($crate::MethodDefinition::class(name, method, arity));
    });

    ($def:tt, {
        type: class,
        name: $cls:tt,
        meta: $meta:tt,
        struct: $struct:tt,
        $($rest:tt)*
    }, {
        type: instance_method,
        name: $name:tt,
        self: { ownership: { $($ownership:tt)* }, name: $self:tt },
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE, SPRINTF_TO_S, Qnil, rb_raise};

        #[repr(C)]
        struct CallResult {
            error_klass: VALUE,
            value: VALUE
        }

        extern "C" fn __ruby_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            let result = __rust_method__(rb_self, $($arg),*);

            if result.error_klass == unsafe { Qnil } {
                result.value
            } else {
                unsafe { rb_raise(result.error_klass, SPRINTF_TO_S, result.value) }
            }
        }

        #[inline]
        fn __rust_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> CallResult {
            let checked = __checked_call__(rb_self, $($arg),*);

            match checked {
                Ok(val) => CallResult { error_klass: unsafe { Qnil }, value: $crate::ToRuby::to_ruby(val) },
                Err(err) => CallResult { error_klass: err.exception.inner(), value: err.message }
            }
        }

        #[inline]
        fn __checked_call__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> Result<$($ret)*, $crate::ExceptionInfo> {
            #[allow(unused_imports)]
            use $crate::{ToRust};

            let rust_self = match $crate::UncheckedValue::<codegen_self_pointer_type! { struct: $struct, ownership: { $($ownership)* }, type: $cls }>::to_checked(rb_self) {
                Ok(v)  => v,
                Err(e) => return Err($crate::ExceptionInfo::with_message(e))
            };

            $(
                let $arg = match $crate::UncheckedValue::<$argty>::to_checked($arg) {
                    Ok(v) => v,
                    Err(e) => return Err($crate::ExceptionInfo::type_error(e))
                };
            )*

            let rust_self = rust_self.to_rust();

            $(
                let $arg = $crate::ToRust::to_rust($arg);
            )*

            handle_exception! {
                rust_self.$name($($arg),*)
            }
        }

        let name = cstr!(stringify!($name));
        let method = __ruby_method__ as *const $crate::libc::c_void;
        let arity = method_arity!($($arg)*);

        $def.define_method($crate::MethodDefinition::instance(name, method, arity))
    });

    ($def:tt, {
        type: class,
        name: $cls:tt,
        meta: $meta:tt,
        struct: $struct:tt,
        $($rest:tt)*
    }, {
        type: initializer,
        name: $name:tt,
        self: { ownership: {}, name: $self:tt },
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        impl $cls {
            pub fn new($($arg : $argty),*) -> $($ret)* {
                $cls::$name(unsafe { $crate::sys::Qnil } , $($arg),*)
            }
        }

        extern "C" fn __initialize__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            let result = __checked_initialize__(rb_self $(, $arg)*);

            match result {
                Ok(rust_self) => {
                    let data = Box::new(rust_self);
                    unsafe { $crate::sys::Data_Set_Struct_Value(rb_self, ::std::mem::transmute(data)) };
                }
                Err(err) => { println!("TYPE ERROR: {:?}", err); }
            }

            rb_self
        }

        fn __checked_initialize__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> Result<$cls, String> {
            #[allow(unused_imports)]
            use $crate::{ToRust};

            $(
                let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
            )*

            $(
                let $arg = $crate::ToRust::to_rust($arg);
            )*

            Ok($cls::initialize(rb_self, $($arg),*))
        }

        let arity = method_arity!($($arg)*);
        let method = __initialize__ as *const $crate::libc::c_void;

        $def.define_method($crate::MethodDefinition::instance(cstr!("initialize"), method, arity));
    });
}

#[macro_export]
macro_rules! codegen_class_coercions {
    ({
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: (),
        methods: $methods:tt
    }) => (
        impl $crate::UncheckedValue<$cls> for $crate::sys::VALUE {
            fn to_checked(self) -> $crate::CheckResult<$cls> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $cls == ::std::mem::transmute(sys::rb_obj_class(self)) } {
                    Ok(unsafe { CheckedValue::new(self) })
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self)).to_string_lossy() };
                    Err(format!("No implicit conversion of {} into {}", val, stringify!($cls)))
                }
            }
        }

        impl $crate::ToRust<$cls> for $crate::CheckedValue<$cls> {
            fn to_rust(self) -> $cls {
                $cls { helix: self.inner }
            }
        }

        impl_to_ruby!($cls);
        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);
    );

    ({
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: $methods:tt
    }) => (
        impl_struct_to_rust!(&'a $cls, $cls);
        impl_struct_to_rust!(&'a mut $cls, $cls);

        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);

    );
}

#[macro_export]
macro_rules! codegen_self_pointer_type {
    {
        struct: (),
        ownership: $ownership:tt,
        type: $type:tt
    } => {
        $type
    };

    {
        struct: $struct:tt,
        ownership: { $($ownership:tt)* },
        type: $type:tt
    } => {
        $($ownership)* $type
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_to_ruby {
    ($cls:ty) => {
        item! {
            impl<'a> $crate::ToRuby for $cls {
                fn to_ruby(self) -> $crate::sys::VALUE {
                    self.helix
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_struct_to_rust {
    ($cls:ty, $helix_id:tt) => {
        impl<'a> $crate::ToRust<$cls> for $crate::CheckedValue<$cls> {
            fn to_rust(self) -> $cls {
                unsafe { ::std::mem::transmute($crate::sys::Data_Get_Struct_Value(self.inner)) }
            }
        }

        impl<'a> $crate::UncheckedValue<$cls> for $crate::sys::VALUE {
            fn to_checked(self) -> $crate::CheckResult<$cls> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $helix_id == ::std::mem::transmute(sys::rb_obj_class(self)) } {
                    if unsafe { $crate::sys::Data_Get_Struct_Value(self) == ::std::ptr::null_mut() } {
                        Err(format!("Uninitialized {}", $crate::inspect(unsafe { sys::rb_obj_class(self) })))
                    } else {
                        Ok(unsafe { CheckedValue::new(self) })
                    }
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self)).to_string_lossy() };
                    Err(format!("No implicit conversion of {} into {}", val, $crate::inspect(unsafe { sys::rb_obj_class(self) })))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! method_arity {
  ( $($arg:tt)* ) => {
    { 0isize $(+ replace_expr!($arg 1isize))* }
  }
}

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[doc(hidden)]
#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char] as *const ::std::os::raw::c_char
    )
}

#[macro_export]
macro_rules! handle_exception {
    { $($body:tt)* } => {
        let hide_err = ::std::env::var("RUST_BACKTRACE").is_err();
        if hide_err {
            ::std::panic::set_hook(Box::new(|_| {
                // Silence
            }));
        }

        let res = ::std::panic::catch_unwind(|| {
            $($body)*
        });

        if hide_err {
            let _ = ::std::panic::take_hook();
        }

        res.map_err(|e| $crate::ExceptionInfo::from_any(e))
    }
}
