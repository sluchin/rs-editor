//! 起動の引数 (Emacs の Initial options).
//!
//! `-q` (`--no-init-file`), `-Q` (`--quick`), `--no-splash`, `--debug-init`, `--chdir DIR`,
//! `--init-directory DIR`, `--no-site-file`, `--no-site-lisp` (`-nsl`), `--user USER` (`-u USER`) を読みます. 値は `--chdir DIR` と `--chdir=DIR` の両方の書き方を受けます.
//! 知らない引数は, Emacs と同じく, 開くファイルとして扱います.

use std::path::{Path, PathBuf};

use crate::command_loop::{CommandLoop, LoopUi};
use crate::scheme::host::Host;
use crate::scheme::value::Value;

/// 起動の引数を読んだ結果.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Options {
    /// 初期設定ファイルを読まない (`-q`, `--no-init-file`, `-Q`, `--quick`).
    pub no_init_file: bool,
    /// 始めの画面を出さない (`--no-splash`, `-Q`, `--quick`).
    pub no_splash: bool,
    /// 初期設定ファイルの失敗で, 細かい知らせを出す (`--debug-init`).
    pub debug_init: bool,
    /// 始めのディレクトリ (`--chdir`).
    pub chdir: Option<String>,
    /// 初期設定ファイルを探すディレクトリ (`--init-directory`).
    pub init_directory: Option<String>,
    /// 全ての利用者の初期設定 (`site-start.scm`) を読まない (`--no-site-file`, `-Q`, `--quick`).
    pub no_site_file: bool,
    /// 全ての利用者の Lisp のディレクトリを `load-path` に入れない (`--no-site-lisp`, `-nsl`, `-Q`, `--quick`).
    pub no_site_lisp: bool,
    /// ほかの利用者の初期設定ファイルを読む (`--user`, `-u`. 空の名前は, 今の利用者).
    pub user: Option<String>,
    /// 開くファイル (引数の順).
    pub files: Vec<String>,
}

/// 起動の引数を読みます.
///
/// # Arguments
///
/// * `args` - 引数 (プログラムの名前は含めない).
///
/// # Returns
///
/// 読んだ結果. 値を取る引数 (`--chdir`, `--init-directory`) に値が無いときは, 失敗の内容を含む [`Err`].
pub fn parse(args: &[String]) -> Result<Options, String> {
    let mut options = Options::default();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "-q" | "--no-init-file" | "-no-init-file" => options.no_init_file = true,
            "-Q" | "--quick" | "-quick" => {
                options.no_init_file = true;
                options.no_splash = true;
                options.no_site_file = true;
                options.no_site_lisp = true;
            }
            "--no-site-file" | "-no-site-file" => options.no_site_file = true,
            "--no-site-lisp" | "-no-site-lisp" | "-nsl" => options.no_site_lisp = true,
            "--user" | "-user" | "-u" => options.user = Some(value(arg, rest.next())?),
            "--no-splash" | "-no-splash" => options.no_splash = true,
            "--debug-init" | "-debug-init" => options.debug_init = true,
            "--chdir" | "-chdir" => options.chdir = Some(value(arg, rest.next())?),
            "--init-directory" | "-init-directory" => {
                options.init_directory = Some(value(arg, rest.next())?);
            }
            other => {
                if let Some(dir) = other.strip_prefix("--chdir=") {
                    options.chdir = Some(dir.to_string());
                } else if let Some(dir) = other.strip_prefix("--init-directory=") {
                    options.init_directory = Some(dir.to_string());
                } else if let Some(user) = other.strip_prefix("--user=") {
                    options.user = Some(user.to_string());
                } else {
                    options.files.push(other.to_string());
                }
            }
        }
    }
    Ok(options)
}

/// 全ての利用者の Lisp のディレクトリ (Emacs の `site-lisp`). `load-path` に入れ, `site-start.scm` を探す.
pub const SITE_LISP_DIR: &str = "/usr/local/share/rseditor/site-lisp";

/// 値を取る引数の値を返します.
///
/// # Arguments
///
/// * `name` - 引数の名前.
/// * `next` - 次の引数.
///
/// # Returns
///
/// 値. 無いときは [`Err`].
fn value(name: &str, next: Option<&String>) -> Result<String, String> {
    next.cloned()
        .ok_or_else(|| format!("Option `{name}' requires an argument"))
}

/// 初期設定ファイルの候補を, 探す順に返します.
///
/// # Arguments
///
/// * `home` - ホームディレクトリ (分からなければ `None`).
/// * `config_dir` - 設定のディレクトリ (Linux は `~/.config`. 分からなければ `None`).
///
/// # Returns
///
/// `<home>/.rseditor`, `<home>/.rseditor.d/init.scm` (Emacs の `~/.emacs.d/init.el` と同じ) と,
/// `<config_dir>/rseditor/init.scm`.
pub fn init_file_candidates(home: Option<&Path>, config_dir: Option<&Path>) -> Vec<PathBuf> {
    home.into_iter()
        .flat_map(|dir| {
            [
                dir.join(".rseditor"),
                dir.join(".rseditor.d").join("init.scm"),
            ]
        })
        .chain(config_dir.map(|dir| dir.join("rseditor").join("init.scm")))
        .collect()
}

/// 初期設定ファイルのパスを返します. `~/.rseditor`, `~/.rseditor.d/init.scm`,
/// `~/.config/rseditor/init.scm` の順に探して, 初めにあるものを使います.
///
/// # Returns
///
/// あるファイルのパス. どれも無ければ `None`.
pub fn init_file_path() -> Option<PathBuf> {
    init_file_candidates(dirs::home_dir().as_deref(), dirs::config_dir().as_deref())
        .into_iter()
        .find(|path| path.exists())
}

/// 利用者のホームディレクトリを, パスワードファイル (`/etc/passwd` の形) から探します.
///
/// # Arguments
///
/// * `passwd` - パスワードファイルの中身.
/// * `user` - 利用者の名前.
///
/// # Returns
///
/// ホームディレクトリ. 利用者が無いとき, 欄が足りないときは `None`.
pub fn home_of(passwd: &str, user: &str) -> Option<PathBuf> {
    passwd
        .lines()
        .map(|line| line.split(':').collect::<Vec<_>>())
        .find(|fields| fields.first() == Some(&user))
        .and_then(|fields| fields.get(5).map(PathBuf::from))
}

/// 起動の引数に従って, 読む初期設定ファイルを決めます.
///
/// # Arguments
///
/// * `options` - 起動の引数.
/// * `default` - 引数が無いときに読むファイル (`init_file_path`).
///
/// # Returns
///
/// `-q` と `-Q` では `None`. `--init-directory` があれば, そのディレクトリの `init.scm` (無ければ `None`).
/// `--user USER` があれば, USER のホームディレクトリの候補 ([`init_file_candidates`]) の初めにあるもの
/// (利用者が無ければ `None`). どれも無ければ `default`.
pub fn choose_init_file(options: &Options, default: Option<PathBuf>) -> Option<PathBuf> {
    if options.no_init_file {
        return None;
    }
    if let Some(dir) = &options.init_directory {
        return Some(Path::new(dir).join("init.scm")).filter(|path| path.exists());
    }
    match options.user.as_deref().filter(|user| !user.is_empty()) {
        Some(user) => {
            let passwd = std::fs::read_to_string("/etc/passwd").unwrap_or_default();
            let home = home_of(&passwd, user)?;
            let config = home.join(".config");
            init_file_candidates(Some(&home), Some(&config))
                .into_iter()
                .find(|path| path.exists())
        }
        None => default,
    }
}

/// 起動の引数を当てます: 始めのディレクトリ (`--chdir`), `user-emacs-directory` (`--init-directory`),
/// `init-file-debug` (`--debug-init`), `load-path` の `site-lisp` と `site-start.scm` (`--no-site-lisp`, `--no-site-file`). 初期設定ファイルを読む前に呼びます.
///
/// # Arguments
///
/// * `commands` - コマンドループ.
/// * `options` - 起動の引数.
pub fn apply_options<U: Host + LoopUi>(commands: &CommandLoop<U>, options: &Options) {
    let global = commands.evaluator().global();
    if let Some(dir) = &options.chdir {
        commands
            .evaluator()
            .set_default_directory(PathBuf::from(dir));
    }
    if let Some(dir) = &options.init_directory {
        let dir = if dir.ends_with('/') {
            dir.clone()
        } else {
            format!("{dir}/")
        };
        global.define("user-emacs-directory", Value::String(dir));
    }
    global.define("init-file-debug", Value::Boolean(options.debug_init));
    if !options.no_site_lisp {
        if let Some(Value::List(dirs)) = global.get("load-path") {
            let mut path: Vec<Value> = dirs.iter().cloned().collect();
            path.push(Value::String(SITE_LISP_DIR.to_string()));
            global.define("load-path", Value::List(path.into()));
        }
    }
    load_site_start(commands, options, Path::new(SITE_LISP_DIR));
}

/// 全ての利用者の初期設定 (`site-start.scm`) を読みます (Emacs の `site-start.el`).
/// 初期設定ファイルより先に (`apply_options` から) 呼びます. 失敗は `*Warnings*` に出します. `--no-site-file` と `--no-site-lisp` では読みません
/// (Emacs と同じく, `site-start.scm` は `site-lisp` から探す).
///
/// # Arguments
///
/// * `commands` - コマンドループ.
/// * `options` - 起動の引数.
/// * `dir` - `site-start.scm` を探すディレクトリ ([`SITE_LISP_DIR`]).
///
/// # Returns
///
/// 失敗の文 (`<path>: <内容>`). 読めたとき, ファイルが無いときは `None`.
pub fn load_site_start<U: Host + LoopUi>(
    commands: &CommandLoop<U>,
    options: &Options,
    dir: &Path,
) -> Option<String> {
    if options.no_site_file || options.no_site_lisp {
        return None;
    }
    let path = dir.join("site-start.scm");
    if !path.exists() {
        return None;
    }
    log::info!("Loading {}", path.display());
    let error = commands
        .eval(&format!(
            "(load {})",
            Value::String(path.to_string_lossy().into_owned())
        ))
        .err()?;
    let error = format!("{}: {}", path.display(), error);
    let warning = format!("An error occurred while loading the site-start file: {error}");
    let _ = commands.eval(&format!(
        "(display-warning 'initialization {} 'error)",
        Value::String(warning)
    ));
    Some(error)
}

/// 初期設定ファイルを読みます (`user-init-file` にする). 無ければ何もしません.
/// 失敗したら, `*Warnings*` に出し, 失敗の文を返します (Emacs と同じ).
///
/// # Arguments
///
/// * `commands` - コマンドループ.
/// * `path` - 初期設定ファイル.
///
/// # Returns
///
/// 失敗の文 (`<path>: <内容>`. `--debug-init` なら, 失敗した式を続ける). 読めたとき, ファイルが無いときは `None`.
pub fn load_init<U: Host + LoopUi>(
    commands: &CommandLoop<U>,
    path: Option<&Path>,
) -> Option<String> {
    let path = path.filter(|p| p.exists())?;
    log::info!("Loading {}", path.display());
    let debug = commands
        .evaluator()
        .global()
        .get("init-file-debug")
        .is_some_and(|value| value.is_non_nil());
    commands.evaluator().global().define(
        "user-init-file",
        Value::String(path.to_string_lossy().into_owned()),
    );
    let outcome = std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|code| commands.eval(&code).map_err(|e| init_error_text(&e, debug)));
    let failure = outcome.err().map(|error| {
        log::warn!("Could not load {}: {}", path.display(), error);
        let error = format!("{}: {}", path.display(), error);
        let warning = format!("An error occurred while loading the init file: {error}");
        let _ = commands.eval(&format!(
            "(display-warning 'initialization {} 'error)",
            Value::String(warning)
        ));
        error
    });
    commands.sync_lisp_keys();
    failure
}

/// 初期設定ファイルの失敗を, 文にします. `--debug-init` (`debug` が真) なら, 失敗した式を内側から並べます
/// (Emacs の `debug-on-error` の, バックトレースに当たる).
///
/// # Arguments
///
/// * `error` - 失敗.
/// * `debug` - 呼び出しの跡を付けるか.
///
/// # Returns
///
/// 失敗の文 (跡があれば, 改行して `Backtrace:` と式を続ける).
pub fn init_error_text(error: &crate::error::Error, debug: bool) -> String {
    let mut text = error.to_string();
    if debug && !error.trace().is_empty() {
        text.push_str("\nBacktrace:");
        for form in error.trace() {
            text.push_str("\n  ");
            text.push_str(form);
        }
    }
    text
}
