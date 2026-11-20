//! 起動の引数 (`startup`) のテスト.

use std::sync::{Arc, Mutex};

use rseditor_core::command_loop::{CommandLoop, LoopUi};
use rseditor_core::editor::Editor;
use rseditor_core::keymap::Keymap;
use rseditor_core::scheme::editor_host::EditorHost;
use rseditor_core::scheme::evaluator::Evaluator;
use rseditor_core::scheme::host::Host;
use rseditor_core::scheme::value::Value;
use rseditor_core::startup::{
    apply_options, choose_init_file, home_of, load_site_start, parse, Options, SITE_LISP_DIR,
};

/// 偽の画面 (何も頼まない).
struct Ui;

impl Host for Ui {
    fn call(&self, _op: &str, _args: Vec<Value>) -> Result<Value, rseditor_core::error::Error> {
        Ok(Value::Nil)
    }
}

impl LoopUi for Ui {
    fn say(&self, _text: &str) {}

    fn stop_macro(&self) {}

    fn set_display(&self, _truncate_lines: bool, _selective_display: Option<usize>) {}
}

fn setup() -> CommandLoop<Ui> {
    CommandLoop::new(
        EditorHost::new(Editor::new(), Ui),
        Evaluator::new(),
        Arc::new(Mutex::new(Keymap::default())),
    )
    .unwrap()
}

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_parse_without_arguments_is_default() {
    assert_eq!(parse(&[]).unwrap(), Options::default());
}

#[test]
fn test_parse_flags_and_files() {
    let options = parse(&args(&["a.txt", "-q", "--debug-init", "b.txt"])).unwrap();
    assert!(options.no_init_file);
    assert!(options.debug_init);
    assert!(!options.no_splash);
    assert_eq!(options.files, ["a.txt", "b.txt"]);
}

#[test]
fn test_parse_long_and_single_dash_forms() {
    for flag in ["--no-init-file", "-no-init-file", "-q"] {
        assert!(parse(&args(&[flag])).unwrap().no_init_file, "{flag}");
    }
    for flag in ["--no-splash", "-no-splash"] {
        assert!(parse(&args(&[flag])).unwrap().no_splash, "{flag}");
    }
    assert!(parse(&args(&["-debug-init"])).unwrap().debug_init);
}

#[test]
fn test_parse_quick_means_no_init_file_and_no_splash() {
    for flag in ["-Q", "--quick", "-quick"] {
        let options = parse(&args(&[flag])).unwrap();
        assert!(options.no_init_file, "{flag}");
        assert!(options.no_splash, "{flag}");
        assert!(!options.debug_init, "{flag}");
    }
}

#[test]
fn test_parse_values_in_both_forms() {
    let options = parse(&args(&["--chdir", "/tmp/a", "--init-directory=/tmp/b"])).unwrap();
    assert_eq!(options.chdir.as_deref(), Some("/tmp/a"));
    assert_eq!(options.init_directory.as_deref(), Some("/tmp/b"));
    let options = parse(&args(&["--chdir=/tmp/c", "-init-directory", "/tmp/d", "f"])).unwrap();
    assert_eq!(options.chdir.as_deref(), Some("/tmp/c"));
    assert_eq!(options.init_directory.as_deref(), Some("/tmp/d"));
    assert_eq!(options.files, ["f"]);
}

#[test]
fn test_parse_missing_value_fails() {
    assert_eq!(
        parse(&args(&["--chdir"])).unwrap_err(),
        "Option `--chdir' requires an argument"
    );
    assert!(parse(&args(&["--init-directory"])).is_err());
}

#[test]
fn test_parse_unknown_arguments_are_files() {
    assert_eq!(
        parse(&args(&["--unknown", "x"])).unwrap().files,
        ["--unknown", "x"]
    );
}

#[test]
fn test_parse_site_and_user_options() {
    let options = parse(&args(&["--no-site-file", "-nsl", "-u", "bob"])).unwrap();
    assert!(options.no_site_file);
    assert!(options.no_site_lisp);
    assert_eq!(options.user.as_deref(), Some("bob"));
    let options = parse(&args(&["--no-site-lisp", "--user=amy"])).unwrap();
    assert!(!options.no_site_file);
    assert!(options.no_site_lisp);
    assert_eq!(options.user.as_deref(), Some("amy"));
    assert_eq!(
        parse(&args(&["-no-site-file", "-user", "x"]))
            .unwrap()
            .user
            .as_deref(),
        Some("x")
    );
    assert!(parse(&args(&["--no-site-lisp"])).unwrap().user.is_none());
    for flag in ["-Q", "--quick"] {
        let options = parse(&args(&[flag])).unwrap();
        assert!(options.no_site_file && options.no_site_lisp, "{flag}");
    }
    assert!(parse(&args(&["--user"])).is_err());
}

#[test]
fn test_home_of_reads_the_passwd_format() {
    let passwd =
        "root:x:0:0:root:/root:/bin/bash\nbob:x:1000:1000:Bob:/home/bob:/bin/zsh\nshort:x:1\n";
    assert_eq!(home_of(passwd, "bob").unwrap().to_str(), Some("/home/bob"));
    assert!(home_of(passwd, "short").is_none());
    assert!(home_of(passwd, "nobody").is_none());
}

#[test]
fn test_choose_init_file_with_a_user() {
    let default = Some(std::path::PathBuf::from("/default"));
    let options = |user: &str| Options {
        user: Some(user.to_string()),
        ..Options::default()
    };
    // 空の名前は, 今の利用者 (既定)
    assert_eq!(choose_init_file(&options(""), default.clone()), default);
    // 無い利用者と, 初期設定ファイルの無い利用者は, 読まない
    assert_eq!(
        choose_init_file(&options("no-such-user-zz"), default.clone()),
        None
    );
    assert_eq!(choose_init_file(&options("root"), default.clone()), None);
    // -q は, 利用者より強い
    let quiet = Options {
        no_init_file: true,
        ..options("root")
    };
    assert_eq!(choose_init_file(&quiet, default), None);
}

#[test]
fn test_apply_options_adds_site_lisp_to_load_path() {
    let commands = setup();
    apply_options(&commands, &Options::default());
    let path = commands.eval("load-path").unwrap();
    assert!(path.ends_with(&format!("\"{SITE_LISP_DIR}\")")), "{path}");
    let commands = setup();
    apply_options(
        &commands,
        &Options {
            no_site_lisp: true,
            ..Options::default()
        },
    );
    assert!(!commands.eval("load-path").unwrap().contains(SITE_LISP_DIR));
}

#[test]
fn test_load_site_start() {
    let dir = tempfile::tempdir().unwrap();
    let commands = setup();
    // ファイルが無ければ, 何もしない
    assert_eq!(
        load_site_start(&commands, &Options::default(), dir.path()),
        None
    );
    std::fs::write(dir.path().join("site-start.scm"), "(define site-loaded 7)").unwrap();
    // 切る引数
    for options in [
        Options {
            no_site_file: true,
            ..Options::default()
        },
        Options {
            no_site_lisp: true,
            ..Options::default()
        },
    ] {
        assert_eq!(load_site_start(&commands, &options, dir.path()), None);
    }
    assert!(commands.eval("site-loaded").is_err());
    assert_eq!(
        load_site_start(&commands, &Options::default(), dir.path()),
        None
    );
    assert_eq!(commands.eval("site-loaded").unwrap(), "7");
    // 失敗は, 知らせる
    std::fs::write(dir.path().join("site-start.scm"), "(car 1)").unwrap();
    let error = load_site_start(&commands, &Options::default(), dir.path()).unwrap();
    assert!(error.contains("site-start.scm"), "{error}");
}
