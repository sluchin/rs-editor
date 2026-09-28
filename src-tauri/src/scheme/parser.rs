use crate::scheme::value::Value;

pub struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
        }
    }

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

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.current() {
            Some('(') => self.parse_list(),
            Some('"') => self.parse_string(),
            Some('#') => self.parse_boolean(),
            Some(c)
                if c.is_numeric()
                    || (c == '-' && self.peek().is_some_and(|ch| ch.is_numeric())) =>
            {
                self.parse_number()
            }
            Some(c) if c.is_alphabetic() || "+-*/<>=!?".contains(c) => self.parse_symbol(),
            Some(c) => Err(format!("Unexpected character: {}", c)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_list(&mut self) -> Result<Value, String> {
        self.consume('(')?;
        let mut items = Vec::new();
        self.skip_whitespace();

        while self.current() != Some(')') {
            if self.current().is_none() {
                return Err("Unclosed list".to_string());
            }
            items.push(self.parse_value()?);
            self.skip_whitespace();
        }

        self.consume(')')?;
        Ok(Value::List(items))
    }

    fn parse_string(&mut self) -> Result<Value, String> {
        self.consume('"')?;
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

        self.consume('"')?;
        Ok(Value::String(result))
    }

    fn parse_boolean(&mut self) -> Result<Value, String> {
        self.consume('#')?;
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

    fn parse_symbol(&mut self) -> Result<Value, String> {
        let start = self.pos;
        while self
            .current()
            .is_some_and(|c| c.is_alphanumeric() || "+-*/<>=!?_-".contains(c))
        {
            self.pos += 1;
        }

        let symbol: String = self.input[start..self.pos].iter().collect();
        Ok(Value::Symbol(symbol))
    }

    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

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

    fn consume(&mut self, expected: char) -> Result<(), String> {
        if self.current() == Some(expected) {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!("Expected '{}'", expected))
        }
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
    fn errors_on_unclosed_list() {
        assert!(Parser::new("(+ 1 2").parse().is_err());
    }

    #[test]
    fn errors_on_invalid_boolean() {
        assert!(Parser::new("#x").parse().is_err());
    }
}
