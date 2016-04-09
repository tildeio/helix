#include <ruby.h>
#include <ruby/intern.h>

VALUE rb_Qtrue = Qtrue;
VALUE rb_Qfalse = Qfalse;
VALUE rb_Qnil = Qnil;

void Init_helix_runtime() {}

void helix_inspect(void* ptr) {
  printf("ptr: %p\n", ptr);
  printf("str: %s\n", ptr);
  printf("hex: %x\n", ptr);
}
