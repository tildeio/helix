// #[macro_use]
// extern crate helix;

// declare_types! {
//     class NoMacros {
//         def hello(&self) -> String {
//             String::from("hello")
//         }
//     }
// }



extern crate helix;

#[repr(C)]
struct NoMacros {
    helix: helix::Metadata,
}
impl NoMacros {
    pub fn hello(&self) -> String { String::from("hello") }
    fn from_checked_rb_value(value: helix::sys::VALUE) -> NoMacros {
        NoMacros{helix: value,}
    }
}
impl <'a> helix::ToRuby for &'a NoMacros {
    fn to_ruby(self) -> helix::sys::VALUE {
        unsafe { std::mem::transmute(self) }
    }
}
static mut __HELIX_ID: usize = 0;
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    helix::sys::check_version();
    let def =
        helix::ClassDefinition::new("NoMacros").define_method({
            extern "C" fn __ruby_method__(rb_self: helix::sys::VALUE) -> helix::sys::VALUE {
                let checked = __checked_call__(rb_self);
                match checked {
                    Ok(val) => helix::ToRuby::to_ruby(val),
                    Err(err) => {
                        println!("TYPE ERROR: {:?}", err);
                        unsafe { helix::sys::Qnil }
                    }
                }
            }
            fn __checked_call__(rb_self: helix::sys::VALUE) -> Result<String, std::ffi::CString> {
                #[allow(unused_imports)]
                let rust_self = NoMacros::from_checked_rb_value(rb_self);
                Ok(rust_self.hello())
            }
            let name = "hello";
            let arity = { 0isize };
            let method = __ruby_method__ as *const helix::libc::c_void;
            helix::MethodDefinition::new(name, method, arity)
        });
    unsafe { __HELIX_ID = std::mem::transmute(def.class) };
}
