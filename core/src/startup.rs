//! 起動の引数 (Emacs の Initial options).
//!
//! `-q` (`--no-init-file`), `-Q` (`--quick`), `--no-splash`, `--debug-init`, `--chdir DIR`,
//! `--init-directory DIR` を読みます. 値は `--chdir DIR` と `--chdir=DIR` の両方の書き方を受けます.
//! 知らない引数は、Emacs と同じく、開くファイルとして扱います.

/// 起動の引数を読んだ結果.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Options {
    /// 初期設定ファイルを読まない (`-q`, `--no-init-file`, `-Q`, `--quick`).
    pub no_init_file: bool,
    /// 始めの画面を出さない (`--no-splash`, `-Q`, `--quick`).
    pub no_splash: bool,
    /// 初期設定ファイルの失敗で、細かい知らせを出す (`--debug-init`).
    pub debug_init: bool,
    /// 始めのディレクトリ (`--chdir`).
    pub chdir: Option<String>,
    /// 初期設定ファイルを探すディレクトリ (`--init-directory`).
    pub init_directory: Option<String>,
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
            }
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
                } else {
                    options.files.push(other.to_string());
                }
            }
        }
    }
    Ok(options)
}

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
