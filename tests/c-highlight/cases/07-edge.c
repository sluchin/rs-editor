/* 日本語のコメント */
// 日本語の行コメント
char *s = "日本語の文字列";
char c = 'あ';
int 変数 = 1; /* 後ろ */
	int tabbed;	/* tab */
#define M(a) \
	do { /* in macro */ \
		return a; \
	} while (0)
// continued \
still a comment
int x; // trailing \
cont
#if 0
/* inside #if 0 */
#endif
int y = sizeof(long long) + sizeof(unsigned char);
const char *q = "a" /* between */ "b";
int z = a ? b : c;
struct s { int a : 3; unsigned b : 5; };
int arr[] = { [0] = 1, [1] = 2 };
void (*fp)(int) = NULL;
int main(int argc, char **argv) { return argc > 1 ? 0 : 1; }
/* unterminated comment at the end
