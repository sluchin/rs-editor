//! 組み込みの手続きのうち, ファイル, 時刻, 正規表現, 文字列, 並べ替え, エディタの設定 (色, フォント, キー) のテスト.

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use rseditor::scheme::builtins::parse_font;
use rseditor::scheme::evaluator::Evaluator;
use rseditor::scheme::host::Host;
use rseditor::scheme::system::expand_file_name;
use rseditor::scheme::value::Value;
use tempfile::tempdir;

mod common;
use common::FakeEditor;

/// 評価して, 結果を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 結果 (失敗したら panic する).
fn eval(code: &str) -> String {
    Evaluator::new().eval(code).unwrap()
}

/// 評価して, 失敗の内容を返す.
///
/// # Arguments
///
/// * `code` - コード.
///
/// # Returns
///
/// 失敗の内容 (成功したら panic する).
fn eval_err(code: &str) -> String {
    Evaluator::new().eval(code).unwrap_err()
}

/// 文字列を, Scheme の文字列の書き方にする.
///
/// # Arguments
///
/// * `path` - パス.
///
/// # Returns
///
/// `"..."`.
fn quoted(path: &Path) -> String {
    format!("{:?}", path.to_string_lossy())
}

/// 偽のエディタ. 呼ばれた操作を覚え, `buffer-file-name` と `read-string` には, 決めた値を返す.
struct Recorder {
    /// 呼ばれた操作 (`set-font "Mono" 14` のように, 名前と引数).
    calls: Mutex<Vec<String>>,
    /// `buffer-file-name` と `read-string` の返事.
    answer: Value,
}

impl Recorder {
    /// 作る.
    ///
    /// # Arguments
    ///
    /// * `answer` - `buffer-file-name` と `read-string` の返事.
    ///
    /// # Returns
    ///
    /// 偽のエディタ.
    fn new(answer: Value) -> Self {
        Recorder {
            calls: Mutex::new(Vec::new()),
            answer,
        }
    }

    /// 呼ばれた操作を返す.
    ///
    /// # Returns
    ///
    /// 呼ばれた操作 (呼ばれた順).
    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}

impl Host for Recorder {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        let shown: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        self.calls
            .lock()
            .unwrap()
            .push(format!("{} {}", op, shown.join(" ")).trim().to_string());
        Ok(match op {
            "buffer-file-name" | "read-string" | "completing-read" | "find-file-noselect" => {
                self.answer.clone()
            }
            "buffer-list" => Value::List([self.answer.clone()].into()),
            _ => Value::Nil,
        })
    }
}

#[test]
fn test_files() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a/b/note.txt");
    let ev = Evaluator::new();
    // 親のディレクトリも作って書く
    let code = format!(
        "(write-file-string {} \"hello\nworld\n\") (read-file-string {}) (file-exists? {}) (file-directory? {}) (file-directory? {})",
        quoted(&file),
        quoted(&file),
        quoted(&file),
        quoted(&file),
        quoted(dir.path())
    );
    assert_eq!(
        ev.eval(&code).unwrap(),
        "nil\n\"hello\\nworld\\n\"\n#t\n#f\n#t"
    );
    let made = dir.path().join("x/y");
    ev.eval(&format!("(make-directory {})", quoted(&made)))
        .unwrap();
    assert!(made.is_dir());
    // ファイルの一覧 (深くまで. `.` で始まるものは除く. 名前の順)
    fs::write(dir.path().join("a/z.txt"), "").unwrap();
    fs::write(dir.path().join("a/c.md"), "").unwrap();
    fs::create_dir_all(dir.path().join(".git")).unwrap();
    fs::write(dir.path().join(".git/h.txt"), "").unwrap();
    // ディレクトリへのシンボリックリンクは, たどらない (輪になっていても, 終わる)
    std::os::unix::fs::symlink(dir.path(), dir.path().join("a/loop")).unwrap();
    let listed = ev
        .eval(&format!(
            "(directory-files-recursively {} \"\\\\.txt$\")",
            quoted(dir.path())
        ))
        .unwrap();
    assert_eq!(
        listed,
        format!(
            "({} {})",
            quoted(&dir.path().join("a/b/note.txt")),
            quoted(&dir.path().join("a/z.txt"))
        )
    );
    // 無いディレクトリは, 空
    assert_eq!(
        ev.eval("(directory-files-recursively \"/nonexistent-dir-xyz\" \"\")")
            .unwrap(),
        "()"
    );
    // 更新の時刻
    assert_eq!(
        ev.eval(&format!("(> (file-modification-time {}) 0)", quoted(&file)))
            .unwrap(),
        "#t"
    );
    assert_eq!(
        ev.eval("(file-modification-time \"/nonexistent-file-xyz\")")
            .unwrap(),
        "#f"
    );
    // 読めない, 書けない, 作れない
    let missing = dir.path().join("missing.txt");
    assert_eq!(
        ev.eval(&format!("(read-file-string {})", quoted(&missing)))
            .unwrap_err(),
        format!(
            "{}: No such file or directory (os error 2)",
            missing.display()
        )
    );
    let blocked = file.join("child");
    assert!(ev
        .eval(&format!("(write-file-string {} \"\")", quoted(&blocked)))
        .unwrap_err()
        .starts_with(&file.display().to_string()));
    assert!(ev
        .eval(&format!(
            "(write-file-string {} \"\")",
            quoted(&dir.path().join("a"))
        ))
        .unwrap_err()
        .starts_with(&dir.path().join("a").display().to_string()));
    assert!(ev
        .eval(&format!("(make-directory {})", quoted(&blocked)))
        .unwrap_err()
        .starts_with(&blocked.display().to_string()));
    assert_eq!(
        eval_err("(write-file-string \"/tmp/x\" 1)"),
        "write-file-string expects strings"
    );
    assert_eq!(
        eval_err("(directory-files-recursively \"/\" \"(\")"),
        "Invalid regexp: regex parse error:\n    (\n    ^\nerror: unclosed group"
    );
}

#[test]
fn test_expand_file_name() {
    let home = dirs::home_dir().unwrap();
    let base = Path::new("/base");
    assert_eq!(expand_file_name("~", base, &home), home);
    assert_eq!(expand_file_name("~/a", base, &home), home.join("a"));
    assert_eq!(expand_file_name("a", base, &home), base.join("a"));
    assert_eq!(expand_file_name("/abs", base, &home), Path::new("/abs"));
    let home = home.to_string_lossy();
    assert_eq!(
        eval("(expand-file-name \"~/x\") (expand-file-name \"x\") (expand-file-name \"x\" \"/d\") (expand-file-name \"x\" \"~/d\")"),
        format!("\"{0}/x\"\n\"{0}/x\"\n\"/d/x\"\n\"{0}/d/x\"", home)
    );
    assert_eq!(
        eval_err("(expand-file-name)"),
        "expand-file-name expects 1 or 2 arguments"
    );
    assert_eq!(
        eval_err("(expand-file-name 1)"),
        "expand-file-name expects strings"
    );
}

#[test]
fn test_load() {
    let dir = tempdir().unwrap();
    // 相対パスは, 読み込んでいるファイルのディレクトリから数える
    fs::create_dir_all(dir.path().join("lib")).unwrap();
    fs::write(
        dir.path().join("lib/main.sl"),
        "(define here 1) (load \"sub.sl\") (define after (+ here sub))",
    )
    .unwrap();
    fs::write(dir.path().join("lib/sub.sl"), "(define sub 2)").unwrap();
    let ev = Evaluator::new();
    assert_eq!(
        ev.eval(&format!(
            "(load {}) after",
            quoted(&dir.path().join("lib/main.sl"))
        ))
        .unwrap(),
        "#t\n3"
    );
    // 読めない, 読み込めない, 評価が失敗する
    let missing = dir.path().join("none.sl");
    assert_eq!(
        ev.eval(&format!("(load {})", quoted(&missing)))
            .unwrap_err(),
        format!("Cannot open load file: {}", missing.display())
    );
    let invalid = dir.path().join("invalid.sl");
    fs::write(&invalid, [0xff, 0xfe]).unwrap();
    assert!(ev
        .eval(&format!("(load {})", quoted(&invalid)))
        .unwrap_err()
        .starts_with(&format!("{}: ", invalid.display())));
    let broken = dir.path().join("broken.sl");
    fs::write(&broken, "(define").unwrap();
    assert_eq!(
        ev.eval(&format!("(load {})", quoted(&broken))).unwrap_err(),
        format!("{}: Unclosed list", broken.display())
    );
    let failing = dir.path().join("failing.sl");
    fs::write(&failing, "(car 1)").unwrap();
    assert_eq!(
        ev.eval(&format!("(load {})", quoted(&failing)))
            .unwrap_err(),
        format!("{}: car expects a non-empty list", failing.display())
    );
    // 失敗の後は, 読み込んでいるファイルのディレクトリを忘れる
    assert_eq!(
        ev.eval("(expand-file-name \"x\")").unwrap(),
        format!("\"{}/x\"", dirs::home_dir().unwrap().display())
    );
}

#[test]
fn test_load_path_and_require() {
    let dir = tempdir().unwrap();
    let lisp = dir.path().join("lisp");
    fs::create_dir_all(lisp.join("sub")).unwrap();
    // 拡張子 (.scm) は省ける. 読み込んでいるファイルからの相対パス
    fs::write(
        lisp.join("feat.scm"),
        "(define count (+ count 1)) (load \"./sub/inner\") (provide 'feat)",
    )
    .unwrap();
    fs::write(lisp.join("sub/inner.scm"), "(define inner 'yes)").unwrap();
    fs::write(lisp.join("plain"), "(define plain 1)").unwrap();
    fs::write(lisp.join("silent.scm"), "(define silent 1)").unwrap();
    let ev = Evaluator::new();
    // 始めの load-path は, ~/.rseditor.d/lisp と ~/.rseditor.d
    assert_eq!(
        ev.eval("load-path").unwrap(),
        "(\"~/.rseditor.d/lisp\" \"~/.rseditor.d\")"
    );
    ev.eval(&format!(
        "(define count 0) (set! load-path (list {}))",
        quoted(&lisp)
    ))
    .unwrap();
    // load-path から探す (拡張子の無いファイルも)
    assert_eq!(ev.eval("(load \"plain\") plain").unwrap(), "#t\n1");
    assert_eq!(ev.eval("(featurep 'feat)").unwrap(), "#f");
    // require: 読み込むのは, 1 回だけ
    assert_eq!(
        ev.eval("(require 'feat) (require 'feat) count inner (featurep 'feat)")
            .unwrap(),
        "feat\nfeat\n1\nyes\n#t"
    );
    // ファイルを渡す
    fs::write(lisp.join("other.scm"), "(provide 'named)").unwrap();
    assert_eq!(ev.eval("(require 'named \"other\")").unwrap(), "named");
    // provide しないと, 失敗にする
    assert_eq!(
        ev.eval("(require 'silent)").unwrap_err(),
        "Required feature was not provided: silent"
    );
    assert_eq!(
        ev.eval("(require 'nothing)").unwrap_err(),
        "Cannot open load file: nothing"
    );
    // ./ で始まる名前は, load-path から探さない
    assert_eq!(
        ev.eval("(load \"./plain\")").unwrap_err(),
        "Cannot open load file: ./plain"
    );
    // 書き方の誤り
    assert_eq!(
        eval_err("(require \"feat\")"),
        "require expects a feature (symbol) and an optional file"
    );
    assert_eq!(eval_err("(provide \"x\")"), "provide expects a symbol");
    assert_eq!(eval_err("(featurep 1)"), "featurep expects a symbol");
    for value in ["1", "(list 1)"] {
        assert_eq!(
            eval_err(&format!("(set! load-path {}) (load \"x\")", value)),
            "load-path must be a list of directories"
        );
    }
}

#[test]
fn test_time() {
    let ev = Evaluator::new();
    assert_eq!(ev.eval("(> (current-time) 1700000000)").unwrap(), "#t");
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    assert_eq!(
        ev.eval("(format-time-string \"%Y-%m-%d\")").unwrap(),
        format!("{:?}", today)
    );
    // 日付と時刻を, 行き来する
    assert_eq!(
        ev.eval("(format-time-string \"%Y-%m-%d %H:%M:%S\" (date-to-time \"2026-10-25 09:30:15\")) (format-time-string \"%Y/%m/%d %H:%M\" (date-to-time \"2026-10-25 09:30\")) (format-time-string \"%H:%M\" (date-to-time \" 2026-10-25 \"))")
            .unwrap(),
        "\"2026-10-25 09:30:15\"\n\"2026/10/25 09:30\"\n\"00:00\""
    );
    assert_eq!(
        ev.eval("(- (date-to-time \"2026-10-26\") (date-to-time \"2026-10-25\"))")
            .unwrap(),
        "86400"
    );
    assert_eq!(
        eval_err("(date-to-time \"2026-13-01\")"),
        "date-to-time: invalid date \"2026-13-01\""
    );
    assert_eq!(
        eval_err("(format-time-string \"%Q\")"),
        "format-time-string: invalid format \"%Q\""
    );
    assert_eq!(
        eval_err("(format-time-string \"%Y\" 100000000000000000000)"),
        "format-time-string: invalid time"
    );
    assert_eq!(
        eval_err("(format-time-string)"),
        "format-time-string expects 1 or 2 arguments"
    );
    assert_eq!(
        eval_err("(current-time 1)"),
        "current-time expects 0 arguments, given 1"
    );
}

#[test]
fn test_random() {
    let ev = Evaluator::new();
    for _ in 0..20 {
        let n: f64 = ev.eval("(random 3)").unwrap().parse().unwrap();
        assert!((0.0..3.0).contains(&n) && n.fract() == 0.0);
    }
    assert_eq!(eval("(random 1)"), "0");
    assert_eq!(eval_err("(random 0)"), "random expects a positive integer");
    assert_eq!(
        eval_err("(random 1.5)"),
        "random expects a positive integer"
    );
}

#[test]
fn test_regexp() {
    assert_eq!(
        eval("(regexp-match \"a(b)?(c)\" \"xac\") (regexp-match \"z\" \"abc\")"),
        "(\"ac\" #f \"c\")\n#f"
    );
    // 探し始める位置 (文字の数. 終わりも使える)
    assert_eq!(
        eval("(regexp-match \"あ.\" \"あいあう\" 1) (regexp-match \"$\" \"ab\" 2)"),
        "(\"あう\")\n(\"\")"
    );
    assert_eq!(
        eval_err("(regexp-match \"a\" \"ab\" 3)"),
        "regexp-match: index out of range"
    );
    assert_eq!(
        eval_err("(regexp-match \"a\")"),
        "regexp-match expects 2 or 3 arguments"
    );
    // 位置 (文字の数)
    assert_eq!(
        eval("(regexp-match-positions \"い(う)?(x)?\" \"あいう\") (regexp-match-positions \"z\" \"a\")"),
        "((1 3) (2 3) #f)\n#f"
    );
    assert_eq!(eval("(regexp-quote \"a.b*\")"), "\"a\\\\.b\\\\*\"");
    // 同じ正規表現を何度も使い, 覚えておく数を超える (覚えるのは, 1 回の評価のあいだ)
    let code: String = (0..70)
        .map(|i| {
            format!(
                "(regexp-match \"{0}\" \"{0}\") (regexp-match \"{0}\" \"{0}\")",
                i
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    let expected: Vec<String> = (0..70)
        .flat_map(|i| [format!("(\"{}\")", i), format!("(\"{}\")", i)])
        .collect();
    assert_eq!(eval(&code), expected.join("\n"));
}

#[test]
fn test_grep() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, "= Title\nfoo bar\n").unwrap();
    fs::write(&b, "Foo\n").unwrap();
    let missing = dir.path().join("none.txt");
    let code = format!(
        "(grep \"(?i)foo\" (list {} {} {}))",
        quoted(&a),
        quoted(&missing),
        quoted(&b)
    );
    assert_eq!(
        eval(&code),
        format!(
            "(({} 2 \"foo bar\") ({} 1 \"Foo\"))",
            quoted(&a),
            quoted(&b)
        )
    );
    // 相対のパスは, expand-file-name と同じく, 読み込んでいるファイルのディレクトリから読む
    let script = dir.path().join("search.sl");
    fs::write(&script, "(define found (grep \"bar\" (list \"a.txt\")))").unwrap();
    assert_eq!(
        eval(&format!("(load {}) found", quoted(&script))),
        "#t\n((\"a.txt\" 2 \"foo bar\"))"
    );
    assert_eq!(eval_err("(grep \"a\" 1)"), "grep expects a list of files");
    assert_eq!(eval_err("(grep \"a\" '(1))"), "grep expects strings");
}

#[test]
fn test_strings() {
    assert_eq!(
        eval("(string-search \"b\" \"abcb\") (string-search \"b\" \"abcb\" 2) (string-search \"z\" \"abc\") (string-search \"う\" \"あいう\")"),
        "1\n3\n#f\n2"
    );
    assert_eq!(
        eval_err("(string-search \"a\" \"a\" 2)"),
        "string-search: index out of range"
    );
    assert_eq!(
        eval_err("(string-search \"a\")"),
        "string-search expects 2 or 3 arguments"
    );
    assert_eq!(
        eval("(string-prefix? \"ab\" \"abc\") (string-prefix? \"b\" \"abc\") (string-suffix? \"bc\" \"abc\") (string-suffix? \"b\" \"abc\")"),
        "#t\n#f\n#t\n#f"
    );
    assert_eq!(eval("(string-trim \"  a b \n\")"), "\"a b\"");
    assert_eq!(
        eval("(string-join '(\"a\" \"b\") \", \") (string-join '(\"a\" \"b\"))"),
        "\"a, b\"\n\"ab\""
    );
    assert_eq!(
        eval_err("(string-join '(1))"),
        "string-join expects strings"
    );
    assert_eq!(
        eval_err("(string-join)"),
        "string-join expects 1 or 2 arguments"
    );
    assert_eq!(
        eval("(split-string \" a  b \") (split-string \"a,b,\" \",\")"),
        "(\"a\" \"b\")\n(\"a\" \"b\" \"\")"
    );
    assert_eq!(
        eval_err("(split-string \"a\" \"\")"),
        "split-string expects a non-empty separator"
    );
    assert_eq!(
        eval_err("(split-string)"),
        "split-string expects 1 or 2 arguments"
    );
}

#[test]
fn test_cxr() {
    assert_eq!(
        eval("(cadr '(1 2 3)) (cddr '(1 2 3)) (caddr '(1 2 3)) (caar '((1) 2)) (cdar '((1 2) 3))"),
        "2\n(3)\n3\n1\n(2)"
    );
    assert_eq!(
        eval_err("(caddr '(1 2))"),
        "caddr expects a list with enough elements"
    );
}

#[test]
fn test_sort() {
    assert_eq!(
        eval("(sort '(3 1 2 5 4) <) (sort '() <) (sort '(1) <)"),
        "(1 2 3 4 5)\n()\n(1)"
    );
    // 同じものは, 元の順 (安定)
    assert_eq!(
        eval("(sort '((1 a) (0 b) (1 c) (0 d)) (lambda (x y) (< (car x) (car y))))"),
        "((0 b) (0 d) (1 a) (1 c))"
    );
    assert_eq!(eval_err("(sort '(1 a) <)"), "< expects numbers");
}

#[test]
fn test_keywords_evaluate_to_themselves() {
    assert_eq!(eval(":family '(:a 1)"), ":family\n(:a 1)");
}

#[test]
fn test_editor_settings() {
    let ev = Evaluator::new();
    let editor = Recorder::new(Value::String("/home/a.txt".to_string()));
    let code = "(set-background-color \"black\") (set-foreground-color \"white\") (set-frame-font \"DejaVu Sans Mono-14\") (set-frame-font \"Monospace\") (set-face-attribute 'default #f :family \"Noto\" :height 120 :foreground \"gray\" :background \"#000\") (set-face-attribute 'default #f :foreground \"red\") (set-face-attribute 'default #f :height 100) (local-set-key \"C-c a\" 'foo) (find-file \"~/x.txt\") (switch-to-buffer \"*howm*\") (buffer-file-name) (set-buffer-modified-p #f) (read-string \"Search: \") (read-string \"Search: \" \"init\")";
    assert_eq!(
        ev.eval_with(code, &editor).unwrap(),
        "nil\nnil\nnil\nnil\nnil\nnil\nnil\nfoo\nnil\nnil\n\"/home/a.txt\"\n#f\n\"/home/a.txt\"\n\"/home/a.txt\""
    );
    let home = dirs::home_dir().unwrap();
    assert_eq!(
        editor.calls(),
        vec![
            "set-background-color \"black\"".to_string(),
            "set-foreground-color \"white\"".to_string(),
            "set-font \"DejaVu Sans Mono\" 14".to_string(),
            "set-font \"Monospace\" nil".to_string(),
            "set-foreground-color \"gray\"".to_string(),
            "set-background-color \"#000\"".to_string(),
            "set-font \"Noto\" 12".to_string(),
            "set-foreground-color \"red\"".to_string(),
            "set-font nil 10".to_string(),
            "local-set-key \"Ctrl-c a\" \"foo\"".to_string(),
            format!("find-file {:?}", home.join("x.txt").to_string_lossy()),
            "switch-to-buffer \"*howm*\"".to_string(),
            "buffer-file-name".to_string(),
            "set-buffer-modified-p #f".to_string(),
            "read-string \"Search: \" \"\"".to_string(),
            "read-string \"Search: \" \"init\"".to_string(),
        ]
    );
    // ファイルのないバッファ
    let nothing = Recorder::new(Value::Nil);
    assert_eq!(ev.eval_with("(buffer-file-name)", &nothing).unwrap(), "#f");
    assert_eq!(
        ev.eval_with("(read-string \"a\")", &nothing).unwrap_err(),
        "read-string: unexpected input nil"
    );
    for (code, error) in [
        (
            "(set-face-attribute 'bold #f)",
            "set-face-attribute: unsupported face bold",
        ),
        (
            "(set-face-attribute 'default)",
            "set-face-attribute expects a face, a frame and attributes",
        ),
        (
            "(set-face-attribute 'default #f :height)",
            "set-face-attribute expects pairs of an attribute and a value",
        ),
        (
            "(set-face-attribute 'default #f :weight 'bold)",
            "set-face-attribute: unsupported attribute :weight",
        ),
        (
            "(set-face-attribute 'default #f :family 1)",
            "set-face-attribute expects strings",
        ),
        (
            "(set-face-attribute 'default #f :height \"a\")",
            "set-face-attribute expects numbers",
        ),
        (
            "(set-face-attribute 'default #f :foreground 1)",
            "set-face-attribute expects strings",
        ),
        (
            "(local-set-key 1 'a)",
            "local-set-key expects a key and a command name",
        ),
        ("(local-set-key \"\" 'a)", "Invalid key: \"\""),
        ("(read-string)", "read-string expects 1 to 5 arguments"),
        ("(read-string 1)", "read-string expects strings"),
        ("(read-string \"a\" 1)", "read-string expects strings"),
        ("(switch-to-buffer 1)", "switch-to-buffer expects strings"),
        ("(find-file 1)", "find-file expects strings"),
    ] {
        assert_eq!(ev.eval_with(code, &nothing).unwrap_err(), error, "{}", code);
    }
}

#[test]
fn test_buffer_operations() {
    let ev = Evaluator::new();
    let editor = Recorder::new(Value::String("b".to_string()));
    let code = "(buffer-list) (buffer-modified-p) (buffer-modified-p \"b\") (kill-buffer \"b\") (buffer-file-name \"b\") (kill-new \"k\") (completing-read \"P: \" '(\"a\" \"b\")) (completing-read \"P: \" '() #f #f \"i\") (display-buffer \"b\") (display-buffer \"b\" 3) (find-file-noselect \"~/y\") (highlight-regexp \"a+\") (unhighlight-regexp \"a+\") (unhighlight-regexp #t)";
    assert_eq!(
        ev.eval_with(code, &editor).unwrap(),
        "(\"b\")\n#t\n#t\n#t\n\"b\"\nnil\n\"b\"\n\"b\"\nnil\nnil\n\"b\"\nnil\nnil\nnil"
    );
    let home = dirs::home_dir().unwrap();
    assert_eq!(
        editor.calls(),
        vec![
            "buffer-list".to_string(),
            "buffer-modified-p".to_string(),
            "buffer-modified-p \"b\"".to_string(),
            "kill-buffer \"b\"".to_string(),
            "buffer-file-name \"b\"".to_string(),
            "kill-new \"k\"".to_string(),
            "completing-read \"P: \" (\"a\" \"b\") \"\"".to_string(),
            "completing-read \"P: \" () \"i\"".to_string(),
            "display-buffer \"b\"".to_string(),
            "display-buffer \"b\" 3".to_string(),
            format!("find-file-noselect {:?}", home.join("y").to_string_lossy()),
            "highlight-regexp \"a+\"".to_string(),
            "unhighlight-regexp \"a+\"".to_string(),
            "unhighlight-regexp #t".to_string(),
        ]
    );
    // read は始めの式だけを読む. eval は一番外側の環境で評価する
    assert_eq!(
        ev.eval_with("(read \"(a b) %|\") (eval (list '+ 1 2))", &editor)
            .unwrap(),
        "(a b)\n3"
    );
    let nothing = Recorder::new(Value::Nil);
    for (code, error) in [
        ("(read \" \")", "End of file during parsing"),
        (
            "(completing-read \"a\" '(\"b\"))",
            "completing-read: unexpected input nil",
        ),
        (
            "(completing-read \"a\")",
            "completing-read expects 2 to 8 arguments",
        ),
        (
            "(completing-read \"a\" 1)",
            "completing-read expects a list of candidates",
        ),
        (
            "(completing-read \"a\" '(1))",
            "completing-read: not a candidate: 1",
        ),
        (
            "(display-buffer)",
            "display-buffer expects 1 or 2 arguments",
        ),
        (
            "(display-buffer \"b\" \"x\")",
            "display-buffer expects numbers",
        ),
        (
            "(unhighlight-regexp 1)",
            "unhighlight-regexp expects a regexp or #t",
        ),
        (
            "(buffer-file-name \"a\" \"b\")",
            "buffer-file-name expects 0 or 1 arguments",
        ),
        ("(kill-buffer 1)", "kill-buffer expects strings"),
    ] {
        assert_eq!(ev.eval_with(code, &nothing).unwrap_err(), error, "{}", code);
    }
}

#[test]
fn test_parse_font() {
    assert_eq!(
        parse_font("DejaVu Sans Mono-14"),
        ("DejaVu Sans Mono".to_string(), Some(14.0))
    );
    assert_eq!(
        parse_font("Noto Sans-Mono"),
        ("Noto Sans-Mono".to_string(), None)
    );
    assert_eq!(parse_font(" Mono "), ("Mono".to_string(), None));
}

#[test]
fn test_read_with_defaults() {
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    *editor.answers.lock().unwrap() = ["", "x", "", "c", "zz", "", "q", "b", "alpha"]
        .map(String::from)
        .to_vec();
    let code = r#"
(list
 (read-string "A: " #f #f "def")
 (read-string "A: " '() 'hist '("first" "second"))
 (read-string "A: ")
 (completing-read "C: " '("a" "b" "c") #f #t)
 (completing-read "C: " '("a" "b") #f #t "z" 'hist "b")
 (completing-read "C: " '(("a" . 1) ("b" . 2)) (lambda (pair) (string=? (car pair) "b")) #t)
 (completing-read "C: " '(alpha beta) #f '()))
"#;
    assert_eq!(
        ev.eval_with(code, &editor).unwrap(),
        "(\"def\" \"x\" \"\" \"c\" \"b\" \"b\" \"alpha\")"
    );
    assert_eq!(
        editor.prompts.lock().unwrap().clone(),
        vec![
            "A: ",
            "A: ",
            "A: ",
            "C: ",
            "C: ",
            "[No match] C: ",
            "C: ",
            "[No match] C: ",
            "C: "
        ]
    );
    assert_eq!(
        editor.completions.lock().unwrap().clone(),
        vec!["alpha", "beta"]
    );
    for (code, error) in [
        ("(read-string \"a\" #f #f 1)", "read-string expects strings"),
        (
            "(completing-read \"a\" '(\"b\") #f #f #f #f '(1))",
            "completing-read expects strings",
        ),
        (
            "(completing-read \"a\" '(\"b\") (lambda (x) (car 1)))",
            "car expects a non-empty list",
        ),
        (
            "(completing-read \"a\" '(()))",
            "completing-read: not a candidate: ()",
        ),
    ] {
        assert_eq!(ev.eval_with(code, &editor).unwrap_err(), error, "{}", code);
    }
}

#[test]
fn test_hooks() {
    let ev = Evaluator::new();
    assert!(ev.active_hooks().is_empty());
    let code = r#"
(define log '())
(define (note x) (set! log (cons x log)))
(define (first) (note 'first))
(define (second) (note 'second))
(add-hook 'find-file-hook 'first)
(add-hook 'find-file-hook second #t)
(add-hook 'find-file-hook 'first)
(add-hook 'my-hook (lambda (a b) (note (+ a b))) '())
(add-hook 'after-save-hook 'first #f #t)
(run-hooks 'find-file-hook 'unbound-hook)
(run-hook-with-args 'my-hook 1 2)
(remove-hook 'find-file-hook 'first)
(run-hooks 'find-file-hook)
(set! my-hook #f)
(run-hooks 'my-hook)
(define single-hook (lambda () (note 'single)))
(run-hooks 'single-hook)
(reverse log)
"#;
    assert_eq!(
        ev.eval(code).unwrap().lines().last().unwrap(),
        "(first second 3 second single)"
    );
    assert_eq!(ev.active_hooks(), vec!["find-file-hook", "after-save-hook"]);
    let response = rseditor::commands::eval_code(&ev, "1");
    assert_eq!(response.hooks, vec!["find-file-hook", "after-save-hook"]);
    for (code, error) in [
        ("(add-hook 'h)", "add-hook expects 2 to 4 arguments"),
        ("(add-hook 1 'f)", "add-hook expects a hook symbol, given 1"),
        (
            "(remove-hook 'h)",
            "remove-hook expects 2 arguments, given 1",
        ),
        ("(run-hooks 1)", "run-hooks expects a hook symbol, given 1"),
        ("(run-hook-with-args)", "run-hook-with-args expects a hook"),
        (
            "(add-hook 'h 'nothing) (run-hooks 'h)",
            "Unbound variable: nothing",
        ),
        (
            "(add-hook 'h2 (lambda () (car 1))) (run-hooks 'h2)",
            "car expects a non-empty list",
        ),
    ] {
        assert_eq!(ev.eval(code).unwrap_err(), error, "{}", code);
    }
}
