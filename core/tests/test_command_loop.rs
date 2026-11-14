//! コマンドループ (`command_loop`) のテスト. 画面は偽物 (エコー領域と出し方の設定を覚える).

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{printable, CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;

/// 偽の画面.
#[derive(Default)]
struct Ui {
    /// エコー領域.
    message: Mutex<String>,
    /// マクロを止めた回数.
    stopped: Mutex<usize>,
    /// 出し方の設定.
    display: Mutex<(bool, Option<usize>)>,
    /// クリップボード.
    clipboard: Mutex<String>,
}

impl Host for Ui {
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        match op {
            "message" => {
                *self.message.lock().unwrap() = match args.first() {
                    Some(Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                Ok(Value::Nil)
            }
            "gui-set-selection" => {
                if let Some(Value::String(text)) = args.first() {
                    *self.clipboard.lock().unwrap() = text.clone();
                }
                Ok(Value::Nil)
            }
            "gui-selection-value" => Ok(Value::String(self.clipboard.lock().unwrap().clone())),
            other => Err(format!("ui: {other}").into()),
        }
    }
}

impl LoopUi for Ui {
    fn say(&self, text: &str) {
        *self.message.lock().unwrap() = text.to_string();
    }

    fn stop_macro(&self) {
        *self.stopped.lock().unwrap() += 1;
    }

    fn set_display(&self, truncate_lines: bool, selective_display: Option<usize>) {
        *self.display.lock().unwrap() = (truncate_lines, selective_display);
    }
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

fn press(commands: &mut CommandLoop<Ui>, keys: &str) {
    for key in keys.split(' ') {
        commands.handle_key(key);
    }
}

fn text(commands: &CommandLoop<Ui>) -> String {
    commands.host().editor().current_buffer().text()
}

fn message(commands: &CommandLoop<Ui>) -> String {
    commands.host().ui().message.lock().unwrap().clone()
}

#[test]
fn test_printable() {
    assert_eq!(printable("a"), Some('a'));
    assert_eq!(printable("SPC"), Some(' '));
    assert_eq!(printable("あ"), Some('あ'));
    assert_eq!(printable("C-a"), None);
    assert_eq!(printable("\u{7}"), None);
}

#[test]
fn test_keys_commands_and_self_insert() {
    let mut commands = setup();
    assert!(commands.at_command_start());
    press(&mut commands, "a b C-b C-b");
    assert_eq!(text(&commands), "ab");
    assert_eq!(commands.host().editor().current_buffer().point(), 0);
    // 続きのキーを待つ. C-g でやめる
    press(&mut commands, "C-x");
    assert!(!commands.at_command_start());
    assert_eq!(message(&commands), "C-x-");
    press(&mut commands, "C-g");
    assert_eq!(message(&commands), "Quit");
    // 割り当ての無いキー
    press(&mut commands, "C-x C-z");
    assert_eq!(message(&commands), "C-x C-z is undefined");
    // 文字列を入れる (IME で確定した文字). 前置引数の数だけ
    press(&mut commands, "C-u 3");
    assert_eq!(message(&commands), "C-u 3-");
    commands.self_insert("あい");
    assert_eq!(text(&commands), "あいあいあいab");
    assert_eq!(
        commands
            .evaluator()
            .global()
            .get("last-command")
            .unwrap()
            .to_string(),
        "self-insert-command"
    );
    // 続きのキーを待っているときに文字を入れると, 待つのをやめる
    press(&mut commands, "C-x");
    commands.self_insert("x");
    assert!(commands.at_command_start());
}

#[test]
fn test_shifted_keys_fall_back_to_the_unshifted_keys() {
    let mut commands = setup();
    // 割り当ての無い Shift 付きのキーは, Shift を外して引く (S-SPC は空白を入れる)
    press(&mut commands, "a b S-SPC S-<left> C-S-f");
    assert_eq!(text(&commands), "ab ");
    assert_eq!(commands.host().editor().current_buffer().point(), 3);
    // 割り当てがあれば, そのまま (C-S-DEL は kill-whole-line)
    press(&mut commands, "C-S-DEL");
    assert_eq!(text(&commands), "");
    // Shift を外しても無ければ, 外したキーを知らせる
    press(&mut commands, "C-x S-C-z");
    assert_eq!(message(&commands), "C-x C-z is undefined");
}

#[test]
fn test_repeat_and_transient_keys() {
    let mut commands = setup();
    press(&mut commands, "a b c C-a");
    // 前置引数のコマンドは, 繰り返すコマンドにしない. 前置引数は覚える
    press(&mut commands, "C-u 1 C-f");
    let globals = commands.evaluator().global();
    assert_eq!(
        globals.get("last-repeatable-command").unwrap().to_string(),
        "forward-char"
    );
    assert_eq!(globals.get("last-prefix-arg").unwrap().to_string(), "1");
    // C-x z で繰り返し, z でまた繰り返す. ほかのキーは, いつもどおり
    press(&mut commands, "C-x z z");
    assert_eq!(commands.host().editor().current_buffer().point(), 3);
    press(&mut commands, "C-b z");
    assert_eq!(text(&commands), "abzc");
    // 表の要素の形が違えば, 使わない
    commands
        .evaluator()
        .eval("(set! transient-map--keys '(x (\"q\" \"forward-char\") (\"r\" backward-char)))")
        .unwrap();
    press(&mut commands, "q");
    assert_eq!(text(&commands), "abzqc");
    // 続きのキーを待っているときは, 表を見ない (C-x C-x は exchange-point-and-mark)
    press(&mut commands, "C-x");
    commands
        .evaluator()
        .eval("(set-transient-map '((\"C-x\" backward-char)))")
        .unwrap();
    press(&mut commands, "C-x");
    assert_eq!(commands.host().editor().current_buffer().point(), 4);
}

#[test]
fn test_kills_and_yanks_use_the_clipboard() {
    fn clipboard(commands: &CommandLoop<Ui>) -> String {
        commands.host().ui().clipboard.lock().unwrap().clone()
    }
    let mut commands = setup();
    // kill はクリップボードにも書く. 足した kill も
    press(&mut commands, "a b C-SPC C-a M-w");
    assert_eq!(clipboard(&commands), "ab");
    press(&mut commands, "RET c C-a C-k");
    assert_eq!(clipboard(&commands), "cab");
    // ほかのアプリが書いた文字は, yank が最新の kill にして入れる. 自分が書いた文字は重ねない
    *commands.host().ui().clipboard.lock().unwrap() = "xy".to_string();
    press(&mut commands, "C-y C-y");
    assert_eq!(text(&commands), "\nxyxy");
    // select-enable-clipboard が偽なら使わない
    commands
        .evaluator()
        .eval("(set! select-enable-clipboard #f)")
        .unwrap();
    *commands.host().ui().clipboard.lock().unwrap() = "zz".to_string();
    press(&mut commands, "C-y");
    assert_eq!(text(&commands), "\nxyxyxy");
    // クリップボードのコマンドは, select-enable-clipboard を真にして動かす
    press(&mut commands, "C-a");
    commands.execute("clipboard-yank");
    assert_eq!(text(&commands), "\nzzxyxyxy");
}

#[test]
fn test_prefix_arguments_and_failures() {
    let mut commands = setup();
    // 負の数の前置引数でも, 文字は 1 つ入れる
    press(&mut commands, "C-u - 2 a");
    assert_eq!(text(&commands), "a");
    press(&mut commands, "C-u C-u a");
    assert_eq!(text(&commands), "a".repeat(17));
    press(&mut commands, "M-- C-u");
    press(&mut commands, "M-3 b");
    // 失敗したコマンドは, エコー領域に出し, マクロを止める
    commands
        .eval("(define-command (fail) (error \"boom\"))")
        .unwrap();
    commands.execute("fail");
    assert_eq!(message(&commands), "boom");
    assert_eq!(*commands.host().ui().stopped.lock().unwrap(), 1);
    assert_eq!(commands.variable("no-such-variable"), Value::Boolean(false));
}

#[test]
fn test_lisp_keys_local_keys_and_display() {
    let commands = &mut setup();
    commands
        .eval("(define-command (hello) (insert \"hi\")) (global-set-key \"C-c h\" 'hello)")
        .unwrap();
    press(commands, "C-c h");
    assert_eq!(text(commands), "hi");
    // バッファだけのキーが先
    commands
        .eval("(define-command (local) (insert \"L\")) (local-set-key \"C-c h\" 'local) (local-set-key \"C-c x y\" 'local)")
        .unwrap();
    press(commands, "C-c h C-c x y");
    assert_eq!(text(commands), "hiLL");
    // バッファに無いキーは, 全体から
    press(commands, "C-a");
    assert_eq!(commands.host().editor().current_buffer().point(), 0);
    commands.sync_lisp_keys();
    assert!(commands
        .keymap()
        .lock()
        .unwrap()
        .keys_for("hello")
        .contains(&"C-c h".to_string()));
    // 出し方の設定を知らせる
    commands
        .eval("(set! truncate-lines #t) (set! selective-display 2)")
        .unwrap();
    press(commands, "C-f");
    assert_eq!(
        *commands.host().ui().display.lock().unwrap(),
        (true, Some(2))
    );
    // 変更したコマンドの終わりに, マークを外す
    press(commands, "C-SPC a");
    assert!(!commands.host().editor().current_buffer().mark_active());
}

#[test]
fn test_command_hooks() {
    let commands = &mut setup();
    commands
        .eval(
            "(define hook-log '()) \
             (add-hook 'pre-command-hook (lambda () (set! hook-log (cons (list 'pre this-command) hook-log)))) \
             (add-hook 'post-command-hook (lambda () (set! hook-log (cons (list 'post this-command) hook-log))))",
        )
        .unwrap();
    press(commands, "C-a");
    assert_eq!(
        commands.eval("(reverse hook-log)").unwrap(),
        "((pre move-beginning-of-line) (post move-beginning-of-line))"
    );
    // フックの失敗は, エコー領域に出す
    commands
        .eval("(set! post-command-hook (list (lambda () (error \"boom\"))))")
        .unwrap();
    press(commands, "C-e");
    assert!(message(commands).starts_with("Error in post-command-hook:"));
    assert!(message(commands).contains("boom"));
    // リストでない値 (手続き 1 つ) も, フックとして走らせる
    commands
        .eval("(set! post-command-hook '()) (set! pre-command-hook (lambda () (set! hook-log 'single)))")
        .unwrap();
    press(commands, "C-a");
    assert_eq!(commands.eval("hook-log").unwrap(), "single");
}

#[test]
fn test_window_keys() {
    let commands = &mut setup();
    commands.host().editor().set_screen_size(25, 80);
    let buffers = |commands: &mut CommandLoop<Ui>| {
        commands
            .eval("(list (map window-buffer (window-list)) (window-buffer (selected-window)) (window-dedicated-p))")
            .unwrap()
    };
    commands.eval("(get-buffer-create \"a\")").unwrap();
    // C-x 4 4: 次のコマンドのバッファを, 新しいウィンドウに出す
    press(commands, "C-x 4 4");
    assert_eq!(
        message(commands),
        "Display next command buffer in a new window..."
    );
    commands.eval("(switch-to-buffer \"a\")").unwrap();
    press(commands, "C-a");
    assert_eq!(buffers(commands), "((\"*scratch*\" \"a\") \"a\" #f)");
    // 次のコマンドがバッファを出さなければ, 忘れる
    press(commands, "C-x 4 1 C-a");
    commands
        .eval("(switch-to-buffer-other-window \"*scratch*\")")
        .unwrap();
    assert_eq!(
        buffers(commands),
        "((\"*scratch*\" \"a\") \"*scratch*\" #f)"
    );
    // C-x 4 1 の後の C-x 4 C-o は, 選んだウィンドウに出す
    press(commands, "C-x 4 1");
    commands.eval("(display-buffer \"a\")").unwrap();
    press(commands, "C-a");
    assert_eq!(buffers(commands), "((\"a\" \"a\") \"a\" #f)");
    press(commands, "C-x 4 4");
    commands.eval("(display-buffer \"*scratch*\")").unwrap();
    assert_eq!(buffers(commands), "((\"a\" \"*scratch*\") \"a\" #f)");
    // C-x w d: 専用にする. C-u なら強く
    press(commands, "C-x w d");
    assert_eq!(message(commands), "Window is now dedicated to buffer a");
    assert_eq!(
        buffers(commands),
        "((\"a\" \"*scratch*\") \"a\" interactive)"
    );
    press(commands, "C-x w d C-u C-x w d");
    assert_eq!(
        message(commands),
        "Window is now strongly dedicated to buffer a"
    );
    commands
        .eval("(set! toggle-window-dedicated-flag #t)")
        .unwrap();
    press(commands, "C-x w d C-u C-x w d");
    assert_eq!(message(commands), "Window is now dedicated to buffer a");
    press(commands, "C-x w d");
    assert_eq!(
        message(commands),
        "Window is no longer dedicated to buffer a"
    );
    commands
        .eval("(toggle-window-dedicated #f 'side #f) (toggle-window-dedicated #f #f #t)")
        .unwrap();
    assert_eq!(buffers(commands), "((\"a\" \"*scratch*\") \"a\" #f)");
    // windmove: S-<矢印> に割り当てる
    commands.eval("(windmove-default-keybindings)").unwrap();
    press(commands, "S-<down>");
    assert_eq!(
        buffers(commands),
        "((\"a\" \"*scratch*\") \"*scratch*\" #f)"
    );
    press(commands, "S-<down>");
    assert_eq!(message(commands), "Minibuffer is inactive");
    press(commands, "S-<left>");
    assert_eq!(message(commands), "No window left from selected window");
    commands
        .eval("(windmove-default-keybindings '(control meta)) (windmove-default-keybindings 'none)")
        .unwrap();
    press(commands, "C-M-<up>");
    assert_eq!(buffers(commands), "((\"a\" \"*scratch*\") \"a\" #f)");
    press(commands, "<down>");
    assert_eq!(
        buffers(commands),
        "((\"a\" \"*scratch*\") \"*scratch*\" #f)"
    );
    assert!(commands
        .eval("(windmove-default-keybindings 'alt)")
        .is_err());
    assert!(commands.eval("(window-in-direction 'north)").is_err());
}

#[test]
fn test_winner_keys() {
    let commands = &mut setup();
    commands.host().editor().set_screen_size(25, 80);
    let count = |commands: &mut CommandLoop<Ui>| commands.eval("(length (window-list))").unwrap();
    commands.eval("(winner-mode 1)").unwrap();
    // 画面を描く場合は, 変わったウィンドウの設定を, 次のコマンドの後で覚える
    press(commands, "C-x 2 C-a C-x 3 C-a C-x 1 C-a");
    press(commands, "C-c <left>");
    assert_eq!(message(commands), "Winner undo (1 / 3)");
    assert_eq!(count(commands), "3");
    press(commands, "C-c <left>");
    assert_eq!(message(commands), "Winner undo (2 / 3)");
    assert_eq!(count(commands), "2");
    press(commands, "C-c <left> C-c <left>");
    assert_eq!(
        message(commands),
        "No further window configuration undo information"
    );
    press(commands, "C-c <right>");
    assert_eq!(message(commands), "Winner undid undo");
    press(commands, "C-c <right>");
    assert_eq!(
        message(commands),
        "Previous command was not a `winner-undo'"
    );
    // 退屈なバッファのウィンドウは, 戻さない
    commands
        .eval("(delete-other-windows) (split-window-below) (set-window-buffer (cadr (window-list)) (get-buffer-create \"*Completions*\"))")
        .unwrap();
    press(commands, "C-a C-x 1 C-a C-c <left>");
    assert_eq!(
        commands
            .eval("(member \"*Completions*\" (map window-buffer (window-list)))")
            .unwrap(),
        "#f"
    );
    // 覚える設定の数には限りがある
    commands.eval("(set! winner-ring-size 2)").unwrap();
    press(commands, "C-x 2 C-a C-x 1 C-a C-x 2 C-a");
    assert_eq!(commands.eval("(length winner--ring)").unwrap(), "2");
    // 止めると, 覚えない
    commands.eval("(winner-mode 0)").unwrap();
    press(commands, "C-c <left>");
    assert_eq!(message(commands), "Winner mode is turned off");
    commands
        .eval("(simple--call-interactively 'winner-mode)")
        .unwrap();
    assert_eq!(message(commands), "Winner mode enabled");
}
