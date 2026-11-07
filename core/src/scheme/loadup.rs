//! 起動のときに読む Lisp (Emacs の loadup.el に当たる). ビルドのときに埋め込み, 評価器を作るときに, この順で評価します.

/// 読むファイル (`lisp/` からのパスと, 中身).
pub const FILES: &[(&str, &str)] = &[
    ("lisp/subr.scm", include_str!("../../../lisp/subr.scm")),
    ("lisp/window.scm", include_str!("../../../lisp/window.scm")),
    ("lisp/files.scm", include_str!("../../../lisp/files.scm")),
    (
        "lisp/help-fns.scm",
        include_str!("../../../lisp/help-fns.scm"),
    ),
    (
        "lisp/compile.scm",
        include_str!("../../../lisp/compile.scm"),
    ),
    ("lisp/help.scm", include_str!("../../../lisp/help.scm")),
    ("lisp/dired.scm", include_str!("../../../lisp/dired.scm")),
    (
        "lisp/bookmark.scm",
        include_str!("../../../lisp/bookmark.scm"),
    ),
    (
        "lisp/buff-menu.scm",
        include_str!("../../../lisp/buff-menu.scm"),
    ),
];

/// エディタの基本のコマンド (Emacs の `simple.el` などに当たる). 画面とターミナルのアプリが, コマンドループを作るときに,
/// この順で読む ([`crate::scheme::evaluator::Evaluator::load_editor_files`]). 評価器だけを使うテストでは読まない.
pub const EDITOR_FILES: &[(&str, &str)] = &[
    ("lisp/simple.scm", include_str!("../../../lisp/simple.scm")),
    (
        "lisp/replace.scm",
        include_str!("../../../lisp/replace.scm"),
    ),
    (
        "lisp/isearch.scm",
        include_str!("../../../lisp/isearch.scm"),
    ),
    (
        "lisp/uniquify.scm",
        include_str!("../../../lisp/uniquify.scm"),
    ),
    (
        "lisp/windmove.scm",
        include_str!("../../../lisp/windmove.scm"),
    ),
    ("lisp/winner.scm", include_str!("../../../lisp/winner.scm")),
    ("lisp/rect.scm", include_str!("../../../lisp/rect.scm")),
    ("lisp/sort.scm", include_str!("../../../lisp/sort.scm")),
    (
        "lisp/register.scm",
        include_str!("../../../lisp/register.scm"),
    ),
    ("lisp/abbrev.scm", include_str!("../../../lisp/abbrev.scm")),
    (
        "lisp/minibuffer.scm",
        include_str!("../../../lisp/minibuffer.scm"),
    ),
    (
        "lisp/dabbrev.scm",
        include_str!("../../../lisp/dabbrev.scm"),
    ),
    (
        "lisp/hippie-exp.scm",
        include_str!("../../../lisp/hippie-exp.scm"),
    ),
    ("lisp/xref.scm", include_str!("../../../lisp/xref.scm")),
    ("lisp/info.scm", include_str!("../../../lisp/info.scm")),
];
