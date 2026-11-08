//! 起動の引数 (`startup`) のテスト.

use rseditor_core::startup::{parse, Options};

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
