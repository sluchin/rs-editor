enum class E { A, B };
enum class F : int { C };
struct Base { int b; };
class Derived : public Base, private Other {
public:
    Derived(int x);
    ~Derived();
    virtual void run() const override;
    int get() const { return v; }
    static int count;
private:
    int v;
};
std::string name;
ns::Type value, *ptr;
const std::vector<int> &ref = get();
void Derived::run() const { }
int Derived::count = 0;
namespace ns { int inner; }
using Alias = int;
template <typename T> T identity(T v) { return v; }
template <typename T, class U> struct S { T t; };
int g(int &a, const Base &b);
