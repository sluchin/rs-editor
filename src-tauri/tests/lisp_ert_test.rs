//! リポジトリの `tests/lisp/*.scm` (ERT で書いた Scheme のテスト) を, 偽のエディタの上で実行する.
//! Emacs の `emacs -batch -l ert -l test.el -f ert-run-tests-batch-and-exit` に当たる.
//! `load-path` には, リポジトリの `lisp/` を入れる (テストは `(require 'howm)` で読める).

use std::fs;
use std::path::{Path, PathBuf};

use rseditor::scheme::evaluator::Evaluator;

mod common;
use common::FakeEditor;

/// リポジトリの `tests/lisp/*.scm` (名前の順).
///
/// # Returns
///
/// パス.
fn scheme_test_files() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/lisp");
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|e| e == "scm"))
        .collect();
    files.sort();
    files
}

#[test]
fn test_scheme_test_files() {
    let files = scheme_test_files();
    assert!(!files.is_empty());
    let lisp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lisp");
    for file in files {
        // ファイルごとに, 新しい評価器とエディタで
        let ev = Evaluator::new();
        let editor = FakeEditor::new();
        let code = format!(
            "(set! load-path (cons {:?} load-path)) (load {:?}) (ert-run-tests-batch-and-exit)",
            lisp.to_string_lossy(),
            file.to_string_lossy()
        );
        match ev.eval_with(&code, &editor) {
            Ok(report) => println!("{}\n{}", file.display(), report),
            Err(report) => panic!("{}\n{}", file.display(), report),
        }
    }
}
