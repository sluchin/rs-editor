//! Scheme のコードの文字列を, 値 ([`Value`]) の並びに読み込むパーサ.

use crate::scheme::value::Value;

/// 記号に使える, 英数字のほかの文字 (`.` だけの記号は, 引数のリストの `(a . rest)` に使う).
const SYMBOL_CHARS: &str = "+-*/<>=!?_.:&%$~^";

/// Scheme のコードを読み込むパーサ. 1 文字ずつ先頭から読み進めます.
pub struct Parser {
    /// 読み込むコード (1 文字ずつ).
    input: Vec<char>,
    /// 次に読む文字の位置 (`input` の添字).
    pos: usize,
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

    /// 式を 1 つ読み込みます. `'x` は `(quote x)` にします.
    ///
    /// # Returns
    ///
    /// 読み込んだ式. 入力が終わっているか, 式を始められない文字の場合は [`Err`].
    fn parse_value(&mut self) -> Result<Value, String> {
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
            c if c.is_numeric()
                || ("-.".contains(c) && self.peek().is_some_and(|ch| ch.is_numeric())) =>
            {
                self.parse_number()
            }
            c if c.is_alphabetic() || SYMBOL_CHARS.contains(c) => self.parse_symbol(),
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

    /// 真偽値 (`#t` / `#f`) を読み込みます.
    ///
    /// # Returns
    ///
    /// 真偽値. `#` の後が `t` でも `f` でもない場合は [`Err`].
    fn parse_boolean(&mut self) -> Result<Value, String> {
        self.advance('#');
        match self.current() {
            Some('t') => {
                self.pos += 1;
                Ok(Value::Boolean(true))
            }
            Some('f') => {
                self.pos += 1;
                Ok(Value::Boolean(false))
            }
            _ => Err("Invalid boolean".to_string()),
        }
    }

    /// 数 (`-` で始まる負の数, 小数を含む) を読み込みます.
    ///
    /// # Returns
    ///
    /// 数. 数として読めない場合 (`1.2.3` など) は [`Err`].
    fn parse_number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        if self.current() == Some('-') {
            self.pos += 1;
        }

        while self.current().is_some_and(|c| c.is_numeric() || c == '.') {
            self.pos += 1;
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        match num_str.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => Err(format!("Invalid number: {}", num_str)),
        }
    }

    /// 記号 (英数字と [`SYMBOL_CHARS`] の並び) を読み込みます.
    ///
    /// # Returns
    ///
    /// 記号 (失敗しない).
    fn parse_symbol(&mut self) -> Result<Value, String> {
        let start = self.pos;
        while self
            .current()
            .is_some_and(|c| c.is_alphanumeric() || SYMBOL_CHARS.contains(c))
        {
            self.pos += 1;
        }

        let symbol: String = self.input[start..self.pos].iter().collect();
        Ok(Value::Symbol(symbol))
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

    /// 次に読む文字の, 1 つ後の文字を返します (読み進めない).
    ///
    /// # Returns
    ///
    /// 1 つ後の文字. 無い場合は [`None`].
    fn peek(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
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
    }
}
