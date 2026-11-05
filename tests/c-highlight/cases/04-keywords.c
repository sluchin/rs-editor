#include <stdio.h>
#include <stdlib.h>
#include "local.h"

typedef unsigned long ulong_t;
struct node { struct node *next; int value; };
union u { int i; float f; };
enum e { A, B = 2 };
extern int g;
static const volatile int cv = 0;
register int r;
static inline int sq(int x) { return x * x; }

int f(int n)
{
    int i;
    for (i = 0; i < n; i++) {
        if (i == 3)
            continue;
        else if (i == 5)
            break;
    }
    while (n--) {
        do {
            n++;
        } while (0);
    }
    switch (n) {
    case 1:
        break;
    default:
        break;
    }
    goto done;
done:
    return sizeof(int) + sizeof n;
}

_Bool b = true;
_Complex double z;
double d = 1.5;
float fl;
long l;
short s;
signed sg;
char *restrict p;
void *q = NULL;
int fa = false;
auto int au;
__attribute__((unused)) static int attr;
typeof(g) tg;
int u8 = 0x1Fu + 10UL + 1.5e-3 + .5f + 0b101;
