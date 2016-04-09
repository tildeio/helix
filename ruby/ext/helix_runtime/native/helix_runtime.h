#include <ruby.h>
#include <ruby/intern.h>
#include <stdbool.h>

#ifndef HELIXRUNTIME_H
#define HELIXRUNTIME_H

extern VALUE HELIX_Qtrue;
extern VALUE HELIX_Qfalse;
extern VALUE HELIX_Qnil;

long HELIX_RSTRING_LEN(VALUE string);
const char* HELIX_RSTRING_PTR(VALUE string);

long HELIX_RARRAY_LEN(VALUE array);
void* HELIX_RARRAY_PTR(VALUE array);

bool HELIX_RB_TYPE_P(VALUE v, int type);
int HELIX_TYPE(VALUE v);

VALUE HELIX_INT2FIX(int c_int);
VALUE HELIX_FIX2INT(VALUE fix);

extern int HELIX_T_NONE;
extern int HELIX_T_NIL;
extern int HELIX_T_OBJECT;
extern int HELIX_T_CLASS;
extern int HELIX_T_ICLASS;
extern int HELIX_T_MODULE;
extern int HELIX_T_FLOAT;
extern int HELIX_T_STRING;
extern int HELIX_T_REGEXP;
extern int HELIX_T_ARRAY;
extern int HELIX_T_HASH;
extern int HELIX_T_STRUCT;
extern int HELIX_T_BIGNUM;
extern int HELIX_T_FILE;
extern int HELIX_T_FIXNUM;
extern int HELIX_T_TRUE;
extern int HELIX_T_FALSE;
extern int HELIX_T_DATA;
extern int HELIX_T_MATCH;
extern int HELIX_T_SYMBOL;
extern int HELIX_T_RATIONAL;
extern int HELIX_T_COMPLEX;
extern int HELIX_T_UNDEF;
extern int HELIX_T_NODE;
extern int HELIX_T_ZOMBIE;
extern int HELIX_T_MASK;
// extern int HELIX_T_IMEMO = T_IMEMO;

#endif /* HELIXRUNTIME_H */
