//! ターミナルのエディタ (`app`) のテスト: キーを送って、本文、カーソル、画面、エコー領域を確かめる.

mod common;
use common::Fixture;

use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
#[test]
fn test_typing_and_basic_movement() {
    let mut fx = Fixture::new();
    fx.press("h e l l o SPC w o r l d");
    assert_eq!(fx.text(), "hello world");
    fx.press("C-a");
    assert_eq!(fx.point(), 0);
    fx.press("C-e");
    assert_eq!(fx.point(), 11);
    fx.press("C-b C-b <left>");
    assert_eq!(fx.point(), 8);
    fx.press("C-f <right>");
    assert_eq!(fx.point(), 10);
    fx.press("DEL");
    assert_eq!(fx.text(), "hello word");
    fx.press("C-d");
    assert_eq!(fx.text(), "hello wor");
    fx.press("<home> <delete>");
    assert_eq!(fx.text(), "ello wor");
    fx.press("<end> RET x");
    assert_eq!(fx.text(), "ello wor\nx");
    fx.press("TAB");
    assert_eq!(fx.text(), "ello wor\nx    ");
    // 画面: 本文とモードライン
    fx.app.draw();
    let screen = fx.screen();
    assert_eq!(screen[0], "ello wor");
    assert!(screen[1].starts_with("x"));
    assert!(screen[10].starts_with("-:**-  *scratch*   L2"));
}

#[test]
fn test_errors_are_shown_in_the_echo_area() {
    let mut fx = Fixture::new();
    fx.press("C-b");
    assert_eq!(fx.app.message(), "Beginning of buffer");
    fx.press("C-f");
    assert_eq!(fx.app.message(), "End of buffer");
    fx.press("C-n");
    assert_eq!(fx.app.message(), "End of buffer");
    fx.press("C-p");
    assert_eq!(fx.app.message(), "Beginning of buffer");
    fx.press("C-k");
    assert_eq!(fx.app.message(), "End of buffer");
    fx.press("C-d");
    assert_eq!(fx.app.message(), "End of buffer");
    fx.press("C-g");
    assert_eq!(fx.app.message(), "Quit");
    fx.press("M-y");
    assert_eq!(fx.app.message(), "Kill ring is empty");
    fx.press("C-w");
    assert_eq!(
        fx.app.message(),
        "The mark is not set now, so there is no region"
    );
    fx.press("C-x C-x");
    assert_eq!(fx.app.message(), "No mark set in this buffer");
    fx.press("C-/");
    assert_eq!(fx.app.message(), "No further undo information");
}

#[test]
fn test_lines_and_the_goal_column() {
    let mut fx = Fixture::new();
    fx.press("a b c d e RET a RET a b c");
    assert_eq!(fx.text(), "abcde\na\nabc");
    // 短い行を通っても、元の桁に戻る
    fx.press("C-p C-p");
    assert_eq!(fx.point(), 3);
    fx.press("C-n");
    assert_eq!(fx.point(), 7);
    fx.press("C-n");
    assert_eq!(fx.point(), 11);
    fx.press("C-p");
    fx.press("C-p");
    assert_eq!(fx.point(), 3);
    fx.press("<down> <up>");
    assert_eq!(fx.point(), 3);
    // 行の先頭と終わり
    fx.press("M-> M-<");
    assert_eq!((fx.point(), fx.app.message().as_str()), (0, "Mark set"));
    fx.press("M->");
    assert_eq!(fx.point(), 11);
}

#[test]
fn test_words() {
    let mut fx = Fixture::new();
    fx.press("o n e SPC t w o SPC SPC t h r e e");
    fx.press("M-b");
    assert_eq!(fx.point(), 9);
    fx.press("M-b M-b");
    assert_eq!(fx.point(), 0);
    fx.press("M-b");
    assert_eq!(fx.point(), 0);
    fx.press("M-f");
    assert_eq!(fx.point(), 3);
    fx.press("M-f M-f M-f");
    assert_eq!(fx.point(), 14);
    // 単語を消す
    fx.press("M-b M-d");
    assert_eq!(fx.text(), "one two  ");
    fx.press("M-DEL");
    assert_eq!(fx.text(), "one ");
}

#[test]
fn test_kill_yank_and_undo() {
    let mut fx = Fixture::new();
    fx.press("a b c RET d e f");
    fx.press("C-p C-a C-k");
    assert_eq!(fx.text(), "\ndef");
    // 続けて殺すと、1 つにまとまる
    fx.press("C-k");
    assert_eq!(fx.text(), "def");
    fx.press("C-y");
    assert_eq!(fx.text(), "abc\ndef");
    assert_eq!(fx.point(), 4);
    // 殺して、文字を入れ、貼る。yank-pop で前のものに替える
    fx.press("C-a C-k");
    assert_eq!(fx.text(), "abc\n");
    fx.press("x y");
    assert_eq!(fx.text(), "abc\nxy");
    fx.press("C-y");
    assert_eq!(fx.text(), "abc\nxydef");
    fx.press("M-y");
    assert_eq!(fx.text(), "abc\nxyabc\n");
    // undo を続けると、1 つずつ戻る
    fx.press("C-/");
    assert_eq!(fx.text(), "abc\nxydef");
    fx.press("C-/");
    assert_eq!(fx.text(), "abc\nxy");
    fx.press("C-/ C-/");
    assert_eq!(fx.text(), "abc\n");
    assert_eq!(fx.app.message(), "Undo");
    // 別のコマンドを挟むと、最初から (戻したことも、戻せる)
    fx.press("C-a C-/");
    assert_eq!(fx.text(), "abc\nx");
}

#[test]
fn test_yank_pop_and_regions() {
    let mut fx = Fixture::new();
    fx.press("a b c SPC d e f");
    fx.press("C-SPC");
    assert_eq!(fx.app.message(), "Mark set");
    fx.press("M-b");
    fx.press("C-w");
    assert_eq!(fx.text(), "abc ");
    fx.press("C-SPC M-b M-w");
    fx.press("C-e C-y");
    assert_eq!(fx.text(), "abc abc ");
    fx.press("M-y");
    assert_eq!(fx.text(), "abc def");
    // 点とマークを入れ替える. バッファ全体を選ぶ
    fx.press("C-x C-x");
    assert!(fx.app.host().editor().current_buffer().mark().is_some());
    fx.press("C-x h");
    assert_eq!(fx.point(), 0);
    assert_eq!(fx.app.host().editor().current_buffer().mark(), Some(7));
    fx.press("C-@");
    assert_eq!(fx.app.message(), "Mark set");
}

#[test]
fn test_prefix_arguments() {
    let mut fx = Fixture::new();
    fx.press("a b c d e f g h i j");
    fx.press("C-a");
    // C-u は 4, C-u C-u は 16, C-u に数
    fx.press("C-u C-f");
    assert_eq!(fx.point(), 4);
    fx.press("C-u C-u C-b");
    assert_eq!(fx.point(), 0);
    fx.press("C-u 5 C-f");
    assert_eq!(fx.point(), 5);
    fx.press("M-2 C-f");
    assert_eq!(fx.point(), 7);
    fx.press("C-u 1 2 C-b");
    assert_eq!(fx.point(), 0);
    // 負の数
    fx.press("C-u - 3 C-f");
    assert_eq!(fx.point(), 0);
    fx.press("C-u - C-b");
    assert_eq!(fx.point(), 1);
    fx.press("M-- 2 C-f");
    assert_eq!(fx.point(), 0);
    fx.press("C-- 1 C-f");
    assert_eq!(fx.point(), 0);
    // 前置引数のあいだに出すもの
    fx.press("C-u");
    assert_eq!(fx.app.message(), "C-u 4-");
    assert_eq!(fx.app.eval("prefix-arg").unwrap(), "(4)");
    fx.press("-");
    assert_eq!(fx.app.message(), "C-u -");
    fx.press("7");
    assert_eq!(fx.app.message(), "C-u -7-");
    // 文字を前置引数の数だけ入れる
    fx.press("C-g");
    fx.press("C-u 3 z");
    assert_eq!(fx.text(), "zzzabcdefghij");
    // 前置引数の後に C-u を続けて押しなおす
    fx.press("C-u 2 C-u C-f");
    assert_eq!(fx.app.eval("prefix-arg").unwrap(), "#f");
    assert_eq!(fx.app.host().editor().prefix_arg, None);
    // 数字は前置引数が無ければ、文字
    fx.press("5");
    assert_eq!(fx.text().chars().nth(fx.point() - 1), Some('5'));
}

#[test]
fn test_unbound_keys_and_prefix_keys() {
    let mut fx = Fixture::new();
    fx.press("C-x");
    assert_eq!(fx.app.message(), "C-x-");
    fx.press("C-g");
    assert_eq!(fx.app.message(), "Quit");
    fx.press("C-x C-z");
    assert_eq!(fx.app.message(), "C-x C-z is undefined");
    fx.press("C-]");
    assert_eq!(fx.app.message(), "No recursive edit is in progress");
    fx.press("<f5>");
    assert_eq!(fx.app.message(), "<f5> is undefined");
    // 日本語も入れられる
    fx.press("あ い");
    assert_eq!(fx.text(), "あい");
}

#[test]
fn test_lisp_key_bindings_and_eval() {
    let mut fx = Fixture::new();
    fx.app
        .eval("(define-command (insert-greeting) (insert \"hello\")) (global-set-key \"C-c g\" 'insert-greeting)")
        .unwrap();
    fx.press("C-c g");
    assert_eq!(fx.text(), "hello");
    assert_eq!(fx.app.eval("(+ 1 2)").unwrap(), "3");
    assert!(fx.app.eval("(car 1)").is_err());
    // this-command を数にするコマンドでも、前のコマンドは名前になる
    fx.app
        .eval("(define-command (weird) (set! this-command 5))")
        .unwrap();
    fx.feed("w e i r d RET");
    fx.press("M-x");
    assert_eq!(
        fx.app.eval("last-command").unwrap(),
        "execute-extended-command"
    );
    // eval-expression
    fx.feed("( + SPC 1 SPC 2 ) RET");
    fx.press("M-:");
    assert_eq!(fx.app.message(), "3");
    // M-x の履歴を, 直して動かす
    fx.press("a");
    fx.feed("b a c k w a r d - c h a r RET");
    fx.press("M-x");
    fx.feed("RET");
    fx.press("C-x M-:");
    assert_eq!(
        fx.app.eval("command-history").unwrap(),
        "(\"(backward-char)\" \"(weird)\")"
    );
    fx.feed("b a c k w a r d - c h a r RET");
    fx.press("M-X");
    // 履歴が空なら, 失敗
    fx.app.eval("(set! command-history '())").unwrap();
    fx.press("C-x M-:");
    assert_eq!(
        fx.app.message(),
        "There are no previous complex commands to repeat"
    );
}

#[test]
fn test_local_keys_come_before_the_global_keys() {
    let mut fx = Fixture::new();
    fx.app
        .eval("(define-command (insert-local) (insert \"local\")) (local-set-key \"C-c l\" 'insert-local) (local-set-key \"C-f\" 'insert-local)")
        .unwrap();
    fx.press("C-c l");
    assert_eq!(fx.text(), "local");
    // バッファだけのキーが, 全体のキーより先
    fx.press("C-f");
    assert_eq!(fx.text(), "locallocal");
    // ほかのバッファでは, 全体のキー
    fx.app.eval("(switch-to-buffer \"other\")").unwrap();
    fx.press("a C-b C-f");
    assert_eq!(fx.text(), "a");
}

#[test]
fn test_run_loop_and_quit() {
    let mut fx = Fixture::new();
    fx.feed("a b C-x C-c");
    fx.app.run();
    assert_eq!(fx.text(), "ab");
    // 変更したが、ファイルのバッファでないので、聞かずに終わる
    assert!(fx.app.quit());
    // 入力が終わっても終わる
    let mut fx = Fixture::new();
    fx.feed("a");
    fx.app.run();
    assert!(!fx.app.quit());
    assert_eq!(fx.text(), "a");
    assert!(fx.frames.lock().unwrap().len() >= 2);
}

#[test]
fn test_opening_files_from_the_arguments() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    std::fs::write(&a, "from a").unwrap();
    std::fs::write(&b, "from b").unwrap();
    let files = [
        a.to_str().unwrap().to_string(),
        b.to_str().unwrap().to_string(),
    ];
    let fx = Fixture::sized(12, 30, &files).unwrap();
    // 最後のファイルを出す
    assert_eq!(fx.text(), "from b");
    assert_eq!(
        fx.app.host().editor().buffer_list(),
        vec!["*scratch*", "a.txt", "b.txt"]
    );
    // 開けないものは、始められない
    let bad = dir.path().join("bad.bin");
    std::fs::write(&bad, [0xff, 0xfe]).unwrap();
    let bad_arg = [bad.to_str().unwrap().to_string()];
    assert!(Fixture::sized(12, 30, &bad_arg).is_err());
    // ディレクトリは、dired で開く
    let dir_arg = [dir.path().to_str().unwrap().to_string()];
    assert!(Fixture::sized(12, 30, &dir_arg).is_ok());
}

#[test]
fn test_editing_keys_of_step_3() {
    let mut fx = Fixture::new();
    fx.press("a b c d");
    // C-t: 文字を入れ替える. M-t: 単語を入れ替える
    fx.press("C-t");
    assert_eq!(fx.text(), "abdc");
    fx.press("SPC e f SPC g h M-b M-t");
    assert_eq!(fx.text(), "abdc gh ef");
    // C-o: 改行を入れて、カーソルは前. C-M-o: 行を分ける
    let mut fx = Fixture::new();
    fx.press("a b c d");
    fx.press("C-b C-b C-o");
    assert_eq!((fx.text().as_str(), fx.point()), ("ab\ncd", 2));
    fx.press("C-n C-e C-b C-M-o");
    // 新しい行は, カーソルの桁まで字下げする
    assert_eq!(fx.text(), "ab\nc\n d");
}

#[test]
fn test_mark_defun_takes_the_prefix_argument() {
    // "(a)\n\n(b)\n\n(c)": (b) は 5 から
    let keys = "( a ) RET RET ( b ) RET RET ( c ) C-p C-p C-p C-e";
    // 引数なし: 今の defun (前の空の行を含む) の始めがカーソル
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-M-h");
    assert_eq!(fx.point(), 4);
    // 2: 今の defun から 2 つ
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-u 2 C-M-h");
    assert_eq!(fx.point(), 4);
    // 負の数: 向きを変える. 今の defun の終わりをマークにする
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-u - C-M-h C-x C-x");
    assert_eq!(fx.point(), 9);
    // -2: さらに 1 つ前の defun の始めまで
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-u - 2 C-M-h");
    assert_eq!(fx.point(), 0);
    // 0: 今の defun を選ぶが, 後ろの defun の終わりは延ばさない
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-u 0 C-M-h");
    assert_eq!(fx.point(), 4);
    // リージョンがあれば, 負の数でカーソルを前の defun の始めへ動かす
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-M-h C-u - C-M-h");
    assert_eq!(fx.point(), 0);
    let mut fx = Fixture::new();
    fx.press(keys);
    fx.press("C-M-h C-u - 2 C-M-h");
    assert_eq!(fx.point(), 0);
    // リージョンがあれば, 正の数でマークを defun の終わりへ延ばす
    let mut fx = Fixture::new();
    fx.press("( a ) RET RET ( b ) RET RET ( c ) RET C-p C-p C-p C-p C-e");
    fx.press("C-M-h C-u 2 C-M-h C-x C-x");
    assert_eq!(fx.point(), 14);
}

#[test]
fn test_whitespace_keys_take_the_prefix_argument() {
    // C-u 3 M-SPC: 空白を 3 つ残す
    let mut fx = Fixture::new();
    fx.press("a SPC SPC SPC SPC SPC b C-b C-b C-b");
    fx.press("C-u 3 M-SPC");
    assert_eq!(fx.text(), "a   b");
    // 負の数なら, 改行も消して -N 個残す
    let mut fx = Fixture::new();
    fx.press("a SPC RET RET SPC b C-b C-b");
    fx.press("C-u - 2 M-SPC");
    assert_eq!(fx.text(), "a  b");
    // C-u M-\: カーソルの前の空白だけを消す
    let mut fx = Fixture::new();
    fx.press("a SPC SPC SPC b C-b C-b");
    fx.press("C-u M-\\");
    assert_eq!((fx.text().as_str(), fx.point()), ("a b", 1));
    // C-u M-^: 次の行とつなぐ
    let mut fx = Fixture::new();
    fx.press("a RET SPC SPC b RET c C-p C-p C-u M-^");
    assert_eq!(fx.text(), "a b\n  c");
    // リージョンがあれば, リージョンの行を全てつなぐ
    let mut fx = Fixture::new();
    fx.press("a RET SPC b RET SPC c RET d");
    fx.press("M-< C-SPC C-n C-n C-e M-^");
    assert_eq!(fx.text(), "a b c\n  d");
}

#[test]
fn test_whitespace_and_zap_keys() {
    let mut fx = Fixture::new();
    fx.press("a SPC SPC SPC b");
    fx.press("C-b M-\\");
    assert_eq!(fx.text(), "ab");
    fx.press("M-SPC");
    // 空白が無ければ, 空白を 1 つ入れる
    assert_eq!(fx.text(), "a b");
    // M-z: 文字まで殺す (問いは, キーで答える)
    let mut fx = Fixture::new();
    fx.press("a b c d e f");
    fx.press("C-a");
    fx.feed("d");
    fx.press("M-z");
    assert_eq!(fx.text(), "ef");
    // C-x C-o: 空行を 1 つにする. M-^: 前の行とつなぐ
    let mut fx = Fixture::new();
    fx.press("a RET RET RET b");
    fx.press("C-p C-x C-o");
    assert_eq!(fx.text(), "a\n\nb");
    fx.press("C-n M-^");
    assert_eq!(fx.text(), "a\nb");
}

#[test]
fn test_kill_append_buffer_insert_and_redo_keys() {
    let mut fx = Fixture::new();
    fx.press("a b c SPC d e f");
    fx.press("C-a M-d C-M-w M-d");
    fx.press("C-y");
    // 続けて 2 つ殺して、1 つにまとめる (C-M-w)
    assert_eq!(fx.text(), "abc def");
    // undo-redo (C-M-_): 元に戻したことをやり直す
    fx.press("C-/");
    let undone = fx.text();
    fx.press("C-M-_");
    assert_ne!(fx.text(), undone);
    // C-q: 次のキーをそのまま入れる
    let mut fx = Fixture::new();
    fx.feed("TAB");
    fx.press("C-q");
    assert_eq!(fx.text(), "\t");
    // C-x x i: 別のバッファを入れる (名前は問い)
    let mut fx = Fixture::new();
    fx.app
        .host()
        .editor()
        .get_buffer_create("other")
        .insert("OTHER");
    fx.feed("o t h e r RET");
    fx.press("C-x x i");
    assert_eq!(fx.text(), "OTHER");
    // M-j: 改行して、空白を整える
    let mut fx = Fixture::new();
    fx.press("a SPC SPC b C-b C-b M-j");
    assert_eq!(fx.text(), "a\nb");
}

#[test]
fn test_mark_is_deactivated_at_the_end_of_the_command() {
    let mut fx = Fixture::new();
    fx.press("a b c d e C-SPC C-a");
    assert!(fx.app.host().editor().current_buffer().mark_active());
    // 消すコマンドは, マークを使わない状態にする
    fx.press("C-w");
    assert!(!fx.app.host().editor().current_buffer().mark_active());
    // C-u 4 の後の - は, 数の後なら文字
    fx.press("C-u 3 -");
    assert_eq!(fx.text().matches('-').count(), 3);
    // 数の後の M-- は, 符号を反対にする
    fx.press("C-u 2 M-- C-b");
    assert_eq!(fx.app.eval("prefix-arg").unwrap(), "#f");
}

#[test]
fn test_display_options_follow_the_lisp_variables() {
    let mut fx = Fixture::new();
    fx.press("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    fx.app
        .eval("(set! truncate-lines #t) (set! selective-display 3)")
        .unwrap();
    // 次のコマンドの終わりに写す
    fx.press("C-a");
    fx.app.draw();
    let lines = fx.frames.lock().unwrap().last().unwrap().lines();
    assert!(lines[0].starts_with("abcdefghij"));
    fx.app
        .eval("(set! truncate-lines #f) (set! selective-display #f)")
        .unwrap();
    fx.press("C-e");
}

#[test]
fn test_sexp_and_sentence_keys() {
    let mut fx = Fixture::new();
    fx.press("( a SPC b ) SPC c . SPC SPC d . C-a");
    fx.press("C-M-f");
    assert_eq!(fx.point(), 5);
    fx.press("C-M-b");
    assert_eq!(fx.point(), 0);
    fx.press("C-M-d");
    assert_eq!(fx.point(), 1);
    fx.press("C-M-u");
    assert_eq!(fx.point(), 0);
    // 文の終わりと始め
    fx.press("M-e");
    assert_eq!(fx.point(), 8);
    fx.press("M-a");
    assert_eq!(fx.point(), 0);
    // 失敗は, エコー領域に出る
    fx.press("C-M-u");
    assert_eq!(fx.app.message(), "At top level");
    // C-M-SPC は, 式の終わりにマークを置く
    fx.press("C-M-SPC");
    assert_eq!(fx.app.host().editor().current_buffer().mark(), Some(5));
    fx.press("C-a C-M-d C-M-b");
    assert_eq!(fx.app.message(), "No previous sexp");
}

#[test]
fn test_incremental_search() {
    let mut fx = Fixture::new();
    fx.press("f o o SPC b a r SPC f o o");
    fx.press("C-a");
    fx.feed("f o o C-s RET");
    fx.press("C-s");
    // 2 つ目の foo の後ろ
    assert_eq!(fx.point(), 11);
    fx.press("C-a");
    // 見つからなければ, 最後に合った所のまま. C-g で戻って, C-g でやめる
    fx.feed("f o x C-g C-g");
    fx.press("C-s");
    assert_eq!(fx.point(), 0);
    assert_eq!(fx.app.message(), "Quit");
    // 後ろ向き
    fx.press("C-e");
    fx.feed("b a r RET");
    fx.press("C-r");
    assert_eq!(fx.point(), 4);
}

#[test]
fn test_query_replace_and_occur() {
    let mut fx = Fixture::new();
    fx.press("a SPC b SPC a SPC b SPC a");
    fx.press("C-a");
    fx.app.eval("(replace-string \"a\" \"X\")").unwrap();
    assert_eq!(fx.text(), "X b X b X");
    fx.press("C-a");
    fx.feed("b RET Y RET y n");
    fx.press("M-%");
    assert_eq!(fx.text(), "X Y X b X");
    fx.app.eval("(occur \"X\")").unwrap();
    fx.app.draw();
    let screen = fx.screen();
    assert!(screen.iter().any(|l| l.contains("1:X Y X b X")));
}

#[test]
fn test_highlighted_text_is_underlined() {
    let mut fx = Fixture::new();
    fx.press("f o o SPC b a r");
    fx.app.eval("(highlight-regexp \"bar\")").unwrap();
    fx.app.draw();
    let frames = fx.frames.lock().unwrap();
    let row = &frames.last().unwrap().cells[0];
    let marked: String = row.iter().filter(|c| c.underline).map(|c| c.ch).collect();
    assert_eq!(marked, "bar");
}

#[test]
fn test_keyboard_macros() {
    let mut fx = Fixture::new();
    // 覚えながら打った文字も入る. 終えるコマンドのキー (`C-x )`) は, マクロに入れない
    fx.feed("C-x ( a b C-x ) C-x e C-x e");
    fx.app.run();
    assert_eq!(fx.text(), "ababab");
    // 前置引数は, 繰り返す数
    fx.feed("C-u 2 C-x e");
    fx.app.run();
    assert_eq!(fx.text(), "ababababab");
}

#[test]
fn test_keyboard_macro_stops_at_the_first_error() {
    let mut fx = Fixture::new();
    fx.press("a b c");
    fx.feed("C-x ( C-b C-b C-x ) C-x e C-x e C-x e C-x e");
    fx.app.run();
    // 3 回目の `C-b` が先頭で失敗して, マクロは止まる
    assert_eq!(fx.point(), 0);
    assert_eq!(fx.app.message(), "Beginning of buffer");
    // 0 は, 失敗するまで繰り返す
    fx.press("C-e");
    fx.feed("C-u 0 C-x e");
    fx.app.run();
    assert_eq!(fx.point(), 0);
}

#[test]
fn test_keyboard_macro_errors_and_append() {
    let mut fx = Fixture::new();
    fx.press("C-x e");
    assert_eq!(fx.app.message(), "No kbd macro has been defined");
    fx.press("C-x )");
    assert_eq!(fx.app.message(), "Not defining keyboard macro");
    fx.press("C-x ( C-x (");
    assert_eq!(fx.app.message(), "Already defining keyboard macro.");
    // `C-x e` は, 覚えている途中なら, そこで終えて動かす
    fx.feed("x C-x e");
    fx.app.run();
    assert_eq!(fx.text(), "xx");
    // 前置引数があれば, 最後のマクロの続きに覚える
    fx.feed("C-u C-x ( y C-x ) C-x e");
    fx.app.run();
    assert_eq!(fx.text(), "xxyxy");
    // 終えるときの前置引数は, 覚えたものを 1 回目と数えた繰り返しの回数
    fx.feed("C-x ( z C-u 2 C-x ) ");
    fx.app.run();
    assert_eq!(fx.text(), "xxyxyzz");
}

#[test]
fn test_keyboard_macro_records_the_keys_read_by_commands() {
    let mut fx = Fixture::new();
    fx.press("a b c SPC a b c");
    fx.press("C-a");
    // 探す入力 (`C-s c RET`) も覚える
    fx.feed("C-x ( C-s c RET C-x ) C-a C-x e");
    fx.app.run();
    assert_eq!(fx.point(), 3);
    fx.press("C-e C-a");
    fx.feed("C-x e");
    fx.app.run();
    assert_eq!(fx.point(), 3);
}

#[test]
fn test_flush_macro_runs_the_queued_keys() {
    let mut fx = Fixture::new();
    fx.feed("C-x ( q C-x )");
    fx.app.run();
    fx.press("C-x e");
    fx.app.flush_macro();
    assert_eq!(fx.text(), "qq");
    // 繰り返すマクロは, 失敗するまで続く
    fx.press("C-u 0 C-x e C-f");
    fx.app.flush_macro();
    assert!(fx.text().starts_with("qq"));
}

#[test]
fn test_execute_kbd_macro_ignores_bad_arguments() {
    use rseditor_core::scheme::host::Host;
    use rseditor_core::scheme::value::Value;
    let mut fx = Fixture::new();
    // キーの並びでなければ空, 回数が数でなければ 1 回
    fx.app
        .host()
        .call("execute-kbd-macro", vec![Value::Nil, Value::Nil])
        .unwrap();
    fx.app.flush_macro();
    assert_eq!(fx.text(), "");
    // 文字列でない要素は捨てる
    let keys = Value::List(vec![Value::Number(1.0), Value::String("a".to_string())].into());
    fx.app.host().call("execute-kbd-macro", vec![keys]).unwrap();
    fx.app.flush_macro();
    assert_eq!(fx.text(), "a");
}

#[test]
fn test_dabbrev_expand_cycles_through_the_candidates() {
    let mut fx = Fixture::new();
    fx.press("f o o b a r SPC f o o d SPC f o o");
    // 近い順に, 続けて押すと次の候補, 尽きたら略語に戻る
    fx.press("M-/");
    assert_eq!(fx.text(), "foobar food food");
    fx.press("M-/");
    assert_eq!(fx.text(), "foobar food foobar");
    fx.press("M-/");
    assert_eq!(fx.text(), "foobar food foo");
    assert_eq!(
        fx.app.message(),
        "No further dynamic expansion for ‘foo’ found"
    );
}

#[test]
fn test_list_buffers_shows_the_list_in_another_window() {
    let mut fx = Fixture::new();
    fx.press("a b");
    fx.press("C-x C-b");
    // 一覧は別の窓に出て, 選んでいる窓は元のまま
    assert_eq!(fx.text(), "ab");
    let editor = fx.app.host().editor();
    assert_eq!(editor.window_list().len(), 2);
    assert!(editor
        .get_buffer("*Buffer List*")
        .unwrap()
        .text()
        .contains("*scratch*"));
}

#[test]
fn test_dired_starts_from_the_directory_of_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.txt");
    std::fs::write(&a, "from a").unwrap();
    let mut fx = Fixture::sized(12, 60, &[a.to_str().unwrap().to_string()]).unwrap();
    // 始めの入力は, ファイルのディレクトリ
    fx.feed("RET");
    fx.press("C-x d");
    let name = dir
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert_eq!(fx.app.host().editor().current_buffer_name(), name);
    assert!(fx
        .text()
        .starts_with(&format!("  {}:", dir.path().display())));
    assert!(fx.text().contains(" a.txt\n"));
    // 別のウィンドウに出す
    fx.feed("RET");
    fx.press("C-x b");
    fx.feed("RET");
    fx.press("C-x 4 d");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
}

#[test]
fn test_bookmark_set_and_jump_with_the_keys() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.txt");
    std::fs::write(&a, "hello world\n").unwrap();
    let mut fx = Fixture::sized(12, 60, &[a.to_str().unwrap().to_string()]).unwrap();
    let saved = dir.path().join("bookmarks");
    fx.app
        .eval(&format!(
            "(set! bookmark-default-file {:?})",
            saved.display().to_string()
        ))
        .unwrap();
    fx.press("C-f C-f C-f");
    // 空の入力は, 既定の名前 (ファイル名)
    fx.feed("RET");
    fx.press("C-x r m");
    assert!(std::fs::read_to_string(&saved)
        .unwrap()
        .starts_with("((\"a.txt\""));
    fx.press("C-e");
    fx.feed("RET");
    fx.press("C-x r b");
    assert_eq!(fx.point(), 3);
}

#[test]
fn test_find_file_other_window_and_dired_jump_with_the_keys() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    std::fs::write(&a, "from a").unwrap();
    std::fs::write(&b, "from b").unwrap();
    let mut fx = Fixture::sized(12, 60, &[a.to_str().unwrap().to_string()]).unwrap();
    // 別のウィンドウで開く
    let typed = |path: &std::path::Path| {
        let keys: Vec<String> = path
            .display()
            .to_string()
            .chars()
            .map(String::from)
            .collect();
        format!("{} RET", keys.join(" "))
    };
    fx.feed(&typed(&b));
    fx.press("C-x 4 f");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
    assert_eq!(fx.text(), "from b");
    // ファイルのバッファから, dired の, そのファイルの行へ
    fx.press("C-x C-j");
    assert!(fx.text()[fx.point()..].starts_with("b.txt\n"));
    fx.press("C-x 4 C-j");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
}

#[test]
fn test_insert_parentheses_and_count_lines_page_with_the_keys() {
    let mut fx = Fixture::new();
    fx.press("M-(");
    assert_eq!(fx.text(), "()");
    assert_eq!(fx.point(), 1);
    fx.press("C-x l");
    assert_eq!(fx.app.message(), "Page has 1 line (1 + 1)");
}

#[test]
fn test_narrowing_with_the_keys() {
    let mut fx = Fixture::new();
    fx.feed("(a)\n\u{c}\n(b)\n(c)");
    fx.press("C-x [");
    fx.press("C-x n p");
    assert!(fx.app.host().editor().current_buffer().narrowed());
    fx.press("C-x n w");
    assert!(!fx.app.host().editor().current_buffer().narrowed());
    fx.press("C-x n d");
    assert!(fx.app.host().editor().current_buffer().narrowed());
    fx.press("C-x n w");
    fx.press("C-x h");
    fx.press("C-x n n");
    assert!(fx.app.host().editor().current_buffer().narrowed());
}

#[test]
fn test_kmacro_counter_and_naming() {
    let mut fx = Fixture::new();
    // F3 は, 覚えている途中ならカウンターを入れる (1 回ごとに 1 増える). F4 は終える
    fx.feed("<f3> <f3> RET <f4>");
    fx.app.run();
    assert_eq!(fx.text(), "0\n");
    fx.feed("<f4> <f4>");
    fx.app.run();
    assert_eq!(fx.text(), "0\n1\n2\n");
    // 前置引数を付けて始めると, カウンターがその値になる
    fx.feed("C-u 7 C-x C-k s C-x C-k C-i C-x C-k C-i C-u C-x C-k C-i C-x )");
    fx.app.run();
    assert_eq!(fx.text(), "0\n1\n2\n788");
    // 名前を付ける
    fx.feed("C-x C-k n h e l l o RET");
    fx.app.run();
    fx.feed("M-x h e l l o RET");
    fx.app.run();
    assert_eq!(fx.text(), "0\n1\n2\n78891010");
}

#[test]
fn test_abbrev_keys() {
    let mut fx = Fixture::new();
    // 直前の単語を略語にして, 展開する (`C-x a g` で略語を聞かれる)
    fx.feed("h e l l o SPC w o r l d C-x a g z z RET SPC z z C-x ' ");
    fx.app.run();
    assert_eq!(fx.text(), "hello world world");
    // 前置引数 2 で, 2 語
    fx.feed("C-u 2 C-x a g q q RET SPC q q C-x a e");
    fx.app.run();
    assert_eq!(fx.text(), "hello world world world world");
    // 前の語を略語にして, 展開を聞く
    fx.feed("SPC a b c C-x a - a b c d RET");
    fx.app.run();
    assert_eq!(fx.text(), "hello world world world world abcd");
    // 接頭辞の印 (ハイフンは展開で消える)
    fx.feed("SPC p r e M-' z z C-x '");
    fx.app.run();
    assert_eq!(fx.text(), "hello world world world world abcd preworld");
}

#[test]
fn test_completion_keys() {
    let mut fx = Fixture::new();
    // C-M-i: Scheme の名前を補完する
    fx.feed("s t r i n g - a p p e C-M-i");
    fx.app.run();
    assert_eq!(fx.text(), "string-append");
    // C-M-/: バッファの中の語で補完する
    fx.feed("SPC f o o b a r SPC f o o b a z SPC f o o C-M-/");
    fx.app.run();
    assert_eq!(fx.text(), "string-append foobar foobaz fooba");
}

#[test]
fn test_shell_and_read_only_keys() {
    let mut fx = Fixture::new();
    // M-|: マークが無ければ失敗
    fx.feed("M-|");
    fx.app.run();
    assert_eq!(
        fx.app.message(),
        "The mark is not set now, so there is no region"
    );
    // M-!: コマンドを読んで動かす. 出力が短ければエコー領域に出る
    fx.feed("M-! e c h o SPC h i RET");
    fx.app.run();
    assert_eq!(fx.app.message(), "hi");
    // 前置引数があれば, 今のバッファに入れる
    fx.feed("C-u M-! e c h o SPC y o RET");
    fx.app.run();
    assert_eq!(fx.text(), "yo\n");
    // C-x C-q: 読み取り専用にすると, 文字を入れられない
    fx.feed("C-x C-q");
    fx.app.run();
    assert_eq!(fx.app.message(), "Read-Only mode enabled in current buffer");
    fx.feed("a");
    fx.app.run();
    assert_eq!(fx.app.message(), "Buffer is read-only: #<buffer *scratch*>");
    fx.feed("C-x C-q");
    fx.app.run();
    // M-~: 書き換えた印を消す
    fx.feed("a M-~");
    fx.app.run();
    assert_eq!(fx.app.message(), "Modification-flag cleared");
    // C-x x r: 名前を変える
    fx.feed("C-x x r n e w RET");
    fx.app.run();
    assert!(fx
        .app
        .host()
        .editor()
        .buffer_list()
        .contains(&"new".to_string()));
}

#[test]
fn test_window_keys() {
    let mut fx = Fixture::new();
    // C-x 4 b: 別の窓に出す (窓が 1 つなら分ける)
    fx.feed("C-x 4 b o t h e r RET");
    fx.app.run();
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
    assert_eq!(fx.app.host().editor().current_buffer_name(), "other");
    // C-x 4 0: バッファを消して, 窓も消す
    fx.press("C-x 4 0");
    assert_eq!(fx.app.host().editor().window_list().len(), 1);
    assert!(fx.app.host().editor().get_buffer("other").is_none());
    // C-x w 2, C-x w 3: 画面の全体を分ける
    fx.press("C-x w 2 C-x w 3");
    assert_eq!(fx.app.host().editor().window_list().len(), 3);
    // C-x ^ と C-x }: 窓を大きくする. C-x +: 同じ大きさにする
    fx.app.draw();
    let before = fx.app.host().editor().window_total_height(1).unwrap();
    fx.press("C-x ^");
    let after = fx.app.host().editor().window_total_height(1).unwrap();
    assert_eq!(after, before + 1);
    fx.press("C-x } C-x { C-x +");
    // C-x w 0: バッファ (既定は今のもの) を出している窓を消す
    fx.feed("RET");
    fx.press("C-x w 0");
    assert_eq!(fx.app.host().editor().window_list().len(), 1);
    fx.press("C-x 2 C-x w -");
    let selected = fx.app.host().editor().selected_window();
    assert_eq!(
        fx.app.host().editor().window_total_height(selected),
        Some(4)
    );
    // C-x w q: 窓を 1 つ消す
    fx.press("C-x 2 C-x -");
    fx.press("C-x w q");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
}

#[test]
fn test_scroll_other_window_and_move_to_window_line() {
    let mut fx = Fixture::new();
    fx.app
        .eval("(let loop ((i 0)) (if (< i 40) (begin (insert (format \"l%d\\n\" i)) (loop (+ i 1)))))")
        .unwrap();
    fx.press("M-<");
    // M-r: 真ん中, 上, 下の順
    fx.press("M-r");
    assert_eq!(fx.point(), 15);
    fx.press("M-r");
    assert_eq!(fx.point(), 0);
    fx.press("M-r");
    assert_eq!(fx.point(), 27);
    fx.press("M-r");
    assert_eq!(fx.point(), 15);
    // 数を付けると, その行 (負なら, 下から)
    assert_eq!(fx.app.eval("(move-to-window-line 3)").unwrap(), "3");
    assert_eq!(fx.point(), 9);
    assert_eq!(fx.app.eval("(move-to-window-line -2)").unwrap(), "8");
    assert_eq!(fx.point(), 24);
    // 窓の外の行は, 窓の中に収める
    assert_eq!(fx.app.eval("(move-to-window-line 99)").unwrap(), "9");
    // C-M-v: 次の窓を進める
    fx.press("M-< C-x 2 C-M-v");
    // 窓の点は, バッファごと (窓ごとではない)
    assert_eq!(fx.point(), 9);
    fx.press("C-x o M-<");
    fx.press("C-x o");
    fx.press("M-<end> M-<home>");
    assert_eq!(fx.point(), 0);
}

/// *Help* バッファの本文.
fn help_text(fx: &Fixture) -> String {
    let host = fx.app.host();
    let editor = host.editor();
    editor.get_buffer("*Help*").unwrap().text()
}

#[test]
fn test_help_keys() {
    let mut fx = Fixture::new();
    // C-h c: キーが走らせるコマンドを, メッセージに出す
    fx.feed("C-h c C-f");
    fx.app.run();
    assert_eq!(
        fx.app.message(),
        "C-f runs the command forward-char (found in global-map)"
    );
    // 続きのあるキーは, 続けて読む. 割り当てが無ければ, 未定義
    fx.feed("C-h c C-x C-f");
    fx.app.run();
    assert_eq!(
        fx.app.message(),
        "C-x C-f runs the command find-file (found in global-map)"
    );
    fx.feed("C-h c C-M-z");
    fx.app.run();
    assert_eq!(fx.app.message(), "C-M-z is undefined");
    // C-h k: *Help* に説明を出す
    fx.feed("C-h k C-x C-f");
    fx.app.run();
    assert!(help_text(&fx).starts_with("C-x C-f runs the command find-file"));
    // C-h w: コマンドのキー
    fx.feed("C-h w f i n d - f i l e RET");
    fx.app.run();
    assert_eq!(fx.app.message(), "find-file is on C-x C-f");
    // C-h v, C-h x, C-h o: 変数, コマンド, 記号
    fx.feed("C-h v x r e f - p r o j e c t - d i r e c t o r y RET");
    fx.app.run();
    assert!(help_text(&fx).contains("Its value is \".\"."));
    fx.feed("C-h x f i n d - f i l e RET");
    fx.app.run();
    assert!(help_text(&fx).starts_with("find-file is an interactive Lisp function"));
    fx.feed("C-h o x r e f - g o - b a c k RET");
    fx.app.run();
    assert!(help_text(&fx).starts_with("xref-go-back is"));
    // C-h b: 全ての割り当て. C-h l: 最近のキー. C-h e: メッセージの記録
    fx.feed("C-h b");
    fx.app.run();
    assert!(help_text(&fx).contains("C-x C-f                 find-file\n"));
    fx.feed("C-h l");
    fx.app.run();
    assert!(help_text(&fx).contains("C-h l"));
    fx.feed("C-h e");
    fx.app.run();
    let messages = fx
        .app
        .host()
        .editor()
        .get_buffer("*Messages*")
        .unwrap()
        .text();
    assert!(messages.contains("find-file is on C-x C-f"));
    // C-h ?, C-h s, C-h q
    fx.feed("C-h ?");
    fx.app.run();
    assert!(help_text(&fx).starts_with("You have typed C-h"));
    fx.feed("C-h s");
    fx.app.run();
    assert!(help_text(&fx).starts_with("The syntax table"));
    fx.feed("C-h q");
    fx.app.run();
}

#[test]
fn test_help_host_operations() {
    let mut fx = Fixture::new();
    let host = fx.app.host();
    let string = |s: &str| Value::String(s.to_string());
    assert_eq!(
        host.call("key-binding", vec![string("C-x C-f")])
            .unwrap()
            .to_string(),
        "\"find-file\""
    );
    assert_eq!(
        host.call("key-binding", vec![string("C-x")])
            .unwrap()
            .to_string(),
        "#t"
    );
    assert_eq!(
        host.call("key-binding", vec![string("C-M-z")])
            .unwrap()
            .to_string(),
        "#f"
    );
    let bindings = host.call("all-key-bindings", vec![]).unwrap().to_string();
    assert!(bindings.contains("(\"C-x C-f\" \"find-file\")"));
    // 最近のキー: 画面から読んだもの (300 まで)
    assert_eq!(host.call("recent-keys", vec![]).unwrap().to_string(), "()");
    fx.feed("a b");
    fx.app.run();
    assert_eq!(
        fx.app
            .host()
            .call("recent-keys", vec![])
            .unwrap()
            .to_string(),
        "(\"a\" \"b\")"
    );
    fx.feed(&"a ".repeat(305));
    fx.app.run();
    let recent = fx.app.host().call("recent-keys", vec![]).unwrap();
    let Value::List(keys) = recent else {
        panic!("a list")
    };
    assert_eq!(keys.len(), 300);
}

#[test]
fn test_xref_keys() {
    let mut fx = Fixture::new();
    let dir = tempfile::tempdir().unwrap();
    let defs = dir.path().join("defs.scm");
    std::fs::write(&defs, "(define (tui-sample-fn x)\n  x)\n").unwrap();
    fx.app
        .eval(&format!("(load \"{}\")", defs.to_str().unwrap()))
        .unwrap();
    // M-.: 点の所の名前の定義へ. M-,: 戻る. C-M-,: また進む
    fx.press("t u i - s a m p l e - f n");
    fx.press("M-.");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "defs.scm");
    fx.press("M-,");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "*scratch*");
    fx.press("C-M-,");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "defs.scm");
    // C-x 4 .: 別の窓に出す
    fx.press("M-, C-x 4 .");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
    // M-?: 参照 (探すディレクトリの下に, 無い名前)
    fx.app
        .eval(&format!(
            "(set! xref-project-directory \"{}\")",
            dir.path().to_str().unwrap()
        ))
        .unwrap();
    fx.app.eval("(set! xref-prompt-for-identifier #t)").unwrap();
    fx.feed("q q q - n o RET");
    fx.press("C-x 1 M-?");
    assert_eq!(fx.app.message(), "No references found for: qqq-no");
    // C-M-.: 名前の正規表現に合う定義
    fx.feed("^ t u i - s a m p RET");
    fx.press("C-M-.");
    let listed = fx.app.host().editor().get_buffer("*xref*").unwrap().text();
    assert!(listed.contains("tui-sample-fn"));
}

#[test]
fn test_next_error_keys() {
    let mut fx = Fixture::new();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), "one\nfoo\n").unwrap();
    std::fs::write(dir.path().join("b.txt"), "foo\n").unwrap();
    // 結果が無いうちは, 行く所が無い
    fx.press("M-g n");
    assert_eq!(
        fx.app.message(),
        "No buffers contain error message locations"
    );
    fx.app
        .eval(&format!(
            "(grep \"foo\" \"{}\")",
            dir.path().to_str().unwrap()
        ))
        .unwrap();
    // M-g n と C-x ` は次の場所へ, M-g p は前の場所へ
    fx.press("M-g n");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "a.txt");
    assert_eq!(fx.point(), 4);
    fx.press("C-x `");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "b.txt");
    fx.press("M-g n");
    assert_eq!(fx.app.message(), "Moved past last grep hit");
    fx.press("M-g p");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "a.txt");
    fx.press("M-g p");
    assert_eq!(fx.app.message(), "Moved back before first grep hit");
}

#[test]
fn test_info_keys() {
    let mut fx = Fixture::new();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().to_str().unwrap().to_string();
    std::fs::write(format!("{path}/dir"), "Info directory\n").unwrap();
    std::fs::write(format!("{path}/emacs.info"), "x\n").unwrap();
    std::fs::write(format!("{path}/m.info"), "manual m\n").unwrap();
    fx.app
        .eval(&format!("(set! Info-directory-list (list \"{path}\"))"))
        .unwrap();
    fx.press("C-h i");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "*info*");
    fx.press("C-h 4 i");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
    fx.press("C-x 1 C-h r");
    assert_eq!(fx.app.host().editor().current_buffer().text(), "x\n");
    fx.feed("m RET");
    fx.press("C-h R");
    assert_eq!(fx.app.host().editor().current_buffer().text(), "manual m\n");
    fx.feed("f i n d - f i l e RET");
    fx.press("C-h F");
    assert_eq!(
        fx.app.message(),
        "Couldn't find documentation for find-file"
    );
    fx.feed("C-f");
    fx.press("C-h K");
    assert_eq!(
        fx.app.message(),
        "Couldn't find documentation for forward-char"
    );
}

#[test]
fn test_auto_save_every_300_keys() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "hi\n").unwrap();
    let mut fx = Fixture::sized(12, 30, &[file.to_string_lossy().into_owned()]).unwrap();
    let auto = dir.path().join("#a.txt#");
    // 画面から読んだ 300 個のキーの、1 個目で本文を書き換え、あとは動くだけ
    fx.feed("x");
    for _ in 1..299 {
        fx.feed("C-f");
    }
    fx.app.run();
    assert!(!auto.exists());
    fx.feed("C-b");
    fx.app.run();
    assert_eq!(std::fs::read_to_string(&auto).unwrap(), "xhi\n");
    assert_eq!(fx.app.message(), "Auto-saving...done");
    // 書いたら、数え直す
    assert_eq!(fx.app.host().editor().keys_since_auto_save, 0);
}

#[test]
fn test_c_g_stops_a_macro_that_never_fails() {
    let mut fx = Fixture::new();
    // 失敗しないマクロを、失敗するまで繰り返す。画面で C-g を押していれば、止める
    fx.app.eval("(execute-kbd-macro '(\"x\") 0)").unwrap();
    fx.feed("C-g");
    fx.app.run();
    assert_eq!(fx.text(), "");
    assert_eq!(fx.app.message(), "Quit");
    // C-g 以外のキーは、読まずに残す
    fx.app.eval("(execute-kbd-macro '(\"y\") 2)").unwrap();
    fx.feed("a");
    fx.app.run();
    assert_eq!(fx.text(), "yya");
}

#[test]
fn test_kill_ring_rectangles_and_registers_are_shared_by_windows() {
    // 1 つ目のウィンドウで消したものを, 2 つ目のウィンドウで取り出せる
    let mut fx = Fixture::new();
    fx.press("a b c RET d e f C-x 3 C-a C-SPC C-n C-w C-x o C-y");
    assert_eq!(fx.text(), "abc\ndef");
    // 消した矩形 (C-x r k) も共通
    let mut fx = Fixture::new();
    fx.press("a b c RET d e f C-x 3 C-p C-a C-SPC C-n C-f C-x r k");
    assert_eq!(fx.text(), "bc\nef");
    fx.press("C-x o C-p C-a C-x r y");
    assert_eq!(fx.text(), "abc\ndef");
    // レジスタ (C-x r s, C-x r i) も共通
    let mut fx = Fixture::new();
    fx.press("a b c C-x 3 C-a C-SPC C-f C-f");
    fx.feed("a");
    fx.press("C-x r s");
    fx.press("C-x o C-e");
    fx.feed("a");
    fx.press("C-x r i");
    assert_eq!(fx.text(), "abcab");
}

#[test]
fn test_isearch_folds_case_and_m_c_toggles_it() {
    // 小文字だけの検索は, 大文字と小文字を区別しない
    let mut fx = Fixture::new();
    fx.press("a X b a x C-a");
    fx.feed("a x RET");
    fx.press("C-s");
    assert_eq!(fx.point(), 2);
    // M-c で区別する
    let mut fx = Fixture::new();
    fx.press("a X b a x C-a");
    fx.feed("a M-c x RET");
    fx.press("C-s");
    assert_eq!(fx.point(), 5);
    // M-r で正規表現にして, 続きを打つ
    let mut fx = Fixture::new();
    fx.press("a b b b C-a");
    fx.feed("b M-r + RET");
    fx.press("C-s");
    assert_eq!(fx.point(), 4);
}

#[test]
fn test_minibuffer_history_keys() {
    // 入力した値が履歴に残り, M-p で出せる
    let mut fx = Fixture::new();
    fx.press("a RET b RET c RET d");
    fx.feed("2 RET");
    fx.press("M-g g");
    assert_eq!(fx.point(), 2);
    fx.feed("M-p RET");
    fx.press("M-g g");
    assert_eq!(fx.point(), 2);
    // M-r で履歴を正規表現で探す
    fx.feed("3 RET");
    fx.press("M-g g");
    fx.feed("2 RET");
    fx.press("M-g g");
    fx.feed("M-r 3 RET RET");
    fx.press("M-g g");
    assert_eq!(fx.point(), 4);
    // M-n で既定の値を出す
    let mut fx = Fixture::new();
    fx.feed("( i n s e r t SPC ( r e a d - s t r i n g SPC \" p \" SPC \" \" SPC # f SPC \" d f l t \" ) ) RET");
    fx.feed("M-n RET");
    fx.press("M-:");
    assert_eq!(fx.text(), "dflt");
    // 履歴が無ければ知らせる
    let mut fx = Fixture::new();
    fx.feed("M-p M-n C-g");
    fx.press("M-g g");
    // M-r の正規表現の入力をやめる (C-g) と, 空の入力 (前の正規表現を使う), M-s
    let mut fx = Fixture::new();
    fx.press("a RET b RET c RET d");
    for line in ["1", "2", "3"] {
        fx.feed(&format!("{line} RET"));
        fx.press("M-g g");
    }
    fx.feed("M-r C-g C-g");
    fx.press("M-g g");
    fx.feed("M-r 1 RET M-r RET M-s 3 RET RET");
    fx.press("M-g g");
    assert_eq!(fx.point(), 4);
}

#[test]
fn test_quoted_insert_inserts_control_and_meta_keys() {
    // 前置きのキーも, 制御文字としてそのまま入る (C-x は ^X)
    for (key, expected) in [
        ("C-x", "\u{18}"),
        ("C-h", "\u{8}"),
        ("RET", "\r"),
        ("C-m", "\r"),
        ("C-j", "\n"),
        ("C-@", "\u{0}"),
        ("C-SPC", "\u{0}"),
        ("C-?", "\u{7f}"),
        ("ESC", "\u{1b}"),
        ("DEL", "\u{7f}"),
        ("M-a", "\u{e1}"),
        ("C-M-x", "\u{98}"),
        ("a", "a"),
    ] {
        let mut fx = Fixture::new();
        fx.feed(key);
        fx.press("C-q");
        assert_eq!(fx.text(), expected, "C-q {key}");
    }
    // 文字にならないキーは, 入れない
    let mut fx = Fixture::new();
    fx.feed("<f1>");
    fx.press("C-q");
    assert_eq!(fx.text(), "");
}

#[test]
fn test_disabled_commands_ask_before_they_run() {
    // C-x C-u: n でやめる, SPC で 1 回だけ, y で以後は聞かない
    let mut fx = Fixture::new();
    fx.press("a b c C-a C-SPC C-e");
    fx.feed("n");
    fx.press("C-x C-u");
    assert_eq!(fx.text(), "abc");
    fx.feed("C-g");
    fx.press("C-x C-u");
    assert_eq!(fx.text(), "abc");
    // 他のキーは聞き直す
    fx.feed("x SPC");
    fx.press("C-x C-u");
    assert_eq!(fx.text(), "ABC");
    // SPC では有効にならない. y で有効にした C-x C-l は, 以後は聞かない
    fx.press("C-a C-SPC C-e");
    fx.feed("y");
    fx.press("C-x C-l");
    assert_eq!(fx.text(), "abc");
    fx.press("C-a C-SPC C-e");
    fx.feed("SPC");
    fx.press("C-x C-u");
    assert_eq!(fx.text(), "ABC");
    fx.press("C-a C-SPC C-e C-x C-l");
    assert_eq!(fx.text(), "abc");
    // ! で全てを有効にする
    let mut fx = Fixture::new();
    fx.press("a b C-a C-SPC C-e");
    fx.feed("!");
    fx.press("C-x C-n");
    assert_eq!(fx.point(), 2);
    fx.press("C-x C-u");
    assert_eq!(fx.text(), "AB");
    // M-x でも聞く (プログラムから呼ぶときは聞かない)
    let mut fx = Fixture::new();
    fx.press("a b C-a C-SPC C-e");
    fx.feed("u p c a s e - r e g i o n RET n");
    fx.press("M-x");
    assert_eq!(fx.text(), "ab");
}

/// 文字列を打つキーの並びにする.
fn typed(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\n' => "RET".to_string(),
            ' ' => "SPC".to_string(),
            c => c.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn test_fill_paragraph_in_a_docstring_and_a_region() {
    // docstring の中: 長い行が詰められる
    let mut fx = Fixture::new();
    fx.feed(&typed("scheme-mode\n"));
    fx.press("M-x");
    fx.press(&typed(
        "(foo \"aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq rrr sss\")",
    ));
    fx.press("C-b C-b C-b M-q");
    assert_eq!(
        fx.text(),
        "(foo \"aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq\nrrr sss\")"
    );
    // 複数行の docstring: 最初の行は別にして, 残りが 1 つの文章として詰められる
    let mut fx = Fixture::new();
    fx.feed(&typed("scheme-mode\n"));
    fx.press("M-x");
    fx.press(&typed("(foo \"aaa\nbbb\nccc\")"));
    fx.press("C-b C-b M-q");
    assert_eq!(fx.text(), "(foo \"aaa\nbbb ccc\")");
    // リージョン: 全ての段落が詰められる
    let mut fx = Fixture::new();
    fx.press(&typed("aaa\nbbb\n\nccc\nddd\n"));
    fx.press("C-x h M-q");
    assert_eq!(fx.text(), "aaa bbb\n\nccc ddd\n");
}
