//! 起動のときに読む Lisp (Emacs の loadup.el に当たる). ビルドのときに埋め込み, 評価器を作るときに, この順で評価します.

/// 読むファイル (`lisp/` からのパスと, 中身).
pub const FILES: &[(&str, &str)] = &[
    ("lisp/files.scm", include_str!("../../../lisp/files.scm")),
    (
        "lisp/help-fns.scm",
        include_str!("../../../lisp/help-fns.scm"),
    ),
];
