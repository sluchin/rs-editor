//! 組み込みの手続きのうち, ファイル, 時刻, 正規表現を使うもの (`load`, `read-file-string`, `grep` など).
//!
//! 相対パスは, 読み込んでいるファイル (`load`) のディレクトリ, またはホームディレクトリから数えます.

use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::format::{Item, StrftimeItems};
use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use regex::Regex;

use crate::atomic::write_atomic;
use crate::scheme::builtins::{fixed, index, number, string};
use crate::scheme::evaluator::Machine;
use crate::scheme::parser::Parser;
use crate::scheme::value::Value;

/// 組み込みの手続きの名前 (ファイル, 時刻, 正規表現).
pub const NAMES: &[&str] = &[
    "load",
    "require",
    "provide",
    "featurep",
    "expand-file-name",
    "file-exists?",
    "file-directory?",
    "read-file-string",
    "write-file-string",
    "make-directory",
    "make-temp-file",
    "delete-file",
    "delete-directory",
    "directory-files-recursively",
    "file-modification-time",
    "file-newer-than-file-p",
    "current-time",
    "format-time-string",
    "date-to-time",
    "random",
    "getenv",
    "regexp-match",
    "regexp-match-positions",
    "regexp-quote",
    "grep",
    "browse-url",
];

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

/// Lisp のファイルの拡張子 (`load` で, 省ける).
pub const LISP_SUFFIX: &str = ".scm";

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
        "load" => fixed::<1>(name, args).and_then(|[file]| {
            let file = string(name, &file)?;
            let path = locate_library(machine, file)?;
            load_file(machine, &path)
        }),
        "require" => require(machine, name, args),
        "provide" => fixed::<1>(name, args).and_then(|[feature]| match feature {
            Value::Symbol(feature) => {
                machine.evaluator.provide(&feature);
                Ok(Value::Symbol(feature))
            }
            _ => Err("provide expects a symbol".to_string()),
        }),
        "featurep" => fixed::<1>(name, args).and_then(|[feature]| match feature {
            Value::Symbol(feature) => Ok(Value::Boolean(machine.evaluator.featurep(&feature))),
            _ => Err("featurep expects a symbol".to_string()),
        }),
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
            write_atomic(&path, text.as_bytes())
                .map_err(|e| format!("{}: {}", path.display(), e))?;
            Ok(Value::Nil)
        }),
        "make-directory" => fixed::<1>(name, args).and_then(|args| {
            let path = expand_args(machine, name, &args)?;
            std::fs::create_dir_all(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
            Ok(Value::Nil)
        }),
        "make-temp-file" => make_temp_file(name, &args),
        "delete-file" => fixed::<1>(name, args).and_then(|args| {
            let path = expand_args(machine, name, &args)?;
            std::fs::remove_file(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
            Ok(Value::Nil)
        }),
        "delete-directory" => {
            // 2 つ目の引数が真なら, 中身ごと消す
            let (path, recursive) = match args.as_slice() {
                [path] => (path, false),
                [path, recursive] => (path, recursive.is_true()),
                _ => return Some(Err("delete-directory expects 1 or 2 arguments".to_string())),
            };
            let path = match expand_args(machine, name, std::slice::from_ref(path)) {
                Ok(path) => path,
                Err(e) => return Some(Err(e)),
            };
            let removed = if recursive {
                std::fs::remove_dir_all(&path)
            } else {
                std::fs::remove_dir(&path)
            };
            removed
                .map(|_| Value::Nil)
                .map_err(|e| format!("{}: {}", path.display(), e))
        }
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
        "file-newer-than-file-p" => fixed::<2>(name, args).and_then(|[a, b]| {
            // FILE1 が FILE2 より新しければ真 (FILE1 だけがあるときも真. FILE1 が無ければ偽)
            let a = expand_args(machine, name, &[a])?;
            let b = expand_args(machine, name, &[b])?;
            let time =
                |path: &std::path::Path| std::fs::metadata(path).and_then(|m| m.modified()).ok();
            Ok(Value::Boolean(match (time(&a), time(&b)) {
                (Some(a), Some(b)) => a > b,
                (a, _) => a.is_some(),
            }))
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
        "regexp-match" | "regexp-match-positions" => regexp_match(name, &args),
        "regexp-quote" => fixed::<1>(name, args)
            .and_then(|[text]| Ok(Value::String(regex::escape(string(name, &text)?)))),
        "grep" => fixed::<2>(name, args)
            .and_then(|[pattern, files]| grep(machine, name, &pattern, &files)),
        // 環境変数の値. 無ければ #f
        "getenv" => {
            fixed::<1>(name, args).and_then(|[variable]| {
                Ok(std::env::var(string(name, &variable)?)
                    .map_or(Value::Boolean(false), Value::String))
            })
        }
        "browse-url" => fixed::<1>(name, args).and_then(|[url]| {
            let url = string(name, &url)?.to_string();
            let program = match machine.evaluator.global().get("browse-url-generic-program") {
                Some(Value::String(program)) => program,
                _ => return Err("browse-url-generic-program must be a program".to_string()),
            };
            browse_url(&program, &url)?;
            Ok(Value::Nil)
        }),
        _ => return None,
    };
    Some(result)
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
fn locate_library(machine: &Machine, file: &str) -> Result<PathBuf, String> {
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
        .ok_or_else(|| format!("Cannot open load file: {}", file))
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
fn load_path(machine: &Machine) -> Result<Vec<String>, String> {
    let invalid = || "load-path must be a list of directories".to_string();
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
fn require(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, String> {
    let (feature, file) = match args.as_slice() {
        [Value::Symbol(feature)] => (feature.clone(), feature.clone()),
        [Value::Symbol(feature), file] => (feature.clone(), string(name, file)?.to_string()),
        _ => return Err("require expects a feature (symbol) and an optional file".to_string()),
    };
    if !machine.evaluator.featurep(&feature) {
        let path = locate_library(machine, &file)?;
        load_file(machine, &path)?;
        if !machine.evaluator.featurep(&feature) {
            return Err(format!("Required feature was not provided: {}", feature));
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
fn load_file(machine: &mut Machine, path: &Path) -> Result<Value, String> {
    let path = path.to_path_buf();
    log::info!("Loading {}", path.display());
    let code = std::fs::read_to_string(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let values = Parser::new(&code)
        .parse()
        .map_err(|e| format!("{}: {}", path.display(), e))?;
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

/// ディレクトリの下 (深くまで) の, 名前が正規表現に合うファイルを集めます. `.` で始まるものは, 除きます.
/// ディレクトリへのシンボリックリンクは, たどりません (輪になっていると終わらない. Emacs と同じ).
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
            if !path.is_symlink() {
                collect_files(&path, pattern, files);
            }
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
pub(crate) fn regexp(pattern: &str) -> Result<Regex, String> {
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
fn make_temp_file(name: &str, args: &[Value]) -> Result<Value, String> {
    let (prefix, directory) = match args {
        [prefix] => (string(name, prefix)?, false),
        [prefix, flag] => (string(name, prefix)?, flag.is_true()),
        _ => return Err("make-temp-file expects 1 or 2 arguments".to_string()),
    };
    let builder = tempfile::Builder::new().prefix(prefix).to_owned();
    let path = if directory {
        builder.tempdir().map(|dir| dir.keep())
    } else {
        builder
            .tempfile()
            .and_then(|file| file.keep().map(|(_, path)| path).map_err(|e| e.error))
    }
    .map_err(|e| format!("make-temp-file: {}", e))?;
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

/// ファイルの中から, 正規表現に合う行を探します (howm の検索に使う, grep).
///
/// # Arguments
///
/// * `machine` - 評価 (相対パスの基のディレクトリ).
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `pattern` - 正規表現.
/// * `files` - ファイルのパスのリスト (`expand-file-name` と同じく読む. 読めないファイルは, 飛ばす).
///
/// # Returns
///
/// 合った行ごとの `(パス 行の番号 行)` のリスト (ファイルの順, 行の順. 行の番号は 1 から).
fn grep(machine: &Machine, name: &str, pattern: &Value, files: &Value) -> Result<Value, String> {
    let regex = regexp(string(name, pattern)?)?;
    let Value::List(files) = files else {
        return Err("grep expects a list of files".to_string());
    };
    let mut found = Vec::new();
    for file in files.iter() {
        let path = string(name, file)?;
        let Ok(text) = std::fs::read_to_string(machine.expand_file_name(path)) else {
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

/// URL を, プログラムで開きます (終わるのを待たない).
///
/// # Arguments
///
/// * `program` - プログラム (`xdg-open` など).
/// * `url` - URL.
///
/// # Returns
///
/// 動かせなければ, 失敗の内容.
pub fn browse_url(program: &str, url: &str) -> Result<(), String> {
    let mut child = std::process::Command::new(program)
        .arg(url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("browse-url: {}: {}", program, e))?;
    // 終わったプログラムを片付ける
    std::thread::spawn(move || child.wait());
    Ok(())
}
