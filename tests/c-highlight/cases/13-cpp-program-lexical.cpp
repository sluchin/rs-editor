// A small C++ program for the comparison with Emacs.
#include <iostream>
#include <memory>
#include <string>
#include <vector>

#define LOG(x) std::cout << (x) << '\n'

namespace app {

/** A shape. */
class Shape {
public:
    virtual ~Shape() = default;
    virtual double area() const noexcept = 0;
    static constexpr int kSides = 0;
};

template <typename T>
class Box final : public Shape {
public:
    explicit Box(T w, T h) : w_(w), h_(h) {}
    double area() const noexcept override { return static_cast<double>(w_ * h_); }

private:
    T w_;
    T h_;
};

enum class Color : unsigned char { Red, Green };

struct Point {
    int x = 0, y = 0;
    bool operator==(const Point &o) const { return x == o.x && y == o.y; }
};

using Points = std::vector<Point>;

inline int sum(const Points &ps)
{
    int total = 0;
    for (const auto &p : ps) {
        total += p.x + p.y;
    }
    return total;
}

} // namespace app

int main(int argc, char *argv[])
{
    auto box = std::make_unique<app::Box<int>>(2, 3);
    const char *raw = R"json({"a": [1, 2, 3], "b": "x"})json";
    long big = 1'000'000L + 0x7fff'ffffL;
    try {
        if (argc > 1 && argv[1] != nullptr) {
            throw std::runtime_error("bad");
        }
    } catch (const std::exception &e) {
        LOG(e.what());
    }
    switch (argc) {
    case 0:
        break;
    default:
        break;
    }
    return box->area() > 5.0 ? 0 : 1;
}
