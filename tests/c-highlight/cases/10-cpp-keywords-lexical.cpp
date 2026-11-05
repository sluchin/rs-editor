#include <vector>
namespace ns { class A : public B { public: virtual void f() override final; private: int x; protected: int y; }; }
template <typename T, class U> struct S { using type = T; };
auto s = R"(raw "text" \ here)";
auto t = R"x(multi
line)x";
auto u = u8R"(a)" + LR"(b)" + uR"(c)" + UR"(d)";
auto n = 1'000'000 + 0x1'F;
bool b = true && false; void *p = nullptr; int *q = NULL;
char8_t c8; char16_t c16; char32_t c32; wchar_t w;
constexpr int ce = 1; consteval int cv(); constinit int ci = 0; thread_local int tl;
try { throw 1; } catch (...) { }
new int; delete p; this->x; operator+;
static_cast<int>(1); dynamic_cast<A*>(p); reinterpret_cast<int*>(p); const_cast<int*>(q);
alignas(8) int al; alignof(int); decltype(1) dt; noexcept(true); typeid(int); explicit A(int);
concept C = requires { 1; }; co_await x; co_return; co_yield 1;
export module m; import m;
x = a and b or not c xor d bitand e bitor f compl g;
friend class F; mutable int m; typename T::x y; typedef int I; enum class E { A }; union U {};
int main() { for (auto x : v) {} if constexpr (1) {} return 0; }
