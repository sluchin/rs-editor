//! `scheme::po` のテスト.

use rseditor_core::scheme::po::*;
#[test]
fn parses_entries_and_skips_fuzzy_and_untranslated() {
    let catalog = parse(concat!(
        "# header\n",
        "msgid \"\"\n",
        "msgstr \"Content-Type: text/plain; charset=UTF-8\\n\"\n",
        "\n",
        "#: lisp/a.scm\n",
        "msgctxt \"function:a\"\n",
        "msgid \"\"\n",
        "\"Line \\\"one\\\".\\n\"\n",
        "\"Tab\\there \\\\ end\"\n",
        "msgstr \"一\\n二\"\n",
        "\n\n",
        "#, fuzzy, c-format\n",
        "#| msgid \"old\"\n",
        "msgctxt \"function:b\"\n",
        "msgid \"B\"\n",
        "msgstr \"古い\"\n",
        "\n",
        "msgctxt \"variable:c\"\n",
        "msgid \"C\"\n",
        "msgstr \"\"\n",
        "\n",
        "#~ msgctxt \"function:d\"\n",
        "msgctxt \"function:e\"\n",
        "msgid \"E\"\n",
        "msgstr \"い\" \n",
        "unknown \"x\"\n",
        "\"ignored\"\n",
        "msgstr\n",
        "\"\\\n",
    ));
    assert_eq!(
        catalog.get(&(
            "function:a".to_string(),
            "Line \"one\".\nTab\there \\ end".to_string()
        )),
        Some(&"一\n二".to_string())
    );
    assert_eq!(
        catalog.get(&("function:e".to_string(), "E".to_string())),
        Some(&"い".to_string())
    );
    // ヘッダ (msgctxt が無い), fuzzy, 翻訳の無いものは, 入れない
    assert_eq!(catalog.len(), 2);
    assert_eq!(unquote("no quotes"), "");
    // 終わりの \ は, 捨てる
    assert_eq!(unquote("\"a\\\""), "a");
}

#[test]
fn translates_with_the_embedded_files() {
    assert!(translate(
        "ja",
        "variable:howm-directory",
        "Directory where memos are stored."
    )
    .is_some());
    assert_eq!(translate("ja", "variable:howm-directory", "changed"), None);
    assert_eq!(
        translate(
            "xx",
            "variable:howm-directory",
            "Directory where memos are stored."
        ),
        None
    );
}
