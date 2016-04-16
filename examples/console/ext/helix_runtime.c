#include <ruby.h>
#include <ruby/intern.h>
#include <stdbool.h>

VALUE helix_Qtrue = Qtrue;
VALUE helix_Qfalse = Qfalse;
VALUE helix_Qnil = Qnil;

int HELIX_RSTRING_LEN(VALUE string) {
  return RSTRING_LEN(string);
}

const char* HELIX_RSTRING_PTR(VALUE string) {
  return RSTRING_PTR(string);
}

bool HELIX_RB_TYPE_P(VALUE v, int type) {
  return RB_TYPE_P(v, type);
}

int HELIX_T_NONE = T_NONE;
int HELIX_T_NIL = T_NIL;
int HELIX_T_OBJECT = T_OBJECT;
int HELIX_T_CLASS = T_CLASS;
int HELIX_T_ICLASS = T_ICLASS;
int HELIX_T_MODULE = T_MODULE;
int HELIX_T_FLOAT = T_FLOAT;
int HELIX_T_STRING = T_STRING;
int HELIX_T_REGEXP = T_REGEXP;
int HELIX_T_ARRAY = T_ARRAY;
int HELIX_T_HASH = T_HASH;
int HELIX_T_STRUCT = T_STRUCT;
int HELIX_T_BIGNUM = T_BIGNUM;
int HELIX_T_FILE = T_FILE;
int HELIX_T_FIXNUM = T_FIXNUM;
int HELIX_T_TRUE = T_TRUE;
int HELIX_T_FALSE = T_FALSE;
int HELIX_T_DATA = T_DATA;
int HELIX_T_MATCH = T_MATCH;
int HELIX_T_SYMBOL = T_SYMBOL;
int HELIX_T_RATIONAL = T_RATIONAL;
int HELIX_T_COMPLEX = T_COMPLEX;
int HELIX_T_UNDEF = T_UNDEF;
int HELIX_T_NODE = T_NODE;
int HELIX_T_ZOMBIE = T_ZOMBIE;
int HELIX_T_MASK = T_MASK;
// int HELIX_T_IMEMO = T_IMEMO;

void Init_helix_runtime() {}

void helix_inspect(void* ptr) {
  printf("ptr: %p\n", ptr);
  printf("str: %s\n", ptr);
  printf("hex: %x\n", ptr);
}
