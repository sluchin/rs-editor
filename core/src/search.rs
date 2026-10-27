//! 探す (Emacs の `re-search-forward`, `re-search-backward`, `looking-at`, `search-forward`, `match-data`).
//!
//! Emacs の正規表現 (`\(` `\|` `\{` など) を Rust の regex の書き方に直して探します.
//! 期待値は, `emacs -Q --batch` の結果 (`tools/syntax/run.sh` で作る `core/tests/search.json`) と比べます.
//! 単語の境界 (`\<`, `\>`, `\b`, `\_<`) は, 構文表 (標準) の単語と記号の文字で, 先読みと後読みで書きます.
//! 合わせられない書き方 (`\=`, `\c`, 一部の構文クラス) は, 失敗にします.
//! 括弧の中の繰り返しで, 空の繰り返しが最後に合ったときの括弧の位置 (`\(a*\)*b` の `\1`) は, Emacs と違うことがあります.

use fancy_regex::Regex;

use crate::buffer::Buffer;

/// 単語の文字 (Emacs の標準の構文表の, 単語の構文).
const WORD: &str = r"\p{L}\p{N}$%";

/// 記号の文字 (単語の文字と, 標準の構文表の記号の構文).
const SYMBOL: &str = r"\p{L}\p{N}$%_\-+*/&|<>=";

/// 単語 (`word`) の境界の式. 始め (`start`), 終わり (`end`), どちらか (`either`).
fn boundary(set: &str, kind: &str) -> String {
    let start = format!("(?<![{set}])(?=[{set}])");
    let end = format!("(?<=[{set}])(?![{set}])");
    match kind {
        "start" => start,
        "end" => end,
        // `\b` は, バッファの始めと終わりにも合う
        _ => format!(r"(?:\A|\z|{start}|{end})"),
    }
}

/// Emacs の正規表現を, Rust の regex の書き方にします.
///
/// # Arguments
///
/// * `pattern` - Emacs の正規表現.
///
/// # Errors
///
/// 書き方が誤っているか, 合わせられない書き方のとき.
pub fn translate(pattern: &str) -> Result<String, String> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    // 選択肢の始めか (`^` が行の始めを表し, `*` が文字になる所)
    let mut at_start = true;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;
        match c {
            '\\' => {
                let Some(&n) = chars.get(i) else {
                    return Err("Trailing backslash".to_string());
                };
                i += 1;
                match n {
                    '(' => {
                        if chars.get(i) == Some(&'?') && chars.get(i + 1) == Some(&':') {
                            i += 2;
                            out.push_str("(?:");
                        } else if chars.get(i) == Some(&'?') {
                            return Err("Numbered groups are not supported".to_string());
                        } else {
                            out.push('(');
                        }
                        at_start = true;
                        continue;
                    }
                    ')' => out.push(')'),
                    '|' => {
                        out.push('|');
                        at_start = true;
                        continue;
                    }
                    '{' => {
                        let close = (i..chars.len().saturating_sub(1))
                            .find(|&j| chars[j] == '\\' && chars[j + 1] == '}')
                            .ok_or("Unmatched \\{")?;
                        let body: String = chars[i..close].iter().collect();
                        if body.is_empty() || !body.chars().all(|c| c.is_ascii_digit() || c == ',')
                        {
                            return Err("Invalid content of \\{\\}".to_string());
                        }
                        out.push_str(&format!("{{{body}}}"));
                        i = close + 2;
                    }
                    '}' => return Err("Unmatched \\}".to_string()),
                    d @ '1'..='9' => out.push_str(&format!(r"\{d}")),
                    'w' => out.push_str(&format!("[{WORD}]")),
                    'W' => out.push_str(&format!("[^{WORD}]")),
                    's' | 'S' => {
                        let Some(&class) = chars.get(i) else {
                            return Err("Invalid syntax designator".to_string());
                        };
                        i += 1;
                        let set = match class {
                            '-' | ' ' => r" \t\n\r\x0c".to_string(),
                            'w' => WORD.to_string(),
                            '_' => r"_\-+*/&|<>=".to_string(),
                            _ => return Err("Unsupported syntax class".to_string()),
                        };
                        out.push_str(&format!("[{}{set}]", if n == 'S' { "^" } else { "" }));
                    }
                    '_' => match chars.get(i) {
                        Some('<') => {
                            i += 1;
                            out.push_str(&boundary(SYMBOL, "start"));
                        }
                        Some('>') => {
                            i += 1;
                            out.push_str(&boundary(SYMBOL, "end"));
                        }
                        _ => return Err("Invalid regular expression".to_string()),
                    },
                    '<' => out.push_str(&boundary(WORD, "start")),
                    '>' => out.push_str(&boundary(WORD, "end")),
                    'b' => out.push_str(&boundary(WORD, "either")),
                    'B' => out.push_str(&format!("(?!{})", boundary(WORD, "either"))),
                    '`' => out.push_str(r"\A"),
                    '\'' => out.push_str(r"\z"),
                    '=' | 'c' | 'C' | 'e' | 'E' => {
                        return Err("Unsupported regular expression".to_string())
                    }
                    other => out.push_str(&regex::escape(&other.to_string())),
                }
                at_start = false;
            }
            '[' => {
                let (set, next) = bracket(&chars, i)?;
                out.push_str(&set);
                i = next;
                at_start = false;
            }
            '*' | '+' | '?' => {
                if at_start {
                    out.push('\\');
                    out.push(c);
                    at_start = false;
                } else {
                    out.push(c);
                }
            }
            '^' => {
                if at_start {
                    out.push_str("(?m:^)");
                } else {
                    out.push_str(r"\^");
                }
            }
            '$' => {
                let rest = &chars[i..];
                let at_end = rest.is_empty()
                    || rest.starts_with(&['\\', ')'])
                    || rest.starts_with(&['\\', '|']);
                out.push_str(if at_end { "(?m:$)" } else { r"\$" });
                at_start = false;
            }
            '.' => {
                out.push('.');
                at_start = false;
            }
            other => {
                out.push_str(&regex::escape(&other.to_string()));
                at_start = false;
            }
        }
    }
    Ok(out)
}

/// 括弧の式 `[...]` を, Rust の書き方にします. `i` は `[` の次の位置.
///
/// # Returns
///
/// Rust の文字クラスと, `]` の次の位置.
fn bracket(chars: &[char], mut i: usize) -> Result<(String, usize), String> {
    let mut out = String::from("[");
    if chars.get(i) == Some(&'^') {
        out.push('^');
        i += 1;
    }
    let mut first = true;
    loop {
        let Some(&c) = chars.get(i) else {
            return Err("Unmatched [ or [^".to_string());
        };
        i += 1;
        match c {
            ']' if !first => {
                out.push(']');
                return Ok((out, i));
            }
            '[' if chars.get(i) == Some(&':') => {
                let close = (i + 1..chars.len().saturating_sub(1))
                    .find(|&j| chars[j] == ':' && chars[j + 1] == ']')
                    .ok_or("Unmatched [ or [^")?;
                let name: String = chars[i + 1..close].iter().collect();
                out.push_str(class_set(&name)?);
                i = close + 2;
            }
            '\\' | '[' | ']' | '&' | '~' | '^' => {
                out.push('\\');
                out.push(c);
            }
            other => out.push(other),
        }
        first = false;
    }
}

/// 文字クラス `[:name:]` の, Rust の文字クラスの中身 (Emacs は Unicode の文字も含める).
fn class_set(name: &str) -> Result<&'static str, String> {
    Ok(match name {
        "alpha" => r"\p{Alphabetic}",
        "alnum" => r"\p{Alphabetic}\p{Nd}",
        "digit" => "0-9",
        "xdigit" => "0-9A-Fa-f",
        "upper" => r"\p{Lu}",
        "lower" => r"\p{Ll}",
        "space" => r" \t\n\r\x0c",
        "blank" => r"\t\p{Zs}",
        "punct" => r"!-/:-@\[-`{-~\p{P}",
        "word" => r"\p{L}\p{N}$%",
        "cntrl" => r"\x00-\x1f\x7f",
        "graph" => r"!-~\p{L}\p{N}\p{P}\p{S}",
        "print" => r" -~\p{L}\p{N}\p{P}\p{S}\p{Zs}",
        "ascii" => r"\x00-\x7f",
        "nonascii" | "multibyte" => r"\x{80}-\x{10ffff}",
        "unibyte" => r"\x00-\x7f",
        _ => return Err(format!("Invalid character class name: {name}")),
    })
}

/// 探す正規表現.
#[derive(Debug, Clone)]
pub struct Pattern {
    /// 作った regex.
    regex: Regex,
}

impl Pattern {
    /// Emacs の正規表現から作ります.
    ///
    /// # Arguments
    ///
    /// * `pattern` - Emacs の正規表現.
    /// * `case_fold` - 大文字と小文字を区別しないか (Emacs の `case-fold-search`).
    ///
    /// # Errors
    ///
    /// 正規表現が誤っているとき.
    pub fn new(pattern: &str, case_fold: bool) -> Result<Self, String> {
        Self::compile(&translate(pattern)?, case_fold)
    }

    /// 文字そのものを探す正規表現を作ります (Emacs の `search-forward` の文字).
    pub fn literal(text: &str, case_fold: bool) -> Self {
        // 文字をエスケープした正規表現は, 必ず作れる
        Self::compile(&regex::escape(text), case_fold).unwrap()
    }

    /// Rust の regex の書き方から作ります.
    fn compile(pattern: &str, case_fold: bool) -> Result<Self, String> {
        let source = if case_fold {
            format!("(?i){pattern}")
        } else {
            pattern.to_string()
        };
        Regex::new(&source)
            .map(|regex| Pattern { regex })
            .map_err(|e| format!("Invalid regexp: {e}"))
    }
}

impl Buffer {
    /// 最後に合った所 (全体と, 括弧ごと. 合わなかった括弧は `None`. Emacs の `match-data`).
    pub fn match_data(&self) -> &[Option<(usize, usize)>] {
        &self.match_data
    }

    /// 位置 `start` (バイト) から合う所を探します. 合った所の終わりが `limit` (バイト) を越えるときは,
    /// 本文を `limit` で切って, 同じ始めで探し直します (Emacs は `limit` を越える合い方をしない).
    ///
    /// # Arguments
    ///
    /// * `anchored` - 真なら, `start` で始まる所だけ.
    ///
    /// # Returns
    ///
    /// 括弧ごとの, 始めと終わり (バイト).
    fn match_from(
        pattern: &Pattern,
        haystack: &str,
        mut start: usize,
        limit: usize,
        anchored: bool,
    ) -> Option<Vec<Option<(usize, usize)>>> {
        loop {
            let caps = pattern.regex.captures_from_pos(haystack, start).ok()??;
            let whole = caps.get(0)?;
            if anchored && whole.start() != start {
                return None;
            }
            if whole.start() > limit {
                return None;
            }
            let mut found = Some(caps);
            if whole.end() > limit {
                found = pattern
                    .regex
                    .captures_from_pos(&haystack[..limit], whole.start())
                    .ok()
                    .flatten()
                    .filter(|c| c.get(0).is_some_and(|m| m.start() == whole.start()));
            }
            if let Some(caps) = found {
                return Some(
                    caps.iter()
                        .map(|m| m.map(|m| (m.start(), m.end())))
                        .collect(),
                );
            }
            if anchored {
                return None;
            }
            // 次の文字から探し直す
            start = whole.start()
                + haystack[whole.start()..]
                    .chars()
                    .next()
                    .map_or(1, char::len_utf8);
            if start > limit {
                return None;
            }
        }
    }

    /// バイトの位置の合った所を, 文字の位置にして覚えます.
    fn remember(&mut self, found: &[Option<(usize, usize)>]) {
        // Emacs の `match-data` は, 最後に合った括弧より後ろの, 合わなかった括弧を持たない
        let used = found.iter().rposition(Option::is_some).map_or(0, |i| i + 1);
        self.match_data = found[..used]
            .iter()
            .map(|m| m.map(|(a, b)| (self.text.byte_to_char(a), self.text.byte_to_char(b))))
            .collect();
    }

    /// カーソルの所から後ろへ探して, 合った所の終わりへカーソルを動かします (Emacs の `re-search-forward`).
    ///
    /// # Arguments
    ///
    /// * `pattern` - 探す正規表現.
    /// * `bound` - 合った所の終わりの限り (省くと, バッファの終わり).
    /// * `count` - 何回目に合った所まで進むか (1 以上).
    ///
    /// # Returns
    ///
    /// 動いた先の位置. 合わなければ `None` (カーソルも合った所の記録も, 変えない).
    pub fn re_search_forward(
        &mut self,
        pattern: &Pattern,
        bound: Option<usize>,
        count: usize,
    ) -> Option<usize> {
        let haystack = self.text.to_string();
        let limit = self
            .text
            .char_to_byte(bound.unwrap_or(self.len()).min(self.len()));
        let mut from = self.text.char_to_byte(self.point);
        let mut last = None;
        for _ in 0..count.max(1) {
            let found = Self::match_from(pattern, &haystack, from, limit, false)?;
            from = found[0]?.1;
            // 途中までしか合わなくても, 合った所は覚える (Emacs と同じ)
            self.remember(&found);
            last = Some(from);
        }
        self.point = self.text.byte_to_char(last?);
        Some(self.point)
    }

    /// カーソルの所から前へ探して, 合った所の始めへカーソルを動かします (Emacs の `re-search-backward`).
    /// 合った所の終わりは, 探し始めたカーソルの位置を越えない.
    ///
    /// # Arguments
    ///
    /// * `pattern` - 探す正規表現.
    /// * `bound` - 合った所の始めの限り (省くと, バッファの始め).
    /// * `count` - 何回目に合った所まで進むか (1 以上).
    ///
    /// # Returns
    ///
    /// 動いた先の位置. 合わなければ `None` (カーソルも合った所の記録も, 変えない).
    pub fn re_search_backward(
        &mut self,
        pattern: &Pattern,
        bound: Option<usize>,
        count: usize,
    ) -> Option<usize> {
        let haystack = self.text.to_string();
        let lower = bound.unwrap_or(0).min(self.len());
        let mut limit = self.point;
        let mut found_at = None;
        for _ in 0..count.max(1) {
            let (start, found) = (lower..=limit).rev().find_map(|pos| {
                let byte = self.text.char_to_byte(pos);
                let limit_byte = self.text.char_to_byte(limit);
                Self::match_from(pattern, &haystack, byte, limit_byte, true).map(|f| (pos, f))
            })?;
            limit = start;
            self.remember(&found);
            found_at = Some(start);
        }
        self.point = found_at?;
        Some(self.point)
    }

    /// カーソルの所から合うか (Emacs の `looking-at`). 合えば, 合った所を覚える.
    pub fn looking_at(&mut self, pattern: &Pattern) -> bool {
        let haystack = self.text.to_string();
        let from = self.text.char_to_byte(self.point);
        match Self::match_from(pattern, &haystack, from, haystack.len(), true) {
            Some(found) => {
                self.remember(&found);
                true
            }
            None => false,
        }
    }
}
