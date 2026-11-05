//! 宣言の名前 (`core/src/highlight/decl.rs` と、`#define` の名前) のテスト.
//! 期待値は、Emacs 30 の `c-mode` の font-lock の結果 (`tests/c-highlight/cases/14-decls.c` で、同じ入力を Emacs と比べる).

use rseditor_core::highlight::{tokenize, Language};

/// 宣言の名前だけを (本文の中の文字列, 種類の名前) の並びにする (キーワード、組み込みの型と定数、数値などは除く).
fn names_in(language: Language, text: &str) -> Vec<(String, &'static str)> {
    let chars: Vec<char> = text.chars().collect();
    let spec = language.spec().unwrap();
    tokenize(language, text)
        .iter()
        .map(|s| {
            (
                chars[s.start..s.end].iter().collect::<String>(),
                s.kind.name(),
            )
        })
        .filter(|(word, kind)| {
            matches!(
                *kind,
                "type" | "constant" | "function-name" | "variable-name"
            ) && !spec.is_type(word)
                && !spec.is_constant(word)
        })
        .collect()
}

/// C の宣言の名前.
fn names(text: &str) -> Vec<(String, &'static str)> {
    names_in(Language::C, text)
}

/// `(文字列, 種類)` の並びを作る.
fn expect(items: &[(&str, &'static str)]) -> Vec<(String, &'static str)> {
    items.iter().map(|(t, k)| (t.to_string(), *k)).collect()
}

#[test]
fn test_variables() {
    assert_eq!(
        names("int count, *ptr, arr[10] = {1, 2}, **pp;"),
        expect(&[
            ("count", "variable-name"),
            ("ptr", "variable-name"),
            ("arr", "variable-name"),
            ("pp", "variable-name"),
        ])
    );
    // 型の名前は、型の色. 前後の `const` `static` は、型の名前を邪魔しない
    assert_eq!(
        names("static const point_t *cp, q; point_t const *r;"),
        expect(&[
            ("point_t", "type"),
            ("cp", "variable-name"),
            ("q", "variable-name"),
            ("point_t", "type"),
            ("r", "variable-name"),
        ])
    );
    // 初期値の中の括弧、波括弧、`,` は、宣言を分けない
    assert_eq!(
        names("int a = f(1, 2), b = (3, 4), c[2] = {5, 6};"),
        expect(&[
            ("a", "variable-name"),
            ("b", "variable-name"),
            ("c", "variable-name"),
        ])
    );
    // ビットフィールド
    assert_eq!(
        names("struct { unsigned a : 1, b : 7; } flags;"),
        expect(&[
            ("a", "variable-name"),
            ("b", "variable-name"),
            ("flags", "variable-name"),
        ])
    );
    // 本文の終わりで、`;` が無くても
    assert_eq!(names("int x"), expect(&[("x", "variable-name")]));
    // `a * b` は、型 a のポインタの宣言と読む (Emacs と同じ)
    assert_eq!(
        names("a * b;"),
        expect(&[("a", "type"), ("b", "variable-name")])
    );
}

#[test]
fn test_not_declarations() {
    // 式と呼び出しには、色が付かない
    assert_eq!(
        names("x = y; foo(x); a * 2; a + b; x.y = 1; r->y = 2;"),
        vec![]
    );
    // 型の後ろが、宣言子の形でなければ、宣言ではない
    assert_eq!(names("int x y;"), vec![]);
    assert_eq!(names("int 3;"), vec![]);
    assert_eq!(names("int x {"), vec![]);
    // 型の名前が単独なら、型の名前ではない
    assert_eq!(names("point_t;"), vec![]);
    // 名前の無い struct は、本体が無ければ宣言ではない
    assert_eq!(names("struct ; int"), vec![]);
    // Emacs は、`_Complex double z;` の `z` に色を付けない
    assert_eq!(names("_Complex double z;"), vec![]);
    assert_eq!(names("double _Imaginary w;"), vec![]);
    // 宣言でないと分かったら、読み取った名前は捨てる
    assert_eq!(names("point_t *3;"), vec![]);
}

#[test]
fn test_functions() {
    assert_eq!(
        names("static point_t make(int x, point_t *b, ...) { return p; }"),
        expect(&[
            ("point_t", "type"),
            ("make", "function-name"),
            ("x", "variable-name"),
            ("point_t", "type"),
            ("b", "variable-name"),
        ])
    );
    // 宣言 (本体なし)、`void` だけの引数、型だけの引数、名前の無い引数
    assert_eq!(
        names("int f1(void); int f2(point_t, int); char *dup(const char *s);"),
        expect(&[
            ("f1", "function-name"),
            ("f2", "function-name"),
            ("point_t", "type"),
            ("dup", "function-name"),
            ("s", "variable-name"),
        ])
    );
    // 関数の後ろの `__attribute__` と、複数の宣言子
    assert_eq!(
        names("void f(void) __attribute__((noreturn)), g(int);"),
        expect(&[("f", "function-name"), ("g", "function-name")])
    );
    // 引数の形が崩れていても、読み飛ばす. 色が付くのは、型と名前を読めた引数だけ
    assert_eq!(
        names("int f(x + 1, point_t *3, int a);"),
        expect(&[("f", "function-name"), ("a", "variable-name")])
    );
    // 閉じない引数の並びは、宣言ではない
    assert_eq!(names("int f(int a"), vec![]);
    // 引数の中の括弧
    assert_eq!(
        names("int f(int a[(1)], int (*cb)(int x));"),
        expect(&[
            ("f", "function-name"),
            ("a", "variable-name"),
            ("cb", "function-name"),
            ("x", "variable-name"),
        ])
    );
}

#[test]
fn test_function_pointers_and_typedefs() {
    // 関数ポインタの変数は、関数の名前の色. 引数の無い `(**fq)` は、変数の名前の色 (Emacs と同じ)
    assert_eq!(
        names("int (*fp)(int, int); int (**fq);"),
        expect(&[("fp", "function-name"), ("fq", "variable-name")])
    );
    // `typedef` で作る名前は、型の色
    assert_eq!(
        names("typedef unsigned long ulong_t; typedef int (*cb_t)(int); typedef void fn_t(int);"),
        expect(&[("ulong_t", "type"), ("cb_t", "type"), ("fn_t", "type"),])
    );
    // 関数ポインタの形が崩れていれば、宣言ではない
    assert_eq!(names("int (*3)(int);"), vec![]);
    assert_eq!(names("int (*fp x;"), vec![]);
    assert_eq!(names("int (*fp)(int"), vec![]);
}

#[test]
fn test_aggregates() {
    assert_eq!(
        names("struct node; struct node { struct node *next; int value; };"),
        expect(&[
            ("node", "type"),
            ("node", "type"),
            ("node", "type"),
            ("next", "variable-name"),
            ("value", "variable-name"),
        ])
    );
    // 型の名前と、変数の名前
    assert_eq!(
        names("typedef struct point { int x, y; } point_t; union u { int i; float f; } v;"),
        expect(&[
            ("point", "type"),
            ("x", "variable-name"),
            ("y", "variable-name"),
            ("point_t", "type"),
            ("u", "type"),
            ("i", "variable-name"),
            ("f", "variable-name"),
            ("v", "variable-name"),
        ])
    );
    // 列挙子は、変数の名前の色 (Emacs と同じ)
    assert_eq!(
        names("enum color { RED, GREEN = (1 + 2), BLUE, } c; enum { ONLY } o; enum { 1 };"),
        expect(&[
            ("color", "type"),
            ("RED", "variable-name"),
            ("GREEN", "variable-name"),
            ("BLUE", "variable-name"),
            ("c", "variable-name"),
            ("ONLY", "variable-name"),
            ("o", "variable-name"),
        ])
    );
    // 閉じない列挙子の並びは、本文の終わりまで
    assert_eq!(
        names("enum e { A, B"),
        expect(&[
            ("e", "type"),
            ("A", "variable-name"),
            ("B", "variable-name")
        ])
    );
}

#[test]
fn test_statements_in_function_bodies() {
    let text = "void g(void) {\n\
        point_t q;\n\
        for (int i = 0; i < n; i++) { other_t z; }\n\
        if (a) goto done;\n\
      again:\n\
        switch (i) { case 1: int k; break; default: int m; }\n\
        x = c ? a : b;\n\
        else point_t e;\n\
        do point_t d; while (0);\n\
        ns::x = 1;\n\
        goto again;\n\
      done:\n\
        return;\n\
    }";
    assert_eq!(
        names(text),
        expect(&[
            ("g", "function-name"),
            ("point_t", "type"),
            ("q", "variable-name"),
            ("i", "variable-name"),
            ("other_t", "type"),
            ("z", "variable-name"),
            ("done", "constant"),
            ("again", "constant"),
            ("k", "variable-name"),
            ("m", "variable-name"),
            ("point_t", "type"),
            ("e", "variable-name"),
            ("point_t", "type"),
            ("d", "variable-name"),
            ("again", "constant"),
            ("done", "constant"),
        ])
    );
    // `goto` の次が名前でなければ、色を付けない
    assert_eq!(names("goto 1; goto *p;"), vec![]);
}

#[test]
fn test_attributes_typeof_and_auto() {
    assert_eq!(
        names("__attribute__((unused)) static int attr; typeof(x) tx; __attribute__ int y;"),
        expect(&[("attr", "variable-name"), ("tx", "variable-name")])
    );
    assert_eq!(
        names_in(Language::Cpp, "auto a = 1; decltype(a) b;"),
        expect(&[("a", "variable-name"), ("b", "variable-name")])
    );
    assert_eq!(
        names_in(Language::Cpp, "class A { int x; };"),
        expect(&[("A", "type"), ("x", "variable-name")])
    );
}

#[test]
fn test_code_tokens_skip_comments_directives_and_literals() {
    // コメント、文字列、プリプロセッサの行の中は、宣言ではない
    assert_eq!(
        names("/* int a; */ // int b;\nchar *s = \"int c;\"; 'x';\n#include <int d;>\n#error int e;\nint f;"),
        expect(&[("s", "variable-name"), ("f", "variable-name")])
    );
    // 数値の次の `:` はラベルではない
    assert_eq!(names("1 : x"), vec![]);
    // C++ の生の文字列も、字句 1 つ (中の `;` で文が終わらない)
    assert_eq!(
        names_in(Language::Cpp, "auto s = R\"x(int a; )x\"; int b;"),
        expect(&[("s", "variable-name"), ("b", "variable-name")])
    );
    // 行の継続のあとの行は、指示の続き
    assert_eq!(
        names("#define A \\\n  int b;\nint c;"),
        expect(&[("A", "variable-name"), ("c", "variable-name")])
    );
}

#[test]
fn test_define_names() {
    assert_eq!(
        names("#define MAX_LEN 10\n#define EMPTY\n#  define INDENTED 1\n#undef MAX_LEN\n"),
        expect(&[
            ("MAX_LEN", "variable-name"),
            ("EMPTY", "variable-name"),
            ("INDENTED", "variable-name"),
        ])
    );
    // 名前の直後に `(` があれば、関数のような形で、名前は関数の名前、引数は変数の名前
    assert_eq!(
        names("#define SQ(x) ((x) * (x))\n#define ADD(a,b) a+b\n#define V(fmt, ...) f(fmt, __VA_ARGS__)\n"),
        expect(&[
            ("SQ", "function-name"),
            ("x", "variable-name"),
            ("ADD", "function-name"),
            ("a", "variable-name"),
            ("b", "variable-name"),
            ("V", "function-name"),
            ("fmt", "variable-name"),
        ])
    );
    // 空白があれば、関数のような形ではない
    assert_eq!(names("#define F (x)\n"), expect(&[("F", "variable-name")]));
    // 閉じない引数は、行の終わりまで. 本文の終わりでも
    assert_eq!(
        names("#define G(a, b\nint c;"),
        expect(&[
            ("G", "function-name"),
            ("a", "variable-name"),
            ("b", "variable-name"),
            ("c", "variable-name"),
        ])
    );
    assert_eq!(
        names("#define H(a"),
        expect(&[("H", "function-name"), ("a", "variable-name")])
    );
    // 名前が無い、数字で始まる、キーワードなら、色を付けない
    assert_eq!(
        names("#define\n#define 1x y\n#define if 1\n#define \n"),
        vec![]
    );
    // 引数の数字とキーワードにも、色を付けない
    assert_eq!(
        names("#define K(1, int, z) 1\n"),
        expect(&[("K", "function-name"), ("z", "variable-name")])
    );
}

/// C++ の宣言の名前.
fn cpp(text: &str) -> Vec<(String, &'static str)> {
    names_in(Language::Cpp, text)
}

#[test]
fn test_cpp_qualified_names_and_template_arguments() {
    // `::` の前は定数の色
    assert_eq!(
        cpp("std::string name; ns::Type value, *ptr;"),
        expect(&[
            ("std", "constant"),
            ("string", "type"),
            ("name", "variable-name"),
            ("ns", "constant"),
            ("Type", "type"),
            ("value", "variable-name"),
            ("ptr", "variable-name"),
        ])
    );
    // テンプレートの引数: `::` の前と、`<` が続く名前だけに色を付ける (Emacs と同じ)
    assert_eq!(
        cpp("const std::vector<int> &r = g(); Foo<Bar> m; std::map<std::string, int> t; std::vector<std::vector<int>> grid;"),
        expect(&[
            ("std", "constant"),
            ("vector", "type"),
            ("r", "variable-name"),
            ("Foo", "type"),
            ("m", "variable-name"),
            ("std", "constant"),
            ("map", "type"),
            ("std", "constant"),
            ("t", "variable-name"),
            ("std", "constant"),
            ("vector", "type"),
            ("std", "constant"),
            ("vector", "type"),
            ("grid", "variable-name"),
        ])
    );
    // `<` が閉じなければ、テンプレートの引数ではない (式の `a < b`)
    assert_eq!(cpp("a < b; x < (y; z"), vec![]);
    assert_eq!(cpp("a < b"), vec![]);
    // C では、`::` も `<` も、特別ではない
    assert_eq!(names("std::string name;"), vec![]);
}

#[test]
fn test_cpp_classes_and_members() {
    let text = "enum class E { A }; enum class F : int { B }; enum struct G;\n\
        class D : public Base, private ns::Other {\n\
        public:\n\
          D(int x);\n\
          explicit D(double d) : v(d), w{1} { }\n\
          D() = default;\n\
          virtual ~D() { f(1); }\n\
          virtual void run() const override = 0;\n\
          int get() const { g(2); return v; }\n\
        };\n";
    assert_eq!(
        cpp(text),
        expect(&[
            ("E", "type"),
            ("A", "variable-name"),
            ("F", "type"),
            ("B", "variable-name"),
            ("G", "type"),
            ("D", "type"),
            ("Base", "type"),
            ("ns", "constant"),
            ("Other", "type"),
            ("D", "function-name"),
            ("x", "variable-name"),
            ("D", "function-name"),
            ("d", "variable-name"),
            ("D", "function-name"),
            ("D", "function-name"),
            ("run", "function-name"),
            ("get", "function-name"),
        ])
    );
    // 本体の中の関数の呼び出しには、色を付けない. クラスの外の、型の無い呼び出しにも
    assert_eq!(
        cpp("foo(1); class A { void f() { g(1); } };"),
        expect(&[("A", "type"), ("f", "function-name")])
    );
    // C では、型の無い宣言を読まない
    assert_eq!(names("struct s { foo(int x); };"), expect(&[("s", "type")]));
    // コンストラクタの形が崩れていれば、読まない
    assert_eq!(
        cpp("class A { A(int x) junk ; B(; ~ 1; };"),
        expect(&[("A", "type")])
    );
}

#[test]
fn test_cpp_out_of_class_definitions() {
    assert_eq!(
        cpp("void D::run() const { }\nint D::count = 0;\nD::D(double d) : v(1) { }\nD::~D() { }\nint D::get(int i) const { return v; }\n"),
        expect(&[
            ("D", "constant"),
            ("run", "function-name"),
            ("D", "constant"),
            ("count", "variable-name"),
            ("D", "constant"),
            ("D", "function-name"),
            ("d", "variable-name"),
            ("D", "constant"),
            ("D", "function-name"),
            ("D", "constant"),
            ("get", "function-name"),
            ("i", "variable-name"),
        ])
    );
    // 修飾した名前の呼び出しは、ファイルの先頭でだけ、宣言と読む. 関数の中では読まない
    assert_eq!(
        cpp("void f() { a::b(c); }"),
        expect(&[("f", "function-name")])
    );
}

#[test]
fn test_cpp_namespace_using_and_templates() {
    assert_eq!(
        cpp("namespace ns { int inner; }\nusing namespace std;\nusing Vec = std::vector<int>;\nusing Bad = 3;\nusing 1 = x;\n"),
        expect(&[
            ("ns", "constant"),
            ("inner", "variable-name"),
            ("std", "constant"),
            ("Vec", "type"),
            ("std", "constant"),
            ("vector", "type"),
            ("Bad", "type"),
        ])
    );
    assert_eq!(
        cpp("template <typename T, class U, int N, int M = 3, Foo F> struct S { T t; };\ntemplate <typename T> T identity(T v) { return v; }\n"),
        expect(&[
            ("T", "type"),
            ("U", "type"),
            ("N", "variable-name"),
            ("M", "variable-name"),
            ("S", "type"),
            ("T", "type"),
            ("t", "variable-name"),
            ("T", "type"),
            ("T", "type"),
            ("identity", "function-name"),
            ("T", "type"),
            ("v", "variable-name"),
        ])
    );
    // 閉じない `template <` は、読み飛ばす
    assert_eq!(cpp("template < int x;"), vec![]);
    // 範囲 for
    assert_eq!(
        cpp("void f() { for (auto x : v) { y = x; } for (int i = 0; i < n; i++) { } }"),
        expect(&[
            ("f", "function-name"),
            ("x", "variable-name"),
            ("i", "variable-name"),
        ])
    );
}

#[test]
fn test_cpp_operator_functions() {
    // 色が付くのは、演算子の記号 (クラスの中). クラスの外 `A::operator+` は、`::` の前の名前だけ
    assert_eq!(
        cpp("struct V { V operator+(const V &o) const; int operator()(int a); V &operator[](int i); bool operator<<(int q) { return q; } };"),
        expect(&[
            ("V", "type"),
            ("V", "type"),
            ("+", "function-name"),
            ("V", "type"),
            ("o", "variable-name"),
            ("(", "function-name"),
            (")", "function-name"),
            ("a", "variable-name"),
            ("V", "type"),
            ("[", "function-name"),
            ("]", "function-name"),
            ("i", "variable-name"),
            ("<", "function-name"),
            ("<", "function-name"),
            ("q", "variable-name"),
        ])
    );
    assert_eq!(
        cpp("V V::operator+(const V &o) const { return o; }"),
        expect(&[
            ("V", "type"),
            ("V", "constant"),
            ("V", "type"),
            ("o", "variable-name"),
        ])
    );
    // 記号の無い `operator` は、宣言ではない
    assert_eq!(cpp("operator ;"), vec![]);
}

#[test]
fn test_cpp_typename_and_array_references() {
    assert_eq!(
        cpp("void f(typename T::value_type x); typename T::x y;"),
        expect(&[
            ("f", "function-name"),
            ("T", "constant"),
            ("value_type", "type"),
            ("x", "variable-name"),
            ("T", "constant"),
            ("x", "type"),
            ("y", "variable-name"),
        ])
    );
    assert_eq!(
        cpp("int (*pa)[4]; int (&ra)[4] = arr; typedef int (*Fn)(int); typedef int (&Arr)[4]; typedef int (*PA)[4];"),
        expect(&[
            ("pa", "variable-name"),
            ("ra", "variable-name"),
            ("Fn", "type"),
            ("PA", "type"),
        ])
    );
}

#[test]
fn test_cpp_declarations_in_conditions() {
    assert_eq!(
        cpp("void k() { if (int x = f()) { } if (auto it = m.find(key); it != m.end()) { } while (Node *n = next()) { } switch (int s = g()) { } }"),
        expect(&[
            ("k", "function-name"),
            ("x", "variable-name"),
            ("it", "variable-name"),
            ("Node", "type"),
            ("n", "variable-name"),
            ("s", "variable-name"),
        ])
    );
    // 宣言でない条件は、色を付けない (`a * b == c` の `==` は初期値ではない)
    assert_eq!(
        cpp("void k() { if (a * b == c) { } if (x == y) { } if (int z) { } if (int w = 1 { } }"),
        expect(&[("k", "function-name")])
    );
}
