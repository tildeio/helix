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

trb_buf_t
trb_string_to_buf(VALUE str) {
  return STR2BUF(str);
}

VALUE
trb_cObject() {
  return rb_cObject;
}

#define CHECK_FFI(expr) expr

typedef void activesupport_duration_t;
bool activesupport_duration_alloc(activesupport_duration_t*);
bool activesupport_duration_free(activesupport_duration_t*);
bool activesupport_duration_noop(activesupport_duration_t*, VALUE);

void*
trb_Data_Get_Struct(VALUE self) {
  void* data;
  Data_Get_Struct(self, void, data);
  return data;
}

VALUE
trb_Data_Wrap_Struct(VALUE klass, void* mark, void* free, void* data) {
  return Data_Wrap_Struct(klass, mark, free, data);
}

static VALUE
duration_alloc(VALUE self) {
  activesupport_duration_t* data;
  CHECK_FFI(activesupport_duration_alloc(&data));
  return Data_Wrap_Struct(self, NULL, activesupport_duration_free, data);
}

static VALUE
duration_noop(VALUE self) {
  void* data;
  Data_Get_Struct(self, activesupport_duration_t, data);
  activesupport_duration_noop(data, self);
  return Qnil;
}

void Init_native_activesupport_duration( void ) {
  ID Duration_ID = rb_intern("Duration");
  VALUE cDuration;

  ID ActiveSupport_ID = rb_intern("ActiveSupport");
  VALUE cActiveSupport;

  if (rb_const_defined(rb_cObject, ActiveSupport_ID)) {
    cActiveSupport = rb_const_get(rb_cObject, ActiveSupport_ID);
  } else {
    cActiveSupport = rb_define_module("ActiveSupport");
  }

  if (rb_const_defined(cActiveSupport, Duration_ID)) {
    cDuration = rb_const_get(cActiveSupport, Duration_ID);
    rb_define_alloc_func(cDuration, duration_alloc);
  } else {
    cDuration = rb_define_class_under(cActiveSupport, "Duration", rb_cObject);
    rb_define_alloc_func(cDuration, duration_alloc);
  }

  rb_define_method(cDuration, "noop", duration_noop, 0);
}
