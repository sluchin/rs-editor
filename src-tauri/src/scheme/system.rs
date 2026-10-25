//! 組み込みの手続きのうち, ファイル, 時刻, 正規表現を使うもの (`load`, `read-file-string`, `grep` など).
//!
//! 相対パスは, 読み込んでいるファイル (`load`) のディレクトリ, またはホームディレクトリから数えます.

use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::format::{Item, StrftimeItems};
use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use regex::Regex;

use crate::scheme::builtins::{fixed, index, number, string};
use crate::scheme::evaluator::Machine;
use crate::scheme::parser::Parser;
use crate::scheme::value::Value;

/// 組み込みの手続きの名前 (ファイル, 時刻, 正規表現).
pub const NAMES: &[&str] = &[
    "load",
    "expand-file-name",
    "file-exists?",
    "file-directory?",
    "read-file-string",
    "write-file-string",
    "make-directory",
    "directory-files-recursively",
    "file-modification-time",
    "current-time",
    "format-time-string",
    "date-to-time",
    "random",
    "regexp-match",
    "regexp-quote",
    "grep",
];

/// 正規表現を覚えておく数 (超えたら, 全て忘れる).
const REGEXP_CACHE_SIZE: usize = 64;

thread_local! {
    /// 作った正規表現 (同じ正規表現を, 何度も作らないように).
    static REGEXPS: std::cell::RefCell<HashMap<String, Regex>> = std::cell::RefCell::new(HashMap::new());
}

/// 組み込みの手続き (ファイル, 時刻, 正規表現) を呼びます.
///
/// # Arguments
///
/// * `machine` - 評価 (`load` で使う).
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容. この手続きでなければ `None`.
pub(crate) fn call(
    machine: &mut Machine,
    name: &str,
    args: Vec<Value>,
) -> Option<Result<Value, String>> {
    let result = match name {
        "load" => load(machine, name, args),
        "expand-file-name" => {
            if !(1..=2).contains(&args.len()) {
                return Some(Err("expand-file-name expects 1 or 2 arguments".to_string()));
            }
            expand_args(machine, name, &args).map(|path| Value::String(path_string(&path)))
        }
        "file-exists?" | "file-directory?" => fixed::<1>(name, args).and_then(|args| {
            let path = expand_args(machine, name, &args)?;
            Ok(Value::Boolean(if name == "file-exists?" {
                path.exists()
            } else {
                path.is_dir()
            }))
        }),
        "read-file-string" => fixed::<1>(name, args).and_then(|args| {
            let path = expand_args(machine, name, &args)?;
            std::fs::read_to_string(&path)
                .map(Value::String)
                .map_err(|e| format!("{}: {}", path.display(), e))
        }),
        "write-file-string" => fixed::<2>(name, args).and_then(|[path, text]| {
            let path = expand_args(machine, name, &[path])?;
            let text = string(name, &text)?;
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("{}: {}", dir.display(), e))?;
            }
            std::fs::write(&path, text).map_err(|e| format!("{}: {}", path.display(), e))?;
            Ok(Value::Nil)
        }),
        "make-directory" => fixed::<1>(name, args).and_then(|args| {
            let path = expand_args(machine, name, &args)?;
            std::fs::create_dir_all(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
            Ok(Value::Nil)
        }),
        "directory-files-recursively" => fixed::<2>(name, args).and_then(|[dir, pattern]| {
            let dir = expand_args(machine, name, &[dir])?;
            let pattern = regexp(string(name, &pattern)?)?;
            let mut files = Vec::new();
            collect_files(&dir, &pattern, &mut files);
            files.sort();
            Ok(Value::List(files.into_iter().map(Value::String).collect()))
        }),
        "file-modification-time" => fixed::<1>(name, args).and_then(|args| {
            let path = expand_args(machine, name, &args)?;
            Ok(match std::fs::metadata(&path).and_then(|m| m.modified()) {
                Ok(time) => Value::Number(seconds(time)),
                Err(_) => Value::Boolean(false),
            })
        }),
        "current-time" => fixed::<0>(name, args).map(|_| Value::Number(seconds(SystemTime::now()))),
        "format-time-string" => format_time_string(name, &args),
        "date-to-time" => fixed::<1>(name, args)
            .and_then(|[date]| date_to_time(string(name, &date)?).map(Value::Number)),
        "random" => fixed::<1>(name, args).and_then(|[limit]| {
            let limit = number(name, &limit)?;
            if limit < 1.0 || limit.fract() != 0.0 {
                return Err("random expects a positive integer".to_string());
            }
            let random = std::collections::hash_map::RandomState::new()
                .build_hasher()
                .finish();
            Ok(Value::Number((random % limit as u64) as f64))
        }),
        "regexp-match" => regexp_match(name, &args),
        "regexp-quote" => fixed::<1>(name, args)
            .and_then(|[text]| Ok(Value::String(regex::escape(string(name, &text)?)))),
        "grep" => fixed::<2>(name, args).and_then(|[pattern, files]| grep(name, &pattern, &files)),
        _ => return None,
    };
    Some(result)
}

/// ファイルを読んで, 一番外側の環境で評価します (Emacs の `load`).
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 引数 (ファイルのパス).
///
/// # Returns
///
/// `#t`. 読めないときと, 評価が失敗したときは, ファイルのパスを添えた失敗の内容.
fn load(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, String> {
    let path = expand_args(machine, name, &fixed::<1>(name, args)?)?;
    log::info!("Loading {}", path.display());
    let code = std::fs::read_to_string(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let values = Parser::new(&code)
        .parse()
        .map_err(|e| format!("{}: {}", path.display(), e))?;
    machine
        .load_dirs
        .push(path.parent().map(Path::to_path_buf).unwrap_or_default());
    let global = machine.evaluator.global().clone();
    let result = values
        .iter()
        .try_for_each(|value| machine.eval(value, &global).map(|_| ()));
    machine.load_dirs.pop();
    result.map_err(|e| format!("{}: {}", path.display(), e))?;
    Ok(Value::Boolean(true))
}

/// 引数のパス (と, 相対パスの基のディレクトリ) から, 絶対パスを作ります.
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
fn expand_args(machine: &Machine, name: &str, args: &[Value]) -> Result<PathBuf, String> {
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

/// パスを, 絶対パスにします. `~` はホームディレクトリにします.
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
    if path == "~" {
        home.to_path_buf()
    } else if let Some(rest) = path.strip_prefix("~/") {
        home.join(rest)
    } else {
        base.join(path)
    }
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

/// ディレクトリの下 (深くまで) の, 名前が正規表現に合うファイルを集めます. `.` で始まるものは, 除きます.
///
/// # Arguments
///
/// * `dir` - ディレクトリ (読めなければ, 何もしない).
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
            collect_files(&path, pattern, files);
        } else if pattern.is_match(&name) {
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
fn format_time_string(name: &str, args: &[Value]) -> Result<Value, String> {
    let (template, time) = match args {
        [template] => (template, seconds(SystemTime::now())),
        [template, time] => (template, number(name, time)?),
        _ => return Err("format-time-string expects 1 or 2 arguments".to_string()),
    };
    let template = string(name, template)?;
    let items: Vec<Item> = StrftimeItems::new(template).collect();
    if items.contains(&Item::Error) {
        return Err(format!("format-time-string: invalid format {:?}", template));
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
fn date_to_time(date: &str) -> Result<f64, String> {
    let date = date.trim();
    let time = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M"))
        .or_else(|_| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d").map(|d| d.and_time(Default::default()))
        })
        .map_err(|_| format!("date-to-time: invalid date {:?}", date))?;
    Local
        .from_local_datetime(&time)
        .earliest()
        .map(|t| t.timestamp() as f64)
        .ok_or_else(|| format!("date-to-time: invalid date {:?}", date))
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
fn regexp(pattern: &str) -> Result<Regex, String> {
    REGEXPS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(regex) = cache.get(pattern) {
            return Ok(regex.clone());
        }
        let regex = Regex::new(pattern).map_err(|e| format!("Invalid regexp: {}", e))?;
        if cache.len() >= REGEXP_CACHE_SIZE {
            cache.clear();
        }
        cache.insert(pattern.to_string(), regex.clone());
        Ok(regex)
    })
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
fn regexp_match(name: &str, args: &[Value]) -> Result<Value, String> {
    let (pattern, text, start) = match args {
        [pattern, text] => (pattern, text, 0),
        [pattern, text, start] => (pattern, text, index(name, start)?),
        _ => return Err("regexp-match expects 2 or 3 arguments".to_string()),
    };
    let regex = regexp(string(name, pattern)?)?;
    let text = string(name, text)?;
    let count = text.chars().count();
    if start > count {
        return Err("regexp-match: index out of range".to_string());
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
                    Some(group) => Value::String(group.as_str().to_string()),
                    None => Value::Boolean(false),
                })
                .collect(),
        ),
        None => Value::Boolean(false),
    })
}

/// ファイルの中から, 正規表現に合う行を探します (howm の検索に使う, grep).
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `pattern` - 正規表現.
/// * `files` - ファイルのパスのリスト (読めないファイルは, 飛ばす).
///
/// # Returns
///
/// 合った行ごとの `(パス 行の番号 行)` のリスト (ファイルの順, 行の順. 行の番号は 1 から).
fn grep(name: &str, pattern: &Value, files: &Value) -> Result<Value, String> {
    let regex = regexp(string(name, pattern)?)?;
    let Value::List(files) = files else {
        return Err("grep expects a list of files".to_string());
    };
    let mut found = Vec::new();
    for file in files {
        let path = string(name, file)?;
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            if regex.is_match(line) {
                found.push(Value::List(vec![
                    Value::String(path.to_string()),
                    Value::Number((i + 1) as f64),
                    Value::String(line.to_string()),
                ]));
            }
        }
    }
    Ok(Value::List(found))
}
