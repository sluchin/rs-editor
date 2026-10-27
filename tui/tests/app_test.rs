//! ターミナルのエディタ (`app`) のテスト: キーを送って、本文、カーソル、画面、エコー領域を確かめる.

mod common;
use common::Fixture;

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
    assert_eq!(fx.text(), "ello wor\nx\t");
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
    assert_eq!(fx.app.message(), "Previous command was not a yank");
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
    assert_eq!(fx.app.message(), "C-] is undefined");
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
    assert!(fx.app.eval("(car '())").is_err());
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
    let dir_arg = [dir.path().to_str().unwrap().to_string()];
    assert!(Fixture::sized(12, 30, &dir_arg).is_err());
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
