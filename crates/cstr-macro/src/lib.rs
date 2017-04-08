#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char] as *const ::std::os::raw::c_char
    )
}

#[cfg(test)]
mod tests {
    use ::std::ffi::CStr;
    use ::std::os::raw::c_char;

    #[test]
    fn has_right_type() {
        let val = cstr!("hello world");
        assert_eq!(to_str(val), "hello world");
    }

    const hello: *const c_char = cstr!("hello");

    #[test]
    fn can_be_used_in_const_position() {
        assert_eq!(to_str(hello), "hello");
    }

    fn to_str<'a>(input: *const c_char) -> &'a str {
        unsafe { CStr::from_ptr(input) }.to_str().unwrap()
    }
}
