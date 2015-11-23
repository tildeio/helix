#include <ruby.h>
#include <ruby/encoding.h>

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

int trb_string_is_blank(trb_buf_t self);

static VALUE
str_is_blank(VALUE self) {
  return trb_string_is_blank(STR2BUF(self)) ? Qtrue : Qfalse;
}

void Init_fast_blank( void ) {
  rb_define_method(rb_cString, "blank?", str_is_blank, 0);
}
