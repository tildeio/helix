#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate libcruby_sys as sys;

#[macro_use]
extern crate libcruby as rb;

use sys::VALUE;

declare_types! {
    class RbConsole for Console {
        init(call) {}

        def log(string: VALUE) {
            println!("{:?}", string);
        }
    }
}

// init! {
//     RbConsole::setup();
//     // Console.define_method("log", log);
// }

use std::mem;

macro_rules! cstr {
    ( $x: expr ) => { $x.as_ptr() as *const i8 }
}

// #[no_mangle]
// pub extern "C" fn Init_libconsole() {
//     unsafe {
//         let Console = rb_define_class(cstr!(b"Console\0"), rb_cObject);
//         rb_define_method(Console, cstr!(b"log\0"), log as void_ptr, 1);
//     };
// }

// extern "C" fn log(_: VALUE, message: VALUE) -> VALUE {
//     #[repr(C)]
//     struct CheckTypeArgs {
//         value: VALUE,
//         rb_type: isize,
//     }

//     extern "C" fn CheckType(args: &CheckTypeArgs) -> VALUE {
//         unsafe { rb_check_type(args.value, args.rb_type) };
//         Qnil
//     }

//     let result = std::panic::catch_unwind(|| {
//         with_protect(CheckType,
//                      &CheckTypeArgs {
//                          value: message,
//                          rb_type: T_STRING,
//                      });
//     });

//     if let Err(state) = result {
//         let state = state.downcast_ref::<RubyException>().unwrap();
//         unsafe { rb_jump_tag(*state) };
//     } else {
//         if unsafe { RB_TYPE_P(message, T_STRING) } {
//             let size = unsafe { RSTRING_LEN(message) };
//             let ptr = unsafe { RSTRING_PTR(message) };
//             let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
//             let string = unsafe { std::str::from_utf8_unchecked(slice) };
//             println!("size: {}", size);
//             println!("ptr: {:?}", ptr);
//             println!("string: {}", string);
//             Qtrue
//         } else {
//             Qfalse
//         }
//     }


// }
// fn with_protect<T>(func: extern "C" fn(&T) -> VALUE, arg: &T) {
//     let mut state: RubyException = RubyException::new();
//     let arg: void_ptr = unsafe { mem::transmute(arg) };
//     let func: extern "C" fn(void_ptr) -> VALUE = unsafe { mem::transmute(func) };

//     unsafe { rb_protect(func, arg, &mut state as *mut RubyException) };

//     if state == RubyException::new() {
//         println!("IT WORKED");
//     } else {
//         panic!(state);
//     }
// }
