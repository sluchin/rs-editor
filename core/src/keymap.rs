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
    ("C-x SPC", "rectangle-mark-mode"),
    ("C-x r k", "kill-rectangle"),
    ("C-x r y", "yank-rectangle"),
    ("C-x r t", "string-rectangle"),
    ("C-x r c", "clear-rectangle"),
    ("C-x r d", "delete-rectangle"),
    ("C-x r o", "open-rectangle"),
    ("C-x r M-w", "copy-rectangle-as-kill"),
    ("C-x r N", "rectangle-number-lines"),
    ("C-x r s", "copy-to-register"),
    ("C-x r x", "copy-to-register"),
    ("C-x r i", "insert-register"),
    ("C-x r r", "copy-rectangle-to-register"),
    ("C-x r n", "number-to-register"),
    ("C-x r +", "increment-register"),
    ("C-x r SPC", "point-to-register"),
    ("C-x r C-SPC", "point-to-register"),
    ("C-x r C-@", "point-to-register"),
    ("C-x r w", "window-configuration-to-register"),
    ("C-x r j", "jump-to-register"),
    ("C-x r g", "insert-register"),
    ("M-'", "abbrev-prefix-mark"),
    ("C-x a g", "add-global-abbrev"),
    ("C-x a +", "add-mode-abbrev"),
    ("C-x a l", "add-mode-abbrev"),
    ("C-x a C-a", "add-mode-abbrev"),
    ("C-x '", "expand-abbrev"),
    ("C-x a '", "expand-abbrev"),
    ("C-x a e", "expand-abbrev"),
    ("C-x a -", "inverse-add-global-abbrev"),
    ("C-x a i g", "inverse-add-global-abbrev"),
    ("C-x a i l", "inverse-add-mode-abbrev"),
    ("C-M-/", "dabbrev-completion"),
    ("C-M-i", "complete-symbol"),
    ("M-!", "shell-command"),
    ("M-|", "shell-command-on-region"),
    ("M-&", "async-shell-command"),
    ("C-x C-q", "read-only-mode"),
    ("C-x x r", "rename-buffer"),
    ("C-x x u", "rename-uniquely"),
    ("C-x x n", "clone-buffer"),
    ("C-x 4 c", "clone-indirect-buffer-other-window"),
    ("C-x C-r", "find-file-read-only"),
    ("C-x 4 r", "find-file-read-only-other-window"),
    ("C-x C-d", "list-directory"),
    ("M-~", "not-modified"),
    ("C-x x g", "revert-buffer-quick"),
    ("C-x (", "kmacro-start-macro"),
    ("C-x C-k s", "kmacro-start-macro"),
    ("C-x C-k C-s", "kmacro-start-macro"),
    ("C-x C-k n", "kmacro-name-last-macro"),
    ("C-x C-k C-i", "kmacro-insert-counter"),
    ("C-x C-k e", "kmacro-end-and-call-macro"),
    ("C-x C-k x", "kmacro-to-register"),
    ("<f3>", "kmacro-start-macro-or-insert-counter"),
    ("<f4>", "kmacro-end-or-call-macro"),
    ("C-x )", "kmacro-end-macro"),
    ("C-x e", "kmacro-end-and-call-macro"),
    ("M-u", "upcase-word"),
    ("M-l", "downcase-word"),
    ("M-c", "capitalize-word"),
    ("C-x C-u", "upcase-region"),
    ("C-x C-l", "downcase-region"),
    ("M-g g", "goto-line"),
    ("M-g M-g", "goto-line"),
    ("M-g n", "next-error"),
    ("M-g M-n", "next-error"),
    ("C-x `", "next-error"),
    ("M-g p", "previous-error"),
    ("M-g M-p", "previous-error"),
    ("C-v", "scroll-up-command"),
    ("<next>", "scroll-up-command"),
    ("M-v", "scroll-down-command"),
    ("<prior>", "scroll-down-command"),
    ("C-l", "recenter-top-bottom"),
    ("RET", "newline"),
    ("TAB", "indent-for-tab-command"),
    ("C-M-\\", "indent-region"),
    ("C-x TAB", "indent-rigidly"),
    ("M-i", "tab-to-tab-stop"),
    ("M-q", "fill-paragraph"),
    ("C-x .", "set-fill-prefix"),
    ("M-;", "comment-dwim"),
    ("M-(", "insert-parentheses"),
    ("M-)", "move-past-close-and-reindent"),
    ("C-s", "isearch-forward"),
    ("C-r", "isearch-backward"),
    ("C-M-s", "isearch-forward-regexp"),
    ("C-M-r", "isearch-backward-regexp"),
    ("M-s w", "isearch-forward-word"),
    ("M-s _", "isearch-forward-symbol"),
    ("M-s .", "isearch-forward-symbol-at-point"),
    ("M-s M-.", "isearch-forward-thing-at-point"),
    ("M-%", "query-replace"),
    ("C-M-%", "query-replace-regexp"),
    ("M-s o", "occur"),
    ("M-s h r", "highlight-regexp"),
    ("M-s h u", "unhighlight-regexp"),
    ("M-s h p", "highlight-phrase"),
    ("M-s h l", "highlight-lines-matching-regexp"),
    ("M-s h .", "highlight-symbol-at-point"),
    ("C-x C-;", "comment-line"),
    ("C-x ;", "comment-set-column"),
    ("DEL", "delete-backward-char"),
    ("C-d", "delete-char"),
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
    ("M-/", "dabbrev-expand"),
    ("M-^", "delete-indentation"),
    ("C-x C-o", "delete-blank-lines"),
    ("M-SPC", "cycle-spacing"),
    ("M-j", "default-indent-new-line"),
    ("C-M-j", "default-indent-new-line"),
    ("C-q", "quoted-insert"),
    ("C-M-w", "append-next-kill"),
    ("C-x x i", "insert-buffer"),
    ("C-M-_", "undo-redo"),
    ("C-?", "undo-redo"),
    ("C-S-DEL", "kill-whole-line"),
    ("C-x f", "set-fill-column"),
    ("C-x =", "what-cursor-position"),
    ("C-x 8 RET", "insert-char"),
    ("M-g c", "goto-char"),
    ("M-g TAB", "move-to-column"),
    ("C-x n g", "goto-line-relative"),
    ("C-x x t", "toggle-truncate-lines"),
    ("C-M-S-v", "scroll-other-window-down"),
    ("M-<prior>", "scroll-other-window-down"),
    ("C-M-S-l", "recenter-other-window"),
    ("C-x C-@", "pop-global-mark"),
    ("C-x C-SPC", "pop-global-mark"),
    ("C-x z", "repeat"),
    ("C-<insert>", "kill-ring-save"),
    ("S-<insert>", "yank"),
    ("S-<delete>", "kill-region"),
    ("C-x ESC ESC", "repeat-complex-command"),
    ("C-x M-ESC", "repeat-complex-command"),
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
    ("C-x l", "count-lines-page"),
    ("C-x n n", "narrow-to-region"),
    ("C-x n w", "widen"),
    ("C-x n p", "narrow-to-page"),
    ("C-x n d", "narrow-to-defun"),
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
    ("C-x C-b", "list-buffers"),
    ("C-x C-c", "save-buffers-kill-terminal"),
    ("C-x b", "switch-to-buffer"),
    ("C-x d", "dired"),
    ("C-x r m", "bookmark-set"),
    ("C-x r M", "bookmark-set-no-overwrite"),
    ("C-x r b", "bookmark-jump"),
    ("C-x r l", "bookmark-bmenu-list"),
    ("C-x 4 d", "dired-other-window"),
    ("C-x C-j", "dired-jump"),
    ("C-x 4 C-j", "dired-jump-other-window"),
    ("C-x 4 f", "find-file-other-window"),
    ("C-x 4 C-f", "find-file-other-window"),
    ("C-x k", "kill-buffer"),
    ("C-x 2", "split-window-below"),
    ("C-x 3", "split-window-right"),
    ("C-x 0", "delete-window"),
    ("C-x 1", "delete-other-windows"),
    ("C-x o", "other-window"),
    ("C-x 4 b", "switch-to-buffer-other-window"),
    ("C-x 4 C-o", "display-buffer"),
    ("C-x 4 0", "kill-buffer-and-window"),
    ("C-x 4 4", "other-window-prefix"),
    ("C-x 4 1", "same-window-prefix"),
    ("C-x <left>", "previous-buffer"),
    ("C-x <right>", "next-buffer"),
    ("C-x C-<left>", "previous-buffer"),
    ("C-x C-<right>", "next-buffer"),
    ("C-M-v", "scroll-other-window"),
    ("M-<home>", "beginning-of-buffer-other-window"),
    ("M-<end>", "end-of-buffer-other-window"),
    ("M-r", "move-to-window-line-top-bottom"),
    ("C-x +", "balance-windows"),
    ("C-x ^", "enlarge-window"),
    ("C-x }", "enlarge-window-horizontally"),
    ("C-x {", "shrink-window-horizontally"),
    ("C-x -", "shrink-window-if-larger-than-buffer"),
    ("C-x w 0", "delete-windows-on"),
    ("C-x w 2", "split-root-window-below"),
    ("C-x w 3", "split-root-window-right"),
    ("C-x w -", "fit-window-to-buffer"),
    ("C-x w q", "quit-window"),
    ("C-x w d", "toggle-window-dedicated"),
    ("C-x p !", "project-shell-command"),
    ("C-x p &", "project-async-shell-command"),
    ("C-x p f", "project-find-file"),
    ("C-x p F", "project-or-external-find-file"),
    ("C-x p b", "project-switch-to-buffer"),
    ("C-x p d", "project-find-dir"),
    ("C-x p D", "project-dired"),
    ("C-x p c", "project-compile"),
    ("C-x p k", "project-kill-buffers"),
    ("C-x p p", "project-switch-project"),
    ("C-x p g", "project-find-regexp"),
    ("C-x p G", "project-or-external-find-regexp"),
    ("M-x", "execute-extended-command"),
    ("M-:", "eval-expression"),
    ("C-x C-e", "eval-last-sexp"),
    ("M-=", "count-words-region"),
    ("C-m", "newline"),
    ("C-i", "indent-for-tab-command"),
    ("C-j", "electric-newline-and-maybe-indent"),
    ("C-M-q", "indent-sexp"),
    ("M-@", "mark-word"),
    ("C-]", "abort-recursive-edit"),
    ("C-M-c", "exit-recursive-edit"),
    ("<f8>", "project-tree"),
    ("<f9>", "project-tree-focus"),
    ("M-X", "execute-extended-command-for-buffer"),
    ("C-x M-:", "repeat-complex-command"),
    ("C-h f", "describe-function"),
    ("C-h v", "describe-variable"),
    ("C-h x", "describe-command"),
    ("C-h o", "describe-symbol"),
    ("C-h k", "describe-key"),
    ("C-h m", "describe-mode"),
    ("C-h a", "apropos-command"),
    ("C-h d", "apropos-documentation"),
    ("C-h c", "describe-key-briefly"),
    ("C-h w", "where-is"),
    ("C-h b", "describe-bindings"),
    ("C-h l", "view-lossage"),
    ("C-h e", "view-echo-area-messages"),
    ("C-h s", "describe-syntax"),
    ("C-h ?", "help-for-help"),
    ("C-h C-h", "help-for-help"),
    ("C-h q", "help-quit"),
    ("C-h C-q", "help-quick-toggle"),
    ("C-h C-s", "search-forward-help-for-help"),
    ("C-h 4 s", "help-find-source"),
    ("C-h i", "info"),
    ("C-h 4 i", "info-other-window"),
    ("C-h R", "info-display-manual"),
    ("C-h r", "info-emacs-manual"),
    ("C-h F", "Info-goto-emacs-command-node"),
    ("C-h K", "Info-goto-emacs-key-command-node"),
    ("M-.", "xref-find-definitions"),
    ("M-,", "xref-go-back"),
    ("C-M-,", "xref-go-forward"),
    ("M-?", "xref-find-references"),
    ("C-M-.", "xref-find-apropos"),
    ("C-x 4 .", "xref-find-definitions-other-window"),
];

/// キーの割り当て.
#[derive(Debug, Clone)]
pub struct Keymap {
    /// 割り当て (キーの並びと, コマンド). 後ろのものが先に使われる.
    entries: Vec<(Vec<String>, String)>,
}

/// メジャーモードの割り当て (Emacs の `lisp-interaction-mode-map` などに当たる. モード, キー, コマンド).
/// Scheme はこのエディタの Lisp なので, `scheme-mode` でも評価する.
const MODE_KEYS: &[(&str, &str, &str)] = &[
    ("lisp-interaction-mode", "C-j", "eval-print-last-sexp"),
    ("lisp-interaction-mode", "C-M-x", "eval-defun"),
    ("scheme-mode", "C-M-x", "eval-defun"),
];

/// メジャーモード `mode` の割り当てを返します.
pub fn mode_keymap(mode: &str) -> Keymap {
    let mut keymap = Keymap::empty();
    for (_, keys, command) in MODE_KEYS.iter().filter(|(m, _, _)| *m == mode) {
        keymap.bind(keys, command);
    }
    keymap
}

/// バッファ `buffer` のキー (`local-set-key`) とメジャーモードのキーの割り当てを返します. 全体の割り当てより先に引く
/// (Emacs の `key-binding` と同じく, [`lookup_in`] で引く).
pub fn buffer_keymap(buffer: &crate::buffer::Buffer) -> Keymap {
    let mut keymap = mode_keymap(&buffer.major_mode());
    for (local, command) in buffer.local_keys() {
        keymap.bind(local, command);
    }
    keymap
}

/// キーの並び `keys` を, バッファの割り当て `local` ([`buffer_keymap`]), 全体の割り当て `global` の順に引きます.
pub fn lookup_in(local: &Keymap, global: &Keymap, keys: &[String]) -> Lookup {
    match local.lookup(keys) {
        Lookup::Unbound => global.lookup(keys),
        found => found,
    }
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
    /// 割り当ての無いキーの割り当て (バッファだけのキーに使う).
    pub fn empty() -> Self {
        Keymap {
            entries: Vec::new(),
        }
    }

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

    /// 全ての割り当て (キーの並びと, コマンド. 後で割り当てたもので置き換えたものは, 含めない).
    pub fn bindings(&self) -> Vec<(String, String)> {
        self.entries
            .iter()
            .map(|(keys, command)| (keys.join(" "), command.clone()))
            .collect()
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

/// Shift を外したキーを返します (Emacs の shift-translation. 割り当ての無い `C-S-DEL` を `C-DEL` として引く).
///
/// # Arguments
///
/// * `key` - キー (Emacs の書き方. `C-M-S-v`).
///
/// # Returns
///
/// Shift (`S-`) を外したキー. Shift が無ければ `None`.
pub fn unshifted(key: &str) -> Option<String> {
    let mut rest = key;
    let mut modifiers = Vec::new();
    while rest.len() > 2 && rest.as_bytes()[1] == b'-' && "ACHMSs".contains(&rest[..1]) {
        modifiers.push(&rest[..2]);
        rest = &rest[2..];
    }
    let shift = modifiers.iter().position(|m| *m == "S-")?;
    modifiers.remove(shift);
    Some(modifiers.concat() + rest)
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
