//! キーの割り当て (Emacs の global-map の, 基本のもの).

/// キーを引いた結果.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    /// コマンド.
    Command(String),
    /// 続きのキーを待つ (`C-x` など).
    Prefix,
    /// 割り当てが無い.
    Unbound,
}

/// 始めの割り当て (キーの並びと, コマンド).
const DEFAULT: &[(&str, &str)] = &[
    ("C-f", "forward-char"),
    ("<right>", "forward-char"),
    ("C-b", "backward-char"),
    ("<left>", "backward-char"),
    ("C-n", "next-line"),
    ("<down>", "next-line"),
    ("C-p", "previous-line"),
    ("<up>", "previous-line"),
    ("C-a", "move-beginning-of-line"),
    ("<home>", "move-beginning-of-line"),
    ("C-e", "move-end-of-line"),
    ("<end>", "move-end-of-line"),
    ("M-<", "beginning-of-buffer"),
    ("M->", "end-of-buffer"),
    ("M-f", "forward-word"),
    ("M-b", "backward-word"),
    ("M-m", "back-to-indentation"),
    ("C-x C-n", "set-goal-column"),
    ("M-g g", "goto-line"),
    ("M-g M-g", "goto-line"),
    ("C-v", "scroll-up-command"),
    ("<next>", "scroll-up-command"),
    ("M-v", "scroll-down-command"),
    ("<prior>", "scroll-down-command"),
    ("C-l", "recenter-top-bottom"),
    ("RET", "newline"),
    ("TAB", "indent-for-tab-command"),
    ("DEL", "delete-backward-char"),
    ("C-d", "delete-forward-char"),
    ("<delete>", "delete-forward-char"),
    ("C-k", "kill-line"),
    ("M-d", "kill-word"),
    ("M-DEL", "backward-kill-word"),
    ("C-w", "kill-region"),
    ("M-w", "kill-ring-save"),
    ("C-t", "transpose-chars"),
    ("M-t", "transpose-words"),
    ("C-x C-t", "transpose-lines"),
    ("C-o", "open-line"),
    ("C-M-o", "split-line"),
    ("M-z", "zap-to-char"),
    ("M-\\", "delete-horizontal-space"),
    ("M-^", "delete-indentation"),
    ("C-x C-o", "delete-blank-lines"),
    ("M-SPC", "cycle-spacing"),
    ("M-j", "default-indent-new-line"),
    ("C-M-j", "default-indent-new-line"),
    ("C-q", "quoted-insert"),
    ("C-M-w", "append-next-kill"),
    ("C-x x i", "insert-buffer"),
    ("C-M-_", "undo-redo"),
    ("C-y", "yank"),
    ("M-y", "yank-pop"),
    ("C-u", "universal-argument"),
    ("M--", "negative-argument"),
    ("C--", "negative-argument"),
    ("M-0", "digit-argument"),
    ("M-1", "digit-argument"),
    ("M-2", "digit-argument"),
    ("M-3", "digit-argument"),
    ("M-4", "digit-argument"),
    ("M-5", "digit-argument"),
    ("M-6", "digit-argument"),
    ("M-7", "digit-argument"),
    ("M-8", "digit-argument"),
    ("M-9", "digit-argument"),
    ("C-M-f", "forward-sexp"),
    ("C-M-b", "backward-sexp"),
    ("C-M-n", "forward-list"),
    ("C-M-p", "backward-list"),
    ("C-M-u", "backward-up-list"),
    ("C-M-d", "down-list"),
    ("C-M-a", "beginning-of-defun"),
    ("C-M-e", "end-of-defun"),
    ("C-M-k", "kill-sexp"),
    ("C-M-DEL", "backward-kill-sexp"),
    ("C-M-SPC", "mark-sexp"),
    ("C-M-@", "mark-sexp"),
    ("C-M-h", "mark-defun"),
    ("M-{", "backward-paragraph"),
    ("M-}", "forward-paragraph"),
    ("M-h", "mark-paragraph"),
    ("M-a", "backward-sentence"),
    ("M-e", "forward-sentence"),
    ("M-k", "kill-sentence"),
    ("C-x DEL", "backward-kill-sentence"),
    ("C-M-t", "transpose-sexps"),
    ("C-x [", "backward-page"),
    ("C-x ]", "forward-page"),
    ("C-x C-p", "mark-page"),
    ("C-SPC", "set-mark-command"),
    ("C-@", "set-mark-command"),
    ("C-x C-x", "exchange-point-and-mark"),
    ("C-x h", "mark-whole-buffer"),
    ("C-/", "undo"),
    ("C-_", "undo"),
    ("C-x u", "undo"),
    ("C-g", "keyboard-quit"),
    ("C-x C-f", "find-file"),
    ("C-x C-s", "save-buffer"),
    ("C-x C-w", "write-file"),
    ("C-x s", "save-some-buffers"),
    ("C-x C-v", "find-alternate-file"),
    ("C-x i", "insert-file"),
    ("C-x C-c", "save-buffers-kill-terminal"),
    ("C-x b", "switch-to-buffer"),
    ("C-x k", "kill-buffer"),
    ("C-x 2", "split-window-below"),
    ("C-x 3", "split-window-right"),
    ("C-x 0", "delete-window"),
    ("C-x 1", "delete-other-windows"),
    ("C-x o", "other-window"),
    ("M-x", "execute-extended-command"),
    ("M-:", "eval-expression"),
    ("M-X", "execute-extended-command-for-buffer"),
    ("C-x M-:", "repeat-complex-command"),
    ("C-h f", "describe-function"),
];

/// キーの割り当て.
#[derive(Debug, Clone)]
pub struct Keymap {
    /// 割り当て (キーの並びと, コマンド). 後ろのものが先に使われる.
    entries: Vec<(Vec<String>, String)>,
}

impl Default for Keymap {
    fn default() -> Self {
        let mut keymap = Keymap {
            entries: Vec::new(),
        };
        for (keys, command) in DEFAULT {
            keymap.bind(keys, command);
        }
        keymap
    }
}

impl Keymap {
    /// キー (`C-x C-f` のように空白で区切る) に, コマンドを割り当てます. 同じキーの割り当ては, 置き換える.
    pub fn bind(&mut self, keys: &str, command: &str) {
        let keys: Vec<String> = keys.split(' ').map(str::to_string).collect();
        self.entries.retain(|(k, _)| *k != keys);
        self.entries.push((keys, command.to_string()));
    }

    /// キーの並びを引きます.
    pub fn lookup(&self, keys: &[String]) -> Lookup {
        if let Some((_, command)) = self.entries.iter().rev().find(|(k, _)| k == keys) {
            return Lookup::Command(command.clone());
        }
        if self
            .entries
            .iter()
            .any(|(k, _)| k.len() > keys.len() && k[..keys.len()] == *keys)
        {
            return Lookup::Prefix;
        }
        Lookup::Unbound
    }

    /// コマンドに割り当てたキー (Emacs の `where-is-internal`).
    pub fn keys_for(&self, command: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|(_, c)| c == command)
            .map(|(k, _)| k.join(" "))
            .collect()
    }
}

/// Lisp で割り当てたキー (CodeMirror の書き方. `Ctrl-h f`) を, Emacs の書き方 (`C-h f`) にします.
pub fn from_codemirror(keys: &str) -> String {
    keys.split(' ')
        .map(|key| {
            let parts: Vec<&str> = key.split('-').collect();
            let (name, modifiers) = parts.split_last().unwrap();
            let mut out = String::new();
            for m in modifiers {
                out.push_str(match *m {
                    "Ctrl" => "C-",
                    "Alt" | "Meta" => "M-",
                    "Shift" => "S-",
                    _ => "",
                });
            }
            // 最後の `-` 自身は, 分けると空の名前になる
            let name = if name.is_empty() { "-" } else { name };
            out.push_str(match name {
                "Enter" => "RET",
                "Backspace" => "DEL",
                "Delete" => "<delete>",
                "Escape" => "ESC",
                "Tab" => "TAB",
                "Space" | " " => "SPC",
                "ArrowUp" => "<up>",
                "ArrowDown" => "<down>",
                "ArrowLeft" => "<left>",
                "ArrowRight" => "<right>",
                "Home" => "<home>",
                "End" => "<end>",
                "PageUp" => "<prior>",
                "PageDown" => "<next>",
                other => other,
            });
            out
        })
        .collect::<Vec<_>>()
        .join(" ")
}
