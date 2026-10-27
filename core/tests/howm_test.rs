//! howm (`lisp/howm.scm`) のテスト. 偽のエディタ (バッファを持つ) の上で, メニュー, メモを作る, 一覧, 検索, 予定と todo, リンク, 日付を確かめる.

use std::fs;
use std::path::{Path, PathBuf};

use rseditor_core::scheme::evaluator::Evaluator;
use tempfile::{tempdir, TempDir};

mod common;
use common::FakeEditor;

/// howm を読み込んだ評価器と, 偽のエディタと, メモのディレクトリを作る.
///
/// # Returns
///
/// 評価器, 偽のエディタ, メモのディレクトリ (消さないように持つ).
fn setup() -> (Evaluator, FakeEditor, TempDir) {
    let dir = tempdir().unwrap();
    let howm = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lisp/howm.scm");
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

/// メモを開く (howm-open. howm のバッファにする).
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `path` - メモのパス.
fn open(ev: &Evaluator, editor: &FakeEditor, path: &Path) {
    ev.eval_with(&format!("(howm-open {:?} 1)", path), editor)
        .unwrap();
}

/// 今の行の文字に置いたカーソルで, RET (howm-action) を押す.
///
/// # Arguments
///
/// * `ev` - 評価器.
/// * `editor` - 偽のエディタ.
/// * `at` - カーソルを置く文字.
fn press(ev: &Evaluator, editor: &FakeEditor, at: &str) {
    editor.move_to(at);
    ev.run_command("howm-action", editor).unwrap();
}

#[test]
fn test_commands_and_keys() {
    let (ev, editor, _dir) = setup();
    for name in [
        "howm",
        "howm-menu",
        "howm-create",
        "howm-remember",
        "howm-dup",
        "howm-list-all",
        "howm-list-around",
        "howm-search-fixed",
        "howm-search-migemo",
        "howm-occur",
        "howm-history",
        "howm-random-walk",
        "howm-kill-all",
        "howm-insert-date",
        "howm-action",
    ] {
        assert!(ev.commands().iter().any(|c| c == name), "{}", name);
    }
    for (key, command) in [
        ("Ctrl-c , ,", "howm-menu"),
        ("Ctrl-c , e", "howm-remember"),
        ("Ctrl-c , A", "howm-list-around"),
        ("Ctrl-c , d", "howm-insert-date"),
    ] {
        assert!(ev
            .key_bindings()
            .iter()
            .any(|k| k.key == key && k.command == command));
    }
    // require で読めるように, provide している
    assert_eq!(ev.eval_with("(featurep 'howm)", &editor).unwrap(), "#t");
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
    // 下線 (リンク, 日付など)
    assert_eq!(editor.buffer().highlights.len(), 5);
    // 保存した (ことにする)
    ev.eval_with("(insert \"買い物\")", &editor).unwrap();
    fs::write(&file, editor.text()).unwrap();

    // メニュー (無ければ, 既定のメニューを書く)
    ev.run_command("howm", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:%menu%*");
    let menu = editor.text();
    let lines: Vec<&str> = menu.lines().collect();
    assert_eq!(lines[0], "= <<< %menu%");
    assert!(
        lines[1].starts_with(&format!("[{} ", today)),
        "{}",
        lines[1]
    );
    assert!(lines[1].ends_with("] R[menu 更新]"), "{}", lines[1]);
    assert!(
        menu.contains(" 作成:     e[速記]      c[新規] D[複製]"),
        "{}",
        menu
    );
    assert!(menu.contains("を 7 日先まで表示"));
    assert!(menu.contains("上位 50 個まで表示"));
    assert!(menu.contains(" | 買い物"), "{}", menu);
    // 別のメニュー (%ボタンの説明% など) は, 出さない
    assert!(!menu.contains("◆"));
    assert!(dir.path().join("0000-00-00-000000.txt").exists());
    assert!(!editor.buffer().modified);
    assert_eq!(editor.key("c").as_deref(), Some("howm-menu-key-c"));
    assert_eq!(editor.key("R").as_deref(), Some("howm-menu-key-R"));
    assert_eq!(editor.key("Enter").as_deref(), Some("howm-action"));
    assert!(editor
        .buffer()
        .highlights
        .iter()
        .any(|h| h.contains("新規")));

    // 最近のメモの行で RET を押すと, そのメモを開く
    press(&ev, &editor, " | 買い物");
    assert_eq!(editor.buffer().file.as_deref(), Some(file.as_str()));

    // ボタンの上の RET. 一覧の n で, もう 1 つの窓に出す
    ev.run_command("howm", &editor).unwrap();
    press(&ev, &editor, "[一覧]");
    assert_eq!(editor.name(), "*howmS*");
    assert!(editor.text().ends_with(" | 買い物"));
    assert_eq!(editor.message(), "1 items");
    assert_eq!(editor.key("n").as_deref(), Some("howm-summary-next"));
    assert_eq!(
        editor.displayed.lock().unwrap().last().cloned(),
        Some((file.clone(), Some(1)))
    );

    // 動かすものの無い所
    ev.run_command("howm", &editor).unwrap();
    press(&ev, &editor, "最近のメモ");
    assert_eq!(editor.name(), "*howmM:%menu%*");
    assert_eq!(editor.message(), "No action here");

    // キーで, メニューを開く前のバッファから新しいメモを作る (同じ秒でも, 別のファイル)
    ev.run_command("howm-menu-key-c", &editor).unwrap();
    let new = editor.buffer().file.unwrap();
    assert_ne!(new, file);
    assert!(editor.text().starts_with("= \n["));

    // メニューを出し直す. メニューを直す
    ev.run_command("howm", &editor).unwrap();
    ev.run_command("howm-menu-key-R", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:%menu%*");
    ev.run_command("howm-menu-edit", &editor).unwrap();
    assert!(editor.text().starts_with("= <<< %menu%"));
    // メニューの外では, 既定のメニューを出す
    ev.run_command("howm-menu-refresh", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:%menu%*");
    // 別のメニューを開く (>>> %ボタンの説明%)
    press(&ev, &editor, ">>> %ボタンの説明%");
    assert_eq!(editor.name(), "*howmM:%ボタンの説明%*");
    assert!(editor.text().starts_with("= <<< %ボタンの説明%\n\n◆ 作成"));
}

#[test]
fn test_menu_features() {
    let (ev, editor, dir) = setup();
    write_memo(dir.path(), "a.txt", "= 検索のメモ\n検索する\n");
    let menu = write_memo(
        dir.path(),
        "menu.txt",
        concat!(
            "= <<< %menu%\n",
            "%\"x\"[一覧] [速記] [foo] %sdays %tnum\n",
            "%\"e\"評価%| %eval%(message (buffer-name)) %|です\n",
            "%\"!\"呼ぶ %call%howm-list-all\n",
            "移動: %\"jump%\"\n",
            "見える%|見えない%|可視%|消える\n",
            "次の行\n",
            "数: %here%howm-menu-recent-num, %here%(car x)\n",
            "%here%(howm-menu-search \"検索\")\n",
            "%here%(howm-menu-legend)\n",
            ">>> %sub%\n",
            ">>> %none%\n",
            "= <<< %sub%\n",
            "sub %\"q\"[menu 更新]\n",
        ),
    );
    ev.eval_with(&format!("(set! howm-menu-file {:?})", menu), &editor)
        .unwrap();
    ev.run_command("howm", &editor).unwrap();
    assert_eq!(
        editor.text(),
        concat!(
            "= <<< %menu%\n",
            "x[一覧] [速記] [foo] 7 50\n",
            "e評価です\n",
            "!呼ぶ %call%howm-list-all\n",
            "移動: jump\n",
            "見える可視次の行\n",
            "数: 20, Not allowed: (car x)\n",
            "a | = 検索のメモ\n",
            "a | 検索する\n",
            "[予定・Todo] … 予定@, 〆切! を 7 日先まで表示.\n",
            "〆切!, todo+, 覚書-, 保留~ を上位 50 個まで表示.\n",
            ">>> %sub%\n",
            ">>> %none%",
        )
    );
    // キー: ボタン, %eval% (見えない所でも), %call%, 動くだけ. 動かすのは, メニューを開く前のバッファ
    ev.run_command("howm-menu-key-x", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
    assert_eq!(editor.message(), "1 items");
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &editor)
        .unwrap();
    ev.run_command("howm", &editor).unwrap();
    ev.run_command("howm-menu-key-e", &editor).unwrap();
    assert_eq!(editor.message(), "*scratch*");
    assert_eq!(editor.name(), "*scratch*");
    ev.run_command("howm", &editor).unwrap();
    ev.run_command("howm-menu-key-!", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
    ev.run_command("howm", &editor).unwrap();
    ev.run_command("howm-menu-key-j", &editor).unwrap();
    assert_eq!(editor.current_line(), "移動: jump");
    // RET: %call%, 使えるボタン, 使えないボタン, 項目
    press(&ev, &editor, "%call%");
    assert_eq!(editor.name(), "*howmS*");
    ev.run_command("howm", &editor).unwrap();
    press(&ev, &editor, "[速記]");
    assert_eq!(editor.name(), "*howm-remember*");
    ev.run_command("howm-remember-discard", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
    ev.run_command("howm", &editor).unwrap();
    press(&ev, &editor, "[foo]");
    assert_eq!(editor.message(), "No action here");
    press(&ev, &editor, "a | 検索する");
    assert_eq!(editor.current_line(), "検索する");
    // 別のメニュー
    ev.run_command("howm", &editor).unwrap();
    press(&ev, &editor, ">>> %none%");
    assert_eq!(editor.message(), "No menu: %none%");
    press(&ev, &editor, ">>> %sub%");
    assert_eq!(editor.name(), "*howmM:%sub%*");
    assert_eq!(editor.text(), "= <<< %sub%\nsub q[menu 更新]\n");
    ev.eval_with("(insert \"x\")", &editor).unwrap();
    ev.run_command("howm-menu-key-q", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:%sub%*");
    assert_eq!(editor.text(), "= <<< %sub%\nsub q[menu 更新]\n");
    ev.run_command("howm-menu-edit", &editor).unwrap();
    assert_eq!(
        editor.buffer().file,
        Some(menu.to_string_lossy().into_owned())
    );
    // ファイルを, メニューとして開く
    ev.eval_with("(howm-menu-open \"menu.txt\")", &editor)
        .unwrap();
    assert_eq!(editor.name(), "*howmM:menu*");
    assert!(editor.text().ends_with("= <<< %sub%\nsub q[menu 更新]\n"));
    ev.run_command("howm-refresh", &editor).unwrap();
    assert_eq!(editor.name(), "*howmM:menu*");
    // [設定] (初期設定ファイル)
    ev.run_command("howm-settings", &editor).unwrap();
    assert_eq!(editor.message(), "No init file");
    ev.eval_with(&format!("(set! user-init-file {:?})", menu), &editor)
        .unwrap();
    ev.run_command("howm-settings", &editor).unwrap();
    assert_eq!(
        editor.buffer().file,
        Some(menu.to_string_lossy().into_owned())
    );
    // メモが無いときの一覧
    fs::remove_file(dir.path().join("a.txt")).unwrap();
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

    // 小文字だけなら, 大文字と小文字を区別しない. come-from の行を先に. キーワードを補完する
    answer(&editor, &["emacs"]);
    ev.run_command("howm-search-fixed", &editor).unwrap();
    assert_eq!(*editor.completions.lock().unwrap(), vec!["Emacs"]);
    assert_eq!(
        editor.text(),
        "2026-09-01-100000 | <<< Emacs\n2026-10-01-100000 | emacs を使う\n2026-10-01-100000 | >>> emacs\n2026-09-01-100000 | = Emacs"
    );
    assert_eq!(editor.message(), "4 items");
    // 一覧で動くと, その行のメモを, もう 1 つの窓に出す
    ev.run_command("howm-summary-next", &editor).unwrap();
    assert_eq!(
        editor.displayed.lock().unwrap().last().cloned(),
        Some((new.to_string_lossy().into_owned(), Some(2)))
    );
    ev.run_command("howm-summary-previous", &editor).unwrap();
    assert_eq!(
        editor.displayed.lock().unwrap().last().cloned(),
        Some((old.to_string_lossy().into_owned(), Some(2)))
    );
    // 2 行目の RET で, そのメモのその行を開く
    press(&ev, &editor, "2026-10-01-100000 | emacs");
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
    // 行の終わり (リンクの外) と, リンクの無い所では, 改行を入れる
    open(&ev, &editor, &old);
    editor.move_to("エディタ");
    ev.run_command("howm-action-or-newline", &editor).unwrap();
    assert_eq!(editor.text(), "= Emacs\n<<< Emacs\n\nエディタ\n");
    editor.move_to("\n\nエディタ");
    ev.run_command("howm-action-or-newline", &editor).unwrap();
    assert_eq!(editor.text(), "= Emacs\n<<< Emacs\n\n\nエディタ\n");
    // <<< の上
    editor.move_to("<<< Emacs");
    ev.run_command("howm-action-or-newline", &editor).unwrap();
    assert_eq!(editor.message(), "2 items");

    // 大文字を含むと, 区別する
    answer(&editor, &["Emacs"]);
    ev.run_command("howm-search-fixed", &editor).unwrap();
    assert_eq!(editor.message(), "2 items");
    // 正規表現
    answer(&editor, &["^= .*記$"]);
    ev.run_command("howm-search-regexp", &editor).unwrap();
    assert_eq!(editor.text(), "2026-10-01-100000 | = 日記");
    // 見つからない, 空の入力
    answer(&editor, &["none", "", ""]);
    ev.run_command("howm-search-fixed", &editor).unwrap();
    assert_eq!(editor.message(), "No match: none");
    ev.run_command("howm-search-fixed", &editor).unwrap();
    ev.run_command("howm-search-regexp", &editor).unwrap();
    assert_eq!(editor.message(), "No match: none");

    // 履歴 (新しい順. 同じ言葉は 1 つ. ファイルにも書く)
    ev.run_command("howm-history", &editor).unwrap();
    assert_eq!(editor.text(), "none\n^= .*記$\nEmacs\nemacs");
    assert_eq!(
        fs::read_to_string(dir.path().join(".howm-history")).unwrap(),
        "none\n^= .*記$\nEmacs\nemacs"
    );
    press(&ev, &editor, "Emacs");
    assert_eq!(editor.message(), "2 items");
    ev.run_command("howm-history", &editor).unwrap();
    assert!(editor.text().starts_with("Emacs\nnone"));
    // 履歴の数の上限
    ev.eval_with("(set! howm-history-limit 2)", &editor)
        .unwrap();
    answer(&editor, &["日記"]);
    ev.run_command("howm-search-fixed", &editor).unwrap();
    ev.run_command("howm-history", &editor).unwrap();
    assert_eq!(editor.text(), "日記\nEmacs");

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
    // 一覧で動いても, もう 1 つの窓に出さない設定
    ev.eval_with("(set! howm-view-contents #f)", &editor)
        .unwrap();
    let shown = editor.displayed.lock().unwrap().len();
    ev.run_command("howm-list-all", &editor).unwrap();
    assert_eq!(editor.displayed.lock().unwrap().len(), shown);
}

#[test]
fn test_keywords_wiki_and_files() {
    let (ev, editor, dir) = setup();
    write_memo(dir.path(), "a.txt", "= Emacs\n<<< Emacs\n");
    let b = write_memo(
        dir.path(),
        "b.txt",
        "= 学ぶ\nいつか Emacs を学ぶ\n[[新しい言葉]] [[Emacs]]\nfile://~/c.txt\n",
    );
    let c = write_memo(dir.path(), "c.txt", "= c\n");
    open(&ev, &editor, &b);
    // キーワード ("<<< 言葉" の言葉) に, 下線を引く
    assert_eq!(editor.buffer().highlights.last().unwrap(), "Emacs");
    // キーワードは, どこにあってもリンク
    editor.move_to("Emacs を");
    ev.run_command("howm-action-or-newline", &editor).unwrap();
    assert_eq!(
        editor.text(),
        "a | <<< Emacs\nb | いつか Emacs を学ぶ\nb | [[新しい言葉]] [[Emacs]]\na | = Emacs"
    );
    // [[言葉]]: come-from があれば探す
    open(&ev, &editor, &b);
    press(&ev, &editor, "[[Emacs]]");
    assert_eq!(editor.name(), "*howmS*");
    // 無ければ, "= <<< 言葉" のメモを作る
    open(&ev, &editor, &b);
    press(&ev, &editor, "[[新しい言葉]]");
    assert!(editor.buffer().file.unwrap().ends_with(".txt"));
    assert!(editor.text().starts_with("= <<< 新しい言葉\n["));
    assert_eq!(editor.buffer().point, editor.text().chars().count() + 1);
    // file://
    let home = dirs::home_dir().unwrap();
    open(&ev, &editor, &b);
    editor.with_buffer(|buffer| {
        let text: String = buffer.text.iter().collect();
        buffer.text = text
            .replace("~/c.txt", &c.to_string_lossy())
            .chars()
            .collect();
    });
    press(&ev, &editor, "file://");
    assert_eq!(editor.buffer().file, Some(c.to_string_lossy().into_owned()));
    assert!(!home.as_os_str().is_empty());
    // キーワードを補完して入れる
    answer(&editor, &["Emacs", ""]);
    ev.run_command("howm-insert-keyword", &editor).unwrap();
    ev.run_command("howm-insert-keyword", &editor).unwrap();
    assert_eq!(editor.text(), "Emacs= c\n");
    // [[%名前%]] は, そのメニューを開く
    write_memo(dir.path(), "d.txt", "[[%menu%]]\n");
    open(&ev, &editor, &dir.path().join("d.txt"));
    press(&ev, &editor, "[[%menu%]]");
    assert_eq!(editor.name(), "*howmM:%menu%*");
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
    press(&ev, &editor, "〆切\n");
    assert_eq!(editor.current_line(), format!("[{}]! 〆切", day(1)));
    // 記号の上なら, メモのその所で RET を押す (取り消す)
    ev.run_command("howm", &editor).unwrap();
    answer(&editor, &["x"]);
    press(&ev, &editor, "! 〆切");
    assert_eq!(
        editor.buffer().file.unwrap(),
        dir.path().join("r.txt").to_string_lossy()
    );
    assert_eq!(
        editor.current_line(),
        format!("[{}]. cancel [{}]:! 〆切", day(0), day(1))
    );
    // 何もないとき
    fs::remove_file(dir.path().join("r.txt")).unwrap();
    ev.run_command("howm-list-schedule", &editor).unwrap();
    assert_eq!(editor.message(), "No schedule");
    ev.run_command("howm-list-todo", &editor).unwrap();
    assert_eq!(editor.message(), "No todo");
}

#[test]
fn test_reminder_action() {
    let (ev, editor, dir) = setup();
    let memo = write_memo(
        dir.path(),
        "r.txt",
        "= r\n[2026-10-20]+7 買い物\n[2026-10-21]! 〆切\n[2026-10-22]- 覚書\n[2026-10-23]@ 予定\n",
    );
    open(&ev, &editor, &memo);
    let today = day(0);
    // 済み (RET)
    answer(&editor, &[""]);
    press(&ev, &editor, "+7 買い物");
    assert_eq!(
        editor.current_line(),
        format!("[{}]. [2026-10-20]:+7 買い物", today)
    );
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "RET (done), x (cancel), symbol (type), num (laziness): "
    );
    // 種類と猶予を変える
    answer(&editor, &["~", "30", "!3"]);
    press(&ev, &editor, "- 覚書");
    assert_eq!(editor.current_line(), "[2026-10-22]~ 覚書");
    press(&ev, &editor, "~ 覚書");
    assert_eq!(editor.current_line(), "[2026-10-22]~30 覚書");
    press(&ev, &editor, "~30 覚書");
    assert_eq!(editor.current_line(), "[2026-10-22]!3 覚書");
    // 読めない入力
    answer(&editor, &["zz"]);
    press(&ev, &editor, "@ 予定");
    assert_eq!(editor.message(), "Can't understand zz.");
    assert_eq!(editor.current_line(), "[2026-10-23]@ 予定");
}

#[test]
fn test_date_action() {
    let (ev, editor, dir) = setup();
    let memo = write_memo(dir.path(), "d.txt", "= 日付\n[2026-10-25] 会議\n");
    open(&ev, &editor, &memo);
    // RET: その日付のメモの一覧
    answer(&editor, &[""]);
    press(&ev, &editor, "[2026-10-25]");
    assert_eq!(editor.text(), "d | [2026-10-25] 会議");
    assert_eq!(
        editor.prompts.lock().unwrap().last().unwrap(),
        "2026-10-25 [日] RET(list), +num(shift), yymmdd(set), ~yymmdd(repeat), .(today), [(-1), ](+1): "
    );
    open(&ev, &editor, &memo);
    let cases = [
        ("+3", "2026-10-28"),
        ("-1", "2026-10-27"),
        ("1101", "2026-11-01"),
        ("261231", "2026-12-31"),
        ("5", "2026-12-05"),
        ("20270102", "2027-01-02"),
        ("120270131", "2027-01-31"),
    ];
    for (input, expected) in cases {
        answer(&editor, &[input]);
        press(&ev, &editor, "[20");
        assert_eq!(
            editor.current_line(),
            format!("[{}] 会議", expected),
            "{}",
            input
        );
        // カーソルは動かさない
        assert_eq!(editor.buffer().point, 6);
    }
    answer(&editor, &["."]);
    press(&ev, &editor, "[20");
    assert_eq!(editor.current_line(), format!("[{}] 会議", day(0)));
    // [ と ] で 1 日ずつ. . は今日. ほかで終わり
    answer(&editor, &["20270102", "[", "]", "]", ".", ""]);
    press(&ev, &editor, "[20");
    press(&ev, &editor, "[20");
    assert_eq!(editor.current_line(), format!("[{}] 会議", day(0)));
    let prompts = editor.prompts.lock().unwrap().clone();
    assert_eq!(
        prompts[prompts.len() - 2],
        "2027-01-03 [日] RET(ok), [(-1), ](+1), .(today): "
    );
    answer(&editor, &["]", ""]);
    press(&ev, &editor, "[20");
    assert_eq!(editor.current_line(), format!("[{}] 会議", day(1)));
    // 読めない入力
    answer(&editor, &["zzz"]);
    press(&ev, &editor, "[20");
    assert_eq!(editor.message(), "Can't understand zzz.");
}

#[test]
fn test_date_repeat() {
    let (ev, editor, dir) = setup();
    let cases: [(&str, &str, &str, &[&str]); 5] = [
        (
            "2027-01-03",
            "~20270108",
            "",
            &["04", "05", "06", "07", "08"],
        ),
        ("2027-01-03", "~0120", "w", &["10", "17"]),
        ("2027-01-03", "~09", "3", &["06", "09"]),
        ("2027-01-31", "~0430", "m", &["02-28", "03-28", "04-28"]),
        (
            "2024-02-29",
            "~270301",
            "y",
            &["2025-02-28", "2026-02-28", "2027-02-28"],
        ),
    ];
    for (date, until, every, expected) in cases {
        let memo = write_memo(
            dir.path(),
            "d.txt",
            &format!("= 繰り返し\n- [{}] 会議\n", date),
        );
        open(&ev, &editor, &memo);
        answer(&editor, &[until, every]);
        press(&ev, &editor, "[20");
        let lines: Vec<String> = editor.text().lines().skip(2).map(str::to_string).collect();
        let wanted: Vec<String> = expected
            .iter()
            .map(|d| {
                let full = match d.len() {
                    2 => format!("{}{}", &date[..8], d),
                    5 => format!("{}{}", &date[..5], d),
                    _ => d.to_string(),
                };
                format!("- [{}] 会議", full)
            })
            .collect();
        assert_eq!(lines, wanted, "{} {}", until, every);
        assert_eq!(editor.message(), format!("{} lines", expected.len()));
        assert_eq!(editor.current_line(), format!("- [{}] 会議", date));
        ev.eval_with("(kill-buffer)", &editor).unwrap_or_default();
        editor
            .buffers
            .lock()
            .unwrap()
            .retain(|_, b| b.file.is_none());
    }
    // 行の数の上限
    ev.eval_with("(set! howm-date-repeat-max 2)", &editor)
        .unwrap();
    let memo = write_memo(dir.path(), "d.txt", "[2027-01-03]\n");
    open(&ev, &editor, &memo);
    answer(&editor, &["~1231", ""]);
    press(&ev, &editor, "[20");
    assert_eq!(editor.text(), "[2027-01-03]\n[2027-01-04]\n[2027-01-05]\n");
}

#[test]
fn test_insert_date_and_time() {
    let (ev, editor, _dir) = setup();
    answer(&editor, &["", "+1"]);
    ev.run_command("howm-insert-date", &editor).unwrap();
    assert_eq!(editor.text(), format!("[{}]", day(0)));
    assert!(editor.prompts.lock().unwrap()[0].contains(" RET(ok), +num(shift)"));
    ev.run_command("howm-insert-date", &editor).unwrap();
    assert_eq!(editor.text(), format!("[{}][{}]", day(0), day(1)));
    // カーソルは, 入れた日付の後
    assert_eq!(editor.buffer().point, 25);
    ev.eval_with("(delete-region 1 (point-max))", &editor)
        .unwrap();
    ev.run_command("howm-insert-dtime", &editor).unwrap();
    assert!(editor.text().starts_with(&format!("[{} ", day(0))));
    assert_eq!(editor.text().chars().count(), 18);
}

#[test]
fn test_remember() {
    let (ev, editor, dir) = setup();
    ev.run_command("howm-remember", &editor).unwrap();
    assert_eq!(editor.name(), "*howm-remember*");
    assert_eq!(
        editor.key("Ctrl-c Ctrl-c").as_deref(),
        Some("howm-remember-submit")
    );
    assert_eq!(
        editor.key("Ctrl-c Ctrl-k").as_deref(),
        Some("howm-remember-discard")
    );
    // 空なら, 保存しない
    ev.run_command("howm-remember-submit", &editor).unwrap();
    assert_eq!(editor.message(), "Empty memo");
    assert_eq!(editor.name(), "*howm-remember*");
    // 速記のバッファで, もう一度呼んでも, 戻る所は変わらない
    ev.run_command("howm-remember", &editor).unwrap();
    ev.eval_with("(insert \"\\n題名\\n本文\\n\")", &editor)
        .unwrap();
    ev.run_command("howm-remember-submit", &editor).unwrap();
    assert_eq!(editor.name(), "*scratch*");
    assert!(!editor
        .buffers
        .lock()
        .unwrap()
        .contains_key("*howm-remember*"));
    let file = editor
        .message()
        .strip_prefix("Saved: ")
        .unwrap()
        .to_string();
    let text = fs::read_to_string(&file).unwrap();
    assert!(file.starts_with(&*dir.path().to_string_lossy()));
    assert!(
        text.starts_with(&format!("= 題名\n[{} ", day(0))),
        "{}",
        text
    );
    assert!(text.ends_with("]\n\n本文\n"), "{}", text);
    // 取り消す
    ev.run_command("howm-remember", &editor).unwrap();
    ev.eval_with("(insert \"捨てる\")", &editor).unwrap();
    ev.run_command("howm-remember-discard", &editor).unwrap();
    assert_eq!(editor.name(), "*scratch*");
    assert_eq!(editor.message(), "Discarded");
    assert!(!editor
        .buffers
        .lock()
        .unwrap()
        .contains_key("*howm-remember*"));
}

#[test]
fn test_dup_and_kill_ring() {
    let (ev, editor, dir) = setup();
    let memo = write_memo(dir.path(), "m.txt", "前書き\n= A\n本文A\n= B\n本文B\n\n");
    open(&ev, &editor, &memo);
    // 題を kill ring へ
    editor.move_to("本文B");
    ev.run_command("howm-title-to-kill-ring", &editor).unwrap();
    editor.move_to("本文A");
    ev.run_command("howm-title-to-kill-ring", &editor).unwrap();
    // 題の無い所は, ファイルの名前
    editor.move_to("前書き");
    ev.run_command("howm-title-to-kill-ring", &editor).unwrap();
    ev.run_command("howm-name-to-kill-ring", &editor).unwrap();
    // ファイルの無いバッファは, バッファの名前
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &editor)
        .unwrap();
    ev.run_command("howm-title-to-kill-ring", &editor).unwrap();
    ev.run_command("howm-name-to-kill-ring", &editor).unwrap();
    assert_eq!(
        *editor.kill_ring.lock().unwrap(),
        vec![
            "B".to_string(),
            "A".to_string(),
            "m".to_string(),
            memo.to_string_lossy().into_owned(),
            "*scratch*".to_string(),
            "*scratch*".to_string(),
        ]
    );
    assert_eq!(editor.message(), "*scratch*");
    // 複製: カーソルのメモ (題の行から, 次の題の行の前まで)
    open(&ev, &editor, &memo);
    editor.move_to("本文B");
    ev.run_command("howm-dup", &editor).unwrap();
    let copy = editor.buffer().file.unwrap();
    assert_ne!(copy, memo.to_string_lossy());
    assert_eq!(fs::read_to_string(&copy).unwrap(), "= B\n本文B\n");
    assert_eq!(editor.text(), "= B\n本文B\n");
    open(&ev, &editor, &memo);
    editor.move_to("本文A");
    ev.run_command("howm-dup", &editor).unwrap();
    assert_eq!(editor.text(), "= A\n本文A\n");
}

#[test]
fn test_buffers() {
    let (ev, editor, dir) = setup();
    let a = write_memo(dir.path(), "a.txt", "= a\n");
    let b = write_memo(dir.path(), "b.txt", "= b\n");
    let elsewhere = tempdir().unwrap();
    let other = elsewhere.path().join("other.txt");
    fs::write(&other, "other").unwrap();
    open(&ev, &editor, &a);
    open(&ev, &editor, &b);
    ev.eval_with("(insert \"書き換え\")", &editor).unwrap();
    ev.eval_with(&format!("(find-file {:?})", other), &editor)
        .unwrap();
    ev.run_command("howm-list-all", &editor).unwrap();
    // howm のバッファを消す (書き換えたメモと, ほかのファイルは残す)
    ev.run_command("howm-kill-all", &editor).unwrap();
    assert_eq!(editor.message(), "2 buffers killed");
    let mut names: Vec<String> = editor.buffers.lock().unwrap().keys().cloned().collect();
    names.sort();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"*scratch*".to_string()));
    assert!(names.contains(&b.to_string_lossy().into_owned()));
    // バッファの一覧
    ev.run_command("howm-list-buffers", &editor).unwrap();
    assert_eq!(editor.message(), "list-buffers");
    // 下線を引き直す
    open(&ev, &editor, &a);
    ev.eval_with("(unhighlight-regexp #t)", &editor).unwrap();
    ev.run_command("howm-refresh", &editor).unwrap();
    assert_eq!(editor.message(), "Refreshed");
    assert_eq!(editor.buffer().highlights.len(), 5);
}

#[test]
fn test_occur_around_and_walk() {
    let (ev, editor, dir) = setup();
    // メモが無いときの酔歩
    ev.run_command("howm-random-walk", &editor).unwrap();
    assert_eq!(editor.name(), "*scratch*");
    let a = write_memo(dir.path(), "a.txt", "= a\n本文1\n本文2\n>>> b\n");
    let b = write_memo(dir.path(), "b.txt", "= b\n<<< b\n");
    write_memo(dir.path(), "c.txt", "= c\n");
    // バッファの中を探す
    ev.run_command("howm-occur", &editor).unwrap();
    assert_eq!(editor.message(), "Not a file buffer");
    open(&ev, &editor, &a);
    answer(&editor, &["本文", "", "zzz"]);
    ev.run_command("howm-occur", &editor).unwrap();
    assert_eq!(editor.text(), "a | 本文1\na | 本文2");
    open(&ev, &editor, &a);
    ev.run_command("howm-occur", &editor).unwrap();
    assert_eq!(editor.name(), a.to_string_lossy());
    ev.run_command("howm-occur", &editor).unwrap();
    assert_eq!(editor.message(), "No match: zzz");
    // 前後: 全てのメモの一覧で, 今のメモの行に置く
    open(&ev, &editor, &b);
    ev.run_command("howm-list-around", &editor).unwrap();
    assert_eq!(editor.text(), "c | c\nb | b\na | a");
    assert_eq!(editor.current_line(), "b | b");
    assert_eq!(
        editor.displayed.lock().unwrap().last().cloned(),
        Some((b.to_string_lossy().into_owned(), Some(1)))
    );
    // メモでないバッファでは, 先頭
    ev.eval_with("(switch-to-buffer \"*scratch*\")", &editor)
        .unwrap();
    ev.run_command("howm-list-around", &editor).unwrap();
    assert_eq!(editor.current_line(), "c | c");
    // 酔歩: メモなら, リンクを探す. 一覧なら, ランダムな行を開く. リンクが無ければ, ランダムなメモ
    open(&ev, &editor, &a);
    ev.run_command("howm-random-walk", &editor).unwrap();
    assert_eq!(editor.name(), "*howmS*");
    assert_eq!(editor.text(), "b | <<< b\nb | = b\na | >>> b");
    assert_eq!(editor.message(), "Random walk: C-c , w で次へ");
    ev.run_command("howm-random-walk", &editor).unwrap();
    assert!(editor.buffer().file.is_some());
    write_memo(dir.path(), "d.txt", "リンクなし\n");
    open(&ev, &editor, &dir.path().join("d.txt"));
    ev.eval_with("(delete-region 1 (point-max))", &editor)
        .unwrap();
    ev.run_command("howm-random-walk", &editor).unwrap();
    assert!(editor.buffer().file.is_some());
}

#[test]
fn test_migemo() {
    let (ev, editor, dir) = setup();
    write_memo(dir.path(), "a.txt", "= かな\nしんぶんとコンニチハ\nkitte\n");
    for (roma, kana) in [
        ("kitte", "きって"),
        ("konnnichiha", "こんにちは"),
        ("shinbun", "しんぶん"),
        ("kyou-n", "きょうーん"),
        ("x1", "x1"),
    ] {
        assert_eq!(
            ev.eval_with(&format!("(howm-roma-to-hiragana {:?})", roma), &editor)
                .unwrap(),
            format!("{:?}", kana)
        );
    }
    assert_eq!(
        ev.eval_with("(howm-hiragana-to-katakana \"きょうはa\")", &editor)
            .unwrap(),
        "\"キョウハa\""
    );
    answer(&editor, &["shinbun", "konnnichiha", "KITTE", ""]);
    ev.run_command("howm-search-migemo", &editor).unwrap();
    assert_eq!(editor.text(), "a | しんぶんとコンニチハ");
    ev.run_command("howm-search-migemo", &editor).unwrap();
    assert_eq!(editor.text(), "a | しんぶんとコンニチハ");
    ev.run_command("howm-search-migemo", &editor).unwrap();
    assert_eq!(editor.text(), "a | kitte");
    ev.run_command("howm-search-migemo", &editor).unwrap();
    assert_eq!(editor.text(), "a | kitte");
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

#[test]
fn test_find_file_and_after_save_hooks() {
    let (ev, editor, dir) = setup();
    // howm を読むと, エディタが実行するフックに入る
    assert_eq!(ev.active_hooks(), vec!["find-file-hook", "after-save-hook"]);
    let memo = write_memo(
        dir.path(),
        "2026/10/2026-10-25-120000.txt",
        "= a\n<<< alpha\n",
    );
    let other = tempdir().unwrap();
    let text = write_memo(other.path(), "x.txt", "<<< alpha\n");
    let png = write_memo(dir.path(), "2026/10/a.png", "");
    // C-x C-f で開いたメモは, メモのバッファにする (メモでなければ, 何もしない)
    for (path, howm) in [(&memo, true), (&text, false), (&png, false)] {
        ev.eval_with(
            &format!("(find-file {:?}) (run-hooks 'find-file-hook)", path),
            &editor,
        )
        .unwrap();
        assert_eq!(editor.key("Enter").is_some(), howm, "{:?}", path);
    }
    ev.eval_with("(set! howm-set-mode-on-find-file #f)", &editor)
        .unwrap();
    let memo2 = write_memo(dir.path(), "2026/10/2026-10-25-130000.txt", "= b\n");
    ev.eval_with(
        &format!("(find-file {:?}) (run-hooks 'find-file-hook)", memo2),
        &editor,
    )
    .unwrap();
    assert_eq!(editor.key("Enter"), None);
    // 保存したら, キーワードを集め直して下線を引き直し, メニューを出し直して, 戻る
    ev.eval_with(
        "(howm-menu) (define saved 0) (add-hook 'howm-after-save-hook (lambda () (set! saved (+ saved 1))))",
        &editor,
    )
    .unwrap();
    open(&ev, &editor, &memo);
    let name = editor.name();
    write_memo(
        dir.path(),
        "2026/10/2026-10-25-130000.txt",
        "= b\n<<< beta\n",
    );
    // メニューは, 前のバッファ (メモ) を覚えている
    ev.eval_with("(set! howm-menu-previous-buffer \"prev\")", &editor)
        .unwrap();
    ev.eval_with("(run-hooks 'after-save-hook)", &editor)
        .unwrap();
    assert_eq!(editor.name(), name);
    assert!(editor
        .buffer()
        .highlights
        .iter()
        .any(|r| r.contains("beta")));
    assert_eq!(
        ev.eval_with("(list saved howm-menu-previous-buffer)", &editor)
            .unwrap(),
        "(1 \"prev\")"
    );
    // メモでないバッファ, 引き直さない設定, メニューを出していないとき
    ev.eval_with(
        "(switch-to-buffer \"*scratch*\") (run-hooks 'after-save-hook)",
        &editor,
    )
    .unwrap();
    ev.eval_with(
        "(set! howm-refresh-after-save #f) (set! howm-menu-refresh-after-save #f) (kill-buffer howm-menu-name)",
        &editor,
    )
    .unwrap();
    open(&ev, &editor, &memo);
    ev.eval_with(
        "(unhighlight-regexp #t) (run-hooks 'after-save-hook) (howm-menu-refresh-background)",
        &editor,
    )
    .unwrap();
    assert!(editor.buffer().highlights.is_empty());
    assert_eq!(editor.name(), name);
    assert_eq!(ev.eval_with("saved", &editor).unwrap(), "2");
}
