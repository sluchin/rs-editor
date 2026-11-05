auto b = R"abc(x)abc" + R"(a)b)";
auto c = R"(a\)" + 1;
int y = 1'000;
auto e = R"(
l1
)";
auto f = R"x(a)y)x";
char g = 'a'; auto h = u8"s" + L'c';
auto bad = R"a b(x)a b";
