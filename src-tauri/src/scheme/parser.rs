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
                Ok(Value::List(vec![
                    Value::Symbol("quote".to_string()),
                    quoted,
                ]))
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
        Ok(Value::List(items))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_one(input: &str) -> Value {
        Parser::new(input)
            .parse()
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
    }

    #[test]
    fn parses_integers() {
        assert_eq!(parse_one("42"), Value::Number(42.0));
    }

    #[test]
    fn parses_negative_numbers() {
        assert_eq!(parse_one("-3.5"), Value::Number(-3.5));
    }

    #[test]
    fn parses_symbols() {
        assert_eq!(parse_one("foo-bar?"), Value::Symbol("foo-bar?".to_string()));
    }

    #[test]
    fn parses_booleans() {
        assert_eq!(parse_one("#t"), Value::Boolean(true));
        assert_eq!(parse_one("#f"), Value::Boolean(false));
    }

    #[test]
    fn parses_strings_with_escapes() {
        assert_eq!(
            parse_one(r#""hello\nworld""#),
            Value::String("hello\nworld".to_string())
        );
    }

    #[test]
    fn parses_nested_lists() {
        let value = parse_one("(+ 1 (* 2 3))");
        match value {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], Value::Symbol("+".to_string()));
                assert_eq!(items[1], Value::Number(1.0));
                assert_eq!(
                    items[2],
                    Value::List(vec![
                        Value::Symbol("*".to_string()),
                        Value::Number(2.0),
                        Value::Number(3.0),
                    ])
                );
            }
            _ => panic!("expected a list"),
        }
    }

    #[test]
    fn parses_empty_list() {
        assert_eq!(parse_one("()"), Value::List(vec![]));
    }

    #[test]
    fn skips_comments() {
        let values = Parser::new("; a comment\n(+ 1 1)").parse().unwrap();
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn parses_multiple_top_level_forms() {
        let values = Parser::new("1 2 3").parse().unwrap();
        assert_eq!(
            values,
            vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]
        );
    }

    #[test]
    fn parses_quote_and_dots() {
        assert_eq!(
            parse_one("'(a . b)"),
            Value::List(vec![
                Value::Symbol("quote".to_string()),
                Value::List(vec![
                    Value::Symbol("a".to_string()),
                    Value::Symbol(".".to_string()),
                    Value::Symbol("b".to_string()),
                ]),
            ])
        );
        assert_eq!(parse_one(".5"), Value::Number(0.5));
        assert_eq!(parse_one("a.b:c"), Value::Symbol("a.b:c".to_string()));
    }

    #[test]
    fn errors_on_unclosed_list() {
        assert!(Parser::new("(+ 1 2").parse().is_err());
    }

    #[test]
    fn errors_on_invalid_boolean() {
        assert!(Parser::new("#x").parse().is_err());
        assert!(Parser::new("#tx").parse().is_err());
    }

    #[test]
    fn parses_long_booleans() {
        assert_eq!(parse_one("#true"), Value::Boolean(true));
        assert_eq!(parse_one("#false"), Value::Boolean(false));
    }

    #[test]
    fn parses_atoms_like_emacs() {
        // 数として読めない, 数字で始まる並びは, 記号
        assert_eq!(parse_one("1+"), Value::Symbol("1+".to_string()));
        assert_eq!(parse_one("1.2.3"), Value::Symbol("1.2.3".to_string()));
        assert_eq!(parse_one("+5"), Value::Number(5.0));
        assert_eq!(parse_one("-.5"), Value::Number(-0.5));
        assert_eq!(parse_one("1e3"), Value::Number(1000.0));
        // 数の形でないものは, 数として読めても記号
        assert_eq!(parse_one("inf"), Value::Symbol("inf".to_string()));
        assert_eq!(parse_one("-"), Value::Symbol("-".to_string()));
        // Unicode の数字の文字は, 記号
        assert_eq!(parse_one("①"), Value::Symbol("①".to_string()));
        assert_eq!(parse_one("x①"), Value::Symbol("x①".to_string()));
    }

    #[test]
    fn errors_on_too_deep_nesting() {
        // 上限まで潜るので, 評価と同じく, 大きなスタックで読む
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(|| {
                let deep = format!("{}{}", "(".repeat(MAX_DEPTH + 1), ")".repeat(MAX_DEPTH + 1));
                assert_eq!(Parser::new(&deep).parse().unwrap_err(), "Too deeply nested");
                let quoted = format!("{}x", "'".repeat(MAX_DEPTH + 1));
                assert_eq!(
                    Parser::new(&quoted).parse().unwrap_err(),
                    "Too deeply nested"
                );
                // 上限までは, 読める
                let ok = format!("{}{}", "(".repeat(MAX_DEPTH), ")".repeat(MAX_DEPTH));
                assert!(Parser::new(&ok).parse().is_ok());
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
