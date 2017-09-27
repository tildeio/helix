#[macro_export]
macro_rules! codegen_init {
    { [ $($class:tt)* ] } => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn Init_native() {
            $crate::sys::check_version();

            $(
                codegen_class_binding!($class, $class);
            )*
        }
    }
}

#[macro_export]
macro_rules! codegen_class_binding {
    { $class:tt, {
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        meta: { pub: $pub:tt, reopen: false },
        struct: (),
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;
        let def = $crate::ClassDefinition::new(cstr!($($ruby_name)*));

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $rust_name = transmute(def.class) };
    });

    { $class:tt, {
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        meta: { pub: $pub:tt, reopen: true },
        struct: (),
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;
        let def = $crate::ClassDefinition::reopen(cstr!($($ruby_name)*));

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $rust_name = transmute(def.class) };
    });

    { $class:tt, {
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: { $($struct:tt)* },
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;

        extern "C" fn __alloc__(_klass: $crate::sys::VALUE) -> $crate::sys::VALUE {
            $rust_name::__alloc_with__(None)
        }

        let def = $crate::ClassDefinition::wrapped(cstr!($($ruby_name)*), __alloc__);

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $rust_name = transmute(def.class) }
    });

}

#[macro_export]
macro_rules! codegen_define_method {
    ($def:tt, {
        type: class,
        rust_name: $cls_rust_name:tt,
        $($rest:tt)*
    }, {
        type: class_method,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        self: (),
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE};
        use $crate::{Error};

        extern "C" fn __ruby_method__(_: VALUE, $($arg : VALUE),*) -> VALUE {
            let result = __rust_method__($($arg),*);

            match result {
                Ok(value) => return value,
                Err(exception) => unsafe { exception.raise() }
            }
        }

        #[inline]
        fn __rust_method__($($arg : $crate::sys::VALUE),*) -> Result<VALUE, Error> {
            #[allow(unused_imports)]
            use $crate::{FromRuby, ToRuby};

            $(
                let $arg = try!(<$argty>::from_ruby($arg));
            )*

            $(
                let $arg = <$argty>::from_checked($arg);
            )*

            let result: Result<$($ret)*, Error> = handle_exception! {
                $cls_rust_name::$rust_name($($arg),*)
            };

            result.and_then(ToRuby::to_ruby)
        }

        let name = cstr!($($ruby_name)*);
        let method = __ruby_method__ as *const $crate::libc::c_void;
        let arity = method_arity!($($arg)*);

        $def.define_method($crate::MethodDefinition::class(name, method, arity));
    });

    ($def:tt, {
        type: class,
        rust_name: $cls_rust_name:tt,
        ruby_name: $cls_ruby_name:tt,
        meta: $meta:tt,
        struct: $struct:tt,
        $($rest:tt)*
    }, {
        type: instance_method,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        self: { ownership: { $($ownership:tt)* }, name: $self:tt },
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE};
        use $crate::{Error};

        extern "C" fn __ruby_method__(rb_self: VALUE, $($arg : VALUE),*) -> VALUE {
            let result = __rust_method__(rb_self, $($arg),*);

            match result {
                Ok(value) => return value,
                Err(exception) => unsafe { exception.raise() }
            }
        }

        #[inline]
        fn __rust_method__(rb_self: VALUE, $($arg : VALUE),*) -> Result<VALUE, Error> {
            #[allow(unused_imports)]
            use $crate::{FromRuby, ToRuby};

            let rust_self = try!(<codegen_self_pointer_type! { struct: $struct, ownership: { $($ownership)* }, type: $cls_rust_name }>::from_ruby(rb_self));

            $(
                let $arg = try!(<$argty>::from_ruby($arg));
            )*

            let rust_self = <codegen_self_pointer_type! { struct: $struct, ownership: { $($ownership)* }, type: $cls_rust_name }>::from_checked(rust_self);

            $(
                let $arg = <$argty>::from_checked($arg);
            )*

            let result: Result<$($ret)*, Error> = handle_exception! {
                rust_self.$rust_name($($arg),*)
            };

            result.and_then(ToRuby::to_ruby)
        }

        let name = cstr!($($ruby_name)*);
        let method = __ruby_method__ as *const $crate::libc::c_void;
        let arity = method_arity!($($arg)*);

        $def.define_method($crate::MethodDefinition::instance(name, method, arity))
    });

    ($def:tt, {
        type: class,
        rust_name: $cls_rust_name:tt,
        ruby_name: $cls_ruby_name:tt,
        meta: $meta:tt,
        struct: $struct:tt,
        $($rest:tt)*
    }, {
        type: initializer,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        self: { ownership: {}, name: $self:tt },
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE};
        use $crate::{Error};

        impl $cls_rust_name {
            pub fn new($($arg : $argty),*) -> $($ret)* {
                $cls_rust_name::$rust_name(unsafe { $crate::sys::Qnil } , $($arg),*)
            }
        }

        extern "C" fn __ruby_initialize__(rb_self: VALUE, $($arg : VALUE),*) -> VALUE {
            let result = __rust_initialize__(rb_self $(, $arg)*);

            match result {
                Ok(value) => return value,
                Err(exception) => unsafe { exception.raise() }
            }
        }

        #[inline]
        fn __rust_initialize__(rb_self: VALUE, $($arg : VALUE),*) -> Result<VALUE, Error> {
            #[allow(unused_imports)]
            use $crate::{FromRuby};
            use $crate::sys::{Data_Set_Struct_Value};

            $(
                let $arg = try!(<$argty>::from_ruby($arg));
            )*

            $(
                let $arg = <$argty>::from_checked($arg);
            )*

            let rust_self = Box::new($cls_rust_name::initialize(rb_self, $($arg),*));

            unsafe { Data_Set_Struct_Value(rb_self, ::std::mem::transmute(rust_self)) };

            Ok(rb_self)
        }

        let arity = method_arity!($($arg)*);
        let method = __ruby_initialize__ as *const $crate::libc::c_void;

        $def.define_method($crate::MethodDefinition::instance(cstr!($($ruby_name)*), method, arity));
    });
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

#[macro_export]
macro_rules! handle_exception {
    { $($body:tt)* } => {
        {
            let hide_err = ::std::env::var("RUST_BACKTRACE").is_err();

            if hide_err {
                ::std::panic::set_hook(Box::new(|_| {
                    // Silence
                }));
            }

            // TODO: Poison any objects that cross the boundary to prevent them
            // from being used in Ruby and triggering panics over and over again.
            let res = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                $($body)*
            }));

            if hide_err {
                let _ = ::std::panic::take_hook();
            }

            res.map_err(|e| $crate::Error::from_any(e))
        }
    }
}
