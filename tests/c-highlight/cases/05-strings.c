char *a = "plain";
char *b = "esc \" quote \\ done";
char *c = "line one \
line two";
char d = 'x';
char e = '\'';
char f = '\\';
char g = '"';
char *h = "it's";
char *i = "a" "b" "c";
char *j = L"wide";
char *k = u8"utf8";
int l = 'a' + 'b';
/* "not a string" */
// "not a string either"
char *m = "/* not a comment */";
char *n = "// not a comment";
char *o = "unterminated
int after = 1;
