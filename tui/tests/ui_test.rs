//! 画面の要る操作 (`ui`): ミニバッファの入力、補完、キーの問い、スクロールなどのテスト.

mod common;
use common::Fixture;

/// 文字列を、キーの並び (空白で区切る) にする.
fn keys_of(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c == ' ' {
                "SPC".to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn test_read_string_editing_keys() {
    let fx = Fixture::new();
    // 初めの入力に、書き足し、動かし、消す
    fx.feed("a b c C-a X C-e Y C-b C-b Z C-f C-k RET");
    assert_eq!(
        fx.app.eval("(read-string \"Q: \" \"ini\")").unwrap(),
        "\"XiniabZc\""
    );
    // 後ろへ消す, 前へ消す, 行の先頭と終わり, 矢印, 文字でないキーは無視する
    fx.feed("a b c d e DEL <left> <left> C-d <delete> <home> <right> C-q <end> f RET");
    assert_eq!(fx.app.eval("(read-string \"Q: \")").unwrap(), "\"abf\"");
    // 画面の最後の行に、問いと入力を出す
    fx.feed("x y");
    let _ = fx.app.eval("(read-string \"Prompt: \")");
    let screen = fx.screen();
    assert_eq!(screen.last().unwrap(), "Prompt: xy");
    assert_eq!(fx.cursor(), (11, 10));
    // C-g でやめる. 入力が終わっても、やめる
    fx.feed("a C-g");
    assert_eq!(fx.app.eval("(read-string \"Q: \")").unwrap_err(), "Quit");
    fx.feed("a");
    assert_eq!(fx.app.eval("(read-string \"Q: \")").unwrap_err(), "Quit");
    // 幅の広い文字の問い
    fx.feed("RET");
    fx.app.eval("(read-string \"名前: \")").unwrap();
    assert_eq!(fx.cursor(), (11, 6));
}

#[test]
fn test_read_key_names() {
    let fx = Fixture::new();
    for (key, name) in [
        ("y", "\"y\""),
        ("SPC", "\" \""),
        ("RET", "\"Enter\""),
        ("DEL", "\"Backspace\""),
        ("<delete>", "\"Delete\""),
        ("ESC", "\"Escape\""),
        ("!", "\"!\""),
    ] {
        fx.feed(key);
        assert_eq!(fx.app.eval("(read-key \"Save? \")").unwrap(), name, "{key}");
    }
    assert_eq!(fx.screen().last().unwrap(), "Save?");
    fx.feed("C-g");
    assert_eq!(fx.app.eval("(read-key)").unwrap_err(), "Quit");
}

#[test]
fn test_completing_read() {
    let fx = Fixture::new();
    let ask = "(completing-read \"Pick: \" '(\"apple\" \"apricot\" \"banana\"))";
    // 1 つに決まる
    fx.feed("b TAB RET");
    assert_eq!(fx.app.eval(ask).unwrap(), "\"banana\"");
    assert_eq!(fx.app.message(), "[Sole completion]");
    // 共通の前の部分まで補い、候補を出す
    fx.feed("a TAB RET");
    assert_eq!(fx.app.eval(ask).unwrap(), "\"ap\"");
    assert_eq!(fx.app.message(), "apple apricot");
    // 合うものが無い
    fx.feed("z TAB RET");
    assert_eq!(fx.app.eval(ask).unwrap(), "\"z\"");
    assert_eq!(fx.app.message(), "[No match]");
    // 補完しない入力では、何も変えない
    fx.feed("q TAB RET");
    assert_eq!(fx.app.eval("(read-string \"Q: \")").unwrap(), "\"q\"");
    // 候補に無い入力は、聞き直す
    fx.feed("z RET DEL a p p l e RET");
    let strict = "(completing-read \"Pick: \" '(\"apple\" \"apricot\") #f #t)";
    assert_eq!(fx.app.eval(strict).unwrap(), "\"apple\"");
}

#[test]
fn test_file_name_completion() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("alpha.txt"), "").unwrap();
    std::fs::write(dir.path().join("alps.txt"), "").unwrap();
    std::fs::create_dir(dir.path().join("beta")).unwrap();
    let base = dir.path().to_str().unwrap();
    let mut fx = Fixture::new();
    let ask = |fx: &mut Fixture, typed: &str| {
        fx.feed(&format!("{} TAB RET", keys_of(typed)));
        fx.app.eval("(read-file-name \"File: \")").unwrap()
    };
    assert_eq!(
        ask(&mut fx, &format!("{base}/b")),
        format!("\"{base}/beta/\"")
    );
    assert_eq!(
        ask(&mut fx, &format!("{base}/al")),
        format!("\"{base}/alp\"")
    );
    assert_eq!(
        fx.app.message(),
        "alpha.txt alps.txt".replace(
            "alpha.txt alps.txt",
            &format!("{base}/alpha.txt {base}/alps.txt")
        )
    );
    assert_eq!(
        ask(&mut fx, &format!("{base}/zz")),
        format!("\"{base}/zz\"")
    );
    assert_eq!(fx.app.message(), "[No match]");
    // ディレクトリの無い名前は、今のディレクトリから探す. 読めないディレクトリは、候補が無い
    assert_eq!(ask(&mut fx, "no-such-file-zz"), "\"no-such-file-zz\"");
    assert_eq!(ask(&mut fx, "/no-such-dir-zz/a"), "\"/no-such-dir-zz/a\"");
}

#[test]
fn test_find_file_and_save() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("note.txt");
    std::fs::write(&path, "hello\n").unwrap();
    let mut fx = Fixture::new();
    fx.feed(&format!("{} RET", keys_of(path.to_str().unwrap())));
    fx.press("C-x C-f");
    assert_eq!(fx.text(), "hello\n");
    fx.press("x");
    fx.press("C-x C-s");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "xhello\n");
    assert!(fx.app.message().starts_with("Wrote "));
    // 保存するものが無い
    fx.press("C-x C-s");
    assert_eq!(fx.app.message(), "(No changes need to be saved)");
    // 別の名前に書く. 別のバッファに移る
    let other = dir.path().join("copy.txt");
    fx.feed(&format!("{} RET", keys_of(other.to_str().unwrap())));
    fx.press("C-x C-w");
    assert_eq!(std::fs::read_to_string(&other).unwrap(), "xhello\n");
    fx.feed("*scratch* RET");
    fx.press("C-x b");
    assert_eq!(fx.app.host().editor().current_buffer_name(), "*scratch*");
}

#[test]
fn test_kill_buffer_command() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("k.txt");
    std::fs::write(&path, "k").unwrap();
    let mut fx = Fixture::new();
    fx.feed(&format!("{} RET", keys_of(path.to_str().unwrap())));
    fx.press("C-x C-f");
    // 既定は今のバッファ. 書き換えていなければ、すぐ消す
    fx.feed("RET");
    fx.press("C-x k");
    assert_eq!(fx.app.host().editor().buffer_list(), vec!["*scratch*"]);
    // 書き換えたファイルのバッファは、聞いてから消す. no なら、やめる
    fx.feed(&format!("{} RET", keys_of(path.to_str().unwrap())));
    fx.press("C-x C-f");
    fx.press("x");
    fx.feed("RET n o RET");
    fx.press("C-x k");
    assert_eq!(fx.app.message(), "Quit");
    assert_eq!(fx.app.host().editor().buffer_list().len(), 2);
    fx.feed("RET y e s RET");
    fx.press("C-x k");
    assert_eq!(fx.app.host().editor().buffer_list(), vec!["*scratch*"]);
}

#[test]
fn test_exit_flows() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("e.txt");
    std::fs::write(&path, "e").unwrap();
    // 変更が無ければ、すぐ終わる
    let mut fx = Fixture::sized(12, 30, &[path.to_str().unwrap().to_string()]).unwrap();
    fx.press("C-x C-c");
    assert!(fx.app.quit());
    // 変更があれば保存するか聞く. y で保存して、終わる
    let mut fx = Fixture::sized(12, 30, &[path.to_str().unwrap().to_string()]).unwrap();
    fx.press("x");
    fx.feed("y");
    fx.press("C-x C-c");
    assert!(fx.app.quit());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "xe");
    // 保存せずに、終わらない
    let mut fx = Fixture::sized(12, 30, &[path.to_str().unwrap().to_string()]).unwrap();
    fx.press("x");
    fx.feed("n n o RET");
    fx.press("C-x C-c");
    assert!(!fx.app.quit());
    // 保存せずに、終わる
    fx.feed("n y e s RET");
    fx.press("C-x C-c");
    assert!(fx.app.quit());
}

#[test]
fn test_windows_commands() {
    let mut fx = Fixture::new();
    fx.press("a b c");
    fx.press("C-x 2");
    assert_eq!(fx.app.host().editor().window_list(), vec![1, 2]);
    fx.press("C-x o");
    assert_eq!(fx.app.host().editor().selected_window(), 2);
    fx.press("C-x 3");
    assert_eq!(fx.app.host().editor().window_list().len(), 3);
    fx.press("C-x 0");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
    fx.press("M-2 C-x o");
    assert_eq!(fx.app.host().editor().window_list().len(), 2);
    fx.press("C-x 1");
    assert_eq!(fx.app.host().editor().window_list().len(), 1);
    fx.press("C-x 0");
    assert_eq!(
        fx.app.message(),
        "Attempt to delete minibuffer or sole ordinary window"
    );
}

#[test]
fn test_scrolling() {
    let mut fx = Fixture::sized(8, 20, &[]).unwrap();
    // 画面より長い本文
    for i in 0..30 {
        fx.press(&format!("l {} {}", i / 10, i % 10));
        fx.press("RET");
    }
    fx.press("M-<");
    // 画面の高さ - 2 行ずつ進む
    fx.press("C-v");
    assert_eq!(fx.point(), 16);
    fx.press("M-v");
    assert_eq!(fx.point(), 0);
    fx.press("C-v C-l");
    fx.app.draw();
    let screen = fx.screen();
    assert_eq!(screen[0], "l01");
    // 最後まで進むと、エラー
    fx.press("M->");
    fx.press("C-v");
    assert_eq!(fx.app.message(), "End of buffer");
    fx.press("M-<");
    fx.press("<prior>");
    assert_eq!(fx.app.message(), "Beginning of buffer");
}
