// TODO: Can we change this to use the macro from libcruby?
#[macro_export]
macro_rules! ruby_try {
    { $val:expr } => { $val.unwrap_or_else(|e| panic!($crate::Error::from_ruby(e)) ) }
}

#[macro_export]
macro_rules! ruby_funcall {
    // NOTE: Class and method cannot be variables. If that becomes necessary, I think we'll have to pass them
    ($rb_class:expr, $meth:expr, $( $arg:expr ),*) => {
        {
            use $crate::ToRuby;

            // This method takes a Ruby Array of arguments
            // If there is a way to make this behave like a closure, we could further simplify things.
            #[allow(unused_variables)]
            extern "C" fn __ruby_funcall_cb(arg_ary: *mut $crate::sys::void) -> *mut $crate::sys::void {
                unsafe {
                    // Is this safe here?
                    let arg_ary = $crate::sys::VALUE::wrap(arg_ary);
                    // NOTE: We're using rb_intern_str, not rb_intern in the hopes that this means
                    //   Ruby will clean up the string in the event that there is an exception
                    $crate::sys::rb_funcallv($rb_class, sys::rb_intern_str(String::from($meth).to_ruby().expect("valid string")),
                                                $crate::sys::RARRAY_LEN(arg_ary), $crate::sys::RARRAY_PTR(arg_ary)).as_ptr()
                }
            }

            let mut state = $crate::sys::EMPTY_EXCEPTION;

            let res = unsafe {
                let mut values_ary: Vec<$crate::sys::VALUE> = Vec::new();
                $(
                    // We have to create this iteratively since we have to call to_ruby individually
                    values_ary.push($arg.to_ruby().expect("could convert to Ruby"));
                )*
                let ruby_values_ary = $crate::sys::rb_ary_new_from_values(values_ary.len() as isize, values_ary.as_mut_ptr());
                $crate::sys::rb_protect(__ruby_funcall_cb, ruby_values_ary.as_ptr(), &mut state)
            };

            if !state.is_empty() {
                panic!($crate::Error::from_ruby(state));
            }

            $crate::sys::VALUE::wrap(res)
        }
    };

    ($rb_class:expr, $meth:expr) => {
        ruby_funcall!($rb_class, $meth, )
    }
}