//! docstring の翻訳 (gettext の PO 形式のファイル. `po/*.po`).
//!
//! 翻訳のファイルは, `tools/po/update.py` が作る `po/rseditor.pot` から作ります.
//! `msgctxt` は `function:名前` か `variable:名前`, `msgid` は英語の docstring, `msgstr` は翻訳です.
//! gettext の実行時の仕組み (libintl) は使わず, ビルドのときに埋め込んだファイルを, ここで読みます.
//! 翻訳の無いものと, `fuzzy` (元の docstring が変わった) の翻訳は, 使いません.

use std::collections::HashMap;
use std::sync::OnceLock;

/// 言語と, その PO のファイルの中身.
pub const LANGUAGES: &[(&str, &str)] = &[("ja", include_str!("../../../po/ja.po"))];

/// 翻訳の表 (`(msgctxt, msgid)` から `msgstr`).
pub type Catalog = HashMap<(String, String), String>;

/// 読んでいる途中の, PO の 1 つの項目.
#[derive(Default)]
struct Entry {
    /// `#, fuzzy` の印があるか.
    fuzzy: bool,
    /// `msgctxt`.
    ctxt: Option<String>,
    /// `msgid`.
    id: String,
    /// `msgstr`.
    text: String,
}

/// 読んでいる所 (続きの `"..."` の行を, どこに足すか).
#[derive(Clone, Copy)]
enum Field {
    Ctxt,
    Id,
    Text,
}

impl Entry {
    /// 項目を, 表に入れます (`msgctxt` と翻訳があり, fuzzy でないものだけ).
    ///
    /// # Arguments
    ///
    /// * `catalog` - 表.
    fn finish(self, catalog: &mut Catalog) {
        if let Some(ctxt) = self.ctxt {
            if !self.fuzzy && !self.text.is_empty() {
                catalog.insert((ctxt, self.id), self.text);
            }
        }
    }

    /// 項目の文字列に, 続きを足します.
    ///
    /// # Arguments
    ///
    /// * `field` - 足す所.
    /// * `text` - 足す文字列 (戻した後).
    fn push(&mut self, field: Field, text: &str) {
        match field {
            Field::Ctxt => self.ctxt.get_or_insert_with(String::new).push_str(text),
            Field::Id => self.id.push_str(text),
            Field::Text => self.text.push_str(text),
        }
    }
}

/// PO の `"..."` を, 中身の文字列にします (`\n`, `\t`, `\"`, `\\` を戻す).
///
/// # Arguments
///
/// * `quoted` - `"` で囲んだ文字列.
///
/// # Returns
///
/// 中身. `"` で囲んでいなければ, 空.
pub fn unquote(quoted: &str) -> String {
    let inner = quoted
        .trim()
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or("");
    let mut text = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            text.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => text.push('\n'),
            Some('t') => text.push('\t'),
            Some(other) => text.push(other),
            None => {}
        }
    }
    text
}

/// PO のファイルを読んで, 翻訳の表を作ります.
///
/// # Arguments
///
/// * `po` - PO のファイルの中身.
///
/// # Returns
///
/// 翻訳の表.
pub fn parse(po: &str) -> Catalog {
    let mut catalog = Catalog::new();
    let mut entry = Entry::default();
    let mut field = None;
    for line in po.lines().map(str::trim_end) {
        if line.is_empty() {
            // 空の行で, 項目が終わる
            std::mem::take(&mut entry).finish(&mut catalog);
            field = None;
        } else if let Some(flags) = line.strip_prefix("#,") {
            entry.fuzzy |= flags.split(',').any(|flag| flag.trim() == "fuzzy");
        } else if line.starts_with('#') {
            // コメント, ファイルの場所, 前の msgid (#|), 古い項目 (#~)
        } else if line.starts_with('"') {
            if let Some(field) = field {
                entry.push(field, &unquote(line));
            }
        } else if let Some((keyword, rest)) = line.split_once(char::is_whitespace) {
            field = match keyword {
                "msgctxt" => Some(Field::Ctxt),
                "msgid" => Some(Field::Id),
                "msgstr" => Some(Field::Text),
                _ => None,
            };
            if let Some(field) = field {
                entry.push(field, &unquote(rest));
            }
        }
    }
    entry.finish(&mut catalog);
    catalog
}

/// 埋め込んだ全ての言語の, 翻訳の表 (始めて使うときに読む).
fn catalogs() -> &'static HashMap<&'static str, Catalog> {
    static CATALOGS: OnceLock<HashMap<&'static str, Catalog>> = OnceLock::new();
    CATALOGS.get_or_init(|| {
        LANGUAGES
            .iter()
            .map(|(language, po)| (*language, parse(po)))
            .collect()
    })
}

/// docstring の翻訳を引きます.
///
/// # Arguments
///
/// * `language` - 言語 (`ja` など).
/// * `context` - `function:名前` か `variable:名前`.
/// * `doc` - 元の docstring (英語).
///
/// # Returns
///
/// 翻訳. 言語か翻訳が無いか, 元の docstring が変わっていれば [`None`].
pub fn translate(language: &str, context: &str, doc: &str) -> Option<&'static str> {
    catalogs()
        .get(language)?
        .get(&(context.to_string(), doc.to_string()))
        .map(String::as_str)
}
