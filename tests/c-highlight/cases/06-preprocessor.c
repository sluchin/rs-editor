#include <stdio.h>
#include <sys/types.h>
#include "a.h" // trailing
#include MACRO_HEADER
#  include <b.h>
#define A 1
# define B(x, y) ((x) + (y))
#define LONG(x) \
    do { \
        if (x) return; \
    } while (0)
#undef A
#if defined(B) && !defined C
int one;
#elif defined D || 0
int two;
#else
int three;
#endif
#ifdef X
#ifndef Y
#endif
#endif
#error stop here
#warning careful /* not a comment? */
#pragma once
#line 10 "x.c"
#
return after;
   #define INDENTED 1
int x = 1; /* # not a directive */
int y = 2;
#define STR "text // not comment"
#define CH 'a'
