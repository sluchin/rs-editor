#include <vector>
namespace outer { int a; }
using namespace std;
using Vec = std::vector<int>;
class Base {
public:
    Base() = default;
    Base(int x, const Other &o) : v(x), w(o) { }
    virtual ~Base() { }
    explicit Base(double d);
    virtual void draw() const = 0;
    static Base *create(int kind);
    std::vector<std::vector<int>> grid;
    Foo<Bar> member;
    int v;
    Other w;
};
Base::Base(double d) : v(1) { }
Base::~Base() { }
int Base::get(int i) const { return v + i; }
template <typename T> class Box { T item; };
template <class T, int N> T pick(T arr, int n) { return arr; }
std::map<std::string, int> table;
void f(std::string s, const std::vector<int> &v)
{
    std::string local;
    Foo<Bar> *p = nullptr;
    auto it = v.begin();
    for (auto x : v) { local += x; }
}
