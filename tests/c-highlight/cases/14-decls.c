#include <stdio.h>
#define MAX_LEN 10
#define SQ(x) ((x) * (x))
#define ADD(a, b) ((a) + (b))
#define VARIADIC(fmt, ...) printf(fmt, __VA_ARGS__)
#define EMPTY
#  define INDENTED 1
#undef MAX_LEN
typedef unsigned long ulong_t;
typedef struct point { int x, y; } point_t;
typedef int (*cb_t)(int);
typedef void fn_t(int);
struct node;
struct node { struct node *next; int value; };
enum color { RED, GREEN = 2, BLUE };
enum { ONLY } only_var;
union u { int i; float f; };
static const int counter = 0;
int count, *ptr, arr[10] = {1, 2};
extern char *name;
size_t n;
point_t origin;
const point_t *cp;
struct node *p2, q2;
point_t arr2[3], *pp;
volatile unsigned counter2;
int bits : 3;
struct { unsigned a : 1; unsigned b : 7; } flags;
int (*fp)(int, int);
char *dup(const char *s);
int f1(void);
int f2(int a, point_t *b, ...);
int f3(point_t, int);
struct point *getp(void);
static void helper(void);
void helper(void) { }
static inline int add(int a, int b) { return a + b; }
static point_t make(int x) { return origin; }
__attribute__((unused)) static int attr;
typeof(count) tc;
int main(int argc, char **argv)
{
    int local = 1;
    size_t len = strlen(name);
    point_t p;
    point_t *r = &p, s;
    struct node *head = NULL;
    unsigned long v;
    other_t z;
    static int calls;
    int i, j = 2;
    for (int k = 0; k < 10; k++) {
        foo(k);
    }
    for (i = 0; i < 3; i++) local += i;
    p.x = 1;
    r->y = 2;
    local = local * len;
    if (local) goto done;
again:
    switch (i) { case 1: break; default: break; }
    local = local ? i : j;
    if (local) goto again;
done:
    return 0;
}
