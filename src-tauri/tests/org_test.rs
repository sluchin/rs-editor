//! Org mode (`lisp/org.scm`) のテスト. 偽のエディタ (バッファを持つ) の上で, 見出しの開け閉め, 見出しとリストと表の編集,
//! TODO, 日付, タグ, リンク, アジェンダ, キャプチャを確かめる.

use std::fs;
use std::path::Path;

use rseditor::scheme::evaluator::Evaluator;
use tempfile::{tempdir, TempDir};

mod common;
use common::{FakeBuffer, FakeEditor};

/// org を読み込んだ評価器と, 偽のエディタと, Org のディレクトリを作る.
///
/// # Returns
///
/// 評価器, 偽のエディタ, Org のディレクトリ (消さないように持つ).
fn setup() -> (Evaluator, FakeEditor, TempDir) {
    let dir = tempdir().unwrap();
    let org = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lisp/org.scm");
    let ev = Evaluator::new();
    let editor = FakeEditor::new();
    ev.eval_with(
        &format!(
            "(load {:?}) (set! org-directory {:?}) (set! org-default-notes-file {:?})",
            org.to_string_lossy(),
            dir.path().to_string_lossy(),
            dir.path().join("notes.org").to_string_lossy()
        ),
        &editor,
    )
    .unwrap();
    (ev, editor, dir)
}

/// 評価する (失敗したら panic する).
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `code` - コード.
///
/// # Returns
///
/// 結果.
fn run(ev: &Evaluator, editor: &FakeEditor, code: &str) -> String {
    ev.eval_with(code, editor)
        .unwrap_or_else(|e| panic!("{}: {}", code, e))
}

/// 評価して, 失敗の内容を返す.
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `code` - コード.
///
/// # Returns
///
/// 失敗の内容 (成功したら panic する).
fn fail(ev: &Evaluator, editor: &FakeEditor, code: &str) -> String {
    ev.eval_with(code, editor)
        .expect_err(&format!("{} did not fail", code))
}

/// 出しているバッファの本文を変え, カーソルを `|` の所に置く (`|` は消す).
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `text` - 本文 (`|` を 1 つ含む).
fn set_text(editor: &FakeEditor, text: &str) {
    set_text_mark(editor, text, '|');
}

/// 出しているバッファの本文を変え, カーソルを印の所に置く (印は消す).
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `text` - 本文 (印を 1 つ含む).
/// * `mark` - カーソルの印.
fn set_text_mark(editor: &FakeEditor, text: &str, mark: char) {
    let at = text.find(mark).expect("no cursor");
    let point = text[..at].chars().count() + 1;
    let body: Vec<char> = text.replacen(mark, "", 1).chars().collect();
    editor.with_buffer(|b| {
        b.text = body;
        b.point = point;
        b.folded.clear();
    });
}

/// 出しているバッファの本文に, カーソルの所 (`|`) を入れて返す.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
///
/// # Returns
///
/// 本文.
fn shown(editor: &FakeEditor) -> String {
    shown_mark(editor, '|')
}

/// 出しているバッファの本文に, カーソルの所の印を入れて返す.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `mark` - カーソルの印.
///
/// # Returns
///
/// 本文.
fn shown_mark(editor: &FakeEditor, mark: char) -> String {
    let buffer = editor.buffer();
    let mut text = buffer.text.clone();
    text.insert(buffer.point - 1, mark);
    text.into_iter().collect()
}

/// 本文を置き, コマンドを動かし, カーソルの入った本文を返す.
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `before` - 本文 (`|` がカーソル).
/// * `code` - コード.
///
/// # Returns
///
/// 動かした後の本文 (`|` がカーソル).
fn edit(ev: &Evaluator, editor: &FakeEditor, before: &str, code: &str) -> String {
    set_text(editor, before);
    run(ev, editor, code);
    shown(editor)
}

/// read-string と completing-read の答えを足す.
///
/// # Arguments
///
/// * `editor` - 偽のエディタ.
/// * `answers` - 答え (前から使う).
fn answer(editor: &FakeEditor, answers: &[&str]) {
    editor
        .answers
        .lock()
        .unwrap()
        .extend(answers.iter().map(|a| a.to_string()));
}

/// 今日から n 日後の日付.
///
/// # Arguments
///
/// * `days` - 日数.
/// * `format` - 書式 (`%Y-%m-%d` など).
///
/// # Returns
///
/// 日付.
fn day(days: i64, format: &str) -> String {
    (chrono::Local::now() + chrono::Duration::days(days))
        .format(format)
        .to_string()
}

#[test]
fn test_mode_faces_and_find_file_hook() {
    let (ev, editor, dir) = setup();
    let file = dir.path().join("a.org");
    fs::write(&file, "#+STARTUP: overview\n* a\nbody\n").unwrap();
    run(
        &ev,
        &editor,
        &format!(
            "(find-file {:?}) (run-hooks 'find-file-hook)",
            file.to_string_lossy()
        ),
    );
    assert_eq!(editor.key("Tab").as_deref(), Some("org-cycle"));
    assert_eq!(editor.key("Ctrl-c Ctrl-t").as_deref(), Some("org-todo"));
    let buffer = editor.buffer();
    assert_eq!(
        buffer.faces.get("^(?:\\*{8})*\\*{1}(?: .*)?$"),
        Some(&(Some("org-level-1".to_string()), Some(0)))
    );
    assert_eq!(
        buffer.faces.get("^\\*+ +(TODO)(?= |$)"),
        Some(&(Some("org-todo".to_string()), Some(1)))
    );
    assert!(!buffer
        .faces
        .values()
        .any(|f| f.0.as_deref() == Some("org-hide")));
    // #+STARTUP: overview なら, 閉じて開く
    assert_eq!(buffer.folded, vec![(24, 29)]);
    // .org でないファイルは, そのまま
    let txt = dir.path().join("b.txt");
    run(
        &ev,
        &editor,
        &format!(
            "(find-file {:?}) (run-hooks 'find-file-hook)",
            txt.to_string_lossy()
        ),
    );
    assert_eq!(editor.key("Tab"), None);
    // 前の見出しを隠す, org-mode-hook, #+STARTUP: content
    set_text(&editor, "#+STARTUP: content\n|* a\nx\n** b\ny\n");
    run(
        &ev,
        &editor,
        "(set! org-hide-leading-stars #t) (define hooked #f) (add-hook 'org-mode-hook (lambda () (set! hooked #t))) (org-mode) hooked",
    );
    let buffer = editor.buffer();
    assert_eq!(
        buffer.faces.get("^(\\*+)\\*(?: |$)"),
        Some(&(Some("org-hide".to_string()), Some(1)))
    );
    assert_eq!(buffer.folded, vec![(23, 25), (30, 32)]);
    assert_eq!(run(&ev, &editor, "hooked"), "#t");
    // キーワードが無ければ, TODO の色を付けない
    run(
        &ev,
        &editor,
        "(set! org-todo-keywords '()) (org-fontify) (set! org-todo-keywords '((sequence \"TODO\" \"DONE\")))",
    );
    assert!(!editor
        .buffer()
        .faces
        .values()
        .any(|f| f.0.as_deref() == Some("org-todo")));
}

#[test]
fn test_cycle() {
    let (ev, editor, _dir) = setup();
    set_text(&editor, "|* a\nbody\n** b\nbtext\n** c\n* d\n");
    let folded = |editor: &FakeEditor| editor.buffer().folded;
    run(&ev, &editor, "(org-cycle)");
    assert_eq!(editor.message(), "FOLDED");
    assert_eq!(folded(&editor), vec![(4, 25)]);
    run(&ev, &editor, "(org-cycle)");
    assert_eq!(editor.message(), "CHILDREN");
    assert_eq!(folded(&editor), vec![(14, 20)]);
    run(&ev, &editor, "(org-cycle)");
    assert_eq!(editor.message(), "SUBTREE");
    assert!(folded(&editor).is_empty());
    run(&ev, &editor, "(org-cycle)");
    assert_eq!(editor.message(), "FOLDED");
    // カーソルを動かすと, 続けて押したことにならない (CHILDREN の後でも, 閉じる)
    run(&ev, &editor, "(org-cycle) (goto-char 2) (org-cycle)");
    assert_eq!(editor.message(), "FOLDED");
    // 子の無い見出し, 空の見出し
    run(
        &ev,
        &editor,
        "(goto-char 1) (org-cycle) (goto-char 10) (org-cycle)",
    );
    assert_eq!(editor.message(), "SUBTREE (NO CHILDREN)");
    run(&ev, &editor, "(org-cycle)");
    assert_eq!(editor.message(), "FOLDED");
    run(&ev, &editor, "(goto-char 21) (org-cycle)");
    assert_eq!(editor.message(), "EMPTY ENTRY");
    // 全体: OVERVIEW, CONTENTS, SHOW ALL
    run(&ev, &editor, "(goto-char 1) (org-shifttab)");
    assert_eq!(editor.message(), "OVERVIEW");
    assert_eq!(folded(&editor), vec![(4, 25)]);
    run(&ev, &editor, "(org-shifttab)");
    assert_eq!(editor.message(), "CONTENTS...done");
    assert_eq!(folded(&editor), vec![(4, 9), (14, 20)]);
    run(&ev, &editor, "(org-shifttab)");
    assert_eq!(editor.message(), "SHOW ALL");
    assert!(folded(&editor).is_empty());
    // 見出しの無いバッファ
    set_text(&editor, "|text\n");
    run(&ev, &editor, "(org-global-cycle)");
    assert_eq!(editor.message(), "OVERVIEW");
    // 見出しでも表でもなければ, 字下げ
    run(&ev, &editor, "(org-cycle)");
    assert_eq!(
        editor.commands.lock().unwrap().last().unwrap(),
        "indent-for-tab-command"
    );
    // 隠れた所を見せる
    set_text(&editor, "|* a\nb\n");
    run(&ev, &editor, "(org-cycle) (org-reveal 5)");
    assert!(folded(&editor).is_empty());
}

#[test]
fn test_insert_heading() {
    let (ev, editor, _dir) = setup();
    let cases = [
        // 見出しの終わり
        ("* a|\nx\n", "(org-meta-return)", "* a\n* |\nx\n"),
        // 行の始め: 前に入れる
        ("|** a\n", "(org-insert-heading)", "** |\n** a\n"),
        // 題の途中: 分ける (タグは残す)
        (
            "* TODO ab|cd :t:\n",
            "(org-meta-return)",
            "* TODO ab                                                                 :t:\n* cd|\n",
        ),
        ("* ab| \n", "(org-meta-return)", "* ab\n* |\n"),
        // 文の行: 始めなら見出しにし, 途中なら分ける
        ("* a\n|text\n", "(org-meta-return)", "* a\n* |text\n"),
        ("* a\nte|xt\n", "(org-meta-return)", "* a\nte\n* |xt\n"),
        ("|text\n", "(org-meta-return)", "* |text\n"),
        // TODO の見出し (前が済みなら, 始めの状態)
        ("* WAIT a|\n", "(set! org-todo-keywords '((sequence \"TODO\" \"WAIT\" \"DONE\"))) (org-insert-todo-heading)", "* WAIT a\n* WAIT |\n"),
        ("* DONE a|\n", "(org-insert-todo-heading)", "* DONE a\n* TODO |\n"),
        // 木の後
        (
            "* a|\n** b\n* c\n",
            "(org-insert-heading-respect-content)",
            "* a\n** b\n* |\n* c\n",
        ),
        (
            "* a|\n",
            "(org-insert-todo-heading-respect-content)",
            "* a\n* TODO |\n",
        ),
        ("te|xt\n* a\n", "(org-insert-heading-respect-content)", "text\n* |\n* a\n"),
        ("te|xt", "(org-insert-heading-respect-content)", "text\n* |"),
        ("|", "(org-insert-heading-respect-content)", "* |"),
    ];
    for (before, code, after) in cases {
        assert_eq!(
            edit(&ev, &editor, before, code),
            after,
            "{} {}",
            before,
            code
        );
    }
}

#[test]
fn test_move_headings() {
    let (ev, editor, _dir) = setup();
    let cases = [
        ("* a|\n** b\n", "(org-metaright)", "** a|\n** b\n"),
        (
            "** a :t:|\n",
            "(org-metaleft)",
            "* a                                                                       :t:|\n",
        ),
        ("* a|\n** b\n", "(org-shiftmetaright)", "** a|\n*** b\n"),
        ("** a|\n*** b\n", "(org-shiftmetaleft)", "* a|\n** b\n"),
        ("* a\n|* b\nx\n", "(org-metaup)", "|* b\nx\n* a\n"),
        ("* a\n|* b\n", "(org-shiftmetaup)", "|* b\n* a\n"),
        ("* a|\nx\n* b\n", "(org-metadown)", "* b\n* a|\nx\n"),
        ("* a|\n* b\n", "(org-shiftmetadown)", "* b\n* a|\n"),
        // 文の行は, 入れ換える. 左右は, 言葉ごとに動く
        ("a\n|b\n", "(org-metaup)", "|b\na\n"),
        ("a|\nb\n", "(org-metadown)", "b\na|\n"),
        ("a\n|b\n", "(org-shiftmetaup)", "|b\na\n"),
        ("a|\nb\n", "(org-shiftmetadown)", "b\na|\n"),
        ("a| b", "(org-metaright) (org-metaleft)", "a| b"),
        // C-c *
        ("* a|\n", "(org-ctrl-c-star)", "a|\n"),
        ("* a\nb|\n", "(org-ctrl-c-star)", "* a\n** b|\n"),
        ("- [ ] a|\n", "(org-ctrl-c-star)", "* TODO a|\n"),
        ("- [X] a|\n", "(org-ctrl-c-star)", "* DONE a|\n"),
        ("- a|\n", "(org-ctrl-c-star)", "* a|\n"),
    ];
    for (before, code, after) in cases {
        assert_eq!(
            edit(&ev, &editor, before, code),
            after,
            "{} {}",
            before,
            code
        );
    }
    // 閉じた木は, 動かしても閉じたまま
    set_text(&editor, "* a\n|* b\nx\n");
    run(&ev, &editor, "(org-cycle) (org-metaup)");
    assert!(editor.buffer().folded.contains(&(4, 6)));
    for (before, code, error) in [
        (
            "* a|\n",
            "(org-metaleft)",
            "Cannot promote to level 0.  UNDO to recover if necessary",
        ),
        (
            "* a|\n",
            "(org-metaup)",
            "Cannot move past superior level or buffer limit",
        ),
        (
            "* a\n** b|\n* c\n",
            "(org-metadown)",
            "Cannot move past superior level or buffer limit",
        ),
        ("a|\n", "(org-metaup)", "Cannot move further up"),
        ("a|", "(org-metadown)", "Cannot move further down"),
        (
            "a|",
            "(org-shiftmetaleft)",
            "This command is active in special context like tables, headlines or items",
        ),
        (
            "a|",
            "(org-shiftmetaright)",
            "This command is active in special context like tables, headlines or items",
        ),
        (
            "a|\n",
            "(org-metaleft) (org-do-promote)",
            "Before first headline at position 2 in buffer *scratch*",
        ),
    ] {
        set_text(&editor, before);
        assert_eq!(fail(&ev, &editor, code), error, "{} {}", before, code);
    }
}

#[test]
fn test_navigation() {
    let (ev, editor, _dir) = setup();
    set_text(&editor, "|* a\nx\n** b\n** c\n* d\n");
    run(&ev, &editor, "(org-next-visible-heading)");
    assert_eq!(editor.current_line(), "** b");
    run(&ev, &editor, "(org-forward-heading-same-level)");
    assert_eq!(editor.current_line(), "** c");
    run(&ev, &editor, "(org-forward-heading-same-level)");
    assert_eq!(editor.current_line(), "** c");
    run(&ev, &editor, "(org-backward-heading-same-level)");
    assert_eq!(editor.current_line(), "** b");
    run(&ev, &editor, "(outline-up-heading)");
    assert_eq!(editor.current_line(), "* a");
    run(&ev, &editor, "(org-forward-heading-same-level)");
    assert_eq!(editor.current_line(), "* d");
    run(&ev, &editor, "(org-previous-visible-heading)");
    assert_eq!(editor.current_line(), "** c");
    // 本文からは, 見出しへ戻る
    run(
        &ev,
        &editor,
        "(goto-char 5) (org-backward-heading-same-level)",
    );
    assert_eq!(editor.current_line(), "* a");
    // 隠れた見出しは飛ばす. 無ければ, 始めと終わり
    run(&ev, &editor, "(org-cycle) (org-next-visible-heading)");
    assert_eq!(editor.current_line(), "* d");
    run(&ev, &editor, "(org-next-visible-heading)");
    assert_eq!(editor.buffer().point, 21);
    run(&ev, &editor, "(goto-char 1) (org-previous-visible-heading)");
    assert_eq!(editor.buffer().point, 1);
    assert_eq!(
        fail(&ev, &editor, "(outline-up-heading)"),
        "Already at top level of the outline"
    );
}

#[test]
fn test_lists() {
    let (ev, editor, _dir) = setup();
    let cases = [
        // 項目を足す (前, 分ける, 数を振り直す)
        ("- a|\n", "(org-meta-return)", "- a\n- |\n"),
        ("- |a\n", "(org-meta-return)", "- |\n- a\n"),
        ("- a| b\n", "(org-meta-return)", "- a\n- |b\n"),
        ("1. a|\n2. b\n", "(org-meta-return)", "1. a\n2. |\n3. b\n"),
        (
            "- [ ] a|\n",
            "(org-insert-todo-heading)",
            "- [ ] a\n- [ ] |\n",
        ),
        (
            "- a\n  continued|\n",
            "(org-meta-return)",
            "- a\n  continued\n- |\n",
        ),
        // 字下げ (M-左右は項目だけ, M-S-左右は子も)
        (
            "- a\n- b|\n  - c\n",
            "(org-metaright)",
            "- a\n  - b|\n  - c\n",
        ),
        ("- a\n  - b|\n", "(org-metaleft)", "- a\n- b|\n"),
        (
            "- a\n- b|\n  - c\n",
            "(org-shiftmetaright)",
            "- a\n  - b|\n    - c\n",
        ),
        (
            "- a\n  - b|\n    - c\n",
            "(org-shiftmetaleft)",
            "- a\n- b|\n  - c\n",
        ),
        ("- a|\n- b\n", "(org-shiftmetaright)", " - a|\n - b\n"),
        (" - a|\n", "(org-shiftmetaleft)", "- a|\n"),
        ("1. a\n2. b|\n", "(org-metaright)", "1. a\n   1. b|\n"),
        // 動かす
        ("- a\n- b|\n  - c\n", "(org-metaup)", "- b|\n  - c\n- a\n"),
        ("1. a|\n2. b\n", "(org-metadown)", "1. b\n2. a|\n"),
        ("- a\n- |b\n", "(org-shiftmetaup)", "- |b\n- a\n"),
        ("- |a\n- b\n", "(org-shiftmetadown)", "- b\n- |a\n"),
        // 印を変える
        ("|- a\n- b\n", "(org-shiftright)", "|+ a\n+ b\n"),
        ("|- a\n", "(org-shiftleft)", "|1) a\n"),
        ("  |- a\n", "(org-shiftright) (org-shiftright)", "  |* a\n"),
        ("- a|\n", "(org-ctrl-c-minus)", "+ a|\n"),
        // 項目にする
        ("* a|\n", "(org-ctrl-c-minus)", "- a|\n"),
        ("  a|\n", "(org-ctrl-c-minus)", "  - a|\n"),
        // 次と前の項目
        ("- a|\n- b\n", "(org-shiftdown)", "- a\n|- b\n"),
        ("- a\n- b|\n", "(org-shiftup)", "|- a\n- b\n"),
    ];
    for (before, code, after) in cases {
        assert_eq!(
            edit(&ev, &editor, before, code),
            after,
            "{} {}",
            before,
            code
        );
    }
    for (before, code, error) in [
        (
            "- a|\n",
            "(org-metaright)",
            "At first item: use S-M-<left/right> to move the whole list",
        ),
        (
            "- a\n  - b|\n  - c\n",
            "(org-metaright)",
            "Cannot indent the first item of a list",
        ),
        (
            "- a|\n",
            "(org-metaleft)",
            "At first item: use S-M-<left/right> to move the whole list",
        ),
        (
            "- a\n- b|\n",
            "(org-metaleft)",
            "Cannot outdent top-level items",
        ),
        (
            "- a\n  - b|\n    - c\n",
            "(org-metaleft)",
            "Cannot outdent an item without its children",
        ),
        (
            "- a|\n",
            "(org-shiftmetaleft)",
            "Cannot outdent beyond margin",
        ),
        ("- a|\n", "(org-metaup)", "Cannot move this item further up"),
        (
            "- a|\n",
            "(org-metadown)",
            "Cannot move this item further down",
        ),
        ("- a|\n", "(org-shiftup)", "On first item"),
        ("- a|\n", "(org-shiftdown)", "On last item"),
        (
            "a|",
            "(org-toggle-checkbox)",
            "Cannot toggle checkbox outside of a list",
        ),
        ("a|", "(org-move-item #t)", "Not at an item"),
        ("a|", "(org-indent-item 1 #f)", "Not at an item"),
        ("a|", "(org-cycle-list-bullet #t)", "Not at an item"),
        ("a|", "(org-item-move-point #t)", "Not in an item"),
        (
            "a|",
            "(org-shiftup)",
            "To use shift-selection with Org mode, customize `org-support-shift-select'",
        ),
        (
            "a|",
            "(org-shiftright)",
            "To use shift-selection with Org mode, customize `org-support-shift-select'",
        ),
    ] {
        set_text(&editor, before);
        assert_eq!(fail(&ev, &editor, code), error, "{} {}", before, code);
    }
}

#[test]
fn test_checkboxes_and_statistics() {
    let (ev, editor, _dir) = setup();
    let cases = [
        // チェックボックスと, 見出しの [/] [%]
        (
            "* a [/] [%]\n- [ ] x|\n- [ ] y\n",
            "(org-ctrl-c-ctrl-c)",
            "* a [1/2] [50%]\n- [X] x|\n- [ ] y\n",
        ),
        ("- [X] x|\n", "(org-toggle-checkbox)", "- [ ] x|\n"),
        // 子が全て済みなら, 親も済み. 一部なら [-]
        (
            "- [ ] p [/]\n  - [ ] a|\n  - [X] b\n",
            "(org-toggle-checkbox)",
            "- [X] p [2/2]\n  - [X] a|\n  - [X] b\n",
        ),
        (
            "- [ ] p\n  - [ ] a|\n  - [ ] b\n",
            "(org-toggle-checkbox)",
            "- [-] p\n  - [X] a|\n  - [ ] b\n",
        ),
        (
            "- [X] p\n  - [X] a|\n  - [ ] b\n",
            "(org-toggle-checkbox)",
            "- [ ] p\n  - [ ] a|\n  - [ ] b\n",
        ),
        // 子の見出しの TODO を数える
        (
            "* a [/]|\n** TODO b\n** DONE c\n** d\n",
            "(org-update-statistics-cookies)",
            "* a [1/2]|\n** TODO b\n** DONE c\n** d\n",
        ),
        (
            "* a [%]|\n",
            "(org-update-statistics-cookies)",
            "* a [0%]|\n",
        ),
        // チェックボックスの無い項目: 印を直す
        ("1. a\n1. b|\n", "(org-ctrl-c-ctrl-c)", "1. a\n2. b|\n"),
        // 数える印の行
        ("text [/]|\n", "(org-ctrl-c-ctrl-c)", "text [/]|\n"),
    ];
    for (before, code, after) in cases {
        assert_eq!(
            edit(&ev, &editor, before, code),
            after,
            "{} {}",
            before,
            code
        );
    }
    set_text(&editor, "- x|\n");
    run(&ev, &editor, "(org-toggle-checkbox)");
    assert_eq!(editor.message(), "Item has no checkbox");
    set_text(&editor, "x|\n");
    assert_eq!(
        fail(&ev, &editor, "(org-ctrl-c-ctrl-c)"),
        "C-c C-c can do nothing useful here"
    );
    set_text(&editor, "#+TITLE: x|\n");
    run(&ev, &editor, "(org-ctrl-c-ctrl-c)");
    assert_eq!(editor.message(), "Local setup has been refreshed");
}

#[test]
fn test_todo() {
    let (ev, editor, _dir) = setup();
    let cases = [
        // C-c C-t は, 同じ組の中で回す. 本文からも, 見出しを変える
        ("* a|\n", "(org-todo)", "* TODO a|\n"),
        ("* TODO a|\n", "(org-todo)", "* DONE a|\n"),
        ("* DONE a|\n", "(org-todo)", "* a|\n"),
        ("* a\nx|\n", "(org-todo)", "* TODO a\nx|\n"),
        // S-左右は, 全ての状態を回す
        ("* a|\n", "(org-shiftright)", "* TODO a|\n"),
        ("* a|\n", "(org-shiftleft)", "* DONE a|\n"),
        ("* TODO a|\n", "(org-shiftleft)", "* a|\n"),
        // 優先度 (無ければ既定, 範囲を越えれば消す)
        ("* TODO a|\n", "(org-shiftup)", "* TODO [#B] a|\n"),
        ("* [#B] a|\n", "(org-shiftup)", "* [#A] a|\n"),
        ("* [#A] a|\n", "(org-shiftup)", "* a|\n"),
        ("* [#C] a|\n", "(org-shiftdown)", "* a|\n"),
        ("* a|\n", "(org-shiftdown)", "* [#B] a|\n"),
        // 状態の変わった見出しの, 上の [/] を書き直す
        (
            "* p [/]\n** TODO a|\n",
            "(org-todo)",
            "* p [1/1]\n** DONE a|\n",
        ),
    ];
    for (before, code, after) in cases {
        assert_eq!(
            edit(&ev, &editor, before, code),
            after,
            "{} {}",
            before,
            code
        );
    }
    // 済みにすると CLOSED を書き, 戻すと消す
    let today = day(0, "%Y-%m-%d %a");
    let after = edit(
        &ev,
        &editor,
        "* TODO a|\nbody\n",
        "(set! org-log-done 'time) (org-todo)",
    );
    assert!(
        after.starts_with(&format!("* DONE a|\nCLOSED: [{} ", today)),
        "{}",
        after
    );
    assert_eq!(edit(&ev, &editor, &after, "(org-todo)"), "* a|\nbody\n");
    // 予定の行があれば, その始めに CLOSED を書く
    let after = edit(
        &ev,
        &editor,
        "* TODO a|\nSCHEDULED: <2026-10-01 Thu>\n",
        "(org-todo)",
    );
    assert!(
        after.starts_with(&format!("* DONE a|\nCLOSED: [{} ", today)),
        "{}",
        after
    );
    assert!(
        after.ends_with("] SCHEDULED: <2026-10-01 Thu>\n"),
        "{}",
        after
    );
    run(&ev, &editor, "(set! org-log-done #f)");
    // 繰り返す予定: 日付を進め, 始めの状態に戻し, 記録を書く
    let after = edit(
        &ev,
        &editor,
        "* TODO a|\nSCHEDULED: <2026-10-01 Thu +1w> DEADLINE: <2026-10-02 Fri .+1d>\nbody\n",
        "(org-todo)",
    );
    let lines: Vec<&str> = after.lines().collect();
    assert_eq!(lines[0], "* TODO a|");
    assert_eq!(
        lines[1],
        format!(
            "DEADLINE: <{} .+1d> SCHEDULED: <2026-10-08 Thu +1w>",
            day(1, "%Y-%m-%d %a")
        )
    );
    assert_eq!(lines[2], ":PROPERTIES:");
    assert!(lines[3].starts_with(&format!(":LAST_REPEAT: [{} ", today)));
    assert_eq!(lines[4], ":END:");
    assert!(lines[5].starts_with(&format!(
        "- State \"DONE\"       from \"TODO\"       [{} ",
        today
    )));
    assert_eq!(lines[6], "body");
    assert!(
        editor.message().starts_with("Entry repeats: DEADLINE: <"),
        "{}",
        editor.message()
    );
    // ++ は今日より後まで進める. 記録しなければ, LAST_REPEAT は変えない
    let after = edit(
        &ev,
        &editor,
        "* TODO a|\nSCHEDULED: <2020-01-01 Wed ++1y>\n:PROPERTIES:\n:LAST_REPEAT: [2020-01-01 Wed 10:00]\n:END:\n",
        "(set! org-log-repeat #f) (org-todo)",
    );
    let year = day(0, "%Y").parse::<i32>().unwrap();
    assert!(
        after.contains(&format!("SCHEDULED: <{}-01-01", year + 1)),
        "{}",
        after
    );
    assert!(after.contains(":LAST_REPEAT: [2020-01-01 Wed 10:00]"));
    run(&ev, &editor, "(set! org-log-repeat 'time)");
    // PROPERTIES があれば, LAST_REPEAT を書き換え, 記録は :END: の後
    let after = edit(
        &ev,
        &editor,
        "* TODO a|\nSCHEDULED: <2020-01-01 Wed +1d>\n:PROPERTIES:\n:X: 1\n:LAST_REPEAT: [2020-01-01 Wed 10:00]\n:END:\n",
        "(org-todo)",
    );
    assert!(
        after.contains(&format!(":X: 1\n:LAST_REPEAT: [{} ", today)),
        "{}",
        after
    );
    assert!(after.contains(":END:\n- State \"DONE\""), "{}", after);
    let after = edit(
        &ev,
        &editor,
        "* TODO a|\nSCHEDULED: <2020-01-01 Wed +1d>\n:PROPERTIES:\n:X: 1\n:END:\n",
        "(org-todo)",
    );
    assert!(
        after.contains(&format!(":X: 1\n:LAST_REPEAT: [{} ", today)),
        "{}",
        after
    );
    // 状態を, キーで選ぶ (SPC で消す)
    run(
        &ev,
        &editor,
        "(set! org-todo-keywords '((sequence \"TODO(t)\" \"WAIT(w@/!)\" \"|\" \"DONE(d)\" \"CANCELED(c)\")))",
    );
    answer(&editor, &["w", "done", " "]);
    assert_eq!(edit(&ev, &editor, "* a|\n", "(org-todo)"), "* WAIT a|\n");
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "[t] TODO  [w] WAIT  [d] DONE  [c] CANCELED  [SPC] clear: "
    );
    assert_eq!(
        edit(&ev, &editor, "* WAIT a|\n", "(org-todo)"),
        "* DONE a|\n"
    );
    assert_eq!(edit(&ev, &editor, "* DONE a|\n", "(org-todo)"), "* a|\n");
    answer(&editor, &["x"]);
    set_text(&editor, "* a|\n");
    assert_eq!(fail(&ev, &editor, "(org-todo)"), "Quit");
    // 組が 2 つ (C-c C-t は, 組の中で回す). 古い書き方の, 言葉だけのリスト
    run(
        &ev,
        &editor,
        "(set! org-todo-keywords '((sequence \"TODO\" \"DONE\") (type \"BUG\" \"FIXED\")))",
    );
    assert_eq!(
        edit(&ev, &editor, "* BUG a|\n", "(org-todo)"),
        "* FIXED a|\n"
    );
    assert_eq!(
        edit(&ev, &editor, "* FIXED a|\n", "(org-shiftright)"),
        "* a|\n"
    );
    assert_eq!(
        edit(&ev, &editor, "* DONE a|\n", "(org-shiftright)"),
        "* BUG a|\n"
    );
    run(
        &ev,
        &editor,
        "(set! org-todo-keywords '(\"OPEN\" \"CLOSED\"))",
    );
    assert_eq!(edit(&ev, &editor, "* a|\n", "(org-todo)"), "* OPEN a|\n");
    assert_eq!(run(&ev, &editor, "(org-todo-keys)"), "()");
}

#[test]
fn test_priority_and_tags() {
    let (ev, editor, _dir) = setup();
    answer(&editor, &["a", " ", "z"]);
    assert_eq!(
        edit(&ev, &editor, "* TODO x|\n", "(org-priority)"),
        "* TODO [#A] x|\n"
    );
    assert_eq!(editor.message(), "Priority of current item set to A");
    assert_eq!(
        edit(&ev, &editor, "* [#A] x|\n", "(org-priority)"),
        "* x|\n"
    );
    assert_eq!(editor.message(), "Priority removed");
    set_text(&editor, "* x|\n");
    assert_eq!(
        fail(&ev, &editor, "(org-priority)"),
        "Priority must be between `A' and `C'"
    );
    // タグ (見出しの右に揃える. 空なら消す)
    answer(&editor, &["a b:c", "", "x"]);
    assert_eq!(
        edit(&ev, &editor, "* TODO x|\n", "(org-set-tags-command)"),
        format!("* TODO x{}:a:b:c:|\n", " ".repeat(77 - 7 - 8))
    );
    assert_eq!(editor.prompts.lock().unwrap().last().unwrap(), "Tags: ");
    assert_eq!(
        edit(&ev, &editor, "* x :a:|\n* y :b:\n", "(org-ctrl-c-ctrl-c)"),
        "* x|\n* y :b:\n"
    );
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "Tags (a b): "
    );
    // 長い題なら, 空白 1 つ. 正の桁なら, その桁から
    let long = "a".repeat(80);
    assert_eq!(
        edit(
            &ev,
            &editor,
            &format!("* {}|\n", long),
            "(org-set-tags-command)"
        ),
        format!("* {} :x:|\n", long)
    );
    answer(&editor, &["x"]);
    assert_eq!(
        edit(
            &ev,
            &editor,
            "* a|\n",
            "(set! org-tags-column 10) (org-set-tags-command)"
        ),
        "* a       :x:|\n"
    );
}

#[test]
fn test_dates() {
    let (ev, editor, _dir) = setup();
    let today = day(0, "%Y-%m-%d");
    let read = |input: &str, default: &str| {
        run(
            &ev,
            &editor,
            &format!("(org-read-date-analyze {:?} {:?})", input, default),
        )
    };
    assert_eq!(read("", "2026-10-20"), "(\"2026-10-20\" #f)");
    assert_eq!(read("+3", "x"), format!("({:?} #f)", day(3, "%Y-%m-%d")));
    assert_eq!(read("-1d", "x"), format!("({:?} #f)", day(-1, "%Y-%m-%d")));
    assert_eq!(read("+2w", "x"), format!("({:?} #f)", day(14, "%Y-%m-%d")));
    assert_eq!(read("++1m", "2026-01-31"), "(\"2026-03-03\" #f)");
    assert_eq!(read("--1y", "2026-01-31"), "(\"2025-01-31\" #f)");
    assert_eq!(read("+", "x"), format!("({:?} #f)", day(1, "%Y-%m-%d")));
    assert_eq!(read("2026-1-5 9:05", "x"), "(\"2026-01-05\" \"09:05\")");
    assert_eq!(
        read("2026-10-30 10:00-11:30", "x"),
        "(\"2026-10-30\" \"10:00-11:30\")"
    );
    assert_eq!(read("3pm", "2026-10-30"), "(\"2026-10-30\" \"15:00\")");
    assert_eq!(read("12am", "2026-10-30"), "(\"2026-10-30\" \"00:00\")");
    assert_eq!(read("10/30/27", "x"), "(\"2027-10-30\" #f)");
    assert_eq!(read("10/30/2028", "x"), "(\"2028-10-30\" #f)");
    // 年を省くと, 過ぎた日なら来年. 日だけなら, 過ぎた日なら来月
    let tomorrow = day(1, "%m-%d");
    let year = day(1, "%Y");
    assert_eq!(
        read(&tomorrow, "x"),
        format!("(\"{}-{}\" #f)", year, tomorrow)
    );
    let (m, d) = tomorrow.split_once('-').unwrap();
    assert_eq!(
        read(&format!("{}/{}", m, d), "x"),
        format!("(\"{}-{}\" #f)", year, tomorrow)
    );
    let yesterday = day(-1, "%m-%d");
    for input in [
        yesterday.clone(),
        yesterday.replace('-', "/"),
        day(-1, "%-d"),
    ] {
        let result = read(&input, "x");
        assert!(result > format!("(\"{}", today), "{} {}", input, result);
    }
    // 曜日 (今日なら今日)
    let weekday = day(2, "%a").to_lowercase();
    assert_eq!(
        read(&weekday, "x"),
        format!("({:?} #f)", day(2, "%Y-%m-%d"))
    );
    assert_eq!(read(&day(0, "%A"), "x"), format!("({:?} #f)", today));
    assert_eq!(
        fail(&ev, &editor, "(org-read-date-analyze \"what\" \"x\")"),
        "Cannot read date: what"
    );
    // 日付を入れる. 日付の上なら変える
    answer(&editor, &["2026-10-30", "+1", "2026-10-31 9:30"]);
    assert_eq!(
        edit(&ev, &editor, "x |", "(org-time-stamp)"),
        "x <2026-10-30 Fri>|"
    );
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        &format!("Date+time [{}]: ", today)
    );
    assert_eq!(
        edit(&ev, &editor, "x <2026-10-30 |Fri>", "(org-time-stamp)"),
        format!("x <{} {}>|", day(1, "%Y-%m-%d"), day(1, "%a"))
    );
    assert_eq!(
        edit(&ev, &editor, "x |", "(org-time-stamp-inactive)"),
        "x [2026-10-31 Sat 09:30]|"
    );
    // SCHEDULED と DEADLINE (新しいものは行の始め. 繰り返しは残す)
    answer(&editor, &["2026-10-30", "2026-11-02", "2026-11-05"]);
    assert_eq!(
        edit(&ev, &editor, "* a|\nbody\n", "(org-schedule)"),
        "* a|\nSCHEDULED: <2026-10-30 Fri>\nbody\n"
    );
    assert_eq!(editor.message(), "Scheduled to <2026-10-30 Fri>");
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        &format!("SCHEDULED Date+time [{}]: ", today)
    );
    assert_eq!(
        edit(
            &ev,
            &editor,
            "* a\nSCHEDULED: <2026-10-30 Fri>\nbo|dy\n",
            "(org-deadline)"
        ),
        "* a\nDEADLINE: <2026-11-02 Mon> SCHEDULED: <2026-10-30 Fri>\nbo|dy\n"
    );
    assert_eq!(editor.message(), "Deadline on <2026-11-02 Mon>");
    assert_eq!(
        edit(
            &ev,
            &editor,
            "* a|\nSCHEDULED: <2026-10-30 Fri +1w>",
            "(org-schedule)"
        ),
        "* a|\nSCHEDULED: <2026-11-05 Thu +1w>"
    );
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "SCHEDULED Date+time [2026-10-30]: "
    );
    // 予定を消す (CLOSED だけの行は, 行ごと消す)
    for (before, after) in [
        ("* a|\nCLOSED: [2026-10-30 Fri]\n", "* a|\n"),
        ("* a|\nCLOSED: [2026-10-30 Fri]", "* a|"),
        ("* a|", "* a|"),
    ] {
        assert_eq!(
            edit(
                &ev,
                &editor,
                before,
                "(org-set-planning (org-lines) 1 \"CLOSED\" #f)"
            ),
            after
        );
    }
    // S-上下左右: カーソルの所 (年, 月, 日, 時, 分) を変える
    let cases = [
        ("<2026-10-30 Fri>", 3, "(org-shiftup)", "<2027-10-30 Sat>"),
        ("<2026-10-30 Fri>", 8, "(org-shiftdown)", "<2026-09-30 Wed>"),
        ("<2026-10-30 Fri>", 11, "(org-shiftup)", "<2026-10-31 Sat>"),
        (
            "<2026-10-31 Sat>",
            14,
            "(org-shiftright)",
            "<2026-11-01 Sun>",
        ),
        ("[2026-10-30 Fri]", 1, "(org-shiftleft)", "[2026-10-29 Thu]"),
        (
            "<2026-10-30 Fri 23:00>",
            18,
            "(org-shiftup)",
            "<2026-10-31 Sat 00:00>",
        ),
        (
            "<2026-10-30 Fri 10:03>",
            21,
            "(org-shiftup)",
            "<2026-10-30 Fri 10:05>",
        ),
        (
            "<2026-10-30 Fri 10:03>",
            21,
            "(org-shiftdown)",
            "<2026-10-30 Fri 10:00>",
        ),
        (
            "<2026-10-30 Fri 00:00>",
            21,
            "(org-shiftdown)",
            "<2026-10-29 Thu 23:55>",
        ),
        (
            "<2026-10-30 Fri 09:00-10:00 +1w -2d>",
            18,
            "(org-shiftup)",
            "<2026-10-30 Fri 10:00-11:00 +1w -2d>",
        ),
    ];
    for (before, at, code, after) in cases {
        // カーソルは, 同じ所に残る
        let mut expected: Vec<char> = after.chars().collect();
        expected.insert(at - 1, '|');
        assert_eq!(
            edit(
                &ev,
                &editor,
                &format!("{}|", before),
                &format!("(goto-char {}) {}", at, code)
            ),
            expected.into_iter().collect::<String>(),
            "{} {} {}",
            before,
            at,
            code
        );
    }
}

#[test]
fn test_tables() {
    let (ev, editor, _dir) = setup();
    // 表の `|` と分けるため, カーソルは `@`
    let edit = |before: &str, code: &str| {
        set_text_mark(&editor, before, '@');
        run(&ev, &editor, code);
        shown_mark(&editor, '@')
    };
    let cases = [
        // 揃える (数の多い列は右に. 全角は 2 桁)
        (
            "|a|bb|\n|-\n|1|日本|\n|22|\n@",
            "(goto-char 2) (org-ctrl-c-ctrl-c)",
            "|  @a | bb   |\n|----+------|\n|  1 | 日本 |\n| 22 |      |\n",
        ),
        // TAB, S-TAB, RET
        (
            "| a@ | b |\n| c | d |\n",
            "(org-cycle)",
            "| a | @b |\n| c | d |\n",
        ),
        (
            "| a | b@ |\n|---+---|\n| c | d |\n",
            "(org-cycle)",
            "| a | b |\n|---+---|\n| @c | d |\n",
        ),
        ("| a | b@ |\n", "(org-cycle)", "| a | b |\n| @  |   |\n"),
        ("| a | b@ |\n", "(org-shifttab)", "| @a | b |\n"),
        ("| a@ | b |\n", "(org-shifttab)", "| @a | b |\n"),
        (
            "| a | b |\n|---+---|\n| c@ | d |\n",
            "(org-shifttab)",
            "| a | @b |\n|---+---|\n| c | d |\n",
        ),
        (
            "| a@ | b |\n| c | d |\n",
            "(org-return)",
            "| a | b |\n| @c | d |\n",
        ),
        (
            "| a | b@ |\n|---+---|\n",
            "(org-return)",
            "| a | b |\n|   | @  |\n|---+---|\n",
        ),
        // 横線, 列と行を動かす, 消す, 足す
        (
            "| a@ | b |\n",
            "(org-ctrl-c-minus)",
            "| a@ | b |\n|---+---|\n",
        ),
        ("| a@ | b |\n", "(org-metaright)", "| b | a@ |\n"),
        ("| a | b@ |\n", "(org-metaleft)", "| b@ | a |\n"),
        ("| a@ |\n| b |\n", "(org-metadown)", "| b |\n| a@ |\n"),
        ("| a |\n| b@ |\n", "(org-metaup)", "| b@ |\n| a |\n"),
        (
            "| a | b@ |\n| c |\n",
            "(org-shiftmetaleft)",
            "| @a |\n| c |\n",
        ),
        ("| a@ | b |\n", "(org-shiftmetaright)", "| @  | a | b |\n"),
        ("| a |\n| b@ |\n", "(org-shiftmetaup)", "| @a |\n"),
        ("| a@ |\n", "(org-shiftmetaup)", "@"),
        ("| a@ |\n", "(org-shiftmetadown)", "| @  |\n| a |\n"),
        // 表を作る
        (
            "  @\nx",
            "(org-table-create-or-convert-from-region)",
            "  | @  |   |\nx",
        ),
    ];
    answer(&editor, &["2x1"]);
    for (before, code, after) in cases {
        assert_eq!(edit(before, code), after, "{} {}", before, code);
    }
    for (code, error) in [
        ("(org-metaright)", "Cannot move column further"),
        ("(org-metaup)", "Cannot move row further"),
        (
            "(org-table-create-or-convert-from-region)",
            "Table already exists",
        ),
    ] {
        set_text_mark(&editor, "| a@ |\n", '@');
        assert_eq!(fail(&ev, &editor, code), error, "{}", code);
    }
    answer(&editor, &["3x2", "x"]);
    assert_eq!(
        edit("* a@\n", "(org-table-create-or-convert-from-region)"),
        "* a\n| @  |   |   |\n|---+---+---|\n|   |   |   |\n"
    );
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "Table size Columns x Rows [e.g. 5x2]: "
    );
    set_text(&editor, "|");
    assert_eq!(
        fail(&ev, &editor, "(org-table-create-or-convert-from-region)"),
        "Invalid table size: x"
    );
}

#[test]
fn test_sort() {
    let (ev, editor, _dir) = setup();
    let text = "* p|\n** TODO b <2026-10-02 Fri>\n** [#A] c 10\n** DONE a 2\nx\n";
    let cases = [
        (
            "a",
            "|* p\n** DONE a 2\nx\n** TODO b <2026-10-02 Fri>\n** [#A] c 10\n",
        ),
        (
            "A",
            "|* p\n** [#A] c 10\n** TODO b <2026-10-02 Fri>\n** DONE a 2\nx\n",
        ),
        (
            "o",
            "|* p\n** TODO b <2026-10-02 Fri>\n** DONE a 2\nx\n** [#A] c 10\n",
        ),
        (
            "p",
            "|* p\n** [#A] c 10\n** TODO b <2026-10-02 Fri>\n** DONE a 2\nx\n",
        ),
        (
            "t",
            "|* p\n** TODO b <2026-10-02 Fri>\n** [#A] c 10\n** DONE a 2\nx\n",
        ),
    ];
    for (key, after) in cases {
        answer(&editor, &[key]);
        assert_eq!(edit(&ev, &editor, text, "(org-sort)"), after, "{}", key);
    }
    assert_eq!(editor.message(), "Sorting entries...done");
    answer(&editor, &["n"]);
    assert_eq!(
        edit(&ev, &editor, "* p|\n** 10 x\n** 9 y\n** z\n", "(org-sort)"),
        "|* p\n** z\n** 9 y\n** 10 x\n"
    );
    answer(&editor, &["q"]);
    set_text(&editor, text);
    assert_eq!(fail(&ev, &editor, "(org-sort)"), "Invalid sorting type `q'");
    set_text(&editor, "* p|\n");
    assert_eq!(fail(&ev, &editor, "(org-sort)"), "Nothing to sort");
}

#[test]
fn test_links() {
    let (ev, editor, dir) = setup();
    let file = dir.path().join("a.org");
    let other = dir.path().join("b.org");
    fs::write(
        &file,
        "* TODO Head [1/2] :t:\nx\n* Two\n:PROPERTIES:\n:CUSTOM_ID: two\n:END:\n",
    )
    .unwrap();
    fs::write(&other, "* Target\n<<here>>\nline 3 text\n").unwrap();
    let path = file.to_string_lossy().to_string();
    let b = other.to_string_lossy().to_string();
    // 覚える (見出し, CUSTOM_ID, ほかのファイルの行, ファイルでないバッファ)
    run(
        &ev,
        &editor,
        &format!("(find-file {:?}) (goto-char 3) (org-store-link)", path),
    );
    assert_eq!(editor.message(), "Stored: Head");
    run(&ev, &editor, "(goto-char 40) (org-store-link)");
    assert_eq!(editor.message(), "Stored: Two");
    let txt = dir.path().join("c.txt");
    fs::write(&txt, "  some line  \n\n").unwrap();
    run(
        &ev,
        &editor,
        &format!(
            "(find-file {:?}) (org-store-link) (goto-char 15) (org-store-link)",
            txt.to_string_lossy()
        ),
    );
    assert_eq!(
        run(&ev, &editor, "(map car org-stored-links)"),
        format!(
            "(\"file:{t}\" \"file:{t}::some line\" \"file:{p}::#two\" \"file:{p}::*Head\")",
            t = txt.to_string_lossy(),
            p = path
        )
    );
    run(
        &ev,
        &editor,
        "(switch-to-buffer \"*scratch*\") (org-store-link)",
    );
    assert_eq!(
        editor.message(),
        "Cannot link to a buffer which is not visiting a file"
    );
    // ホームは ~ にする
    assert_eq!(
        run(
            &ev,
            &editor,
            "(org-abbreviate-file-name (expand-file-name \"~/x.org\"))"
        ),
        "\"~/x.org\""
    );
    // 入れる (覚えたリンクの説明を, 既定にする. 使ったリンクは忘れる)
    answer(
        &editor,
        &[
            &format!("file:{}::*Head", path),
            "Head",
            "https://example.com",
            "",
        ],
    );
    assert_eq!(
        edit(&ev, &editor, "x |", "(org-insert-link)"),
        format!("x [[file:{}::*Head][Head]]|", path)
    );
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "Description: "
    );
    assert_eq!(
        edit(&ev, &editor, "|", "(org-insert-link)"),
        "[[https://example.com]]|"
    );
    assert_eq!(run(&ev, &editor, "(length org-stored-links)"), "3");
    // リンクの上なら, 変える
    answer(&editor, &["https://b.example", "B"]);
    assert_eq!(
        edit(
            &ev,
            &editor,
            "x [[https://a.ex|ample][A]] y",
            "(org-insert-link)"
        ),
        "x [[https://b.example][B]]| y"
    );
    answer(&editor, &[" ", ""]);
    set_text(&editor, "|");
    assert_eq!(fail(&ev, &editor, "(org-insert-link)"), "No link selected");
    // 開く: http は browse-url, file: はエディタ (::*見出し, ::#id, ::行, ::/正規表現/, ::言葉)
    run(&ev, &editor, "(set! browse-url-generic-program \"true\")");
    set_text(&editor, "see https://example.com|/x");
    run(&ev, &editor, "(goto-char 6) (org-open-at-point)");
    set_text(&editor, &format!("[[file:{}::*Two][two]]|", path));
    run(&ev, &editor, "(goto-char 3) (org-open-at-point)");
    assert_eq!(editor.name(), path);
    assert_eq!(editor.current_line(), "* Two");
    // Org mode は, エディタの実行する find-file-hook で始まる
    run(&ev, &editor, "(run-hooks 'find-file-hook)");
    assert_eq!(editor.key("Tab").as_deref(), Some("org-cycle"));
    for (link, line) in [
        (format!("file:{}::#two", path), "* Two"),
        (format!("file:{}::3", b), "line 3 text"),
        (format!("file:{}::99", b), "line 3 text"),
        (format!("file:{}::/^line/", b), "line 3 text"),
        (format!("file:{}::here", b), "<<here>>"),
        (format!("file:{}::Target", b), "* Target"),
        (format!("file:{}::3 text", b), "line 3 text"),
        // 開いているファイルは, カーソルを動かさない
        (b.clone(), "line 3 text"),
    ] {
        run(
            &ev,
            &editor,
            &format!("(switch-to-buffer {:?}) (org-link-open {:?})", path, link),
        );
        assert_eq!(editor.current_line(), line, "{}", link);
    }
    // 同じファイルの中 (*見出し). 相対のパス
    run(
        &ev,
        &editor,
        &format!(
            "(switch-to-buffer {:?}) (org-link-open \"*Head\") (org-link-open \"./b.org\")",
            path
        ),
    );
    assert_eq!(editor.name(), b);
    // 隠れていれば見せる
    run(
        &ev,
        &editor,
        "(goto-char 1) (org-cycle) (org-link-open \"here\")",
    );
    assert!(editor.buffer().folded.is_empty());
    // html と pdf は browse-url
    run(
        &ev,
        &editor,
        "(org-link-open \"file:x.html\") (org-link-open \"./y.pdf\")",
    );
    for (code, error) in [
        (
            "(org-link-open \"*Nothing\")",
            "No match for fuzzy expression: *Nothing",
        ),
        (
            "(org-link-open \"elisp:(foo)\")",
            "Unknown link type: elisp:(foo)",
        ),
        (
            "(org-link-open \"#none\")",
            "No match for fuzzy expression: #none",
        ),
    ] {
        assert_eq!(fail(&ev, &editor, code), error);
    }
    set_text(&editor, "no link|");
    run(&ev, &editor, "(org-open-at-point)");
    assert_eq!(editor.message(), "No link found");
    // RET (org-return-follows-link が #t なら, リンクを開く)
    assert_eq!(edit(&ev, &editor, "a|b", "(org-return)"), "a\n|b");
    run(&ev, &editor, "(set! org-return-follows-link #t)");
    set_text(
        &editor,
        &format!("[[file:{}]]", path).replacen("[[", "[|[", 1),
    );
    run(&ev, &editor, "(org-return)");
    assert_eq!(editor.name(), path);
    assert_eq!(edit(&ev, &editor, "a|b", "(org-return)"), "a\n|b");
    // 数える印を除いた題
    assert_eq!(
        run(&ev, &editor, "(org-strip-cookies \"[1/2] a [50%] b [%]\")"),
        "\"a b\""
    );
    assert_eq!(
        run(
            &ev,
            &editor,
            "(list (org-file-directory \"x\") (org-file-base \"/a/b.c.org\"))"
        ),
        "(\"./\" \"b.c\")"
    );
}

/// アジェンダのファイルを書く.
///
/// # Arguments
///
/// * `dir` - ディレクトリ.
///
/// # Returns
///
/// ファイルのパス.
fn agenda_file(dir: &Path) -> String {
    let file = dir.join("work.org");
    let body = format!(
        "#+CATEGORY: work\n\
         * TODO Write report :doc:\n\
         SCHEDULED: <{past}>\n\
         * TODO Meeting\n\
         <{tomorrow} 10:00-11:00>\n\
         * DONE Old\n\
         DEADLINE: <{d2}>\n\
         * TODO Pay\n\
         DEADLINE: <{d5}>\n\
         * TODO Late\n\
         DEADLINE: <{past}>\n\
         * Weekly\n\
         <{weekly} 9:00 +1w>\n\
         * [#A] Today :x:y:\n\
         SCHEDULED: <{today} 8:30>\n\
         :PROPERTIES:\n\
         :CATEGORY: home\n\
         :END:\n\
         Some words here\n",
        past = day(-6, "%Y-%m-%d %a"),
        tomorrow = day(1, "%Y-%m-%d %a"),
        d2 = day(2, "%Y-%m-%d %a"),
        d5 = day(5, "%Y-%m-%d %a"),
        weekly = day(-14, "%Y-%m-%d %a"),
        today = day(0, "%Y-%m-%d %a"),
    );
    fs::write(&file, body).unwrap();
    file.to_string_lossy().to_string()
}

#[test]
fn test_agenda() {
    let (ev, editor, dir) = setup();
    let file = agenda_file(dir.path());
    fs::write(dir.path().join(".hidden.org"), "* TODO hidden\n").unwrap();
    fs::create_dir_all(dir.path().join("sub")).unwrap();
    fs::write(dir.path().join("sub/deep.org"), "* TODO deep\n").unwrap();
    run(
        &ev,
        &editor,
        &format!(
            "(set! org-agenda-files (list {:?})) (set! org-agenda-start-on-weekday #f) (org-agenda-list)",
            dir.path().to_string_lossy()
        ),
    );
    assert_eq!(editor.name(), "*Org Agenda*");
    let text = editor.text();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], format!("Week-agenda (W{}):", day(0, "%V")));
    let date_line = |n: i64| {
        let week = if day(n, "%u") == "1" {
            format!(" W{}", day(n, "%V"))
        } else {
            String::new()
        };
        format!(
            "{:<10} {:>2} {} {}{}",
            day(n, "%A"),
            day(n, "%-d"),
            day(n, "%B"),
            day(n, "%Y"),
            week
        )
    };
    assert_eq!(lines[1], date_line(0));
    // 時刻の無い行は, 優先度の順 (過ぎた予定, 過ぎた締め切り, 近い締め切り)
    let expected = [
        format!(
            "  home:        8:30...... Scheduled:  [#A] Today{}:x:y:",
            " ".repeat(27)
        ),
        "  work:        9:00...... Weekly".to_string(),
        format!(
            "  work:       Sched. 6x:  TODO Write report{}:doc:",
            " ".repeat(32)
        ),
        "  work:        6 d. ago:  TODO Late".to_string(),
        "  work:       In   5 d.:  TODO Pay".to_string(),
        date_line(1),
    ];
    assert_eq!(&lines[2..8], &expected[..]);
    assert_eq!(lines[8], "  work:       10:00-11:00 TODO Meeting");
    assert_eq!(lines[9], date_line(2));
    assert_eq!(lines[10], "  work:       Deadline:  DONE Old");
    assert!(!text.contains("hidden") && !text.contains("deep"));
    // 行の項目
    run(
        &ev,
        &editor,
        "(goto-char 1) (call-interactively 'next-line) (call-interactively 'next-line)",
    );
    assert_eq!(
        run(&ev, &editor, "(org-agenda-marker)"),
        format!("({:?} 14)", file)
    );
    // SPC (ほかの窓), RET (開く)
    run(
        &ev,
        &editor,
        "(org-agenda-show-and-scroll-up) (org-agenda-goto)",
    );
    assert_eq!(
        editor.displayed.lock().unwrap().last().unwrap(),
        &(file.clone(), Some(14))
    );
    run(&ev, &editor, "(org-agenda-switch-to)");
    assert_eq!(editor.name(), file);
    assert_eq!(editor.current_line(), "* [#A] Today :x:y:");
    // t (状態を変えて, 作り直す. 書き換えたバッファから読む)
    run(&ev, &editor, "(switch-to-buffer \"*Org Agenda*\") (goto-char 1) (org-agenda-next-line) (org-agenda-next-line) (org-agenda-next-line) (org-agenda-next-line) (org-agenda-previous-line) (org-agenda-todo)");
    assert_eq!(editor.name(), "*Org Agenda*");
    assert!(
        editor
            .text()
            .contains("  work:        9:00...... TODO Weekly"),
        "{}",
        editor.text()
    );
    assert_eq!(
        editor.current_line(),
        "  work:        9:00...... TODO Weekly"
    );
    assert!(run(&ev, &editor, &format!("(buffer-modified-p {:?})", file)) == "#t");
    // f, b, ., r, g
    run(&ev, &editor, "(org-agenda-later)");
    assert_eq!(editor.text().lines().nth(1).unwrap(), date_line(7));
    run(&ev, &editor, "(org-agenda-earlier) (org-agenda-earlier)");
    assert_eq!(editor.text().lines().nth(1).unwrap(), date_line(-7));
    run(
        &ev,
        &editor,
        "(set! org-agenda-start-on-weekday 1) (org-agenda-goto-today) (org-agenda-redo)",
    );
    let monday = (0..7).find(|n| day(-n, "%u") == "1").unwrap();
    assert_eq!(editor.text().lines().nth(1).unwrap(), date_line(-monday));
    run(&ev, &editor, "(set! org-agenda-span 1) (org-agenda-list)");
    assert!(editor.text().starts_with("Day-agenda"));
    run(&ev, &editor, "(set! org-agenda-span 3) (org-agenda-list)");
    assert!(editor.text().starts_with("3-days-agenda"));
    // TODO の一覧, タグ, 言葉
    run(&ev, &editor, "(org-todo-list)");
    assert_eq!(
        editor.text(),
        "Global list of TODO items of type: ALL\n  work:       TODO Write report                                            :doc:\n  work:       TODO Meeting\n  work:       TODO Pay\n  work:       TODO Late\n  work:       TODO Weekly\n"
    );
    for (code, error) in [
        ("(org-agenda-later)", "Not allowed in this agenda"),
        ("(org-agenda-goto-today)", "Not allowed in this agenda"),
        (
            "(goto-char 1) (org-agenda-switch-to)",
            "Command not allowed in this line",
        ),
    ] {
        assert_eq!(fail(&ev, &editor, code), error);
    }
    answer(&editor, &["x+y", "doc|x", "-doc", "SOME words"]);
    run(&ev, &editor, "(org-tags-view)");
    assert_eq!(
        editor.text(),
        format!(
            "Headlines with TAGS match: x+y\n  home:       [#A] Today{}:x:y:\n",
            " ".repeat(51)
        )
    );
    run(&ev, &editor, "(org-tags-view)");
    assert_eq!(editor.text().lines().count(), 3);
    run(&ev, &editor, "(org-tags-view) (org-agenda-redo)");
    assert_eq!(editor.text().lines().count(), 7);
    run(&ev, &editor, "(org-search-view)");
    assert_eq!(
        editor.text().lines().next().unwrap(),
        "Search words: some words"
    );
    assert_eq!(editor.text().lines().count(), 2);
    // 選ぶ
    answer(
        &editor,
        &["t", "a Agenda for current week", "m", "", "s", "", "z"],
    );
    run(&ev, &editor, "(org-agenda)");
    assert!(editor.text().starts_with("Global list"));
    run(&ev, &editor, "(org-agenda)");
    assert!(editor.text().contains("-agenda"));
    run(&ev, &editor, "(org-agenda) (org-agenda)");
    assert_eq!(
        fail(&ev, &editor, "(org-agenda)"),
        "Invalid agenda command: z"
    );
    // q (消して, 前のバッファへ)
    run(&ev, &editor, "(org-agenda-quit)");
    assert_eq!(editor.name(), "*scratch*");
    assert!(!run(&ev, &editor, "(buffer-list)").contains("Org Agenda"));
    // ファイルが無い, ファイルを 1 つ (文字列で)
    run(
        &ev,
        &editor,
        &format!("(set! org-agenda-files {:?}) (org-todo-list)", file),
    );
    assert_eq!(editor.text().lines().count(), 6);
    run(
        &ev,
        &editor,
        "(set! org-agenda-files '(\"/nonexistent/x.org\")) (org-todo-list) (org-agenda-quit)",
    );
    assert_eq!(editor.text(), "");
    // 繰り返し (日, 月, 年)
    for (stamp, on, result) in [
        ("<2026-01-31 Sat +1m>", "2026-03-31", "#t"),
        ("<2026-01-31 Sat +2m>", "2026-02-28", "#f"),
        ("<2024-02-29 Thu +1y>", "2028-02-29", "#t"),
        ("<2024-02-29 Thu +1y>", "2025-02-28", "#f"),
        ("<2026-01-01 Thu +3d>", "2026-01-07", "#t"),
        ("<2026-01-01 Thu +3d>", "2025-12-29", "#f"),
        ("<2026-01-01 Thu +1h>", "2026-01-02", "#f"),
    ] {
        assert_eq!(
            run(
                &ev,
                &editor,
                &format!(
                    "(org-timestamp-on? (org-parse-timestamp {:?}) {:?})",
                    stamp, on
                )
            ),
            result,
            "{} {}",
            stamp,
            on
        );
    }
}

#[test]
fn test_capture() {
    let (ev, editor, dir) = setup();
    let notes = dir.path().join("notes.org");
    // 既定のテンプレート (見出しが無ければ作る)
    let source = dir.path().join("src.txt");
    fs::write(&source, "code\n").unwrap();
    run(
        &ev,
        &editor,
        &format!("(find-file {:?})", source.to_string_lossy()),
    );
    answer(&editor, &["t"]);
    run(&ev, &editor, "(org-capture)");
    assert_eq!(editor.name(), "*Capture*");
    assert_eq!(
        shown(&editor),
        format!(
            "* TODO |\n  [{}]\n  [[file:{}::code]]",
            day(0, "%Y-%m-%d %a"),
            source.to_string_lossy()
        )
    );
    assert_eq!(
        editor.key("Ctrl-c Ctrl-c").as_deref(),
        Some("org-capture-finalize")
    );
    assert_eq!(
        editor.message(),
        "Capture buffer.  Finish `C-c C-c', abort `C-c C-k'."
    );
    run(&ev, &editor, "(insert \"Buy milk\") (org-capture-finalize)");
    assert_eq!(editor.name(), source.to_string_lossy());
    assert!(editor.message().starts_with("Captured to "));
    let saved = fs::read_to_string(&notes).unwrap();
    assert!(
        saved.starts_with("* Tasks\n** TODO Buy milk\n  ["),
        "{}",
        saved
    );
    assert!(saved.ends_with("::code]]\n"), "{}", saved);
    // 2 回目は, 見出しの木の終わりに足す
    answer(&editor, &["t Task"]);
    run(
        &ev,
        &editor,
        "(org-capture) (insert \"Second\") (org-capture-finalize)",
    );
    let saved = fs::read_to_string(&notes).unwrap();
    assert!(saved.contains("::code]]\n** TODO Second\n"), "{}", saved);
    // テンプレート: 問い, 既定のある問い, ファイル, 式, 種類 (item, checkitem, plain), file の行き先
    let tmpl = dir.path().join("tmpl.txt");
    fs::write(&tmpl, "from file").unwrap();
    run(
        &ev,
        &editor,
        &format!(
            "(set! org-capture-templates '((\"j\" \"Journal\" entry (file+headline \"journal.org\" \"Inbox\") \"* %^{{Title}} %U\\n%^{{Mood|good|bad}} %[{t}] %(+ 1 2) %f %% %i%?\") \
              (\"i\" \"Item\" item (file+headline \"list.org\" \"L\") \"%?\") \
              (\"c\" \"Check\" checkitem (file \"list.org\") \"x\") \
              (\"p\" \"Plain\" plain (file \"~/../../{p}\") \"text %t %T\") \
              (\"e\" \"Entry\" entry (file \"{e}\") \"** deep\\n*** deeper\")))",
            t = tmpl.to_string_lossy(),
            p = dir.path().join("plain.org").to_string_lossy().trim_start_matches('/'),
            e = dir.path().join("entry.org").to_string_lossy()
        ),
    );
    answer(&editor, &["j", "Hello", "good"]);
    run(&ev, &editor, "(org-capture)");
    let prompts = editor.prompts.lock().unwrap().clone();
    assert_eq!(prompts[prompts.len() - 2..], ["Title: ", "Mood [good]: "]);
    let captured = shown(&editor);
    assert!(captured.starts_with("* Hello ["), "{}", captured);
    assert!(
        captured.ends_with("\ngood from file 3 src.txt % |"),
        "{}",
        captured
    );
    run(&ev, &editor, "(org-capture-finalize)");
    let journal = fs::read_to_string(dir.path().join("journal.org")).unwrap();
    assert!(journal.starts_with("* Inbox\n** Hello ["), "{}", journal);
    fs::write(dir.path().join("list.org"), "* L\n- a\n* M\n").unwrap();
    answer(&editor, &["i"]);
    run(
        &ev,
        &editor,
        "(org-capture) (insert \"b\") (org-capture-finalize)",
    );
    answer(&editor, &["c"]);
    run(&ev, &editor, "(org-capture) (org-capture-finalize)");
    assert_eq!(
        fs::read_to_string(dir.path().join("list.org")).unwrap(),
        "* L\n- a\n- b\n* M\n- [ ] x\n"
    );
    answer(&editor, &["p"]);
    run(&ev, &editor, "(org-capture) (org-capture-finalize)");
    let plain = fs::read_to_string(dir.path().join("plain.org")).unwrap();
    assert!(plain.starts_with("text <"), "{}", plain);
    fs::write(dir.path().join("entry.org"), "* top").unwrap();
    answer(&editor, &["e"]);
    run(&ev, &editor, "(org-capture) (org-capture-finalize)");
    assert_eq!(
        fs::read_to_string(dir.path().join("entry.org")).unwrap(),
        "* top\n* deep\n** deeper\n"
    );
    // ファイルでないバッファから (%a は空), 止める
    run(&ev, &editor, "(switch-to-buffer \"*scratch*\")");
    answer(&editor, &["t"]);
    run(
        &ev,
        &editor,
        "(set! org-capture-templates '()) (org-capture)",
    );
    assert_eq!(
        shown(&editor),
        format!("* TODO |\n  [{}]\n  ", day(0, "%Y-%m-%d %a"))
    );
    run(&ev, &editor, "(org-capture-kill)");
    assert_eq!(editor.name(), "*scratch*");
    assert_eq!(editor.message(), "Capture aborted");
    for (answers, code, error) in [
        (
            vec!["z"],
            "(org-capture)",
            "No capture template referred to by \"z\" keys",
        ),
        (
            vec![],
            "(org-capture-finalize)",
            "This does not seem to be a capture buffer for Org mode",
        ),
        (
            vec![],
            "(org-capture-kill)",
            "This does not seem to be a capture buffer for Org mode",
        ),
    ] {
        answer(&editor, &answers);
        assert_eq!(fail(&ev, &editor, code), error);
    }
    // 行き先のファイル
    assert_eq!(
        run(
            &ev,
            &editor,
            "(list (org-capture-expand-file #f) (org-capture-expand-file \"/a.org\"))"
        ),
        format!(
            "({:?} \"/a.org\")",
            dir.path().join("notes.org").to_string_lossy()
        )
    );
}

#[test]
fn test_switchb() {
    let (ev, editor, dir) = setup();
    assert_eq!(fail(&ev, &editor, "(org-switchb)"), "No Org buffers");
    let file = dir.path().join("a.org");
    fs::write(&file, "* a\n").unwrap();
    editor.show("other", FakeBuffer::default);
    run(
        &ev,
        &editor,
        &format!("(find-file-noselect {:?})", file.to_string_lossy()),
    );
    answer(&editor, &[&file.to_string_lossy()]);
    run(&ev, &editor, "(org-switchb)");
    assert_eq!(editor.name(), file.to_string_lossy());
    assert_eq!(
        *editor.completions.lock().unwrap(),
        vec![file.to_string_lossy().to_string()]
    );
    // C-c l, C-c a, C-c c (Org の手引きの勧めるキー)
    assert_eq!(run(&ev, &editor, "(featurep 'org)"), "#t");
}
