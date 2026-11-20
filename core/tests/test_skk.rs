//! `skk-mode` (`lisp/skk.scm`) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 偽の画面 (何も頼まない).
#[derive(Default)]
struct Ui {
    /// `read-string` の答え (後ろから使う).
    answers: Mutex<Vec<String>>,
}

impl Host for Ui {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        if op == "read-string" {
            return Ok(Value::String(
                self.answers.lock().unwrap().pop().unwrap_or_default(),
            ));
        }
        Ok(Value::Nil)
    }
}

impl LoopUi for Ui {
    fn say(&self, _text: &str) {}

    fn stop_macro(&self) {}

    fn set_display(&self, _truncate_lines: bool, _selective_display: Option<usize>) {}
}

fn setup() -> CommandLoop<Ui> {
    let host = EditorHost::new(Editor::new(), Ui::default());
    CommandLoop::new(
        host,
        Evaluator::new(),
        Arc::new(Mutex::new(Keymap::default())),
    )
    .unwrap()
}

fn eval(commands: &CommandLoop<Ui>, code: &str) -> String {
    commands
        .evaluator()
        .eval_with(code, commands.host())
        .unwrap()
        .to_string()
}

/// 辞書を一時のディレクトリに作って, 変数に入れる. 戻り値は, 利用者の辞書のパス.
fn dictionaries(commands: &CommandLoop<Ui>, dir: &std::path::Path) -> std::path::PathBuf {
    let large = dir.join("large");
    let user = dir.join("user");
    std::fs::write(
        &large,
        ";; okuri-ari entries.\nかんじ /漢字/幹事;note/(concat \"x\")/\nあい /愛/藍/\n",
    )
    .unwrap();
    std::fs::write(&user, "ひと /人/\n").unwrap();
    eval(
        commands,
        &format!(
            "(set! skk-large-jisyo {:?}) (set! skk-jisyo {:?}) (set! skk--dictionary #f)",
            large.to_str().unwrap(),
            user.to_str().unwrap()
        ),
    );
    user
}

/// 文字を 1 つずつ打つ.
fn type_text(commands: &CommandLoop<Ui>, text: &str) {
    for c in text.chars() {
        commands.self_insert(&c.to_string());
    }
}

fn buffer(commands: &CommandLoop<Ui>) -> String {
    eval(commands, "(buffer-string)")
        .trim_matches('"')
        .to_string()
}

fn fresh(commands: &CommandLoop<Ui>) {
    eval(commands, "(erase-buffer) (skk--reset)");
}

#[test]
fn test_skk_romaji_to_kana() {
    let commands = setup();
    assert_eq!(eval(&commands, "(skk-mode 1)"), "#t");
    for (typed, kana) in [
        ("ka", "か"),
        ("kka", "っか"),
        ("nka", "んか"),
        ("nna", "んあ"),
        ("kya", "きゃ"),
        ("shi", "し"),
        ("tsu", "つ"),
        ("chi", "ち"),
        ("xtu", "っ"),
        ("ja", "じゃ"),
        ("fu", "ふ"),
        ("wo", "を"),
        ("-", "ー"),
        (",.", "、。"),
        ("[a]", "「あ」"),
        ("zh", "←"),
        ("n'a", "んあ"),
        ("kqa", "qあ"),
        ("k1", "k1"),
        ("ng", "んg"),
        ("1", "1"),
        ("a b", "あ b"),
    ] {
        fresh(&commands);
        type_text(&commands, typed);
        assert_eq!(buffer(&commands), kana, "{typed}");
    }
}

#[test]
fn test_skk_katakana_and_latin() {
    let commands = setup();
    eval(&commands, "(skk-mode 1)");
    fresh(&commands);
    type_text(&commands, "qkaq");
    type_text(&commands, "ka");
    assert_eq!(buffer(&commands), "カか");
    // latin
    type_text(&commands, "lka");
    assert_eq!(buffer(&commands), "カかka");
    eval(&commands, "(skk-kakutei)");
    type_text(&commands, "a");
    assert_eq!(buffer(&commands), "カかkaあ");
    // 切ると, 何もしない
    eval(&commands, "(skk-mode 0)");
    type_text(&commands, "ka");
    assert_eq!(buffer(&commands), "カかkaあka");
}

#[test]
fn test_skk_conversion() {
    let commands = setup();
    let dir = tempfile::tempdir().unwrap();
    let user = dictionaries(&commands, dir.path());
    eval(&commands, "(skk-mode 1)");
    fresh(&commands);
    type_text(&commands, "Kanji");
    assert_eq!(buffer(&commands), "▽かんじ");
    type_text(&commands, " ");
    assert_eq!(buffer(&commands), "▼漢字");
    type_text(&commands, " ");
    assert_eq!(buffer(&commands), "▼幹事");
    // 候補は, 巡る (注釈と, 式の候補は, 除く)
    type_text(&commands, " ");
    assert_eq!(buffer(&commands), "▼漢字");
    type_text(&commands, "x");
    assert_eq!(buffer(&commands), "▼幹事");
    // 別の文字で, 確定して, その文字を打つ
    type_text(&commands, "a");
    assert_eq!(buffer(&commands), "幹事あ");
    // 利用者の辞書も引く
    fresh(&commands);
    type_text(&commands, "Hito n");
    assert_eq!(buffer(&commands), "人n");
    fresh(&commands);
    type_text(&commands, "Ai ");
    eval(&commands, "(skk-kakutei)");
    assert_eq!(buffer(&commands), "愛");
    // ▽ のまま確定, q でカタカナ
    fresh(&commands);
    type_text(&commands, "Ka");
    eval(&commands, "(skk-kakutei)");
    assert_eq!(buffer(&commands), "か");
    fresh(&commands);
    type_text(&commands, "Kaq");
    assert_eq!(buffer(&commands), "カ");
    // 大文字は, 読みの途中でも, 小文字として使う
    fresh(&commands);
    type_text(&commands, "KaN");
    assert_eq!(buffer(&commands), "▽かn");
    // 読みの終わりの n は ん になる (辞書に無く, 答えも空なら, そのまま確定する)
    type_text(&commands, " ");
    assert_eq!(buffer(&commands), "かん");
    // 辞書に無い読みは, 聞いて, 登録する
    fresh(&commands);
    commands
        .host()
        .ui()
        .answers
        .lock()
        .unwrap()
        .push("無".to_string());
    type_text(&commands, "Nai ");
    assert_eq!(buffer(&commands), "無");
    assert!(std::fs::read_to_string(&user)
        .unwrap()
        .ends_with("ない /無/\n"));
    // 答えが空なら, 登録しない
    fresh(&commands);
    type_text(&commands, "Kore ");
    assert_eq!(buffer(&commands), "これ");
    // 本文が変わって, 印が消えたら, 状態を捨てる
    fresh(&commands);
    type_text(&commands, "Ka");
    eval(&commands, "(erase-buffer)");
    type_text(&commands, "ka");
    assert_eq!(buffer(&commands), "か");
}

#[test]
fn test_skk_return_and_kakutei_without_conversion() {
    let commands = setup();
    let dir = tempfile::tempdir().unwrap();
    dictionaries(&commands, dir.path());
    eval(&commands, "(skk-mode 1)");
    fresh(&commands);
    type_text(&commands, "Ai ");
    eval(&commands, "(skk-return)");
    assert_eq!(buffer(&commands), "愛\\n");
    eval(&commands, "(skk-return)");
    assert_eq!(buffer(&commands), "愛\\n\\n");
    eval(&commands, "(skk-kakutei)");
    assert_eq!(buffer(&commands), "愛\\n\\n\\n");
    // 切ると, ふつうの改行
    eval(&commands, "(skk-mode 0) (skk-kakutei)");
    assert_eq!(buffer(&commands), "愛\\n\\n\\n\\n");
    // 辞書が無くても, かなは打てる
    eval(&commands, "(skk-mode 1) (set! skk-large-jisyo \"/no/such\") (set! skk-jisyo \"/no/such/user\") (set! skk--dictionary #f)");
    fresh(&commands);
    type_text(&commands, "Ka");
    assert_eq!(eval(&commands, "(skk--candidates-of \"か\")"), "()");
}
