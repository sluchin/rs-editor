//! howm (`lisp/howm.sl`) のテスト. 偽のエディタ (バッファを持つ) の上で, メニュー, メモを作る, 一覧, 検索, 予定と todo, リンクを確かめる.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rseditor::scheme::evaluator::Evaluator;
use rseditor::scheme::host::Host;
use rseditor::scheme::value::Value;
use tempfile::{tempdir, TempDir};

/// 偽のエディタのバッファ.
#[derive(Default, Clone)]
struct FakeBuffer {
    /// 本文 (1 文字ずつ).
    text: Vec<char>,
    /// カーソルの位置 (1 から数える).
    point: usize,
    /// ファイルのパス.
    file: Option<String>,
    /// 書き換えたら `true`.
    modified: bool,
    /// バッファだけのキー (キーとコマンド).
    keys: HashMap<String, String>,
}

/// 偽のエディタ (バッファを名前で持つ).
#[derive(Default)]
struct FakeEditor {
    /// バッファ (名前ごと).
    buffers: Mutex<HashMap<String, FakeBuffer>>,
    /// 出しているバッファの名前.
    current: Mutex<String>,
    /// 最後のメッセージ.
    message: Mutex<String>,
    /// read-string の答え (前から使う).
    answers: Mutex<Vec<String>>,
}

impl FakeEditor {
    /// *scratch* だけの, 偽のエディタを作る.
    ///
    /// # Returns
    ///
    /// 偽のエディタ.
    fn new() -> Self {
        let editor = FakeEditor::default();
        editor.buffers.lock().unwrap().insert(
            "*scratch*".to_string(),
            FakeBuffer {
                point: 1,
                ..Default::default()
            },
        );
        *editor.current.lock().unwrap() = "*scratch*".to_string();
        editor
    }

    /// 出しているバッファを返す.
    ///
    /// # Returns
    ///
    /// バッファの複製.
    fn buffer(&self) -> FakeBuffer {
        let current = self.current.lock().unwrap().clone();
        self.buffers.lock().unwrap()[&current].clone()
    }

    /// 出しているバッファの名前を返す.
    ///
    /// # Returns
    ///
    /// 名前.
    fn name(&self) -> String {
        self.current.lock().unwrap().clone()
    }

    /// 出しているバッファの本文を返す.
    ///
    /// # Returns
    ///
    /// 本文.
    fn text(&self) -> String {
        self.buffer().text.iter().collect()
    }

    /// 出しているバッファの, カーソルの行を返す.
    ///
    /// # Returns
    ///
    /// 行.
    fn current_line(&self) -> String {
        let buffer = self.buffer();
        let before: String = buffer.text[..buffer.point - 1].iter().collect();
        let after: String = buffer.text[buffer.point - 1..].iter().collect();
        let start = before.rsplit('\n').next().unwrap().to_string();
        start + after.split('\n').next().unwrap()
    }

    /// 最後のメッセージを返す.
    ///
    /// # Returns
    ///
    /// メッセージ.
    fn message(&self) -> String {
        self.message.lock().unwrap().clone()
    }

    /// カーソルを, 文字の始めに置く.
    ///
    /// # Arguments
    ///
    /// * `text` - 探す文字 (出しているバッファの中).
    fn move_to(&self, text: &str) {
        let body = self.text();
        let byte = body
            .find(text)
            .unwrap_or_else(|| panic!("{} in {}", text, body));
        let point = body[..byte].chars().count() + 1;
        self.with_buffer(|b| b.point = point);
    }

    /// 出しているバッファを書き換える.
    ///
    /// # Arguments
    ///
    /// * `f` - 書き換える関数.
    fn with_buffer<T>(&self, f: impl FnOnce(&mut FakeBuffer) -> T) -> T {
        let current = self.current.lock().unwrap().clone();
        let mut buffers = self.buffers.lock().unwrap();
        f(buffers.get_mut(&current).unwrap())
    }

    /// バッファを出す (無ければ, 作る).
    ///
    /// # Arguments
    ///
    /// * `name` - 名前.
    /// * `make` - 無いときに作るバッファ.
    fn show(&self, name: &str, make: impl FnOnce() -> FakeBuffer) {
        self.buffers
            .lock()
            .unwrap()
            .entry(name.to_string())
            .or_insert_with(make);
        *self.current.lock().unwrap() = name.to_string();
    }

    /// 出しているバッファの, キーのコマンドを返す.
    ///
    /// # Arguments
    ///
    /// * `key` - キー.
    ///
    /// # Returns
    ///
    /// コマンドの名前.
    fn key(&self, key: &str) -> Option<String> {
        self.buffer().keys.get(key).cloned()
    }
}

/// 位置の数を返す.
///
/// # Arguments
///
/// * `v` - 位置.
///
/// # Returns
///
/// 位置 (1 から).
fn pos(v: &Value) -> usize {
    match v {
        Value::Number(n) => *n as usize,
        _ => panic!("not a position: {}", v),
    }
}

/// 文字列を返す.
///
/// # Arguments
///
/// * `v` - 値.
///
/// # Returns
///
/// 文字列.
fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        _ => panic!("not a string: {}", v),
    }
}

impl Host for FakeEditor {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        Ok(match op {
            "point" => Value::Number(self.buffer().point as f64),
            "point-min" => Value::Number(1.0),
            "point-max" => Value::Number(self.buffer().text.len() as f64 + 1.0),
            "goto-char" => {
                let point = pos(&args[0]).clamp(1, self.buffer().text.len() + 1);
                self.with_buffer(|b| b.point = point);
                Value::Number(point as f64)
            }
            "insert" => {
                let inserted: Vec<char> =
                    args.iter().map(text).collect::<String>().chars().collect();
                self.with_buffer(|b| {
                    let at = b.point - 1;
                    b.text.splice(at..at, inserted.iter().copied());
                    b.point += inserted.len();
                    b.modified = true;
                });
                Value::Nil
            }
            "delete-region" => {
                let (from, to) = (pos(&args[0]), pos(&args[1]));
                self.with_buffer(|b| {
                    b.text.drain(from - 1..to - 1);
                    b.point = b.point.min(b.text.len() + 1);
                    b.modified = true;
                });
                Value::Nil
            }
            "buffer-string" => Value::String(self.text()),
            "buffer-substring" => {
                let (from, to) = (pos(&args[0]), pos(&args[1]));
                Value::String(self.buffer().text[from - 1..to - 1].iter().collect())
            }
            "buffer-name" => Value::String(self.name()),
            "buffer-file-name" => self.buffer().file.map_or(Value::Nil, Value::String),
            "message" => {
                *self.message.lock().unwrap() = text(&args[0]);
                Value::Nil
            }
            "switch-to-buffer" => {
                self.show(&text(&args[0]), || FakeBuffer {
                    point: 1,
                    ..Default::default()
                });
                Value::Nil
            }
            "find-file" => {
                let path = text(&args[0]);
                let existing = self
                    .buffers
                    .lock()
                    .unwrap()
                    .iter()
                    .find(|(_, b)| b.file.as_deref() == Some(&path))
                    .map(|(name, _)| name.clone());
                match existing {
                    Some(name) => self.show(&name, FakeBuffer::default),
                    None => self.show(&path.clone(), || FakeBuffer {
                        text: fs::read_to_string(&path)
                            .unwrap_or_default()
                            .chars()
                            .collect(),
                        point: 1,
                        file: Some(path),
                        ..Default::default()
                    }),
                }
                Value::Nil
            }
            "set-buffer-modified-p" => {
                let modified = args[0].is_true();
                self.with_buffer(|b| b.modified = modified);
                Value::Nil
            }
            "local-set-key" => {
                let (key, command) = (text(&args[0]), text(&args[1]));
                self.with_buffer(|b| b.keys.insert(key, command));
                Value::Nil
            }
            "read-string" => Value::String(self.answers.lock().unwrap().remove(0)),
            "command" => match text(&args[0]).as_str() {
                "newline" => return self.call("insert", vec![Value::String("\n".to_string())]),
                other => return Err(format!("Undefined command: {}", other)),
            },
            _ => panic!("unknown op {}", op),
        })
    }
}

/// howm を読み込んだ評価器と, 偽のエディタと, メモのディレクトリを作る.
///
/// # Returns
///
/// 評価器, 偽のエディタ, メモのディレクトリ (消さないように持つ).
fn setup() -> (Evaluator, FakeEditor, TempDir) {
    let dir = tempdir().unwrap();
    let howm = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lisp/howm.sl");
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    ev.eval_with(
        &format!(
            "(load {:?}) (set! howm-directory {:?})",
            howm.to_string_lossy(),
            dir.path().to_string_lossy()
        ),
        &editor,
    )
    .unwrap();
    (ev, editor, dir)
}

/// メモを書く.
///
/// # Arguments
///
/// * `dir` - メモのディレクトリ.
/// * `name` - ファイルの名前 (`2026/10/2026-10-25-120000.txt` など).
/// * `body` - 中身.
///
/// # Returns
///
/// パス.
fn write_memo(dir: &Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, body).unwrap();
    path
}

/// 今日から n 日後の日付.
///
/// # Arguments
///
/// * `days` - 日数.
///
/// # Returns
///
/// `2026-10-25` の形の日付.
fn day(days: i64) -> String {
    (chrono::Local::now() + chrono::Duration::days(days))
        .format("%Y-%m-%d")
        .to_string()
}

#[test]
fn test_commands_and_keys() {
    let (ev, _editor, _dir) = setup();
    for name in [
        "howm",
        "howm-menu",
        "howm-create",
        "howm-list-all",
        "howm-search-fixed",
        "howm-action",
    ] {
        assert!(ev.commands().iter().any(|c| c == name), "{}", name);
    }
    assert!(ev
        .key_bindings()
        .iter()
        .any(|k| k.key == "Ctrl-c , ," && k.command == "howm-menu"));
}

#[test]
fn test_create_and_menu() {
    let (ev, editor, dir) = setup();
    // メモを作る
    ev.run_command("howm-create", &editor).unwrap();
    let file = editor.buffer().file.unwrap();
    let today = day(0);
    assert!(file.starts_with(&dir.path().to_string_lossy().into_owned()));
    assert!(
        file.contains(&format!("/{}-", today)),
        "{} should contain {}",
        file,
        today
    );
    assert!(Path::new(&file).parent().unwrap().is_dir());
    assert!(editor.text().starts_with(&format!("= \n[{} ", today)));
    assert_eq!(editor.buffer().point, 3);
    assert_eq!(
        editor.key("Enter").as_deref(),
        Some("howm-action-or-newline")
    );
    // 保存した (ことにする)
    ev.eval_with("(insert \"買い物\")", &editor).unwrap();
    fs::write(&file, editor.text()).unwrap();

    // メニュー (無ければ, 既定のメニューを書く)
    ev.run_command("howm", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:%menu%*");
    let menu = editor.text();
    assert!(menu.contains(" c[新規] 検索(s[固定] g[正規])"), "{}", menu);
    assert!(menu.contains("を 7 日先まで表示"));
    assert!(menu.contains("上位 50 個まで表示"));
    assert!(menu.contains(" | 買い物"), "{}", menu);
    assert!(dir.path().join("0000-00-00-000000.txt").exists());
    assert!(!editor.buffer().modified);
    assert_eq!(editor.key("c").as_deref(), Some("howm-create"));
    assert_eq!(editor.key("R").as_deref(), Some("howm-menu-refresh"));
    assert_eq!(editor.key("Enter").as_deref(), Some("howm-action"));

    // 最近のメモの行で RET を押すと, そのメモを開く
    editor.move_to(" | 買い物");
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(editor.buffer().file.as_deref(), Some(file.as_str()));

    // ボタンの上の RET
    ev.run_command("howm", &editor).unwrap();
    editor.move_to("[一覧]");
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
    assert!(editor.text().ends_with(" | 買い物"));
    assert_eq!(editor.message(), "1 items");
    assert_eq!(editor.key("n").as_deref(), Some("next-line"));

    // 動かすものの無い所
    ev.run_command("howm", &editor).unwrap();
    editor.move_to("最近のメモ");
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:%menu%*");

    // メニューを直す
    ev.run_command("howm-menu-edit", &editor).unwrap();
    assert!(editor.text().starts_with("= <<< %menu%"));
}

#[test]
fn test_menu_file() {
    let (ev, editor, dir) = setup();
    let menu = write_memo(
        dir.path(),
        "menu.txt",
        "%\"x\"[一覧] [速記] %sdays\n%recent",
    );
    ev.eval_with(&format!("(set! howm-menu-file {:?})", menu), &editor)
        .unwrap();
    ev.run_command("howm", &editor).unwrap();
    // メモがなければ, 項目は無い. 使えないボタンには, キーを割り当てない
    assert_eq!(editor.text(), "x[一覧] [速記] 7");
    assert_eq!(editor.key("x").as_deref(), Some("howm-list-all"));
    editor.move_to("[速記]");
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(editor.message(), "No action here");
    // メモが無いときの一覧
    ev.run_command("howm-list-all", &editor).unwrap();
    assert_eq!(editor.message(), "No memo");
}

#[test]
fn test_search_and_links() {
    let (ev, editor, dir) = setup();
    let old = write_memo(
        dir.path(),
        "2026/09/2026-09-01-100000.txt",
        "= Emacs\n<<< Emacs\nエディタ\n",
    );
    let new = write_memo(
        dir.path(),
        "2026/10/2026-10-01-100000.txt",
        "= 日記\nemacs を使う\n>>> emacs\n",
    );
    // 除くファイル
    write_memo(dir.path(), "2026/10/2026-10-01-100000.txt~", "emacs");

    // 小文字だけなら, 大文字と小文字を区別しない. come-from の行を先に
    editor.answers.lock().unwrap().push("emacs".to_string());
    ev.run_command("howm-search-fixed", &editor).unwrap();
    assert_eq!(
        editor.text(),
        "2026-09-01-100000 | <<< Emacs\n2026-10-01-100000 | emacs を使う\n2026-10-01-100000 | >>> emacs\n2026-09-01-100000 | = Emacs"
    );
    assert_eq!(editor.message(), "4 items");
    // 2 行目の RET で, そのメモのその行を開く
    editor.move_to("2026-10-01-100000 | emacs");
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(
        editor.buffer().file.as_deref(),
        Some(&*new.to_string_lossy())
    );
    assert_eq!(editor.current_line(), "emacs を使う");

    // リンクの上の RET で, 言葉を探す
    editor.move_to(">>> emacs");
    ev.run_command("howm-action-or-newline", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
    assert!(editor.text().starts_with("2026-09-01-100000 | <<< Emacs"));
    // リンクの無い所では, 改行を入れる
    ev.eval_with(&format!("(find-file {:?})", old), &editor)
        .unwrap();
    editor.move_to("エディタ");
    ev.run_command("howm-action-or-newline", &editor).unwrap();
    assert_eq!(editor.text(), "= Emacs\n<<< Emacs\n\nエディタ\n");

    // 大文字を含むと, 区別する
    editor.answers.lock().unwrap().push("Emacs".to_string());
    ev.run_command("howm-search-fixed", &editor).unwrap();
    assert_eq!(editor.message(), "2 items");
    // 正規表現
    editor.answers.lock().unwrap().push("^= .*記$".to_string());
    ev.run_command("howm-search-regexp", &editor).unwrap();
    assert_eq!(editor.text(), "2026-10-01-100000 | = 日記");
    // 見つからない, 空の入力
    editor
        .answers
        .lock()
        .unwrap()
        .extend(["none".to_string(), String::new(), String::new()]);
    ev.run_command("howm-search-fixed", &editor).unwrap();
    assert_eq!(editor.message(), "No match: none");
    ev.run_command("howm-search-fixed", &editor).unwrap();
    ev.run_command("howm-search-regexp", &editor).unwrap();
    assert_eq!(editor.message(), "No match: none");

    // 一覧 (新しい順) と, 最近
    ev.run_command("howm-list-all", &editor).unwrap();
    assert_eq!(
        editor.text(),
        "2026-10-01-100000 | 日記\n2026-09-01-100000 | Emacs"
    );
    ev.run_command("howm-list-recent", &editor).unwrap();
    assert_eq!(editor.message(), "2 items");
    // 一覧の外の行
    ev.eval_with("(goto-char (point-max)) (insert \"\\n\")", &editor)
        .unwrap();
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
}

#[test]
fn test_today_and_yesterday() {
    let (ev, editor, dir) = setup();
    write_memo(
        dir.path(),
        "a.txt",
        &format!("= 今日\n[{} 10:00]\n", day(0)),
    );
    write_memo(
        dir.path(),
        "b.txt",
        &format!("= 昨日\n[{} 10:00]\n", day(-1)),
    );
    ev.run_command("howm-today", &editor).unwrap();
    assert_eq!(editor.text(), format!("a | [{} 10:00]", day(0)));
    ev.run_command("howm-yesterday", &editor).unwrap();
    assert_eq!(editor.text(), format!("b | [{} 10:00]", day(-1)));
}

#[test]
fn test_schedule_and_todo() {
    let (ev, editor, dir) = setup();
    write_memo(
        dir.path(),
        "r.txt",
        &format!(
            "= 予定\n[{}]@ 会議\n[{}]@ 遠い予定\n[{}]+ 買い物\n[{}]! 〆切\n[{}]. 済み\n[{}]- 覚書\n[{}]~30 保留\n[{}]-3 未来の覚書\n",
            day(2),
            day(30),
            day(-7),
            day(1),
            day(-1),
            day(-2),
            day(-10),
            day(5),
        ),
    );
    // 予定は日付の順 (全て)
    ev.run_command("howm-list-schedule", &editor).unwrap();
    assert_eq!(
        editor.text(),
        format!("r | [{}]@ 会議\nr | [{}]@ 遠い予定", day(2), day(30))
    );
    // todo は優先度の順 (済みと予定は除く. 優先度は, 時刻で変わらない範囲にした)
    ev.run_command("howm-list-todo", &editor).unwrap();
    let todo: Vec<String> = editor.text().lines().map(str::to_string).collect();
    assert_eq!(
        todo,
        vec![
            format!("r | [{}]+ 買い物", day(-7)),
            format!("r | [{}]! 〆切", day(1)),
            format!("r | [{}]- 覚書", day(-2)),
            format!("r | [{}]~30 保留", day(-10)),
            format!("r | [{}]-3 未来の覚書", day(5)),
        ]
    );
    // メニューには, 7 日先までの予定と, todo
    ev.run_command("howm", &editor).unwrap();
    let menu = editor.text();
    assert!(
        menu.contains(&format!("\n[{}]@ 会議\n[{}]+ 買い物\n", day(2), day(-7))),
        "{}",
        menu
    );
    assert!(!menu.contains("遠い予定"));
    // 項目の行の RET で, メモのその行を開く
    editor.move_to(&format!("[{}]! 〆切", day(1)));
    ev.run_command("howm-action", &editor).unwrap();
    assert_eq!(editor.current_line(), format!("[{}]! 〆切", day(1)));
    // 何もないとき
    fs::remove_file(dir.path().join("r.txt")).unwrap();
    ev.run_command("howm-list-schedule", &editor).unwrap();
    assert_eq!(editor.message(), "No schedule");
    ev.run_command("howm-list-todo", &editor).unwrap();
    assert_eq!(editor.message(), "No todo");
}

#[test]
fn test_random() {
    let (ev, editor, dir) = setup();
    for i in 0..15 {
        write_memo(
            dir.path(),
            &format!("m{:02}.txt", i),
            &format!("= memo {}\n", i),
        );
    }
    ev.run_command("howm", &editor).unwrap();
    let menu = editor.text();
    let random = menu
        .split("ランダムセレクト")
        .nth(1)
        .unwrap()
        .lines()
        .filter(|l| l.contains(" | memo "))
        .count();
    assert_eq!(random, 10);
    // 最近は 15 個全て (20 まで)
    let recent = menu
        .split("最近のメモ")
        .nth(1)
        .unwrap()
        .split("-----")
        .next()
        .unwrap()
        .lines()
        .filter(|l| l.contains(" | memo "))
        .count();
    assert_eq!(recent, 15);
}
