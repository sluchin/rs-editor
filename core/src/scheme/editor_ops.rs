//! エディタの操作の手続き (`forward-char`, `re-search-forward`, `mark`, `split-window` など).
//!
//! 引数の数と型を確かめて, [`Host`](crate::scheme::host::Host) に頼みます. 位置は Emacs と同じく 1 から数えます.
//! 手続きの動きは, [`EditorHost`](crate::scheme::editor_host::EditorHost) にあります.

use crate::scheme::evaluator::Machine;
use crate::scheme::value::Value;

/// 引数の種類.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// 数.
    Number,
    /// 文字列.
    Text,
    /// 文字列か記号.
    Name,
    /// 何でも.
    Any,
}

/// 手続きの引数の決まり: 名前, 引数の種類 (前の `required` 個は省けない), 省けない引数の数.
const SPECS: &[(&str, &[Kind], usize)] = &[
    ("erase-buffer", &[], 0),
    ("buffer-size", &[], 0),
    ("char-after", &[Kind::Number], 0),
    ("char-before", &[Kind::Number], 0),
    ("forward-char", &[Kind::Number], 0),
    ("backward-char", &[Kind::Number], 0),
    ("forward-line", &[Kind::Number], 0),
    ("line-beginning-position", &[Kind::Number], 0),
    ("line-end-position", &[Kind::Number], 0),
    ("bobp", &[], 0),
    ("eobp", &[], 0),
    ("bolp", &[], 0),
    ("eolp", &[], 0),
    ("delete-char", &[Kind::Number], 1),
    ("insert-before-markers", &[Kind::Text], 1),
    ("current-column", &[], 0),
    ("move-to-column", &[Kind::Number], 1),
    ("match-beginning", &[Kind::Number], 1),
    ("match-end", &[Kind::Number], 1),
    ("match-data", &[], 0),
    ("match-string", &[Kind::Number], 1),
    ("set-match-data", &[Kind::Any], 1),
    ("highlighted-regexps", &[], 0),
    (
        "match-substitute-replacement",
        &[Kind::Text, Kind::Any, Kind::Any, Kind::Any, Kind::Number],
        1,
    ),
    (
        "replace-match",
        &[Kind::Text, Kind::Any, Kind::Any, Kind::Any, Kind::Number],
        1,
    ),
    ("mark", &[], 0),
    ("set-mark", &[Kind::Any], 1),
    ("push-mark", &[Kind::Number, Kind::Any, Kind::Any], 0),
    ("set-mark-position", &[Kind::Number], 1),
    ("global-mark-pop", &[], 0),
    ("pop-mark", &[], 0),
    ("deactivate-mark", &[], 0),
    ("region-beginning", &[], 0),
    ("region-end", &[], 0),
    ("use-region-p", &[], 0),
    ("region-active-p", &[], 0),
    ("activate-mark", &[], 0),
    ("deactivate-mark-requested", &[], 0),
    ("set-deactivate-mark", &[Kind::Any], 1),
    ("point-marker", &[], 0),
    ("marker-position", &[Kind::Number], 1),
    ("marker-release", &[Kind::Number], 1),
    ("kill-append", &[Kind::Text, Kind::Any], 2),
    ("kill-region", &[Kind::Number, Kind::Number], 2),
    ("current-kill", &[Kind::Number, Kind::Any], 1),
    ("insert-for-yank", &[Kind::Text], 1),
    ("undo-boundary", &[], 0),
    ("buffer-undo-list", &[], 0),
    ("primitive-undo", &[Kind::Number, Kind::Any], 2),
    ("set-buffer-undo-list", &[Kind::Any], 1),
    ("current-prefix-arg", &[], 0),
    ("set-current-prefix-arg", &[Kind::Any], 1),
    ("prefix-numeric-value", &[Kind::Any], 1),
    ("scan-lists", &[Kind::Number, Kind::Number, Kind::Number], 3),
    ("scan-sexps", &[Kind::Number, Kind::Number], 2),
    ("syntax-ppss", &[Kind::Number], 0),
    ("char-syntax", &[Kind::Any], 1),
    ("syntax-class-at", &[Kind::Number], 1),
    ("forward-comment", &[Kind::Number], 1),
    ("backward-prefix-chars", &[], 0),
    ("up-list-position", &[Kind::Number, Kind::Number], 2),
    ("beginning-of-defun-position", &[Kind::Number], 1),
    ("end-of-defun-position", &[Kind::Number, Kind::Number], 1),
    ("mark-defun-range", &[Kind::Number, Kind::Any], 1),
    (
        "forward-paragraph-position",
        &[Kind::Number, Kind::Number],
        2,
    ),
    (
        "forward-sentence-position",
        &[Kind::Number, Kind::Number],
        2,
    ),
    ("indent-for-tab-op", &[Kind::Any], 1),
    (
        "indent-region-op",
        &[Kind::Number, Kind::Number, Kind::Any],
        3,
    ),
    ("indent-sexp-op", &[Kind::Any], 1),
    ("newline-and-indent-op", &[Kind::Any], 1),
    (
        "fill-paragraph-op",
        &[Kind::Number, Kind::Any, Kind::Any],
        3,
    ),
    ("comment-dwim-op", &[Kind::Any, Kind::Number, Kind::Any], 3),
    (
        "comment-region-op",
        &[Kind::Number, Kind::Number, Kind::Any],
        3,
    ),
    ("syntax-dialect", &[], 0),
    ("set-syntax-dialect", &[Kind::Name], 1),
    ("selected-window", &[], 0),
    ("window-list", &[], 0),
    ("split-window", &[Kind::Name], 0),
    ("delete-window", &[Kind::Number], 0),
    ("delete-other-windows", &[], 0),
    ("other-window", &[Kind::Number], 0),
    ("select-window", &[Kind::Number], 1),
    ("set-window-buffer", &[Kind::Number, Kind::Text], 2),
    ("window-buffer", &[Kind::Number], 0),
    ("get-buffer", &[Kind::Text], 1),
    ("get-buffer-create", &[Kind::Text], 1),
    ("generate-new-buffer", &[Kind::Text], 1),
    ("set-buffer", &[Kind::Text], 1),
    ("current-buffer", &[], 0),
    (
        "search-forward",
        &[Kind::Text, Kind::Any, Kind::Any, Kind::Number],
        1,
    ),
    (
        "search-backward",
        &[Kind::Text, Kind::Any, Kind::Any, Kind::Number],
        1,
    ),
    (
        "re-search-forward",
        &[Kind::Text, Kind::Any, Kind::Any, Kind::Number],
        1,
    ),
    (
        "re-search-backward",
        &[Kind::Text, Kind::Any, Kind::Any, Kind::Number],
        1,
    ),
    ("looking-at", &[Kind::Text], 1),
    ("skip-chars-forward", &[Kind::Text, Kind::Number], 1),
    ("skip-chars-backward", &[Kind::Text, Kind::Number], 1),
    ("skip-syntax-forward", &[Kind::Text, Kind::Number], 1),
    ("skip-syntax-backward", &[Kind::Text, Kind::Number], 1),
    ("window-body-height", &[], 0),
    ("recenter", &[], 0),
    ("kill-emacs", &[], 0),
];

/// 手続きの名前.
pub const NAMES: &[&str] = &[
    "erase-buffer",
    "buffer-size",
    "char-after",
    "char-before",
    "forward-char",
    "backward-char",
    "forward-line",
    "line-beginning-position",
    "line-end-position",
    "bobp",
    "eobp",
    "bolp",
    "eolp",
    "delete-char",
    "insert-before-markers",
    "current-column",
    "move-to-column",
    "match-beginning",
    "match-end",
    "match-data",
    "match-string",
    "set-match-data",
    "highlighted-regexps",
    "match-substitute-replacement",
    "replace-match",
    "mark",
    "set-mark",
    "push-mark",
    "set-mark-position",
    "global-mark-pop",
    "pop-mark",
    "deactivate-mark",
    "region-beginning",
    "region-end",
    "use-region-p",
    "region-active-p",
    "activate-mark",
    "deactivate-mark-requested",
    "set-deactivate-mark",
    "point-marker",
    "marker-position",
    "marker-release",
    "kill-append",
    "kill-region",
    "current-kill",
    "insert-for-yank",
    "undo-boundary",
    "buffer-undo-list",
    "primitive-undo",
    "set-buffer-undo-list",
    "current-prefix-arg",
    "set-current-prefix-arg",
    "prefix-numeric-value",
    "scan-lists",
    "scan-sexps",
    "syntax-ppss",
    "char-syntax",
    "syntax-class-at",
    "forward-comment",
    "backward-prefix-chars",
    "up-list-position",
    "beginning-of-defun-position",
    "end-of-defun-position",
    "mark-defun-range",
    "forward-paragraph-position",
    "forward-sentence-position",
    "indent-for-tab-op",
    "indent-region-op",
    "indent-sexp-op",
    "newline-and-indent-op",
    "fill-paragraph-op",
    "comment-dwim-op",
    "comment-region-op",
    "syntax-dialect",
    "set-syntax-dialect",
    "selected-window",
    "window-list",
    "split-window",
    "delete-window",
    "delete-other-windows",
    "other-window",
    "select-window",
    "set-window-buffer",
    "window-buffer",
    "get-buffer",
    "get-buffer-create",
    "generate-new-buffer",
    "set-buffer",
    "current-buffer",
    "search-forward",
    "search-backward",
    "re-search-forward",
    "re-search-backward",
    "looking-at",
    "skip-chars-forward",
    "skip-chars-backward",
    "skip-syntax-forward",
    "skip-syntax-backward",
    "window-body-height",
    "recenter",
    "kill-emacs",
];

/// 引数が決まりに合うか確かめます.
fn check(name: &str, kinds: &[Kind], required: usize, args: &[Value]) -> Result<(), String> {
    if args.len() < required || args.len() > kinds.len() {
        return Err(if required == kinds.len() {
            format!("{name} expects {required} arguments")
        } else {
            format!("{name} expects {required} to {} arguments", kinds.len())
        });
    }
    for (arg, kind) in args.iter().zip(kinds) {
        // 省ける引数は, nil でもよい
        let matches = match kind {
            Kind::Number => matches!(arg, Value::Number(_)),
            Kind::Text => matches!(arg, Value::String(_)),
            Kind::Name => matches!(arg, Value::String(_) | Value::Symbol(_)),
            Kind::Any => true,
        };
        if !matches && arg.is_non_nil() {
            return Err(format!(
                "{name} expects {}",
                match kind {
                    Kind::Number => "a number",
                    Kind::Text => "a string",
                    _ => "a name",
                }
            ));
        }
    }
    Ok(())
}

/// 手続きを動かします.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub(crate) fn call(
    machine: &mut Machine,
    name: &str,
    mut args: Vec<Value>,
) -> Result<Value, String> {
    let (_, kinds, required) = SPECS
        .iter()
        .find(|(n, _, _)| *n == name)
        .expect("a name in NAMES has a spec");
    check(name, kinds, *required, &args)?;
    // 探す手続きは, 大文字と小文字を区別しないか (`case-fold-search`. 始めは真) を, 最後の引数で渡す
    if name.contains("search") || name == "looking-at" {
        while args.len() < kinds.len() {
            args.push(Value::Boolean(false));
        }
        let fold = machine
            .evaluator
            .global()
            .get("case-fold-search")
            .is_none_or(|v| v.is_true());
        args.push(Value::Boolean(fold));
    }
    machine.host.call(name, args)
}
