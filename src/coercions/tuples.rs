use sys::{self, VALUE};
use libc::c_long;

use super::{CheckResult, FromRuby, ToRuby, ToRubyResult};
use super::super::{inspect};

#[doc(hidden)]
macro_rules! impl_tuple_coercions {
    ($($name:ident),*) => {
        impl_tuple_from_ruby!(count_items!($($name),*), $($name),*);
        impl_tuple_to_ruby!(count_items!($($name),*), $($name),*);
    };

    ($($any:tt)*) => {
      compile_error!(stringify!("impl_tuple_coercions" $($any)*));
    };
}

#[doc(hidden)]
macro_rules! impl_tuple_from_ruby {
    ($count:expr, $($name:ident),*) => {
        impl<$($name: FromRuby,)*> FromRuby for ($($name,)*) {
            type Checked = ($($name::Checked,)*);

            fn from_ruby(value: VALUE) -> CheckResult<Self::Checked> {
                if unsafe { sys::RB_TYPE_P(value, sys::T_ARRAY) > 0 } {
                    // Make sure we can actually do the conversions for the values.
                    let len = unsafe { sys::RARRAY_LEN(value) };

                    if len != $count {
                        type_error!(value, format!("an array with {} {}", $count, { if $count == 1 { "element" } else { "elements" } }))
                    }

                    extract_tuple_elements_from_ruby!(value, (0), $($name),*);

                    Ok(($($name,)*))
                } else {
                    type_error!(value, "an array")
                }
            }

            fn from_checked(checked: Self::Checked) -> Self {
                #[allow(non_snake_case)]
                let ($($name,)*) = checked;
                ($($name::from_checked($name),)*)
            }
        }
    };

    ($($any:tt)*) => {
      compile_error!(stringify!("impl_tuple_from_ruby" $($any)*));
    };
}

#[doc(hidden)]
macro_rules! extract_tuple_elements_from_ruby {
    ($value:ident, $offset:tt) => {};
    ($value:ident, $offset:tt, $name:ident $($rest:tt)*) => {
        #[allow(non_snake_case)]
        let $name = {
            let val = unsafe { sys::rb_ary_entry($value, $offset as c_long) };
            match $name::from_ruby(val) {
                Ok(v) => v,
                Err(e) => type_error!(format!("Failed to convert {}, element {} has the wrong type: {}", inspect($value), $offset, e)),
            }
        };

        extract_tuple_elements_from_ruby!($value, ($offset + 1) $($rest)*);
    };

    ($($any:tt)*) => {
      compile_error!(stringify!("extract_tuple_elements_from_ruby" $($any)*));
    };
}

#[doc(hidden)]
macro_rules! impl_tuple_to_ruby {
    ($count:expr, $($name:ident),*) => {
        impl<$($name: ToRuby,)*> ToRuby for ($($name,)*) {
            fn to_ruby(self) -> ToRubyResult {
                let ary = unsafe { sys::rb_ary_new_capa($count as c_long) };

                #[allow(non_snake_case)]
                let ($($name,)*) = self;


                $(
                    unsafe { sys::rb_ary_push(ary, $name.to_ruby()?); }
                )*;

                Ok(ary)
            }
        }
    };

    ($($any:tt)*) => {
        compile_error!(stringify!("impl_tuple_to_ruby" $($any)*));
    };
}

#[doc(hidden)]
macro_rules! count_items {
    () => { 0 };
    ($item:tt $(, $rest:tt)*) => { 1 + count_items!($($rest),*) };

  ($($any:tt)*) => {
    compile_error!(stringify!("count_items" $($any)*));
  };
}

impl_tuple_coercions!(A);
impl_tuple_coercions!(A, B);
impl_tuple_coercions!(A, B, C);
impl_tuple_coercions!(A, B, C, D);
impl_tuple_coercions!(A, B, C, D, E);
impl_tuple_coercions!(A, B, C, D, E, F);
impl_tuple_coercions!(A, B, C, D, E, F, G);
impl_tuple_coercions!(A, B, C, D, E, F, G, H);
impl_tuple_coercions!(A, B, C, D, E, F, G, H, I);
impl_tuple_coercions!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_coercions!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_coercions!(A, B, C, D, E, F, G, H, I, J, K, L);
