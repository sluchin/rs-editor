//! Scheme の整形 (`scmfmt`): エディタの `indent-region` と同じ字下げにし, 行の終わりの空白, 閉じ括弧だけの行, 空行の重なりを消します.
//!
//! 文字列 (`"..."`) と `#| |#` のコメントの中, コメントの位置は変えません.

use crate::indent::{indent_lines, Config, IndentLines};
use crate::syntax::Dialect;

/// 行の終わりでの, 文字の並びの状態.
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    /// 式の中 (外).
    Code,
    /// 文字列の中.
    Text,
    /// `#| |#` のコメントの中.
    Block,
}

/// 整形で見る行 1 つ.
struct Line {
    /// 本文 (行の終わりの空白を除く).
    text: String,
    /// 行の始めでの状態.
    start: State,
    /// 行の終わりでの状態.
    end: State,
    /// 行の終わりが, `;` のコメントか.
    comment: bool,
}

/// 行を読んで, 終わりでの状態と, `;` のコメントがあるかを返します.
fn scan(text: &str, start: State) -> (State, bool) {
    let chars: Vec<char> = text.chars().collect();
    let mut state = start;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match state {
            State::Text => match c {
                '\\' => i += 1,
                '"' => state = State::Code,
                _ => {}
            },
            State::Block => {
                if c == '|' && chars.get(i + 1) == Some(&'#') {
                    state = State::Code;
                    i += 1;
                }
            }
            State::Code => match c {
                '"' => state = State::Text,
                ';' => return (State::Code, true),
                '#' if chars.get(i + 1) == Some(&'|') => {
                    state = State::Block;
                    i += 1;
                }
                // 文字 (`#\"`, `#\;`) は, 次の 1 文字を読み飛ばす
                '#' if chars.get(i + 1) == Some(&'\\') => i += 2,
                _ => {}
            },
        }
        i += 1;
    }
    (state, false)
}

/// 閉じ括弧だけの行か.
fn only_closers(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c == ')' || c == ']')
}

/// 空白を直す: 行の終わりの空白, 閉じ括弧だけの行, 空行の重なり, 始めと終わりの空行.
fn tidy(source: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut state = State::Code;
    for raw in source.split('\n') {
        let start = state;
        let (end, comment) = scan(raw, start);
        state = end;
        let text = if end == State::Code {
            raw.trim_end().to_string()
        } else {
            raw.to_string()
        };
        let line = Line {
            text,
            start,
            end,
            comment,
        };
        if line.start == State::Code && line.end == State::Code {
            if line.text.is_empty() {
                // 始めの空行と, 重なった空行
                if lines.last().is_none_or(|p| p.text.is_empty()) {
                    continue;
                }
            } else if only_closers(line.text.trim_start()) {
                if let Some(p) = lines.last_mut() {
                    if !p.text.is_empty() && p.end == State::Code && !p.comment {
                        p.text.push_str(line.text.trim_start());
                        continue;
                    }
                }
            }
        }
        lines.push(line);
    }
    while lines.last().is_some_and(|l| l.text.is_empty()) {
        lines.pop();
    }
    lines
}

/// Scheme のソースを整形して返します (空なら空. そうでなければ, 終わりは改行 1 つ).
pub fn format_source(source: &str) -> String {
    let lines = tidy(source);
    if lines.is_empty() {
        return String::new();
    }
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.text.chars().collect()).collect();
    let config = Config {
        dialect: Dialect::Scheme,
        indent_tabs: false,
        tab_width: 8,
        project_forms: true,
    };
    let result = indent_lines(
        &chars,
        IndentLines {
            from: 1,
            to: chars.len(),
            ..IndentLines::default()
        },
        &config,
    );
    let mut out = String::new();
    for (n, line) in lines.iter().enumerate() {
        // 文字列と, ブロックのコメントの中で始まる行は, 変えない
        match result.lines.get(&(n + 1)) {
            Some(changed) if line.start == State::Code => out.extend(changed),
            _ => out.push_str(&line.text),
        }
        out.push('\n');
    }
    out
}

/// 命令の行の引数を扱い, 出力 (標準出力に出す文) を `out` に足して, 終了の符号を返します.
/// 引数: `[--check] FILE...`. `--check` は, 直す所のあるファイルの名前を出し, あれば 1 を返す (ファイルは書き換えない).
/// 書き換えるときは, 直したファイルの名前を出す. 読めないファイルは, 2 を返す.
pub fn run(args: &[String], out: &mut String) -> i32 {
    let check = args.iter().any(|a| a == "--check");
    let mut code = 0;
    for path in args.iter().filter(|a| *a != "--check") {
        let Ok(source) = std::fs::read_to_string(path) else {
            out.push_str(&format!("{path}: cannot read\n"));
            code = 2;
            continue;
        };
        let formatted = format_source(&source);
        if formatted == source {
            continue;
        }
        out.push_str(&format!("{path}\n"));
        if check {
            code = code.max(1);
        } else if std::fs::write(path, formatted).is_err() {
            out.push_str(&format!("{path}: cannot write\n"));
            code = 2;
        }
    }
    code
}
