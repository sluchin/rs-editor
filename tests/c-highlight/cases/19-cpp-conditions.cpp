int (*pa)[4];
int (&ra)[4] = arr;
void h(int (*pb)[4], char (&rb)[8]);
typedef int (*Fn)(int);
typedef int (&Arr)[4];
typedef int (*PA)[4];
void k() {
  if (int x = f()) { }
  if (auto it = m.find(key); it != m.end()) { }
  while (Node *n = next()) { }
  switch (int s = g()) { }
  if (a * b == c) { }
  if (x == y) { }
  else if (const Foo &r = get()) { }
}
