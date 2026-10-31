//! Scheme のコードの文字列を, 値 ([`Value`]) の並びに読み込むパーサ.

use crate::scheme::evaluator::MAX_DEPTH;
use crate::scheme::value::Value;

/// 記号に使える, 英数字のほかの文字 (`.` だけの記号は, 引数のリストの `(a . rest)` に使う).
const SYMBOL_CHARS: &str = "+-*/<>=!?_.:&%$~^";

/// Scheme のコードを読み込むパーサ. 1 文字ずつ先頭から読み進めます.
pub struct Parser {
    /// 読み込むコード (1 文字ずつ).
    input: Vec<char>,
    /// 次に読む文字の位置 (`input` の添字).
    pos: usize,
    /// 今読んでいる式の, 入れ子の深さ ([`MAX_DEPTH`] まで. 深すぎる入れ子で, スタックがあふれないように).
    depth: usize,
}

impl Parser {
    /// コードを読み込むパーサを作ります.
    ///
    /// # Arguments
    ///
    /// * `input` - 読み込むコード.
    ///
    /// # Returns
    ///
    /// 先頭から読むパーサ.
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
            depth: 0,
        }
    }

    /// コードの全ての式を読み込みます. 空白とコメント (`;` から行末まで) は読み飛ばします.
    ///
    /// # Returns
    ///
    /// 読み込んだ式 (先頭から順). 書き方が誤っている場合は, その内容の [`Err`].
    pub fn parse(&mut self) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.pos >= self.input.len() {
                break;
            }
            values.push(self.parse_value()?);
        }
        Ok(values)
    }

    /// 始めの式だけを読み込みます (後ろは読まない. Emacs の, 文字列を渡した `read`).
    ///
    /// # Returns
    ///
    /// 読み込んだ式. 式が無ければ `None`. 書き方が誤っている場合は, その内容の [`Err`].
    pub fn parse_first(&mut self) -> Result<Option<Value>, String> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Ok(None);
        }
        self.parse_value().map(Some)
    }

    /// 式を 1 つ読み込みます. `'x` は `(quote x)` にします. 入れ子は, [`MAX_DEPTH`] までです.
    ///
    /// # Returns
    ///
    /// 読み込んだ式. 入力が終わっているか, 式を始められない文字か, 入れ子が深すぎる場合は [`Err`].
    fn parse_value(&mut self) -> Result<Value, String> {
        if self.depth >= MAX_DEPTH {
            return Err("Too deeply nested".to_string());
        }
        self.depth += 1;
        let value = self.parse_nested();
        self.depth -= 1;
        value
    }

    /// 式を 1 つ読み込みます (入れ子の深さは, [`Parser::parse_value`] が数える).
    ///
    /// # Returns
    ///
    /// 読み込んだ式. 入力が終わっているか, 式を始められない文字の場合は [`Err`].
    fn parse_nested(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        let c = self
            .current()
            .ok_or_else(|| "Unexpected end of input".to_string())?;
        match c {
            '(' => self.parse_list(),
            '\'' => {
                self.advance('\'');
                let quoted = self.parse_value()?;
                Ok(Value::List(
                    [Value::Symbol("quote".to_string()), quoted].into(),
                ))
            }
            '"' => self.parse_string(),
            '#' => self.parse_boolean(),
            c if is_symbol_char(c) => Ok(self.parse_atom()),
            c => Err(format!("Unexpected character: {}", c)),
        }
    }

    /// `(` から対応する `)` までのリストを読み込みます.
    ///
    /// # Returns
    ///
    /// リスト. `)` で閉じていない場合は [`Err`].
    fn parse_list(&mut self) -> Result<Value, String> {
        self.advance('(');
        let mut items = Vec::new();
        self.skip_whitespace();

        while self.current() != Some(')') {
            if self.current().is_none() {
                return Err("Unclosed list".to_string());
            }
            items.push(self.parse_value()?);
            self.skip_whitespace();
        }

        self.advance(')');
        Ok(Value::List(items.into()))
    }

    /// `"` で囲んだ文字列を読み込みます. エスケープは `\n`, `\t`, `\"`, `\\` が使えます.
    ///
    /// # Returns
    ///
    /// 文字列. 閉じていないか, 使えないエスケープがある場合は [`Err`].
    fn parse_string(&mut self) -> Result<Value, String> {
        self.advance('"');
        let mut result = String::new();

        while self.current() != Some('"') {
            match self.current() {
                Some('\\') => {
                    self.pos += 1;
                    match self.current() {
                        Some('n') => result.push('\n'),
                        Some('t') => result.push('\t'),
                        Some('r') => result.push('\r'),
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        _ => return Err("Invalid escape sequence".to_string()),
                    }
                    self.pos += 1;
                }
                Some(c) => {
                    result.push(c);
                    self.pos += 1;
                }
                None => return Err("Unclosed string".to_string()),
            }
        }

        self.advance('"');
        Ok(Value::String(result))
    }

    /// 真偽値 (`#t` / `#f`. R7RS の `#true` / `#false` も) を読み込みます.
    ///
    /// # Returns
    ///
    /// 真偽値. `#` の後が `t`, `f`, `true`, `false` でない場合は [`Err`].
    fn parse_boolean(&mut self) -> Result<Value, String> {
        self.advance('#');
        match self.token().as_str() {
            "t" | "true" => Ok(Value::Boolean(true)),
            "f" | "false" => Ok(Value::Boolean(false)),
            _ => Err("Invalid boolean".to_string()),
        }
    }

    /// 数か記号を読み込みます. 記号に使える文字の並びを読み, 数の形 (数字, または符号か `.` の後に数字で
    /// 始まる) で, 数として読めれば数, そうでなければ記号にします (Emacs と同じく, `1+` や `1.2.3` は記号).
    ///
    /// # Returns
    ///
    /// 数か記号.
    fn parse_atom(&mut self) -> Value {
        let token = self.token();
        let digits = token.strip_prefix(['+', '-']).unwrap_or(&token);
        let digits = digits.strip_prefix('.').unwrap_or(digits);
        if digits.starts_with(|c: char| c.is_ascii_digit()) {
            if let Ok(n) = token.parse::<f64>() {
                return Value::Number(n);
            }
        }
        Value::Symbol(token)
    }

    /// 記号に使える文字の並びを読みます.
    ///
    /// # Returns
    ///
    /// 読んだ文字 (無ければ空).
    fn token(&mut self) -> String {
        let start = self.pos;
        while self.current().is_some_and(is_symbol_char) {
            self.pos += 1;
        }
        self.input[start..self.pos].iter().collect()
    }

    /// 次に読む文字を返します.
    ///
    /// # Returns
    ///
    /// 次に読む文字. 入力が終わっている場合は [`None`].
    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    /// 空白とコメント (`;` から行末まで) を読み飛ばします.
    fn skip_whitespace(&mut self) {
        while self
            .current()
            .is_some_and(|c| c.is_whitespace() || c == ';')
        {
            if self.current() == Some(';') {
                while self.current() != Some('\n') && self.current().is_some() {
                    self.pos += 1;
                }
            } else {
                self.pos += 1;
            }
        }
    }

    /// 今の文字 (呼ぶ側が確かめた `expected`) を読み飛ばす.
    ///
    /// # Arguments
    ///
    /// * `expected` - 今の文字 (デバッグ版でだけ確かめる).
    fn advance(&mut self, expected: char) {
        debug_assert_eq!(self.current(), Some(expected));
        self.pos += 1;
    }
}

/// 記号 (と数) に使える文字かを返します. 英数字 (Unicode の文字と数字も) と, [`SYMBOL_CHARS`] です.
///
/// # Arguments
///
/// * `c` - 文字.
///
/// # Returns
///
/// 使えれば `true`.
fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(c)
}
