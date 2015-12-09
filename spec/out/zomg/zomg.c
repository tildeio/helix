#include <ruby.h>
#include <ruby/intern.h>

extern void init();

VALUE rb_Qtrue = Qtrue;
VALUE rb_Qfalse = Qfalse;
VALUE rb_Qnil = Qnil;

char* rb_rstring_ptr(VALUE str) {
  return RSTRING_PTR(str);
}

long rb_rstring_len(VALUE str) {
  return RSTRING_LEN(str);
}

void Init_zomg() {
  init();
}
