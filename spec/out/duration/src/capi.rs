use Duration;
use libc;
use std::ptr;
use std::mem::{transmute, uninitialized};
use std::ffi::CString;
use ruby::{
    RubyValue,
    RubyNamespace,
    RubyKernel,
    RubyObject,
    RubyString,
    RubyModule,
    RubyClass,
    RubySliceHelpers,
    rb_cObject,
    trb_Data_Wrap_Struct,
    trb_Data_Get_Struct,
    void_ptr
};

extern "C" fn activesupport_duration_alloc(out: *mut Box<Duration>) -> bool {
    let d = Box::new(Duration::new(0, vec![]));
    unsafe { ptr::write(out, d); };
    true
}

extern "C" fn activesupport_duration_mark() {}
extern "C" fn activesupport_duration_free(_: Box<Duration>) {}

extern "C" fn activesupport_duration_noop(_: &Duration, object: &mut RubyObject) -> bool {
    println!("WAT: {:?}", object.inspect());
    true
}

extern "C" fn duration_noop(mut receiver: RubyObject) -> RubyObject {
    let d: Box<Duration> = unsafe { transmute(trb_Data_Get_Struct(&mut receiver)) };
    activesupport_duration_noop(&d, &mut receiver);
    unsafe { transmute(4) }
}

extern "C" fn duration_alloc(klass: RubyClass) -> RubyObject {
    let d = Box::new(Duration::new(0, vec![]));
    unsafe { trb_Data_Wrap_Struct(klass, transmute(activesupport_duration_mark), transmute(activesupport_duration_free), transmute(d)) }
}

#[no_mangle]
pub extern "C" fn Init_native_activesupport_duration() {
    let duration_id = "Duration".intern();
    let activesupport_id = "ActiveSupport".intern();

    let activesupport = if rb_cObject.is_const_defined(activesupport_id) {
        rb_cObject.const_get::<RubyModule>(activesupport_id)
    } else {
        rb_cObject.define_module("ActiveSupport")
    };

    let duration = if activesupport.is_const_defined(duration_id) {
        activesupport.const_get::<RubyClass>(duration_id)
    } else {
        activesupport.define_class("Duration", rb_cObject)
    };

    duration.define_alloc_func(duration_alloc);
    duration.define_method("noop", 0, duration_noop);
}
