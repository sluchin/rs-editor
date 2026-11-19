//! エディタの操作の手続き (`forward-char`, `re-search-forward`, `mark`, `split-window` など).
//!
//! 引数の数と型を確かめて, [`Host`](crate::scheme::host::Host) に頼みます. 位置は Emacs と同じく 1 から数えます.
//! 手続きの動きは, [`EditorHost`](crate::scheme::editor_host::EditorHost) にあります.

use crate::error::Error;
use crate::scheme::evaluator::Machine;
use crate::scheme::registry::Registry;
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
    (
        "casify-region",
        &[Kind::Text, Kind::Number, Kind::Number],
        3,
    ),
    ("insert-before-markers", &[Kind::Text], 1),
    ("current-column", &[], 0),
    ("move-to-column", &[Kind::Number], 1),
    ("match-beginning", &[Kind::Number], 1),
    ("match-end", &[Kind::Number], 1),
    ("match-data", &[], 0),
    ("match-string", &[Kind::Number, Kind::Any], 1),
    (
        "string-match--op",
        &[Kind::Text, Kind::Text, Kind::Number, Kind::Any, Kind::Any],
        5,
    ),
    ("set-match-data", &[Kind::Any], 1),
    ("highlighted-regexps", &[], 0),
    ("token-spans", &[], 0),
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
    ("rectangle-mark-p", &[], 0),
    ("set-rectangle-mark", &[Kind::Any], 1),
    ("deactivate-mark-requested", &[], 0),
    ("set-deactivate-mark", &[Kind::Any], 1),
    ("point-marker", &[], 0),
    ("marker-position", &[Kind::Number], 1),
    ("marker-release", &[Kind::Number], 1),
    ("marker-set", &[Kind::Number, Kind::Number], 2),
    ("narrow-to-region", &[Kind::Number, Kind::Number], 2),
    ("widen", &[], 0),
    ("buffer-narrowed-p", &[], 0),
    ("restriction-save", &[], 0),
    ("restriction-restore", &[Kind::Number], 1),
    ("kill-append", &[Kind::Text, Kind::Any], 2),
    ("kill-region", &[Kind::Number, Kind::Number], 2),
    ("start-kbd-macro", &[Kind::Any], 0),
    ("end-kbd-macro", &[], 0),
    ("kbd-macro-last", &[], 0),
    ("kbd-macro-defining-p", &[], 0),
    ("kill-ring-length", &[], 0),
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
    (
        "end-of-defun-position",
        &[Kind::Number, Kind::Number, Kind::Any],
        1,
    ),
    (
        "mark-defun-range",
        &[Kind::Number, Kind::Any, Kind::Number],
        1,
    ),
    ("forward-word-position", &[Kind::Number, Kind::Number], 2),
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
    ("plain-newline-and-indent-op", &[Kind::Any], 1),
    (
        "fill-paragraph-op",
        &[Kind::Number, Kind::Any, Kind::Any],
        3,
    ),
    (
        "fill-text-op",
        &[
            Kind::Number,
            Kind::Number,
            Kind::Number,
            Kind::Any,
            Kind::Any,
        ],
        5,
    ),
    ("comment-dwim-op", &[Kind::Any, Kind::Number, Kind::Any], 3),
    (
        "comment-region-op",
        &[Kind::Number, Kind::Number, Kind::Any],
        3,
    ),
    ("syntax-dialect", &[], 0),
    ("set-syntax-dialect", &[Kind::Name], 1),
    ("major-mode-name", &[], 0),
    ("file-local-mode", &[], 0),
    ("set-major-mode-name", &[Kind::Name], 1),
    ("selected-window", &[], 0),
    ("window-list", &[], 0),
    ("split-window", &[Kind::Name], 0),
    ("split-root-window", &[Kind::Name], 0),
    ("window-balance", &[], 0),
    ("window-resize", &[Kind::Number, Kind::Number, Kind::Any], 2),
    ("window-total-height", &[Kind::Number], 0),
    ("window-total-width", &[Kind::Number], 0),
    ("window-edges", &[Kind::Number], 0),
    ("window-dedicated-p", &[Kind::Number], 0),
    ("set-window-dedicated-p", &[Kind::Number, Kind::Any], 1),
    ("delete-window", &[Kind::Number], 0),
    ("delete-other-windows", &[], 0),
    ("set-tab-bar", &[Kind::Any, Kind::Number], 2),
    ("current-window-configuration", &[], 0),
    ("set-window-configuration", &[Kind::Number], 1),
    ("other-window", &[Kind::Number], 0),
    ("select-window", &[Kind::Number], 1),
    ("set-window-buffer", &[Kind::Number, Kind::Text], 2),
    ("window-buffer", &[Kind::Number], 0),
    ("get-buffer", &[Kind::Text], 1),
    ("get-buffer-create", &[Kind::Text], 1),
    ("generate-new-buffer", &[Kind::Text], 1),
    ("files--set-buffer-file-name", &[Kind::Any], 1),
    ("verify-visited-file-modtime", &[], 0),
    ("set-visited-file-modtime", &[], 0),
    ("files--buffer-backed-up-p", &[], 0),
    ("files--set-buffer-backed-up", &[Kind::Any], 1),
    ("do-auto-save", &[], 0),
    ("generate-new-buffer-name", &[Kind::Text, Kind::Any], 1),
    ("rename-buffer", &[Kind::Text, Kind::Any], 1),
    ("buffer-read-only", &[], 0),
    ("barf-if-buffer-read-only", &[], 0),
    ("set-buffer-read-only", &[Kind::Any], 1),
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
    ("executing-kbd-macro-p", &[], 0),
    ("window-body-height", &[], 0),
    ("move-to-window-line", &[Kind::Number], 0),
    ("recenter", &[], 0),
    ("redisplay", &[], 0),
    ("quit-requested-p", &[], 0),
    ("kill-emacs", &[Kind::Any], 0),
    ("toggle-project-tree", &[Kind::Any], 0),
    ("gui-set-selection", &[Kind::Text], 1),
    ("gui-selection-value", &[], 0),
];

/// エディタの操作の手続きを登録します (名前は [`SPECS`] の全て).
///
/// # Arguments
///
/// * `registry` - 登録先の表.
pub fn register(registry: &mut Registry) {
    for (name, _, _) in SPECS {
        registry.register(name, call);
    }
}

/// 引数が決まりに合うか確かめます.
fn check(name: &str, kinds: &[Kind], required: usize, args: &[Value]) -> Result<(), Error> {
    if args.len() < required || args.len() > kinds.len() {
        return Err(Error::wrong_number_of_arguments(name, args.len()));
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
            )
            .into());
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
fn call(machine: &mut Machine, name: &str, mut args: Vec<Value>) -> Result<Value, Error> {
    let (_, kinds, required) = SPECS
        .iter()
        .find(|(n, _, _)| *n == name)
        .expect("a registered name has a spec");
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
