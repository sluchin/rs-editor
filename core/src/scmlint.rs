//! Scheme の lint (`scmlint`): 動かす前に, rs-editor の Scheme の誤りを見つけます (Emacs の byte-compile の警告に当たる).
//!
//! 見つけるもの: 定義していない名前, 同じ名前の定義の重なり, rs-editor の Scheme に無い書き方 (ドット対, `cond` の `=>`,
//! `format` の `%s` `%S` `%d` `%o` `%x` `%X` `%%` (旗の `-` `0` と幅は付けられる) 以外, ベクタ, 文字), `'()` を偽として使った所, docstring の無いコマンド.
//! 行の中に `scmlint:ignore` と書けば (コメントで), その行の指摘は出さない.

use std::collections::{HashMap, HashSet};

use crate::scheme::evaluator::Evaluator;

/// 式の木 (行の番号つき).
#[derive(Debug, Clone)]
enum Node {
    /// 名前, 数, `#t` など.
    Atom(String, usize),
    /// 文字列.
    Str(String, usize),
    /// リスト.
    List(Vec<Node>, usize),
}

impl Node {
    fn line(&self) -> usize {
        match self {
            Node::Atom(_, line) | Node::Str(_, line) | Node::List(_, line) => *line,
        }
    }

    fn atom(&self) -> Option<&str> {
        match self {
            Node::Atom(name, _) => Some(name),
            _ => None,
        }
    }
}

/// 字句を読む.
struct Reader {
    chars: Vec<char>,
    at: usize,
    line: usize,
}

impl Reader {
    fn skip_blank(&mut self) {
        while let Some(&c) = self.chars.get(self.at) {
            if c == '\n' {
                self.line += 1;
            }
            if c == ';' {
                while self.chars.get(self.at).is_some_and(|&c| c != '\n') {
                    self.at += 1;
                }
            } else if c == '#' && self.chars.get(self.at + 1) == Some(&'|') {
                self.at += 2;
                while self.at < self.chars.len()
                    && !(self.chars[self.at] == '|' && self.chars.get(self.at + 1) == Some(&'#'))
                {
                    if self.chars[self.at] == '\n' {
                        self.line += 1;
                    }
                    self.at += 1;
                }
                self.at += 2;
            } else if c.is_whitespace() {
                self.at += 1;
            } else {
                break;
            }
        }
    }

    /// 式を 1 つ読む. 閉じ括弧に当たったら `Ok(None)`. 括弧が足りなければ `Err`.
    fn read(&mut self) -> Result<Option<Node>, (usize, String)> {
        self.skip_blank();
        let line = self.line;
        let Some(&c) = self.chars.get(self.at) else {
            return Ok(None);
        };
        match c {
            '(' | '[' => {
                self.at += 1;
                let mut items = Vec::new();
                loop {
                    self.skip_blank();
                    match self.chars.get(self.at) {
                        None => return Err((line, "unclosed parenthesis".to_string())),
                        Some(')' | ']') => {
                            self.at += 1;
                            return Ok(Some(Node::List(items, line)));
                        }
                        _ => {
                            if let Some(item) = self.read()? {
                                items.push(item);
                            }
                        }
                    }
                }
            }
            ')' | ']' => {
                self.at += 1;
                Err((line, "unexpected closing parenthesis".to_string()))
            }
            '\'' | '`' | ',' => {
                self.at += 1;
                let name = match (c, self.chars.get(self.at)) {
                    ('\'', _) => "quote",
                    ('`', _) => "quasiquote",
                    (_, Some('@')) => {
                        self.at += 1;
                        "unquote-splicing"
                    }
                    _ => "unquote",
                };
                let inner = self.read()?.unwrap_or(Node::List(Vec::new(), line));
                Ok(Some(Node::List(
                    vec![Node::Atom(name.to_string(), line), inner],
                    line,
                )))
            }
            '"' => {
                self.at += 1;
                let mut text = String::new();
                while let Some(&c) = self.chars.get(self.at) {
                    self.at += 1;
                    match c {
                        '"' => break,
                        '\\' => {
                            text.push(c);
                            if let Some(&next) = self.chars.get(self.at) {
                                text.push(next);
                                self.at += 1;
                            }
                        }
                        '\n' => {
                            self.line += 1;
                            text.push(c);
                        }
                        _ => text.push(c),
                    }
                }
                Ok(Some(Node::Str(text, line)))
            }
            '#' if self.chars.get(self.at + 1) == Some(&';') => {
                // `#;` の後ろの式は, コメント
                self.at += 2;
                self.read()?;
                self.skip_blank();
                match self.chars.get(self.at) {
                    None | Some(')' | ']') => Ok(None),
                    _ => self.read(),
                }
            }
            _ => {
                let mut text = String::new();
                // `#\` の後ろの 1 文字は, 区切りでも文字
                if c == '#' && self.chars.get(self.at + 1) == Some(&'\\') {
                    text.push_str("#\\");
                    self.at += 2;
                    if let Some(&next) = self.chars.get(self.at) {
                        text.push(next);
                        self.at += 1;
                    }
                }
                while let Some(&c) = self.chars.get(self.at) {
                    if c.is_whitespace() || "()[]\"';".contains(c) {
                        break;
                    }
                    text.push(c);
                    self.at += 1;
                }
                // ベクタ `#(...)`: 先頭が `#(` のリストにする
                if text == "#" && self.chars.get(self.at) == Some(&'(') {
                    self.at += 1;
                    let mut items = vec![Node::Atom("#(".to_string(), line)];
                    loop {
                        self.skip_blank();
                        match self.chars.get(self.at) {
                            None => return Err((line, "unclosed parenthesis".to_string())),
                            Some(')') => {
                                self.at += 1;
                                return Ok(Some(Node::List(items, line)));
                            }
                            _ => items.extend(self.read()?),
                        }
                    }
                }
                Ok(Some(Node::Atom(text, line)))
            }
        }
    }
}

/// 特別な形と, 評価器が名前なしで扱う語.
const SPECIAL: &[&str] = &[
    "quote",
    "quasiquote",
    "unquote",
    "unquote-splicing",
    "if",
    "define",
    "define-command",
    "set!",
    "lambda",
    "begin",
    "let",
    "let*",
    "letrec",
    "letrec*",
    "cond",
    "case",
    "and",
    "or",
    "when",
    "unless",
    "while",
    "do",
    "else",
    "condition-case",
    "unwind-protect",
    "save-restriction",
    "save-current-buffer",
    "save-excursion",
    "with-current-buffer",
    "ert-deftest",
    "should",
    "should-not",
    "should-error",
    "skip-unless",
];

/// 実行のときに, 核が定義する変数 (`command_loop.rs` の `self-insert-text` と, 評価器の `major-mode`).
const RUNTIME: &[&str] = &["self-insert-text", "major-mode"];

/// 指摘.
pub type Finding = (String, usize, String);

/// ファイル 1 つの式の木.
struct Source {
    name: String,
    forms: Vec<Node>,
    ignored: HashSet<usize>,
}

struct Lint {
    known: HashSet<String>,
    findings: Vec<Finding>,
    file: String,
}

fn is_number(text: &str) -> bool {
    text.parse::<f64>().is_ok()
}

impl Lint {
    fn report(&mut self, line: usize, message: String) {
        self.findings.push((self.file.clone(), line, message));
    }

    fn bound(scope: &[HashSet<String>], name: &str) -> bool {
        scope.iter().any(|s| s.contains(name))
    }

    /// 名前を使う所.
    fn use_name(&mut self, scope: &[HashSet<String>], name: &str, line: usize) {
        if name.starts_with("#\\") {
            self.report(line, "character literals are not supported".to_string());
        } else if name == "#(" {
            self.report(line, "vectors are not supported".to_string());
        } else if name == "." {
            self.report(line, "dotted pairs are not supported".to_string());
        } else if !(name.starts_with('#')
            || name.starts_with(':')
            || is_number(name)
            || Self::bound(scope, name)
            || self.known.contains(name))
        {
            self.report(line, format!("undefined name: {name}"));
        }
    }

    fn walk_all(&mut self, nodes: &[Node], scope: &mut Vec<HashSet<String>>) {
        for node in nodes {
            self.walk(node, scope);
        }
    }

    /// 本体 (内側の `define` は, 本体の中の名前).
    fn body(&mut self, nodes: &[Node], scope: &mut Vec<HashSet<String>>) {
        let mut local = HashSet::new();
        for node in nodes {
            if let Node::List(items, _) = node {
                if matches!(items.first().and_then(Node::atom), Some("define")) {
                    match items.get(1) {
                        Some(Node::Atom(name, _)) => local.insert(name.clone()),
                        Some(Node::List(head, _)) => head
                            .first()
                            .and_then(Node::atom)
                            .is_some_and(|n| local.insert(n.to_string())),
                        _ => false,
                    };
                }
            }
        }
        scope.push(local);
        self.walk_all(nodes, scope);
        scope.pop();
    }

    /// 引数の並びの名前 (`(a b . rest)` と `rest` も).
    fn params(nodes: &[Node]) -> HashSet<String> {
        nodes
            .iter()
            .filter_map(Node::atom)
            .filter(|n| *n != ".")
            .map(str::to_string)
            .collect()
    }

    fn lambda(&mut self, params: Option<&Node>, body: &[Node], scope: &mut Vec<HashSet<String>>) {
        let names = match params {
            Some(Node::List(items, _)) => Self::params(items),
            Some(Node::Atom(name, _)) => HashSet::from([name.clone()]),
            _ => HashSet::new(),
        };
        scope.push(names);
        self.body(body, scope);
        scope.pop();
    }

    /// `(let ((name init) ...) ...)` の束縛の並び: (名前, 初期値).
    fn bindings(nodes: &[Node]) -> Vec<(String, Option<&Node>)> {
        nodes
            .iter()
            .filter_map(|b| match b {
                Node::List(pair, _) => pair
                    .first()
                    .and_then(Node::atom)
                    .map(|n| (n.to_string(), pair.get(1))),
                Node::Atom(name, _) => Some((name.clone(), None)),
                Node::Str(..) => None,
            })
            .collect()
    }

    fn walk_quasi(&mut self, node: &Node, scope: &mut Vec<HashSet<String>>) {
        if let Node::List(items, _) = node {
            if matches!(
                items.first().and_then(Node::atom),
                Some("unquote" | "unquote-splicing")
            ) {
                self.walk_all(&items[1..], scope);
            } else {
                for item in items {
                    self.walk_quasi(item, scope);
                }
            }
        }
    }

    fn walk(&mut self, node: &Node, scope: &mut Vec<HashSet<String>>) {
        let (items, line) = match node {
            Node::Atom(name, line) => return self.use_name(scope, name, *line),
            Node::Str(..) => return,
            Node::List(items, line) => (items, *line),
        };
        let Some(head) = items.first() else { return };
        let args = &items[1..];
        let Some(name) = head.atom() else {
            return self.walk_all(items, scope);
        };
        match name {
            "quote" => {}
            "quasiquote" => args.iter().for_each(|a| self.walk_quasi(a, scope)),
            "define" | "define-command" => {
                if name == "define-command" {
                    let docstring = args
                        .get(1..)
                        .is_some_and(|b| b.len() > 1 && matches!(b[0], Node::Str(..)));
                    if !docstring {
                        self.report(line, "define-command without a docstring".to_string());
                    }
                }
                match args.first() {
                    Some(Node::List(head, _)) => {
                        let params = head.get(1..).unwrap_or_default();
                        self.lambda(Some(&Node::List(params.to_vec(), line)), &args[1..], scope);
                    }
                    _ => self.walk_all(args.get(1..).unwrap_or_default(), scope),
                }
            }
            "lambda" => self.lambda(args.first(), args.get(1..).unwrap_or_default(), scope),
            "let" | "let*" | "letrec" | "letrec*" => self.walk_let(name, args, scope),
            "do" => {
                let specs = match args.first() {
                    Some(Node::List(specs, _)) => Self::bindings(specs),
                    _ => Vec::new(),
                };
                // 初期値は外側で, 更新の式は, 変数のある中で読む
                let inits: Vec<(&Node, &[Node])> = match args.first() {
                    Some(Node::List(specs, _)) => specs
                        .iter()
                        .filter_map(|spec| match spec {
                            Node::List(parts, _) if parts.len() > 1 => {
                                Some((&parts[1], &parts[2..]))
                            }
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                for (init, _) in &inits {
                    self.walk(init, scope);
                }
                scope.push(specs.into_iter().map(|(n, _)| n).collect());
                for (_, steps) in &inits {
                    self.walk_all(steps, scope);
                }
                self.walk_all(args.get(1..).unwrap_or_default(), scope);
                scope.pop();
            }
            "cond" => {
                for clause in args {
                    if let Node::List(parts, _) = clause {
                        let rest = if parts.first().and_then(Node::atom) == Some("else") {
                            &parts[1..]
                        } else {
                            &parts[..]
                        };
                        if rest.iter().any(|p| p.atom() == Some("=>")) {
                            self.report(clause.line(), "cond => is not supported".to_string());
                        }
                        let values: Vec<Node> = rest
                            .iter()
                            .filter(|p| p.atom() != Some("=>"))
                            .cloned()
                            .collect();
                        self.walk_all(&values, scope);
                    }
                }
            }
            "case" => {
                self.walk_all(
                    args.first()
                        .into_iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .as_slice(),
                    scope,
                );
                for clause in args.get(1..).unwrap_or_default() {
                    if let Node::List(parts, _) = clause {
                        self.walk_all(&parts[1..], scope);
                    }
                }
            }
            "set!" => {
                if let Some(Node::Atom(target, target_line)) = args.first() {
                    self.use_name(scope, target, *target_line);
                }
                self.walk_all(args.get(1..).unwrap_or_default(), scope);
            }
            "condition-case" => {
                let var = args.first().and_then(Node::atom).unwrap_or("");
                self.walk_all(args.get(1..2).unwrap_or_default(), scope);
                scope.push(HashSet::from([var.to_string()]));
                for handler in args.get(2..).unwrap_or_default() {
                    if let Node::List(parts, _) = handler {
                        self.walk_all(&parts[1..], scope);
                    }
                }
                scope.pop();
            }
            "ert-deftest" => self.body(args.get(1..).unwrap_or_default(), scope),
            "if" | "when" | "unless" => {
                if let Some(Node::List(test, test_line)) = args.first() {
                    if test.len() == 2
                        && test[0].atom() == Some("quote")
                        && matches!(&test[1], Node::List(empty, _) if empty.is_empty())
                    {
                        self.report(*test_line, "'() is true in this Scheme".to_string());
                    }
                }
                self.walk_all(args, scope);
            }
            _ => {
                if name == "format" {
                    if let Some(Node::Str(text, text_line)) = args.first() {
                        self.check_format(text, *text_line);
                    }
                }
                if items.iter().any(|i| i.atom() == Some(".")) {
                    self.report(line, "dotted pairs are not supported".to_string());
                }
                self.use_name(scope, name, head.line());
                self.walk_all(args, scope);
            }
        }
    }

    fn walk_let(&mut self, name: &str, args: &[Node], scope: &mut Vec<HashSet<String>>) {
        let (label, rest) = match args.first() {
            Some(Node::Atom(label, _)) if name == "let" => (Some(label.clone()), &args[1..]),
            _ => (None, args),
        };
        let Some(Node::List(specs, _)) = rest.first() else {
            return self.walk_all(rest, scope);
        };
        let bindings = Self::bindings(specs);
        let body = &rest[1..];
        match name {
            "let*" => {
                scope.push(HashSet::new());
                for (var, init) in &bindings {
                    if let Some(init) = init {
                        self.walk(init, scope);
                    }
                    scope.last_mut().map(|s| s.insert(var.clone()));
                }
                self.body(body, scope);
                scope.pop();
            }
            "letrec" | "letrec*" => {
                scope.push(bindings.iter().map(|(n, _)| n.clone()).collect());
                for init in bindings.iter().filter_map(|(_, i)| *i) {
                    self.walk(init, scope);
                }
                self.body(body, scope);
                scope.pop();
            }
            _ => {
                for init in bindings.iter().filter_map(|(_, i)| *i) {
                    self.walk(init, scope);
                }
                let mut names: HashSet<String> = bindings.into_iter().map(|(n, _)| n).collect();
                names.extend(label);
                scope.push(names);
                self.body(body, scope);
                scope.pop();
            }
        }
    }

    fn check_format(&mut self, text: &str, line: usize) {
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '\\' {
                i += 1;
            } else if chars[i] == '%' {
                // 旗 (`-` `0`) と幅は, 読み飛ばす
                let mut at = i + 1;
                while chars.get(at).is_some_and(|c| matches!(c, '-' | '0'..='9')) {
                    at += 1;
                }
                match chars.get(at) {
                    Some('s' | 'S' | 'd' | 'o' | 'x' | 'X' | '%') => {}
                    other => {
                        let shown: String = other.map_or("end".to_string(), char::to_string);
                        self.report(line, format!("format does not support %{shown}"));
                    }
                }
                i = at;
            }
            i += 1;
        }
    }
}

/// 名前を定義する所 (一番外側の `define` と `define-command`).
fn top_level_names(form: &Node) -> Vec<(String, usize)> {
    let Node::List(items, line) = form else {
        return Vec::new();
    };
    match items.first().and_then(Node::atom) {
        Some("define" | "define-command") => match items.get(1) {
            Some(Node::Atom(name, _)) => vec![(name.clone(), *line)],
            Some(Node::List(head, _)) => head
                .first()
                .and_then(Node::atom)
                .map(|n| vec![(n.to_string(), *line)])
                .unwrap_or_default(),
            _ => Vec::new(),
        },
        Some("begin") => items[1..].iter().flat_map(top_level_names).collect(),
        _ => Vec::new(),
    }
}

/// 複数のファイル (名前と本文) を検査して, 指摘 (ファイル, 行, 文) を, ファイルと行の順で返します.
pub fn lint(files: &[(String, String)]) -> Vec<Finding> {
    let evaluator = Evaluator::new();
    let mut known: HashSet<String> = evaluator.global().names().into_iter().collect();
    known.extend(SPECIAL.iter().map(|s| s.to_string()));
    // 実行のときに, 核が定義する変数
    known.extend(RUNTIME.iter().map(|s| s.to_string()));
    let mut findings: Vec<Finding> = Vec::new();
    let mut sources = Vec::new();
    for (name, text) in files {
        let mut reader = Reader {
            chars: text.chars().collect(),
            at: 0,
            line: 1,
        };
        let mut forms = Vec::new();
        loop {
            match reader.read() {
                Ok(Some(form)) => forms.push(form),
                Ok(None) => break,
                Err((line, message)) => {
                    findings.push((name.clone(), line, message));
                    break;
                }
            }
        }
        let ignored = text
            .lines()
            .enumerate()
            .filter(|(_, l)| l.contains("scmlint:ignore"))
            .map(|(n, _)| n + 1)
            .collect();
        sources.push(Source {
            name: name.clone(),
            forms,
            ignored,
        });
    }
    // 定義: 名前ごとの (ファイル, 行)
    let mut defined: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for source in &sources {
        for form in &source.forms {
            for (name, line) in top_level_names(form) {
                known.insert(name.clone());
                defined
                    .entry(name)
                    .or_default()
                    .push((source.name.clone(), line));
            }
        }
    }
    for source in &sources {
        for form in &source.forms {
            for (name, line) in top_level_names(form) {
                let first = &defined[&name][0];
                if *first != (source.name.clone(), line) {
                    findings.push((
                        source.name.clone(),
                        line,
                        format!("{name} is already defined at {}:{}", first.0, first.1),
                    ));
                }
            }
        }
        let mut lint = Lint {
            known: known.clone(),
            findings: Vec::new(),
            file: source.name.clone(),
        };
        let mut scope = vec![HashSet::new()];
        lint.walk_all(&source.forms, &mut scope);
        findings.extend(lint.findings);
    }
    let ignored: HashMap<&str, &HashSet<usize>> = sources
        .iter()
        .map(|s| (s.name.as_str(), &s.ignored))
        .collect();
    findings
        .retain(|(file, line, _)| !ignored.get(file.as_str()).is_some_and(|l| l.contains(line)));
    findings.sort();
    findings.dedup();
    findings
}

/// 命令の行の引数 (`FILE...`) を読んで検査し, 指摘を `out` に足して, 終了の符号 (指摘があれば 1, 読めなければ 2) を返します.
pub fn run(args: &[String], out: &mut String) -> i32 {
    let mut files = Vec::new();
    for path in args {
        match std::fs::read_to_string(path) {
            Ok(text) => files.push((path.clone(), text)),
            Err(_) => {
                out.push_str(&format!("{path}: cannot read\n"));
                return 2;
            }
        }
    }
    let findings = lint(&files);
    for (file, line, message) in &findings {
        out.push_str(&format!("{file}:{line}: {message}\n"));
    }
    i32::from(!findings.is_empty())
}
