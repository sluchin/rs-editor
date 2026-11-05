//! C のシンタックスハイライト (`core/src/highlight`) のテスト.
//! 期待値は、Emacs 30 の `c-mode` の font-lock の結果 (`tests/c-highlight/` で、同じ入力を Emacs と比べる).

use std::sync::Arc;

use rseditor_core::editor::Editor;
use rseditor_core::highlight::{tokenize, Language, Span, TokenKind};
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 字句に分けて、(本文の中の文字列, 種類の名前) の並びにする.
fn lex(text: &str) -> Vec<(String, &'static str)> {
    let chars: Vec<char> = text.chars().collect();
    tokenize(Language::C, text)
        .iter()
        .map(|s| (chars[s.start..s.end].iter().collect(), s.kind.name()))
        .collect()
}

/// `(文字列, 種類)` の並びを作る.
fn tokens(items: &[(&str, &'static str)]) -> Vec<(String, &'static str)> {
    items.iter().map(|(t, k)| (t.to_string(), *k)).collect()
}

#[test]
fn test_tables_are_sorted_for_binary_search() {
    let spec = Language::C.spec().unwrap();
    for language in [Language::C, Language::Cpp] {
        let spec = language.spec().unwrap();
        for table in [spec.keywords, spec.types, spec.constants] {
            assert!(table.windows(2).all(|w| w[0] < w[1]), "{table:?}");
        }
        // 型と定数は、キーワードでもある
        for word in spec.types.iter().chain(spec.constants) {
            assert!(spec.is_keyword(word), "{word}");
        }
    }
    assert!(spec.is_keyword("while") && !spec.is_keyword("whilst"));
    assert!(spec.is_type("int") && !spec.is_type("size_t"));
    assert!(spec.is_constant("NULL") && !spec.is_constant("null"));
}

#[test]
fn test_language_from_mode_name() {
    assert_eq!(Language::from_mode_name("c-mode"), Some(Language::C));
    assert_eq!(Language::from_mode_name("c++-mode"), Some(Language::Cpp));
    assert_eq!(Language::from_mode_name("text-mode"), None);
}

#[test]
fn test_kind_names() {
    let names: Vec<&str> = [
        TokenKind::Comment,
        TokenKind::CommentDelimiter,
        TokenKind::DocComment,
        TokenKind::String,
        TokenKind::Keyword,
        TokenKind::Type,
        TokenKind::Constant,
        TokenKind::Preprocessor,
        TokenKind::Number,
        TokenKind::FunctionName,
        TokenKind::VariableName,
    ]
    .iter()
    .map(|k| k.name())
    .collect();
    assert_eq!(
        names,
        [
            "comment",
            "comment-delimiter",
            "doc",
            "string",
            "keyword",
            "type",
            "constant",
            "preprocessor",
            "number",
            "function-name",
            "variable-name"
        ]
    );
}

#[test]
fn test_keywords_types_and_constants() {
    assert_eq!(
        lex("static int x = NULL; return sizeof(unsigned);"),
        tokens(&[
            ("static", "keyword"),
            ("int", "type"),
            ("x", "variable-name"),
            ("NULL", "constant"),
            ("return", "keyword"),
            ("sizeof", "keyword"),
            ("unsigned", "type"),
        ])
    );
    // 語の一部では、色を付けない. `$` は語の文字
    assert_eq!(lex("integer int_x $if if$"), vec![]);
}

/// C++ として字句に分けて、(本文の中の文字列, 種類の名前) の並びにする.
fn lex_cpp(text: &str) -> Vec<(String, &'static str)> {
    let chars: Vec<char> = text.chars().collect();
    tokenize(Language::Cpp, text)
        .iter()
        .map(|s| (chars[s.start..s.end].iter().collect(), s.kind.name()))
        .collect()
}

#[test]
fn test_cpp_keywords_types_and_constants() {
    assert_eq!(
        lex_cpp("class A final : public B { bool b = nullptr; char8_t c; }; and not_eq"),
        tokens(&[
            ("class", "keyword"),
            ("final", "keyword"),
            ("public", "keyword"),
            ("bool", "type"),
            ("b", "variable-name"),
            ("nullptr", "constant"),
            ("char8_t", "type"),
            ("c", "variable-name"),
            ("and", "keyword"),
            ("not_eq", "keyword"),
        ])
    );
    // C では、C++ のキーワードではない
    assert_eq!(lex("class bool nullptr"), vec![]);
}

#[test]
fn test_cpp_raw_strings() {
    // 色が付くのは `(` から `)` まで. 区切りと引用符は除く
    assert_eq!(
        lex_cpp(r#"R"(a "b" \)" u8R"x(c)x" LR"(d)" uR"(e)" UR"(f)""#),
        tokens(&[
            (r#"(a "b" \)"#, "string"),
            ("(c)", "string"),
            ("(d)", "string"),
            ("(e)", "string"),
            ("(f)", "string"),
        ])
    );
    // 行をまたぐ. `)` の後が区切りと引用符でなければ、終わりではない
    assert_eq!(
        lex_cpp("R\"x(l1\n)y\" l2)x\" int"),
        tokens(&[("(l1\n)y\" l2)", "string"), ("int", "type")])
    );
    // 閉じなければ、`(` の次から本文の終わりまで
    assert_eq!(lex_cpp("R\"(open\nint"), tokens(&[("open\nint", "string")]));
    // 区切りが正しくなければ、普通の文字列
    assert_eq!(
        lex_cpp("R\"a b(x)a b\""),
        tokens(&[("\"a b(x)a b\"", "string")])
    );
    assert_eq!(lex_cpp("R\"a)b(x)\"")[0].0, "\"a)b(x)\"");
    assert_eq!(lex_cpp("R\"a\\b(x)\"")[0].0, "\"a\\b(x)\"");
    assert_eq!(lex_cpp("R\"a\"b(x)\"")[0].0, "\"a\"");
    assert_eq!(
        lex_cpp("R\"abcdefghijklmnopq(x)abcdefghijklmnopq\"")[0]
            .0
            .chars()
            .next(),
        Some('"')
    );
    assert_eq!(lex_cpp("R\"abc")[0].0, "abc");
    // 16 文字の区切りは使える
    assert_eq!(
        lex_cpp("R\"abcdefghijklmnop(x)abcdefghijklmnop\"")[0].0,
        "(x)"
    );
    // R の後が引用符でなければ、ただの語. 別の語の後ろの R も
    assert_eq!(lex_cpp("R + FR\"x\""), tokens(&[("\"x\"", "string")]));
    // C では、生の文字列ではない
    assert_eq!(lex("R\"(x)\""), tokens(&[("\"(x)\"", "string")]));
}

#[test]
fn test_cpp_digit_separators() {
    assert_eq!(
        lex_cpp("1'000'000 0x1'F 'a'"),
        tokens(&[
            ("1'000'000", "number"),
            ("0x1'F", "number"),
            ("'a'", "string"),
        ])
    );
    // 区切りは、数字の間だけ
    assert_eq!(
        lex_cpp("1' x"),
        tokens(&[("1", "number"), (" x", "string")])
    );
    // C では、区切りではない
    assert_eq!(lex("1'0"), tokens(&[("1", "number"), ("0", "string")]));
}

#[test]
fn test_numbers() {
    assert_eq!(
        lex("0x1F 1.5e-3 .5f 10UL 1e+9 a-1"),
        tokens(&[
            ("0x1F", "number"),
            ("1.5e-3", "number"),
            (".5f", "number"),
            ("10UL", "number"),
            ("1e+9", "number"),
            ("1", "number"),
        ])
    );
    // 指数の符号でない + と - は、数値の終わり
    assert_eq!(lex("1+2"), tokens(&[("1", "number"), ("2", "number")]));
}

#[test]
fn test_strings_and_characters() {
    assert_eq!(
        lex(r#"s = "a\"b" 'c' '\'' L"w""#),
        tokens(&[
            (r#""a\"b""#, "string"),
            ("'c'", "string"),
            (r"'\''", "string"),
            (r#""w""#, "string"),
        ])
    );
    // 行の継続で、文字列は次の行へ続く
    assert_eq!(lex("\"a\\\nb\" x"), tokens(&[("\"a\\\nb\"", "string")]));
    // 閉じない文字列は、行の終わりまで (始まりの引用符は含めない)
    assert_eq!(
        lex("\"abc\nint"),
        tokens(&[("abc", "string"), ("int", "type")])
    );
    // 本文の終わりで閉じない文字列と、終わりの \
    assert_eq!(lex("\"abc"), tokens(&[("abc", "string")]));
    assert_eq!(lex("\"abc\\"), tokens(&[("abc\\", "string")]));
}

#[test]
fn test_block_comments() {
    assert_eq!(
        lex("/* a b */"),
        tokens(&[
            ("/* ", "comment-delimiter"),
            ("a b", "comment"),
            (" */", "comment-delimiter")
        ])
    );
    assert_eq!(
        lex("/*x*/"),
        tokens(&[
            ("/*", "comment-delimiter"),
            ("x", "comment"),
            ("*/", "comment-delimiter")
        ])
    );
    // 本文が空なら、全体が区切り
    assert_eq!(lex("/**/"), tokens(&[("/**/", "comment-delimiter")]));
    assert_eq!(lex("/*   */"), tokens(&[("/*   */", "comment-delimiter")]));
    assert_eq!(lex("/*****/"), tokens(&[("/*****/", "comment-delimiter")]));
    // 始まりの * は区切り. 終わりの * は、本文
    assert_eq!(
        lex("/*** x ***/"),
        tokens(&[
            ("/*** ", "comment-delimiter"),
            ("x **", "comment"),
            ("*/", "comment-delimiter")
        ])
    );
    // /** のドキュメントのコメントは、全体
    assert_eq!(
        lex("/** x */ int"),
        tokens(&[("/** x */", "doc"), ("int", "type")])
    );
    // /*** は、ドキュメントではない
    assert_eq!(lex("/*** x */")[0].1, "comment-delimiter");
    // 閉じないコメントは、本文の終わりまで
    assert_eq!(
        lex("/* open"),
        tokens(&[("/* ", "comment-delimiter"), ("open", "comment")])
    );
    assert_eq!(lex("/** open"), tokens(&[("/** open", "doc")]));
    // コメントの中は、字句に分けない
    assert_eq!(lex("/* int \"x\" // y */").len(), 3);
}

#[test]
fn test_line_comments() {
    assert_eq!(
        lex("x; // note\nint"),
        tokens(&[
            ("// ", "comment-delimiter"),
            ("note", "comment"),
            ("int", "type")
        ])
    );
    assert_eq!(
        lex("/// doc"),
        tokens(&[("/// ", "comment-delimiter"), ("doc", "comment")])
    );
    assert_eq!(
        lex("//!"),
        tokens(&[("//", "comment-delimiter"), ("!", "comment")])
    );
    // 本文が空
    assert_eq!(lex("//\nx"), tokens(&[("//", "comment-delimiter")]));
    // 行の継続で、次の行もコメント
    assert_eq!(
        lex("// a \\\nb\nint"),
        tokens(&[
            ("// ", "comment-delimiter"),
            ("a \\\nb", "comment"),
            ("int", "type")
        ])
    );
}

#[test]
fn test_preprocessor_directives() {
    assert_eq!(
        lex("#include <a/b.h>\n#  include \"c.h\"\n"),
        tokens(&[
            ("#include", "preprocessor"),
            ("<a/b.h>", "string"),
            ("#  include", "preprocessor"),
            ("\"c.h\"", "string"),
        ])
    );
    // 行頭の空白も、指示の色
    assert_eq!(
        lex("  # define X 1"),
        tokens(&[
            ("  # define", "preprocessor"),
            ("X", "variable-name"),
            ("1", "number")
        ])
    );
    // < が閉じない #include
    assert_eq!(
        lex("#include <abc\nint"),
        tokens(&[("#include", "preprocessor"), ("int", "type")])
    );
    // 行の途中の # は、指示ではない
    assert_eq!(lex("x # y"), vec![]);
    // 名前の無い #
    assert_eq!(lex("#\nint"), tokens(&[("int", "type")]));
    // 行の継続で、指示は続く. 次の行の先頭の # は、指示ではない
    assert_eq!(
        lex("#define A \\\n  # B\nint"),
        tokens(&[
            ("#define", "preprocessor"),
            ("A", "variable-name"),
            ("int", "type")
        ])
    );
    // 指示は、行の終わりで終わる (次の行の < は、#include の < ではない)
    assert_eq!(
        lex("#include <a>\nx < y"),
        tokens(&[("#include", "preprocessor"), ("<a>", "string")])
    );
}

#[test]
fn test_conditions_and_messages() {
    // defined は、#if と #elif の行だけ. 語の中の `_` で区切った部分も (Emacs と同じ)
    assert_eq!(
        lex("#if defined A && !defined_x\n#elif a_defined\n#ifdef defined\n"),
        tokens(&[
            ("#if", "preprocessor"),
            ("defined", "preprocessor"),
            ("defined", "preprocessor"),
            ("#elif", "preprocessor"),
            ("defined", "preprocessor"),
            ("#ifdef", "preprocessor"),
        ])
    );
    // #error と #warning の本文は、文字列 (前後の空白を除く)
    assert_eq!(
        lex("#error  stop here \t\r\n#warning x\nint"),
        tokens(&[
            ("#error", "preprocessor"),
            ("stop here", "string"),
            ("#warning", "preprocessor"),
            ("x", "string"),
            ("int", "type"),
        ])
    );
    // 本文が無い
    assert_eq!(
        lex("#error\nint"),
        tokens(&[("#error", "preprocessor"), ("int", "type")])
    );
    // 本文の終わりで終わる
    assert_eq!(
        lex("#warning w"),
        tokens(&[("#warning", "preprocessor"), ("w", "string")])
    );
}

#[test]
fn test_spans_use_character_positions() {
    // 日本語の文字は、1 文字として数える
    let spans = tokenize(Language::C, "/* あ */ int");
    assert_eq!(
        spans.last(),
        Some(&Span {
            start: 8,
            end: 11,
            kind: TokenKind::Type
        })
    );
}

/// 何も受けない偽の画面.
struct NoUi;

impl Host for NoUi {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => Ok(Value::Nil),
            other => Err(format!("ui: {other}").into()),
        }
    }
}

/// 基本のコマンドとファイルのコマンドを読んだ評価器とエディタ.
fn setup(text: &str) -> (Evaluator, EditorHost<NoUi>) {
    let ev = Evaluator::new();
    let host = EditorHost::new(Editor::new(), NoUi);
    ev.eval_with(include_str!("../../lisp/simple.scm"), &host)
        .unwrap();
    ev.eval_with(include_str!("../../lisp/files.scm"), &host)
        .unwrap();
    host.editor().current_buffer_mut().load(text);
    (ev, host)
}

#[test]
fn test_buffer_spans_follow_the_major_mode_and_the_text() {
    let (ev, host) = setup("int x;");
    // text-mode では、色が付かない
    ev.eval_with("(text-mode)", &host).unwrap();
    assert!(host.editor().current_buffer().token_spans().is_empty());
    assert_eq!(host.editor().current_buffer().highlight_language(), None);
    ev.eval_with("(c-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::C)
    );
    let first = host.editor().current_buffer().token_spans();
    assert_eq!(first.len(), 2);
    // 本文が同じなら、同じ結果を使い回す
    let again = host.editor().current_buffer().token_spans();
    assert!(Arc::ptr_eq(&first, &again));
    // 本文を書き換えると、作り直す
    ev.eval_with(r#"(goto-char (point-max)) (insert " char c;")"#, &host)
        .unwrap();
    let edited = host.editor().current_buffer().token_spans();
    assert!(!Arc::ptr_eq(&first, &edited));
    assert_eq!(edited.len(), 4);
    // C++ に替えると、C++ の表を使う
    ev.eval_with("(c++-mode)", &host).unwrap();
    assert_eq!(
        host.editor().current_buffer().highlight_language(),
        Some(Language::Cpp)
    );
    assert_eq!(host.editor().current_buffer().token_spans().len(), 4);
    // 別のモードに替えると、色が付かない
    ev.eval_with("(text-mode)", &host).unwrap();
    assert!(host.editor().current_buffer().token_spans().is_empty());
}

#[test]
fn test_token_spans_procedure() {
    let (ev, host) = setup("int x; // c");
    ev.eval_with("(c-mode)", &host).unwrap();
    // 位置は 1 から数える. 終わりは含まない
    assert_eq!(
        ev.eval_with("(token-spans)", &host).unwrap(),
        "((1 4 type) (5 6 variable-name) (8 11 comment-delimiter) (11 12 comment))"
    );
    assert_eq!(ev.eval_with("major-mode", &host).unwrap(), "c-mode");
}

#[test]
fn test_set_auto_mode_chooses_c_mode_for_c_files() {
    let (ev, host) = setup("int x;");
    for (file, mode) in [
        ("/tmp/a.c", "c-mode"),
        ("/tmp/a.h", "c-mode"),
        ("/tmp/a.cpp", "c++-mode"),
        ("/tmp/a.cc", "c++-mode"),
        ("/tmp/a.hpp", "c++-mode"),
        ("/tmp/a.c++", "c++-mode"),
        ("/tmp/a.CPP", "c++-mode"),
        ("/tmp/a.ii", "c++-mode"),
        ("/tmp/a.txt", "text-mode"),
        ("/tmp/a.scm", "scheme-mode"),
        ("/tmp/a.el", "emacs-lisp-mode"),
        ("/tmp/a.rs", "rust-mode"),
        ("/tmp/a.txt2", "fundamental-mode"),
    ] {
        ev.eval_with(
            &format!(r#"(files--set-buffer-file-name "{file}") (set-auto-mode) major-mode"#),
            &host,
        )
        .unwrap();
        assert_eq!(ev.eval_with("major-mode", &host).unwrap(), mode, "{file}");
    }
}

#[test]
fn test_line_token_spans_are_clipped_to_the_line() {
    let (ev, host) = setup("int a; /* x\ny */ char b;\n");
    // 言語が無ければ、空
    ev.eval_with("(text-mode)", &host).unwrap();
    assert!(host
        .editor()
        .current_buffer()
        .line_token_spans(0)
        .is_empty());
    ev.eval_with("(c-mode)", &host).unwrap();
    let editor = host.editor();
    let buffer = editor.current_buffer();
    use rseditor_core::highlight::TokenKind::*;
    // 行の中の位置. 複数行のコメントは、行の終わりと始めで切る (本文は 1 行目が 'x'、2 行目が 'y')
    assert_eq!(
        buffer.line_token_spans(0),
        vec![
            (0, 3, Type),
            (4, 5, VariableName),
            (7, 10, CommentDelimiter),
            (10, 11, Comment)
        ]
    );
    assert_eq!(
        buffer.line_token_spans(1),
        vec![
            (0, 1, Comment),
            (1, 4, CommentDelimiter),
            (5, 9, Type),
            (10, 11, VariableName)
        ]
    );
    // 最後の空の行と、本文の外の行
    assert!(buffer.line_token_spans(2).is_empty());
    assert!(buffer.line_token_spans(9).is_empty());
}
