struct V {
  V operator+(const V &o) const;
  bool operator==(const V &o) const { return true; }
  V &operator=(V &&o) noexcept;
  operator bool() const;
  int operator()(int a);
  friend std::ostream &operator<<(std::ostream &os, const V &v);
};
V V::operator+(const V &o) const { return o; }
template <typename T> void f(typename T::value_type x, typename T::iterator it);
void g(int (&arr)[4], int (*fp)(int));
typename T::x y;
MODULE_LICENSE("GPL");
struct W {
  W &operator++();
  W operator++(int);
  int &operator[](int i);
  W *operator->();
  bool operator<(const W &o) const;
  W operator,(const W &o);
  explicit operator const char *() const;
};
int &W::operator[](int i) { return a; }
bool W::operator()(int q) { return q; }
