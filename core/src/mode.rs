//! メジャーモードの情報: モードラインに出す名前、括弧の読み方 (Lisp の方言)、字下げの決まり.
//!
//! Tauri の画面と TUI が、同じ判定を使うため、核に置きます (ファイルの名前からモードを決めるのは、Lisp の `auto-mode-alist` と `set-auto-mode`).
//! ファイルの変数 (1 行目の `-*- ... -*-`) は、Emacs と同じく、`mode`、`lexical-binding`、`indent-tabs-mode`、`tab-width` を読みます.

use serde::Serialize;

use crate::buffer::Buffer;
use crate::highlight::Language;
use crate::syntax::Dialect;

/// ファイルの変数 (Emacs の, 1 行目の `-*- ... -*-`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileVariables {
    /// メジャーモード (`mode: scheme` など).
    pub mode: Option<String>,
    /// 字句の束縛 (`lexical-binding: t`).
    pub lexical_binding: Option<bool>,
    /// 字下げにタブを使うか (`indent-tabs-mode: nil`).
    pub indent_tabs_mode: Option<bool>,
    /// タブの幅 (`tab-width: 4`).
    pub tab_width: Option<usize>,
}

/// 1 行目 (`#!` で始まれば 2 行目) の `-*- ... -*-` から、ファイルの変数を読みます.
/// `-*- scheme -*-` のように、名前だけならメジャーモードです.
///
/// # Arguments
///
/// * `first` - 本文の 1 行目.
/// * `second` - 本文の 2 行目 (無ければ空).
pub fn file_variables(first: &str, second: &str) -> FileVariables {
    let line = if first.starts_with("#!") {
        second
    } else {
        first
    };
    let Some(start) = line.find("-*-") else {
        return FileVariables::default();
    };
    let rest = &line[start + 3..];
    let Some(end) = rest.find("-*-") else {
        return FileVariables::default();
    };
    let body = rest[..end].trim();
    let mut vars = FileVariables::default();
    if !body.contains(':') {
        vars.mode = Some(body.to_string());
        return vars;
    }
    for pair in body.split(';') {
        let (key, value) = pair.split_once(':').unwrap_or((pair, ""));
        let (key, value) = (key.trim(), value.trim());
        match key {
            "mode" => vars.mode = Some(value.to_string()),
            "lexical-binding" => vars.lexical_binding = Some(value != "nil"),
            "indent-tabs-mode" => vars.indent_tabs_mode = Some(value != "nil"),
            "tab-width" if !value.starts_with('0') => vars.tab_width = value.parse().ok(),
            _ => {}
        }
    }
    vars
}

/// メジャーモードの情報.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModeInfo {
    /// モードラインに出す名前.
    pub name: String,
    /// 括弧の読み方 (`scheme` か `elisp`. Lisp でなければ `None`).
    pub language: Option<&'static str>,
    /// 字下げにタブを使うか (Emacs の `indent-tabs-mode`).
    pub indent_tabs: bool,
    /// タブの幅 (Emacs の `tab-width`).
    pub tab_width: usize,
}

impl Buffer {
    /// ファイルの変数 (ファイルを訪れているバッファだけ. 無ければ空).
    pub fn file_variables(&self) -> FileVariables {
        if self.file().is_none() {
            return FileVariables::default();
        }
        file_variables(&self.line_text(0), &self.line_text(1))
    }

    /// メジャーモードの情報を返します.
    pub fn mode_info(&self) -> ModeInfo {
        let vars = self.file_variables();
        let info = |name: &str, language: Option<&'static str>| ModeInfo {
            name: name.to_string(),
            language,
            // Emacs の既定は、タブで字下げし、タブの幅は 8. ファイルの変数で変えられる
            indent_tabs: vars.indent_tabs_mode.unwrap_or(true),
            tab_width: vars.tab_width.unwrap_or(8),
        };
        if self.directory().is_some() {
            return ModeInfo {
                name: "Dired by name".to_string(),
                language: None,
                indent_tabs: true,
                tab_width: 8,
            };
        }
        match self.name() {
            "*Help*" => return info("Help", None),
            "*grep*" => {
                return ModeInfo {
                    name: "Grep".to_string(),
                    language: None,
                    indent_tabs: true,
                    tab_width: 8,
                }
            }
            _ => {}
        }
        let language = match self.highlight_language() {
            Some(Language::Scheme) => Some("scheme"),
            Some(Language::Elisp) => Some("elisp"),
            _ => None,
        };
        let mode = self.mode_name.as_deref().unwrap_or(match self.dialect() {
            Dialect::Fundamental => "fundamental-mode",
            Dialect::Elisp => "emacs-lisp-mode",
            Dialect::Scheme => "scheme-mode",
        });
        let name = match mode {
            "lisp-interaction-mode" => "Lisp Interaction",
            "lisp-data-mode" => "Lisp-Data",
            "emacs-lisp-mode" if vars.lexical_binding == Some(true) => "ELisp/l",
            "emacs-lisp-mode" => "ELisp/d",
            "scheme-mode" => "Scheme",
            "c-mode" => "C",
            "c++-mode" => "C++",
            "rust-mode" => "Rust",
            "typescript-mode" => "TypeScript",
            "org-mode" => "Org",
            "text-mode" => "Text",
            _ => "Fundamental",
        };
        info(name, language)
    }
}
