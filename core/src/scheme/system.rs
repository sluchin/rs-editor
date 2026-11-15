//! 組み込みの手続きのうち, ファイル, 時刻, 正規表現を使うもの (`load`, `read-file-string`, `rust-grep` など).
//!
//! 相対パスは, 読み込んでいるファイル (`load`) のディレクトリ, またはホームディレクトリから数えます.

use crate::error::Error;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::format::{Item, StrftimeItems};
use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use regex::Regex;

use crate::atomic::write_atomic;
use crate::register;
use crate::scheme::builtins::{fixed, index, number, string};
use crate::scheme::evaluator::Machine;
use crate::scheme::parser::Parser;
use crate::scheme::registry::Registry;
use crate::scheme::value::Value;

/// URL を開くプログラムの既定 (Lisp の `browse-url-generic-program` の始めの値).
pub const DEFAULT_BROWSER: &str = if cfg!(target_os = "macos") {
    "open"
} else if cfg!(windows) {
    "explorer"
} else {
    "xdg-open"
};

/// `load-path` の始めの値 (Emacs の `~/.emacs.d` のように, `~/.rseditor.d` に Lisp のファイルを置く).
pub const DEFAULT_LOAD_PATH: &[&str] = &["~/.rseditor.d/lisp", "~/.rseditor.d"];

/// Lisp のファイルの拡張子 (`load` で省ける).
pub const LISP_SUFFIX: &str = ".scm";

/// 正規表現を覚えておく数 (超えたら全て忘れる).
const REGEXP_CACHE_SIZE: usize = 64;

thread_local! {
    /// 作った正規表現 (同じ正規表現を, 何度も作らないように).
    static REGEXPS: std::cell::RefCell<HashMap<String, Regex>> = std::cell::RefCell::new(HashMap::new());
}

/// 組み込みの手続き (ファイル, 時刻, 正規表現) を登録します.
///
/// # Arguments
///
/// * `registry` - 登録先の表.
pub fn register(registry: &mut Registry) {
    register!(
        registry,
        "load" => sys_load,
        "require" => require,
        "provide" => sys_provide,
        "featurep" => sys_featurep,
        "expand-file-name" => sys_expand_file_name,
        "file-exists?" => sys_file_exists_and_file_directory,
        "file-directory?" => sys_file_exists_and_file_directory,
        "read-file-string" => sys_read_file_string,
        "write-file-string" => sys_write_file_string,
        "make-directory" => sys_make_directory,
        "directory-listing" => sys_directory_listing,
        "make-temp-file" => sys_make_temp_file,
        "delete-file" => sys_delete_file,
        "rename-file" => sys_rename_file,
        "directory-files" => sys_directory_files,
        "copy-file" => sys_copy_file,
        "delete-directory" => sys_delete_directory,
        "directory-files-recursively" => sys_directory_files_recursively,
        "grep-files" => sys_grep_files,
        "file-modification-time" => sys_file_modification_time,
        "file-newer-than-file-p" => sys_file_newer_than_file_p,
        "current-time" => sys_current_time,
        "format-time-string" => sys_format_time_string,
        "date-to-time" => sys_date_to_time,
        "random" => sys_random,
        "regexp-match" => sys_regexp_match_and_regexp_match_positions,
        "regexp-match-positions" => sys_regexp_match_and_regexp_match_positions,
        "regexp-quote" => sys_regexp_quote,
        "rust-grep" => sys_rust_grep,
        "rust-zgrep" => sys_rust_zgrep,
        "getenv" => sys_getenv,
        "browse-url" => sys_browse_url,
        "call-shell-command" => sys_call_shell_command,
    );
}

/// `(load FILE)`: Lisp のファイルを探して読み込みます.
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
fn sys_load(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[file]| {
        let file = string(name, &file)?;
        let path = locate_library(machine, file)?;
        load_file(machine, &path)
    })
}

/// `(provide FEATURE)`: 機能を提供済みにします.
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
fn sys_provide(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[feature]| match feature {
        Value::Symbol(feature) => {
            machine.evaluator.provide(&feature);
            Ok(Value::Symbol(feature))
        }
        _ => Err("provide expects a symbol".to_string().into()),
    })
}

/// `(featurep FEATURE)`: 機能が提供済みかを調べます.
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
fn sys_featurep(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[feature]| match feature {
        Value::Symbol(feature) => Ok(Value::Boolean(machine.evaluator.featurep(&feature))),
        _ => Err("featurep expects a symbol".to_string().into()),
    })
}

/// `(expand-file-name NAME [DIR])`: ファイルの名前を, 絶対パスにします (`~` も展開する).
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
fn sys_expand_file_name(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    if !(1..=2).contains(&args.len()) {
        return Err("expand-file-name expects 1 or 2 arguments"
            .to_string()
            .into());
    }
    expand_args(machine, name, &args).map(|path| Value::String(path_string(&path)))
}

/// `(file-exists? PATH)` / `(file-directory? PATH)`: ファイルがあるか, ディレクトリかを調べます.
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
fn sys_file_exists_and_file_directory(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        let path = expand_args(machine, name, &args)?;
        Ok(Value::Boolean(if name == "file-exists?" {
            path.exists()
        } else {
            path.is_dir()
        }))
    })
}

/// `(read-file-string PATH)`: ファイルの中身を文字列で返します.
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
fn sys_read_file_string(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        let path = expand_args(machine, name, &args)?;
        std::fs::read_to_string(&path)
            .map(Value::String)
            .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))
    })
}

/// `(write-file-string PATH TEXT)`: 文字列をファイルに書きます.
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
fn sys_write_file_string(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<2>(name, args).and_then(|[path, text]| {
        let path = expand_args(machine, name, &[path])?;
        let text = string(name, &text)?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| Error::from(format!("{}: {}", dir.display(), e)))?;
        }
        write_atomic(&path, text.as_bytes())
            .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
        Ok(Value::Nil)
    })
}

/// `(make-directory PATH)`: ディレクトリを (親も含めて) 作ります.
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
fn sys_make_directory(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        let path = expand_args(machine, name, &args)?;
        std::fs::create_dir_all(&path)
            .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
        Ok(Value::Nil)
    })
}

/// `(directory-listing DIR)`: ディレクトリの一覧 (`ls` の出力) を返します.
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
fn sys_directory_listing(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        let path = expand_args(machine, name, &args)?;
        directory_listing("ls", &path).map(Value::String)
    })
}

/// `(make-temp-file PREFIX [DIR-FLAG])`: 一時ファイル (かディレクトリ) を作って, パスを返します.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_make_temp_file(
    _machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    make_temp_file(name, &args)
}

/// `(delete-file PATH)`: ファイルを消します.
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
fn sys_delete_file(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        let path = expand_args(machine, name, &args)?;
        std::fs::remove_file(&path)
            .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
        Ok(Value::Nil)
    })
}

/// `(rename-file FILE NEWNAME [OK-IF-ALREADY-EXISTS])`: ファイルの名前を変えます.
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
fn sys_rename_file(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    // (rename-file FILE NEWNAME [OK-IF-ALREADY-EXISTS]): 3 つ目が偽なら, 既にあるファイルは置き換えない
    let (from, to, ok) = match args.as_slice() {
        [from, to] => (from, to, false),
        [from, to, ok] => (from, to, ok.is_true()),
        _ => return Err("rename-file expects 2 or 3 arguments".to_string().into()),
    };
    (|| {
        let from = expand_args(machine, name, std::slice::from_ref(from))?;
        let to = expand_args(machine, name, std::slice::from_ref(to))?;
        if !ok && to.exists() {
            return Err(Error::strings(
                "file-already-exists",
                &["File already exists", &to.display().to_string()],
            ));
        }
        std::fs::rename(&from, &to)
            .map(|()| Value::Nil)
            .map_err(|e| Error::from(format!("{}: {}", from.display(), e)))
    })()
}

/// `(directory-files DIR [FULL MATCH])`: ディレクトリの中の名前の並びを返します.
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
fn sys_directory_files(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    // (directory-files DIR [FULL MATCH]): 名前の並び (. と .. を含み, 名前の順). FULL が真ならディレクトリを付ける. MATCH は正規表現
    if !(1..=3).contains(&args.len()) {
        return Err("directory-files expects 1 to 3 arguments"
            .to_string()
            .into());
    }
    (|| {
        let dir = expand_args(machine, name, &args[..1])?;
        let full = args.get(1).is_some_and(Value::is_true);
        let pattern = match args.get(2) {
            Some(Value::String(pattern)) => Some(
                regex::Regex::new(pattern)
                    .map_err(|e| Error::strings("invalid-regexp", &[&e.to_string()]))?,
            ),
            _ => None,
        };
        let entries = std::fs::read_dir(&dir)
            .map_err(|e| Error::from(format!("Opening directory: {}, {}", e, dir.display())))?;
        let mut names = vec![".".to_string(), "..".to_string()];
        names.extend(
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned()),
        );
        names.sort();
        Ok(Value::List(
            names
                .into_iter()
                .filter(|name| pattern.as_ref().is_none_or(|re| re.is_match(name)))
                .map(|name| {
                    Value::String(if full {
                        path_string(&dir.join(&name))
                    } else {
                        name
                    })
                })
                .collect(),
        ))
    })()
}

/// `(copy-file FILE NEWNAME [OK-IF-ALREADY-EXISTS])`: ファイルを写します.
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
fn sys_copy_file(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    // (copy-file FILE NEWNAME [OK-IF-ALREADY-EXISTS]): 3 つ目が偽なら, 既にあるファイルは置き換えない
    let (from, to, ok) = match args.as_slice() {
        [from, to] => (from, to, false),
        [from, to, ok] => (from, to, ok.is_true()),
        _ => return Err("copy-file expects 2 or 3 arguments".to_string().into()),
    };
    (|| {
        let from = expand_args(machine, name, std::slice::from_ref(from))?;
        let to = expand_args(machine, name, std::slice::from_ref(to))?;
        if !ok && to.exists() {
            return Err(Error::strings(
                "file-already-exists",
                &["File already exists", &to.display().to_string()],
            ));
        }
        std::fs::copy(&from, &to)
            .map(|_| Value::Nil)
            .map_err(|e| Error::from(format!("{}: {}", from.display(), e)))
    })()
}

/// `(delete-directory PATH [RECURSIVE])`: ディレクトリを消します.
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
fn sys_delete_directory(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    // 2 つ目の引数が真なら, 中身ごと消す
    let (path, recursive) = match args.as_slice() {
        [path] => (path, false),
        [path, recursive] => (path, recursive.is_true()),
        _ => {
            return Err("delete-directory expects 1 or 2 arguments"
                .to_string()
                .into())
        }
    };
    let path = expand_args(machine, name, std::slice::from_ref(path))?;
    let removed = if recursive {
        std::fs::remove_dir_all(&path)
    } else {
        std::fs::remove_dir(&path)
    };
    removed
        .map(|_| Value::Nil)
        .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))
}

/// `(directory-files-recursively DIR PATTERN)`: ディレクトリの下の, 正規表現に合うファイルを, 深くまで探します.
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
fn sys_directory_files_recursively(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<2>(name, args).and_then(|[dir, pattern]| {
        let dir = expand_args(machine, name, &[dir])?;
        let pattern = regexp(string(name, &pattern)?)?;
        let mut files = Vec::new();
        collect_files(&dir, &pattern, &mut files);
        files.sort();
        Ok(Value::List(files.into_iter().map(Value::String).collect()))
    })
}

/// `(grep-files DIR)`: ディレクトリの下の, 探す対象のファイルの並びを返します.
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
fn sys_grep_files(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        // (grep-files DIR): DIR の下の, 探すファイル (`.` で始まるもの, node_modules と target の下は, 探さない)
        let dir = expand_args(machine, name, &args)?;
        let mut files = Vec::new();
        collect_grep_files(&dir, &mut files);
        files.sort();
        Ok(Value::List(files.into_iter().map(Value::String).collect()))
    })
}

/// `(file-modification-time PATH)`: ファイルの更新時刻 (秒) を返します. 無ければ `#f`.
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
fn sys_file_modification_time(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|args| {
        let path = expand_args(machine, name, &args)?;
        Ok(match std::fs::metadata(&path).and_then(|m| m.modified()) {
            Ok(time) => Value::Number(seconds(time)),
            Err(_) => Value::Boolean(false),
        })
    })
}

/// `(file-newer-than-file-p FILE1 FILE2)`: FILE1 が FILE2 より新しいかを調べます.
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
fn sys_file_newer_than_file_p(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    fixed::<2>(name, args).and_then(|[a, b]| {
        // FILE1 が FILE2 より新しければ真 (FILE1 だけがあるときも真. FILE1 が無ければ偽)
        let a = expand_args(machine, name, &[a])?;
        let b = expand_args(machine, name, &[b])?;
        let time = |path: &std::path::Path| std::fs::metadata(path).and_then(|m| m.modified()).ok();
        Ok(Value::Boolean(match (time(&a), time(&b)) {
            (Some(a), Some(b)) => a > b,
            (a, _) => a.is_some(),
        }))
    })
}

/// `(current-time)`: 今の時刻 (UNIX 時刻の秒) を返します.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_current_time(_machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<0>(name, args).map(|_| Value::Number(seconds(SystemTime::now())))
}

/// `(format-time-string FORMAT [TIME])`: 時刻を, 書式にしたがって文字列にします.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_format_time_string(
    _machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    format_time_string(name, &args)
}

/// `(date-to-time STRING)`: 日付の文字列を, 時刻 (秒) にします.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_date_to_time(_machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[date]| date_to_time(string(name, &date)?).map(Value::Number))
}

/// `(random LIMIT)`: 0 以上 LIMIT 未満の整数の乱数を返します (LIMIT は正の整数).
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_random(_machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[limit]| {
        let limit = number(name, &limit)?;
        if limit < 1.0 || limit.fract() != 0.0 {
            return Err("random expects a positive integer".to_string().into());
        }
        let random = std::collections::hash_map::RandomState::new()
            .build_hasher()
            .finish();
        Ok(Value::Number((random % limit as u64) as f64))
    })
}

/// `(regexp-match REGEXP STRING [START])` / `(regexp-match-positions ...)`: 正規表現で探して, 合った部分 (か, その位置) を返します.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_regexp_match_and_regexp_match_positions(
    _machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    regexp_match(name, &args)
}

/// `(regexp-quote STRING)`: 正規表現の特別な文字を, 逃がします.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_regexp_quote(_machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[text]| Ok(Value::String(regex::escape(string(name, &text)?))))
}

/// `(rust-grep PATTERN FILES)`: ファイルの中から, 正規表現に合う行を探します.
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
fn sys_rust_grep(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<2>(name, args).and_then(|[pattern, files]| grep(machine, name, &pattern, &files, false))
}

/// `(rust-zgrep PATTERN FILES)`: `.gz` のファイルは展開して, 正規表現に合う行を探します.
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
fn sys_rust_zgrep(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<2>(name, args).and_then(|[pattern, files]| grep(machine, name, &pattern, &files, true))
}

/// `(getenv NAME)`: 環境変数の値を返します. 無ければ `#f`.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_getenv(_machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    // 環境変数の値. 無ければ #f
    fixed::<1>(name, args).and_then(|[variable]| {
        Ok(std::env::var(string(name, &variable)?).map_or(Value::Boolean(false), Value::String))
    })
}

/// `(browse-url URL)`: URL を, 外のプログラムで開きます.
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
fn sys_browse_url(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    fixed::<1>(name, args).and_then(|[url]| {
        let url = string(name, &url)?.to_string();
        let program = match machine.evaluator.global().get("browse-url-generic-program") {
            Some(Value::String(program)) => program,
            _ => {
                return Err("browse-url-generic-program must be a program"
                    .to_string()
                    .into())
            }
        };
        browse_url(&program, &url)?;
        Ok(Value::Nil)
    })
}

/// `(call-shell-command COMMAND INPUT DIRECTORY)`: シェルでコマンドを動かし, (終了コード 出力) を返します.
///
/// # Arguments
///
/// * `_machine` - 評価.
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
fn sys_call_shell_command(
    _machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, Error> {
    // (call-shell-command COMMAND INPUT DIRECTORY): シェルで COMMAND を動かし, (終了コード 出力) を返す.
    // INPUT (文字列, 無ければ #f) は標準入力に, DIRECTORY (無ければ #f) は動かす所. エラーの出力も, 出力に混ぜる
    fixed::<3>(name, args).and_then(|[command, input, dir]| {
        let optional = |value: &Value| match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        };
        let (code, output) = run_shell(
            string(name, &command)?,
            optional(&input).as_deref(),
            optional(&dir).as_deref(),
        )?;
        Ok(Value::List(
            vec![Value::Number(f64::from(code)), Value::String(output)]
                .into_iter()
                .collect(),
        ))
    })
}

/// `load` で読むファイルを探します (Emacs の `load` と同じく, 拡張子 ([`LISP_SUFFIX`]) は省ける).
///
/// - 名前だけ (`"howm"`, `"lib/howm.scm"` のように, `/`, `~`, `./`, `../` で始まらない) なら,
///   読み込んでいるファイルのディレクトリ, `load-path` のディレクトリの順に探す.
///   読み込んでいるファイルが無ければ (初期設定ファイルなど), 最後にホームディレクトリも探す.
/// - そのほかは, そのパス (相対パスは, 読み込んでいるファイルのディレクトリから数える) だけを探す.
/// - それぞれの所で, 拡張子を付けた名前, そのままの名前の順に探す (ファイルだけ).
///
/// # Arguments
///
/// * `machine` - 評価 (読み込んでいるファイルのディレクトリと, `load-path`).
/// * `file` - ファイルの名前かパス.
///
/// # Returns
///
/// 見つけたファイルのパス. 無ければ, `Cannot open load file` の [`Err`].
fn locate_library(machine: &Machine, file: &str) -> Result<PathBuf, Error> {
    let home = dirs::home_dir().unwrap_or_default();
    let plain = !(file.starts_with('/')
        || file.starts_with('~')
        || file.starts_with("./")
        || file.starts_with("../"));
    let loading = machine.load_dirs.last().cloned();
    let mut dirs: Vec<PathBuf> = loading.clone().into_iter().collect();
    if plain {
        dirs.extend(
            load_path(machine)?
                .iter()
                .map(|dir| expand_file_name(dir, &home, &home)),
        );
    }
    if loading.is_none() {
        dirs.push(home.clone());
    }
    dirs.iter()
        .flat_map(|dir| {
            [format!("{}{}", file, LISP_SUFFIX), file.to_string()]
                .map(|name| expand_file_name(&name, dir, &home))
        })
        .find(|path| path.is_file())
        .ok_or_else(|| {
            Error::strings(
                "file-missing",
                &["Cannot open load file", "No such file or directory", file],
            )
        })
}

/// `load-path` (ディレクトリの文字列のリスト) を返します.
///
/// # Arguments
///
/// * `machine` - 評価.
///
/// # Returns
///
/// ディレクトリ. `load-path` が文字列のリストでなければ [`Err`].
fn load_path(machine: &Machine) -> Result<Vec<String>, Error> {
    let invalid = || Error::plain("load-path must be a list of directories");
    match machine.evaluator.global().get("load-path") {
        Some(Value::List(dirs)) => dirs
            .iter()
            .map(|dir| match dir {
                Value::String(dir) => Ok(dir.clone()),
                _ => Err(invalid()),
            })
            .collect(),
        _ => Err(invalid()),
    }
}

/// 機能 (`provide` した名前) が無ければ, ファイルを読み込みます (Emacs の `require`).
/// ファイルは, 引数で渡すか, 機能の名前で, `load` と同じく探します.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 機能 (記号) と, 省ける, ファイルの名前.
///
/// # Returns
///
/// 機能. 読み込んでも, 機能を `provide` しなければ [`Err`].
fn require(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    let (feature, file) = match args.as_slice() {
        [Value::Symbol(feature)] => (feature.clone(), feature.clone()),
        [Value::Symbol(feature), file] => (feature.clone(), string(name, file)?.to_string()),
        _ => {
            return Err("require expects a feature (symbol) and an optional file"
                .to_string()
                .into())
        }
    };
    if !machine.evaluator.featurep(&feature) {
        let path = locate_library(machine, &file)?;
        load_file(machine, &path)?;
        if !machine.evaluator.featurep(&feature) {
            return Err(format!("Required feature was not provided: {}", feature).into());
        }
    }
    Ok(Value::Symbol(feature))
}

/// ファイルを読んで, 一番外側の環境で評価します (Emacs の `load`).
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `path` - ファイルのパス.
///
/// # Returns
///
/// `#t`. 読めないときと, 評価が失敗したときは, ファイルのパスを添えた失敗の内容.
fn load_file(machine: &mut Machine, path: &Path) -> Result<Value, Error> {
    let path = path.to_path_buf();
    log::info!("Loading {}", path.display());
    let code = std::fs::read_to_string(&path)
        .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
    let values = Parser::new(&code)
        .parse()
        .map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
    machine
        .load_dirs
        .push(path.parent().map(Path::to_path_buf).unwrap_or_default());
    machine.load_files.push(path.to_string_lossy().into_owned());
    let global = machine.evaluator.global().clone();
    let result = values
        .iter()
        .try_for_each(|value| machine.eval(value, &global).map(|_| ()));
    machine.load_dirs.pop();
    machine.load_files.pop();
    result.map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
    Ok(Value::Boolean(true))
}

/// 引数のパス (と, 相対パスの基のディレクトリ) から絶対パスを作ります.
///
/// # Arguments
///
/// * `machine` - 評価 (読み込んでいるファイルのディレクトリ).
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - パスと, 省ける, 基のディレクトリ.
///
/// # Returns
///
/// 絶対パス. 引数が文字列でなければ [`Err`].
fn expand_args(machine: &Machine, name: &str, args: &[Value]) -> Result<PathBuf, Error> {
    let path = string(name, &args[0])?;
    Ok(PathBuf::from(match args.get(1) {
        Some(dir) => {
            let home = dirs::home_dir().unwrap_or_default();
            let base = expand_file_name(string(name, dir)?, &home, &home);
            path_string(&expand_file_name(path, &base, &home))
        }
        None => machine.expand_file_name(path),
    }))
}

/// パスを, 絶対パスにします. `~` はホームディレクトリにします. `.` と `..` は除きます.
///
/// # Arguments
///
/// * `path` - パス.
/// * `base` - 相対パスの基のディレクトリ.
/// * `home` - ホームディレクトリ.
///
/// # Returns
///
/// 絶対パス.
pub fn expand_file_name(path: &str, base: &Path, home: &Path) -> PathBuf {
    let joined = if path == "~" {
        home.to_path_buf()
    } else if let Some(rest) = path.strip_prefix("~/") {
        home.join(rest)
    } else {
        base.join(path)
    };
    // `.` を除き, `..` で 1 つ上に戻る (Emacs と同じく, ファイルを見ずに, 文字の上で)
    let mut result = PathBuf::new();
    for component in joined.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir
                if matches!(result.components().next_back(), Some(Component::Normal(_))) =>
            {
                result.pop();
            }
            Component::ParentDir if result.has_root() => {}
            _ => result.push(component),
        }
    }
    result
}

/// パスを, 文字列にします.
///
/// # Arguments
///
/// * `path` - パス.
///
/// # Returns
///
/// 文字列.
fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// ディレクトリの一覧を, `ls -al` で作ります (dired の本文の元. 日付などの書式を一定にするため `LC_ALL=C`).
///
/// # Arguments
///
/// * `program` - `ls` のプログラム.
/// * `path` - ディレクトリ.
///
/// # Returns
///
/// `ls` の出力. 読めない, または `ls` が失敗したときは, 理由の文字列の [`Err`].
pub fn directory_listing(program: &str, path: &Path) -> Result<String, Error> {
    std::fs::read_dir(path).map_err(|e| Error::from(format!("{}: {}", path.display(), e)))?;
    let output = std::process::Command::new(program)
        .args(["-al", "--"])
        .arg(path)
        .env("LC_ALL", "C")
        .output()
        .map_err(|e| Error::from(format!("{program}: {e}")))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr)
            .trim()
            .to_string()
            .into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// ディレクトリの下 (深くまで) の, 名前が正規表現に合うファイルを集めます. `.` で始まるものは, 除きます.
/// ディレクトリへのシンボリックリンクは, たどりません (輪になっていると終わらない. Emacs と同じ).
///
/// # Arguments
///
/// * `dir` - ディレクトリ (読めなければ何もしない).
/// * `pattern` - ファイルの名前の正規表現.
/// * `files` - 集めたファイルのパスを, 入れる先.
fn collect_files(dir: &Path, pattern: &Regex, files: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            if !path.is_symlink() {
                collect_files(&path, pattern, files);
            }
        } else if pattern.is_match(&name) {
            files.push(path_string(&path));
        }
    }
}

/// ディレクトリの下 (深くまで) の, grep で探すファイルを集めます. `.` で始まるもの, `node_modules` と `target` の下は,
/// 入らずに飛ばします (大きなディレクトリを, 先に全て数えない). ディレクトリへのシンボリックリンクは, たどりません.
///
/// # Arguments
///
/// * `dir` - ディレクトリ (読めなければ何もしない).
/// * `files` - 集めたファイルのパスを, 入れる先.
fn collect_grep_files(dir: &Path, files: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            if !path.is_symlink() && name != "node_modules" && name != "target" {
                collect_grep_files(&path, files);
            }
        } else {
            files.push(path_string(&path));
        }
    }
}

/// 時刻を, 1970 年からの秒数にします.
///
/// # Arguments
///
/// * `time` - 時刻.
///
/// # Returns
///
/// 秒数.
fn seconds(time: SystemTime) -> f64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

/// 時刻を, 書式 (`%Y-%m-%d` など. strftime と同じ) で文字列にします. 時刻を省くと, 今の時刻です.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 書式と, 省ける, 時刻 (1970 年からの秒数).
///
/// # Returns
///
/// 文字列. 書式が誤っているときは [`Err`].
fn format_time_string(name: &str, args: &[Value]) -> Result<Value, Error> {
    let (template, time) = match args {
        [template] => (template, seconds(SystemTime::now())),
        [template, time] => (template, number(name, time)?),
        _ => {
            return Err("format-time-string expects 1 or 2 arguments"
                .to_string()
                .into())
        }
    };
    let template = string(name, template)?;
    let items: Vec<Item> = StrftimeItems::new(template).collect();
    if items.contains(&Item::Error) {
        return Err(format!("format-time-string: invalid format {:?}", template).into());
    }
    let time = Local
        .timestamp_opt(time.floor() as i64, 0)
        .single()
        .ok_or_else(|| "format-time-string: invalid time".to_string())?;
    Ok(Value::String(
        time.format_with_items(items.into_iter()).to_string(),
    ))
}

/// 日付 (`2026-10-25`, `2026-10-25 09:30`, `2026-10-25 09:30:15`) を, その土地の時刻として, 1970 年からの秒数にします.
///
/// # Arguments
///
/// * `date` - 日付.
///
/// # Returns
///
/// 秒数. 読めない日付は [`Err`].
fn date_to_time(date: &str) -> Result<f64, Error> {
    let date = date.trim();
    let time = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M"))
        .or_else(|_| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d").map(|d| d.and_time(Default::default()))
        })
        .map_err(|_| Error::from(format!("date-to-time: invalid date {:?}", date)))?;
    Local
        .from_local_datetime(&time)
        .earliest()
        .map(|t| t.timestamp() as f64)
        .ok_or_else(|| format!("date-to-time: invalid date {:?}", date).into())
}

/// 正規表現を作ります (作ったものは, 覚えておく).
///
/// # Arguments
///
/// * `pattern` - 正規表現 (Rust の regex の書き方).
///
/// # Returns
///
/// 正規表現. 誤っていれば [`Err`].
pub(crate) fn regexp(pattern: &str) -> Result<Regex, Error> {
    REGEXPS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(regex) = cache.get(pattern) {
            return Ok(regex.clone());
        }
        let regex =
            Regex::new(pattern).map_err(|e| Error::strings("invalid-regexp", &[&e.to_string()]))?;
        if cache.len() >= REGEXP_CACHE_SIZE {
            cache.clear();
        }
        cache.insert(pattern.to_string(), regex.clone());
        Ok(regex)
    })
}

/// 一時ディレクトリに, 新しいファイルかディレクトリを作ります (Emacs の `make-temp-file`).
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 名前の始めと, 省ける, 真ならディレクトリを作る印.
///
/// # Returns
///
/// 作ったファイルかディレクトリのパス. 作れなければ [`Err`].
fn make_temp_file(name: &str, args: &[Value]) -> Result<Value, Error> {
    let (prefix, directory) = match args {
        [prefix] => (string(name, prefix)?, false),
        [prefix, flag] => (string(name, prefix)?, flag.is_true()),
        _ => return Err("make-temp-file expects 1 or 2 arguments".to_string().into()),
    };
    let builder = tempfile::Builder::new().prefix(prefix).to_owned();
    let path = if directory {
        builder.tempdir().map(|dir| dir.keep())
    } else {
        builder
            .tempfile()
            .and_then(|file| file.keep().map(|(_, path)| path).map_err(|e| e.error))
    }
    .map_err(|e| Error::from(format!("make-temp-file: {}", e)))?;
    Ok(Value::String(path_string(&path)))
}

/// 文字列から, 正規表現に合う所を探します (`(regexp-match "a(b)" "xab")` → `("ab" "b")`).
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 正規表現, 文字列, 省ける, 探し始める位置 (文字の数).
///
/// # Returns
///
/// 合った所の全体と, 括弧ごとの文字列 (合わなかった括弧は `#f`) のリスト. 合わなければ `#f`.
/// `regexp-match-positions` なら, 文字列の代わりに, 始めと終わりの位置 (文字の数) のリスト
/// (`(regexp-match-positions "a(b)" "xab")` → `((1 3) (2 3))`).
fn regexp_match(name: &str, args: &[Value]) -> Result<Value, Error> {
    let (pattern, text, start) = match args {
        [pattern, text] => (pattern, text, 0),
        [pattern, text, start] => (pattern, text, index(name, start)?),
        _ => return Err("regexp-match expects 2 or 3 arguments".to_string().into()),
    };
    let regex = regexp(string(name, pattern)?)?;
    let text = string(name, text)?;
    let count = text.chars().count();
    if start > count {
        return Err(Error::args_out_of_range(vec![
            Value::String(text.to_string()),
            Value::Number(start as f64),
        ]));
    }
    let offset = text
        .char_indices()
        .nth(start)
        .map_or(text.len(), |(offset, _)| offset);
    Ok(match regex.captures_at(text, offset) {
        Some(captures) => Value::List(
            captures
                .iter()
                .map(|group| match group {
                    Some(group) if name == "regexp-match" => {
                        Value::String(group.as_str().to_string())
                    }
                    Some(group) => {
                        let start = text[..group.start()].chars().count();
                        let end = start + group.as_str().chars().count();
                        Value::List([Value::Number(start as f64), Value::Number(end as f64)].into())
                    }
                    None => Value::Boolean(false),
                })
                .collect(),
        ),
        None => Value::Boolean(false),
    })
}

/// ファイルの中から, 正規表現に合う行を探します (howm の検索に使う, rust-grep).
///
/// # Arguments
///
/// * `machine` - 評価 (相対パスの基のディレクトリ).
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `pattern` - 正規表現.
/// * `files` - ファイルのパスのリスト (`expand-file-name` と同じく読む. 読めないファイルは, 飛ばす).
/// * `decompress` - 真なら, 名前が `.gz` で終わるファイルを, gzip として展開して探す (`rust-zgrep`).
///
/// # Returns
///
/// 合った行ごとの `(パス 行の番号 行)` のリスト (ファイルの順, 行の順. 行の番号は 1 から).
fn grep(
    machine: &Machine,
    name: &str,
    pattern: &Value,
    files: &Value,
    decompress: bool,
) -> Result<Value, Error> {
    let regex = regexp(string(name, pattern)?)?;
    let Value::List(files) = files else {
        return Err(format!("{name} expects a list of files").into());
    };
    let mut found = Vec::new();
    for file in files.iter() {
        let path = string(name, file)?;
        let Some(text) = read_for_grep(&machine.expand_file_name(path), decompress) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            if regex.is_match(line) {
                found.push(Value::List(
                    [
                        Value::String(path.to_string()),
                        Value::Number((i + 1) as f64),
                        Value::String(line.to_string()),
                    ]
                    .into(),
                ));
            }
        }
    }
    Ok(Value::List(found.into()))
}

/// `grep` が探すファイルを, 文字列に読みます.
///
/// # Arguments
///
/// * `path` - ファイルのパス.
/// * `decompress` - 真なら, 名前が `.gz` で終わるファイルを, gzip として展開する.
///
/// # Returns
///
/// 本文. 読めない (展開できない, UTF-8 でない) ときは `None`.
fn read_for_grep(path: &str, decompress: bool) -> Option<String> {
    if decompress && path.ends_with(".gz") {
        let mut text = String::new();
        let file = std::fs::File::open(path).ok()?;
        std::io::Read::read_to_string(&mut flate2::read::MultiGzDecoder::new(file), &mut text)
            .ok()?;
        Some(text)
    } else {
        std::fs::read_to_string(path).ok()
    }
}

/// シェル (`sh -c`) でコマンドを動かし, 標準出力とエラーの出力を混ぜて集めます.
///
/// # Arguments
///
/// * `command` - コマンドの文字列.
/// * `input` - 標準入力に渡す文字列 (無ければ空).
/// * `dir` - 動かすディレクトリ (無ければ今の所).
///
/// # Returns
///
/// 終了コード (信号で終わったときは -1) と, 出力. 動かせなければ, 理由の文字列の [`Err`].
pub fn run_shell(
    command: &str,
    input: Option<&str>,
    dir: Option<&str>,
) -> Result<(i32, String), Error> {
    use std::io::Write;
    let mut process = std::process::Command::new("sh");
    process
        .arg("-c")
        .arg(format!("exec 2>&1; {command}"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    if let Some(dir) = dir {
        process.current_dir(dir);
    }
    let mut child = process
        .spawn()
        .map_err(|e| Error::from(format!("sh: {e}")))?;
    let mut stdin = child.stdin.take().expect("stdin is piped");
    let text = input.unwrap_or("").to_string();
    // 出力を読みながら書かないと, 入力が大きいとき止まる
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(text.as_bytes());
    });
    let output = child
        .wait_with_output()
        .map_err(|e| Error::from(format!("sh: {e}")))?;
    let _ = writer.join();
    Ok((
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    ))
}

/// URL を, プログラムで開きます (終わるのを待たない).
///
/// # Arguments
///
/// * `program` - プログラム (`xdg-open` など).
/// * `url` - URL.
///
/// # Returns
///
/// 動かせなければ失敗の内容.
pub fn browse_url(program: &str, url: &str) -> Result<(), Error> {
    let mut child = std::process::Command::new(program)
        .arg(url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| Error::from(format!("browse-url: {}: {}", program, e)))?;
    // 終わったプログラムを片付ける
    std::thread::spawn(move || child.wait());
    Ok(())
}
