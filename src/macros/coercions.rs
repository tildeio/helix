#[macro_export]
macro_rules! codegen_coercions {
    ({
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: (),
        methods: $methods:tt
    }) => (
        impl<'a> $crate::UncheckedValue<$cls> for $crate::ruby::Value<'a> {
            type ToRust = $crate::CheckedValue<'a, $cls>;

            fn to_checked(self) -> $crate::CheckResult<Self::ToRust> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $cls == ::std::mem::transmute(sys::rb_obj_class(self.inner())) } {
                    Ok(unsafe { CheckedValue::new(self) })
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self.inner())).to_string_lossy() };
                    Err(format!("No implicit conversion of {} into {}", val, stringify!($cls)))
                }
            }
        }

        impl<'a> $crate::ToRust<$cls> for $crate::CheckedValue<'a, $cls> {
            fn to_rust(self) -> $cls {
                $cls { helix: unsafe { self.inner.inner() } }
            }
        }

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

        impl $crate::ToRuby for $cls {
            fn to_ruby(self) -> $crate::sys::VALUE {
                $cls::__alloc_with__(Some(Box::new(self)))
            }
        }

        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);
    );
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
        impl<'a> $crate::ToRust<$cls> for $crate::CheckedValue<'a, $cls> {
            fn to_rust(self) -> $cls {
                unsafe { ::std::mem::transmute($crate::sys::Data_Get_Struct_Value(self.inner.inner())) }
            }
        }

        impl<'a> $crate::UncheckedValue<$cls> for $crate::ruby::Value<'a> {
            type ToRust = $crate::CheckedValue<'a, $cls>;

            fn to_checked<'lt>(self) -> $crate::CheckResult<Self::ToRust> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $helix_id == ::std::mem::transmute(sys::rb_obj_class(self.inner())) } {
                    if unsafe { $crate::sys::Data_Get_Struct_Value(self.inner()) == ::std::ptr::null_mut() } {
                        let val = unsafe { sys::rb_obj_class(self.inner()) };
                        let inspect = $crate::inspect(unsafe { $crate::ruby::Value::new(val, self.frame()) });
                        Err(format!("Uninitialized {}", inspect))
                    } else {
                        Ok(unsafe { CheckedValue::new(self) })
                    }
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self.inner())).to_string_lossy() };
                    let target = unsafe { $crate::ruby::Value::new(sys::rb_obj_class(self.inner()), self.frame()) };
                    Err(format!("No implicit conversion of {} into {}", val, $crate::inspect(target)))
                }
            }
        }
    }
}
