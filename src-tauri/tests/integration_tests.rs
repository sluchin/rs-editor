//! ファイル操作, パーサと評価器の連携, エラー処理, 並行処理, 境界の場合の結合テスト.
//!
//! 画面から呼ぶコマンド ([`rseditor::commands`]) を通して確かめる.

use std::path::Path;

use tempfile::TempDir;

use rseditor::commands::{
    eval_code, load_init_file, path_exists, read_file_content, write_file_content,
};
use rseditor::scheme::evaluator::Evaluator;
use rseditor::scheme::host::NoHost;

/// パスを, コマンドに渡す文字列にする.
///
/// # Arguments
///
/// * `path` - パス.
///
/// # Returns
///
/// パスの文字列.
fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// 一時ディレクトリに, ファイルを書く (コマンドを通す).
///
/// # Arguments
///
/// * `dir` - 一時ディレクトリ.
/// * `name` - ファイルの名前.
/// * `content` - 書く内容.
///
/// # Returns
///
/// ファイルのパス.
fn write(dir: &TempDir, name: &str, content: &str) -> String {
    let path = path_string(&dir.path().join(name));
    write_file_content(path.clone(), content.to_string()).unwrap();
    path
}

/// ファイル操作に関連したテスト.
mod file_operations_tests {
    use super::*;

    #[test]
    fn test_create_and_read_file() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "test.scm", "(+ 1 2)");
        assert!(path_exists(path.clone()));
        assert_eq!(read_file_content(path).unwrap(), "(+ 1 2)");
    }

    #[test]
    fn test_write_file_overwrites_existing() {
        let dir = TempDir::new().unwrap();
        write(&dir, "test.scm", "(+ 1 2)");
        let path = write(&dir, "test.scm", "(* 3 4)");
        assert_eq!(read_file_content(path).unwrap(), "(* 3 4)");
    }

    #[test]
    fn test_save_edited_buffer() {
        // 読んだ内容に書き足して保存すると (C-x C-s), 両方の行が残る.
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "test.scm", "(+ 1 2)");
        let edited = read_file_content(path.clone()).unwrap() + "\n(* 3 4)";
        write_file_content(path.clone(), edited).unwrap();
        assert_eq!(read_file_content(path).unwrap(), "(+ 1 2)\n(* 3 4)");
    }

    #[test]
    fn test_read_nonexistent_file_fails() {
        let dir = TempDir::new().unwrap();
        let missing = path_string(&dir.path().join("none.scm"));
        assert!(!path_exists(missing.clone()));
        assert!(read_file_content(missing).is_err());
    }

    #[test]
    fn test_file_with_special_characters() {
        let dir = TempDir::new().unwrap();
        let content = "(define greeting \"Hello, World!\\nHow are you?\")\n(+ 1 2 3)";
        let path = write(&dir, "test_special.scm", content);
        assert_eq!(read_file_content(path).unwrap(), content);
    }

    #[test]
    fn test_empty_file() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "empty.scm", "");
        assert_eq!(read_file_content(path).unwrap(), "");
    }

    #[test]
    fn test_large_file() {
        let dir = TempDir::new().unwrap();
        let content: String = (0..1000).map(|i| format!("(+ {} 1)\n", i)).collect();
        let path = write(&dir, "large.scm", &content);
        assert_eq!(read_file_content(path).unwrap().lines().count(), 1000);
    }

    #[test]
    fn test_unicode_content() {
        let dir = TempDir::new().unwrap();
        let content =
            "(define greeting \"你好世界\")  ; Hello World in Chinese\n(define emoji \"🎉🚀\")";
        let path = write(&dir, "unicode.scm", content);
        assert_eq!(read_file_content(path).unwrap(), content);
    }
}

/// パーサーと評価器の統合テスト (ファイルを読んで, 評価する).
mod parser_evaluator_integration {
    use super::*;

    #[test]
    fn test_basic_arithmetic_pipeline() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "a.scm", "(+ 1 2)");
        let response = load_init_file(&Evaluator::new(), Some(Path::new(&path)), &NoHost);
        assert_eq!(response.result, "3");
        assert_eq!(response.error, None);
    }

    #[test]
    fn test_nested_expressions_pipeline() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "a.scm", "(+ 1 (* 2 3))");
        let response = load_init_file(&Evaluator::new(), Some(Path::new(&path)), &NoHost);
        assert_eq!(response.result, "7");
    }

    #[test]
    fn test_multiple_expressions_pipeline() {
        // 最上位の式ごとの結果を, 改行で区切る. 定義は, 後の式から使える.
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "a.scm", "(define x 10)\n(* x 2)\n; done\n");
        let evaluator = Evaluator::new();
        let response = load_init_file(&evaluator, Some(Path::new(&path)), &NoHost);
        assert_eq!(response.result, "x\n20");
        // 読んだ定義は, 評価器に残る.
        assert_eq!(eval_code(&evaluator, "(+ x 1)").result, "11");
    }
}

/// エラー処理に関連したテスト.
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_handle_missing_file_gracefully() {
        // ファイルの無い初期設定ファイルは, 何もせずに成功する.
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("none.scm");
        let response = load_init_file(&Evaluator::new(), Some(&missing), &NoHost);
        assert_eq!(response.result, "");
        assert_eq!(response.error, None);
    }

    #[test]
    fn test_handle_invalid_code_in_file() {
        // 読めないコードの失敗には, ファイルのパスを添える.
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "broken.scm", "(+ 1");
        let response = load_init_file(&Evaluator::new(), Some(Path::new(&path)), &NoHost);
        assert_eq!(response.error, Some(format!("{}: Unclosed list", path)));
    }

    #[cfg(unix)]
    #[test]
    fn test_handle_permission_errors() {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let dir = TempDir::new().unwrap();
        let path = write(&dir, "restricted.scm", "content");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
        // root では, 権限が無くても読めるので, 読めないときだけ確かめる.
        if fs::read_to_string(&path).is_err() {
            assert!(read_file_content(path.clone()).is_err());
        }
        // 一時ディレクトリを消せるよう, 権限を戻す.
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    }
}

/// 並行処理に関連したテスト.
mod concurrent_operations_tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_concurrent_reads() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "concurrent.scm", "(+ 1 2)");
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let path = path.clone();
                std::thread::spawn(move || read_file_content(path).unwrap())
            })
            .collect();
        for handle in handles {
            assert_eq!(handle.join().unwrap(), "(+ 1 2)");
        }
    }

    #[test]
    fn test_concurrent_evaluations() {
        // 評価は 1 度に 1 つずつ行うので, 別のスレッドからの定義が, 全て残る.
        let evaluator = Arc::new(Evaluator::new());
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let evaluator = Arc::clone(&evaluator);
                std::thread::spawn(move || eval_code(&evaluator, &format!("(define x{} {})", i, i)))
            })
            .collect();
        for handle in handles {
            assert_eq!(handle.join().unwrap().error, None);
        }
        assert_eq!(eval_code(&evaluator, "(+ x0 x1 x2 x3 x4)").result, "10");
    }

    #[test]
    fn test_sequential_write_read() {
        let dir = TempDir::new().unwrap();
        for i in 0..10 {
            let content = format!("(+ 1 {})", i);
            let path = write(&dir, "sequential.scm", &content);
            assert_eq!(read_file_content(path).unwrap(), content);
        }
    }
}

/// エッジケースに関連したテスト.
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_file_with_only_whitespace() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "whitespace.scm", "   \n\t\n   ");
        assert_eq!(read_file_content(path.clone()).unwrap(), "   \n\t\n   ");
        // 式が無いので, 結果は空.
        let response = load_init_file(&Evaluator::new(), Some(Path::new(&path)), &NoHost);
        assert_eq!(response.result, "");
    }

    #[test]
    fn test_file_with_only_comments() {
        let dir = TempDir::new().unwrap();
        let content = "; This is a comment\n; Another comment\n; More comments";
        let path = write(&dir, "comments.scm", content);
        assert_eq!(read_file_content(path.clone()).unwrap(), content);
        let response = load_init_file(&Evaluator::new(), Some(Path::new(&path)), &NoHost);
        assert_eq!(response.result, "");
        assert_eq!(response.error, None);
    }

    #[test]
    fn test_file_with_long_line() {
        let dir = TempDir::new().unwrap();
        let content = format!("(define x \"{}\")", "a".repeat(10000));
        let path = write(&dir, "longline.scm", &content);
        assert_eq!(read_file_content(path).unwrap(), content);
    }

    #[test]
    fn test_file_with_mixed_line_endings() {
        // 改行は, そのまま読み書きする. 評価では, どちらも空白.
        let dir = TempDir::new().unwrap();
        let content = "(+ 1 2)\r\n(* 3 4)\n(- 5 6)";
        let path = write(&dir, "mixed_endings.scm", content);
        assert_eq!(read_file_content(path.clone()).unwrap(), content);
        let response = load_init_file(&Evaluator::new(), Some(Path::new(&path)), &NoHost);
        assert_eq!(response.result, "3\n12\n-1");
    }

    #[test]
    fn test_file_path_with_special_chars() {
        let dir = TempDir::new().unwrap();
        let path = write(&dir, "test-file_v2 (copy).scm", "(+ 1 2)");
        assert_eq!(read_file_content(path).unwrap(), "(+ 1 2)");
    }
}
