use rseditor_core::error::Error;
use rseditor_core::scheme::value::Value;

fn string(s: &str) -> Value {
    Value::String(s.to_string())
}

fn symbol(s: &str) -> Value {
    Value::Symbol(s.to_string())
}

#[test]
fn test_message_of_each_kind() {
    // Emacs の error-message-string と同じ文
    for (error, message) in [
        (Error::plain("boom"), "boom"),
        (Error::new("error", vec![]), "peculiar error"),
        (
            Error::new("error", vec![Value::Number(5.0), string("x")]),
            "peculiar error: \"x\"",
        ),
        (
            Error::new("error", vec![string("a"), Value::Number(1.0), string("b")]),
            "a: 1, \"b\"",
        ),
        (Error::bare("quit"), "Quit"),
        (Error::new("quit", vec![Value::Number(1.0)]), "Quit: 1"),
        (Error::bare("end-of-buffer"), "End of buffer"),
        (Error::bare("beginning-of-buffer"), "Beginning of buffer"),
        (Error::bare("arith-error"), "Arithmetic error"),
        (Error::bare("args-out-of-range"), "Args out of range"),
        (
            Error::args_out_of_range(vec![string("abc"), Value::Number(5.0)]),
            "Args out of range: \"abc\", 5",
        ),
        (
            Error::wrong_type("listp", &Value::Number(1.0)),
            "Wrong type argument: listp, 1",
        ),
        (
            Error::wrong_type("stringp", &Value::Boolean(false)),
            "Wrong type argument: stringp, nil",
        ),
        (Error::search_failed("x"), "Search failed: \"x\""),
        (
            Error::void_variable("foo"),
            "Symbol's value as variable is void: foo",
        ),
        (
            Error::void_function("foo"),
            "Symbol's function definition is void: foo",
        ),
        (
            Error::strings("scan-error", &["Unbalanced parentheses"]),
            "Scan error: \"Unbalanced parentheses\"",
        ),
        (
            Error::strings("invalid-regexp", &["Unmatched ["]),
            "Invalid regexp: \"Unmatched [\"",
        ),
        // end-of-file と user-error と file-error のデータは, 引用符なし
        (
            Error::strings("end-of-file", &["Error reading from stdin"]),
            "End of file during parsing: Error reading from stdin",
        ),
        (Error::bare("end-of-file"), "End of file during parsing"),
        (Error::strings("user-error", &["a", "b"]), "a, b"),
        (Error::bare("user-error"), ""),
        (
            Error::strings(
                "file-missing",
                &["Opening input file", "No such file or directory", "/x"],
            ),
            "Opening input file: No such file or directory, /x",
        ),
        (
            Error::strings("file-already-exists", &["File exists", "/x"]),
            "File exists: /x",
        ),
        (Error::bare("file-error"), "File error"),
        // 知らない種類
        (
            Error::new("foo-bar", vec![Value::Number(1.0), string("x")]),
            "peculiar error: 1, \"x\"",
        ),
        (
            Error::new("buffer-read-only", vec![symbol("#<buffer b>")]),
            "Buffer is read-only: #<buffer b>",
        ),
    ] {
        assert_eq!(error.message(), message, "{:?}", error.kind());
    }
}

#[test]
fn test_conditions() {
    let eof = Error::bare("end-of-buffer");
    assert_eq!(eof.conditions(), ["end-of-buffer", "error"]);
    assert!(eof.matches("end-of-buffer") && eof.matches("error") && eof.matches("t"));
    assert!(!eof.matches("quit") && !eof.is_quit());
    let missing = Error::strings("file-missing", &["a", "b", "c"]);
    assert_eq!(
        missing.conditions(),
        ["file-missing", "file-error", "error"]
    );
    assert!(missing.matches("file-error"));
    // quit は error に入らない. t は何でも受ける
    let quit = Error::bare("quit");
    assert!(quit.is_quit() && quit.matches("quit") && quit.matches("t") && !quit.matches("error"));
    assert!(Error::bare("minibuffer-quit").is_quit());
    assert!(Error::bare("text-read-only").matches("buffer-read-only"));
    // 知らない種類は, 受ける条件が無い
    let unknown = Error::new("whatever", vec![]);
    assert!(unknown.conditions().is_empty());
    assert!(!unknown.matches("whatever") && !unknown.matches("error") && unknown.matches("t"));
}

#[test]
fn test_signal_with_defined_conditions() {
    let error = Error::signal(
        "my-error",
        vec![Value::Number(1.0)],
        vec!["my-error".to_string(), "error".to_string()],
        Some("My error"),
    );
    assert_eq!(error.message(), "My error: 1");
    assert!(error.matches("my-error") && error.matches("error"));
    let plain = Error::signal("my-error", vec![], vec![], None);
    assert_eq!(plain.message(), "peculiar error");
}

#[test]
fn test_value_and_data() {
    let error = Error::args_out_of_range(vec![Value::Number(1.0), Value::Number(2.0)]);
    assert_eq!(error.kind(), "args-out-of-range");
    assert_eq!(error.data(), [Value::Number(1.0), Value::Number(2.0)]);
    assert_eq!(error.to_value().to_string(), "(args-out-of-range 1 2)");
    // (種類 . データ) から: リスト, nil, 1 つの値
    for (data, expected) in [
        (
            Value::List(vec![Value::Number(1.0)].into()),
            vec![Value::Number(1.0)],
        ),
        (Value::Boolean(false), vec![]),
        (Value::Nil, vec![]),
        (Value::Number(3.0), vec![Value::Number(3.0)]),
    ] {
        assert_eq!(Error::from_data("quit", &data).data(), expected.as_slice());
    }
}

#[test]
fn test_string_like_behaviour() {
    let error: Error = "boom".into();
    let other: Error = String::from("boom").into();
    assert_eq!(error, other);
    assert_ne!(error, Error::bare("quit"));
    assert!(error.starts_with("bo") && error.len() == 4);
    assert_eq!(error, "boom");
    assert_eq!(error, *"boom");
    assert_eq!(error, "boom".to_string());
    assert_eq!(format!("{error}"), "boom");
    assert_eq!(format!("{error:?}"), "\"boom\"");
    assert_eq!(serde_json::to_string(&error).unwrap(), "\"boom\"");
}

#[test]
fn test_open_input_file() {
    use std::io::{Error as IoError, ErrorKind};
    let missing = Error::open_input(&IoError::from(ErrorKind::NotFound), "/x");
    assert_eq!(missing.kind(), "file-missing");
    assert_eq!(
        missing.message(),
        "Opening input file: No such file or directory, /x"
    );
    let denied = Error::open_input(&IoError::from(ErrorKind::PermissionDenied), "/y");
    assert_eq!(denied.kind(), "file-error");
    assert_eq!(
        denied.message(),
        "Opening input file: Permission denied, /y"
    );
    let other = Error::open_input(&IoError::other("disk"), "/z");
    assert_eq!(other.kind(), "file-error");
    assert_eq!(other.message(), "Opening input file: disk, /z");
}
