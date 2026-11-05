#define SQ(x) ((x) * (x))
typedef unsigned long ulong_t;
typedef struct point { int x, y; } point_t;
struct node;
struct node { struct node *next; int value; };
enum color { RED, GREEN = 2, BLUE };
union u { int i; float f; };
static const int counter = 0;
int count, *ptr, arr[10] = {1, 2};
extern char *name;
point_t origin;
const point_t *cp;
char *dup(const char *s);
int f2(int a, point_t *b, ...);
static point_t make(int x) { return origin; }
auto inferred = 1;
bool flag = true;
int main(int argc, char **argv)
{
    int local = 1;
    point_t p;
    point_t *r = &p, s;
    auto a = 2;
    for (int k = 0; k < 10; k++) {
        local += k;
    }
done:
    return 0;
}
