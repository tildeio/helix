use { Class };
use sys;
use libc::{c_char, c_int};

pub struct MethodSpecification {
    name: *const c_char,
    function: sys::ANYARGS<sys::VALUE>,
    arity: isize,
}

pub enum MethodDefinition {
    Class(MethodSpecification),
    Instance(MethodSpecification)
}

impl MethodDefinition {
    pub fn class(name: *const c_char, function: sys::ANYARGS<sys::VALUE>, arity: isize) -> MethodDefinition {
        MethodDefinition::Class(MethodSpecification { name: name, function: function, arity: arity })
    }

    pub fn instance(name: *const c_char, function: sys::ANYARGS<sys::VALUE>, arity: isize) -> MethodDefinition {
        MethodDefinition::Instance(MethodSpecification { name: name, function: function, arity: arity })
    }
}

pub struct ClassDefinition {
    pub class: Class,
}

impl ClassDefinition {
    pub fn new(name: *const c_char) -> ClassDefinition {
        let raw_class = unsafe { sys::rb_define_class(name, sys::rb_cObject) };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn wrapped(name: *const c_char, alloc_func: extern "C" fn(klass: sys::VALUE) -> sys::VALUE) -> ClassDefinition {
        let raw_class = unsafe { sys::rb_define_class(name, sys::rb_cObject) };
        unsafe { sys::rb_define_alloc_func(raw_class, alloc_func) };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn reopen(name: *const c_char) -> ClassDefinition {
        let raw_class = unsafe {
            let class_id = sys::rb_intern(name);
            sys::rb_const_get(sys::rb_cObject, class_id)
        };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn define_method(&self, def: MethodDefinition) {
        match def {
            MethodDefinition::Instance(def) => {
                unsafe {
                    sys::rb_define_method(
                        self.class.0,
                        def.name,
                        def.function,
                        def.arity as c_int
                    );
                };
            },
            MethodDefinition::Class(def) => {
                unsafe {
                    sys::rb_define_singleton_method(
                        self.class.0,
                        def.name,
                        def.function,
                        def.arity as c_int
                    );
                };
            }
        }
    }

    pub fn undefine_class_method(&self, name: *const c_char) {
        unsafe {
            sys::rb_undef_method(sys::CLASS_OF(self.class.0), name);
        }
    }
}
