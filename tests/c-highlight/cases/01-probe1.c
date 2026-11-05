/**/ int a; /*x*/ int b; //x
/// doc
/** doc */
//
#  include "x.h"
# define F(a, b) ((a) + (b))
#if 0
int dead;
#endif
#ifdef FOO
#elif defined(BAR)
#endif
#error stop here
#pragma once
#define LONG 1 \
   + 2
char *s = "a\"b\\" "c\
d";
char c = '\'';
int x = sizeof(int) + 1.5e3;
enum color { RED, GREEN };
void f(int n) { label: goto label; }
struct s { unsigned long v; } *p;
