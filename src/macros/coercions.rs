#[macro_export]
macro_rules! codegen_coercions {
    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: (),
        methods: $methods:tt
    }) => (
        impl $crate::FromRuby for $rust_name {
            type Checked = $crate::CheckedValue<$rust_name>;

            fn from_ruby(value: $crate::sys::VALUE) -> $crate::CheckResult<$crate::CheckedValue<$rust_name>> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $rust_name == $crate::as_usize(sys::rb_obj_class(value)) } {
                    Ok(unsafe { CheckedValue::new(value) })
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(value)).to_string_lossy() };
                    type_error!(format!("No implicit conversion of {} into {}", val, stringify!($rust_name)))
                }
            }

            fn from_checked(checked: $crate::CheckedValue<$rust_name>) -> $rust_name {
                $rust_name { helix: checked.to_value() }
            }
        }

        impl_to_ruby!(&'a $rust_name);
        impl_to_ruby!(&'a mut $rust_name);
    );

    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: $methods:tt
    }) => (
        impl $crate::FromRuby for $rust_name {
            type Checked = Box<$rust_name>;

            fn from_ruby(value: $crate::sys::VALUE) -> $crate::CheckResult<Box<$rust_name>> {
                use $crate::{ToError, sys};

                if unsafe { $rust_name != $crate::as_usize(sys::rb_obj_class(value)) } {
                    type_error!(value, stringify!($rust_name));
                }

                let ptr = unsafe { sys::Data_Get_Struct_Value(value) };

                if ptr != ::std::ptr::null_mut() {
                    Ok(unsafe { ::std::mem::transmute(ptr) })
                } else {
                    Err(format!("Uninitialized {}", stringify!($rust_name)).to_error())
                }
            }

            fn from_checked(mut checked: Box<$rust_name>) -> $rust_name {
                unsafe { $crate::sys::Data_Set_Struct_Value(checked.helix, ::std::ptr::null_mut()) };
                checked.helix = $crate::Metadata::uninitialized();
                *checked
            }
        }

        impl_struct_to_rust!(&'a $rust_name, $rust_name);
        impl_struct_to_rust!(&'a mut $rust_name, $rust_name);

        impl $crate::ToRuby for $rust_name {
            fn to_ruby(self) -> $crate::ToRubyResult {
                Ok($rust_name::__alloc_with__(Some(Box::new(self))))
            }
        }

        impl_to_ruby!(&'a $rust_name, $rust_name);
        impl_to_ruby!(&'a mut $rust_name, $rust_name);
    );
}

#[macro_export]
macro_rules! impl_struct_to_rust {
    ($rust_name:ty, $helix_id:tt) => {
        impl<'a> $crate::FromRuby for $rust_name {
            type Checked = $rust_name;

            fn from_ruby(value: $crate::sys::VALUE) -> $crate::CheckResult<$rust_name> {
                use $crate::{ToError, sys};

                if unsafe { $helix_id != $crate::as_usize(sys::rb_obj_class(value)) } {
                    type_error!(value, stringify!($helix_id));
                }

                let ptr = unsafe { sys::Data_Get_Struct_Value(value) };

                if ptr != ::std::ptr::null_mut() {
                    Ok(unsafe { ::std::mem::transmute(ptr) })
                } else {
                    Err(format!("Uninitialized {}", stringify!($helix_id)).to_error())
                }
            }

            fn from_checked(checked: $rust_name) -> $rust_name {
                checked
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_to_ruby {
    ($rust_type:ty, $rust_name) => {
        item! {
            impl<'a> $crate::ToRuby for $rust_type {
                fn to_ruby(self) -> $crate::ToRubyResult {
                    match self.helix.value() {
                        Some(value) => Ok(value),
                        None => Ok($rust_name::__alloc_with__(Some(self))))
                    }
                }
            }
        }
    }
}
