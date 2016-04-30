#include <ruby.h>
#include <ruby/intern.h>
#include <helix_runtime.h>

#define EXPORT_VALUE(name) (rb_define_const(mDummy, #name, HELIX_ ## name))
#define EXPORT_INT(name) (rb_define_const(mDummy, #name, INT2FIX(HELIX_ ##name)))
#define EXPORT_FUNC(name, arity) (rb_define_singleton_method(mDummy, #name, TEST_ ## name, arity))
#define EXPORT_RUBY_FUNC(name, arity) (rb_define_singleton_method(mRuby, #name, TEST_RB_ ## name, arity))

static VALUE TEST_RSTRING_LEN(VALUE _self, VALUE val) {
  return LONG2NUM(HELIX_RSTRING_LEN(val));
}

static VALUE TEST_RB_RSTRING_PTR(VALUE _self, VALUE val) {
  return LONG2NUM((long)RSTRING_PTR(val));
}

static VALUE TEST_RSTRING_PTR(VALUE _self, VALUE val) {
  return LONG2NUM((long)HELIX_RSTRING_PTR(val));
}

static VALUE TEST_RARRAY_LEN(VALUE _self, VALUE val) {
  return LONG2NUM(HELIX_RARRAY_LEN(val));
}

static VALUE TEST_RB_RARRAY_PTR(VALUE _self, VALUE val) {
  return LONG2NUM((long)RARRAY_PTR(val));
}

static VALUE TEST_RARRAY_PTR(VALUE _self, VALUE val) {
  return LONG2NUM((long)HELIX_RARRAY_PTR(val));
}

static VALUE TEST_RB_TYPE_P(VALUE _self, VALUE val, VALUE type) {
  int result = HELIX_RB_TYPE_P(val, FIX2INT(type));
  return result ? Qtrue : Qfalse;
}

static VALUE TEST_TYPE(VALUE _self, VALUE val) {
  return INT2FIX(HELIX_TYPE(val));
}

static VALUE TEST_INT2FIX(VALUE _self, VALUE val) {
  return HELIX_INT2FIX(FIX2INT(val));
}

static VALUE TEST_FIX2INT(VALUE _self, VALUE val) {
  return INT2FIX(HELIX_FIX2INT(val));
}

void Init_dummy() {
  VALUE mDummy = rb_define_module("Dummy");
  VALUE mRuby = rb_define_module_under(mDummy, "Ruby");

  EXPORT_VALUE(Qtrue);
  EXPORT_VALUE(Qfalse);
  EXPORT_VALUE(Qnil);

  EXPORT_INT(T_NONE);
  EXPORT_INT(T_NIL);
  EXPORT_INT(T_OBJECT);
  EXPORT_INT(T_CLASS);
  EXPORT_INT(T_ICLASS);
  EXPORT_INT(T_MODULE);
  EXPORT_INT(T_FLOAT);
  EXPORT_INT(T_STRING);
  EXPORT_INT(T_REGEXP);
  EXPORT_INT(T_ARRAY);
  EXPORT_INT(T_HASH);
  EXPORT_INT(T_STRUCT);
  EXPORT_INT(T_BIGNUM);
  EXPORT_INT(T_FILE);
  EXPORT_INT(T_FIXNUM);
  EXPORT_INT(T_TRUE);
  EXPORT_INT(T_FALSE);
  EXPORT_INT(T_DATA);
  EXPORT_INT(T_MATCH);
  EXPORT_INT(T_SYMBOL);
  EXPORT_INT(T_RATIONAL);
  EXPORT_INT(T_COMPLEX);
  EXPORT_INT(T_UNDEF);
  EXPORT_INT(T_NODE);
  EXPORT_INT(T_ZOMBIE);
  EXPORT_INT(T_MASK);

  EXPORT_FUNC(RSTRING_LEN, 1);
  EXPORT_FUNC(RSTRING_PTR, 1);
  EXPORT_RUBY_FUNC(RSTRING_PTR, 1);
  EXPORT_FUNC(RARRAY_LEN, 1);
  EXPORT_FUNC(RARRAY_PTR, 1);
  EXPORT_RUBY_FUNC(RARRAY_PTR, 1);
  EXPORT_FUNC(RB_TYPE_P, 2);
  EXPORT_FUNC(TYPE, 1);
  EXPORT_FUNC(INT2FIX, 1);
  EXPORT_FUNC(FIX2INT, 1);
}
