#include <stdbool.h>
#include <ruby.h>
#include <ruby/encoding.h>
#include <ruby/intern.h>

typedef struct {
    void* data;
    size_t len;
} trb_buf_t;

static inline trb_buf_t
STR2BUF(VALUE str) {
  return (trb_buf_t) {
    .data = RSTRING_PTR(str),
    .len = RSTRING_LEN(str),
  };
}

#define CHECK_FFI(expr) expr

bool trb_string_is_blank(trb_buf_t self, bool*);

static VALUE
str_is_blank(VALUE self) {
  bool ret;
  CHECK_FFI(trb_string_is_blank(STR2BUF(self), &ret));

  return ret ? Qtrue : Qfalse;
}

void Init_fast_blank( void ) {
  ID String_ID = rb_intern("String");
  VALUE cString;

  if (rb_const_defined(rb_cObject, String_ID)) {
    cString = rb_const_get(rb_cObject, String_ID);
  } else {
    rb_raise(rb_eNotImpError, "Unimplemented new Rust class");
  }

  rb_define_method(cString, "blank?", str_is_blank, 0);
}
