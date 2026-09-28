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
            Some(c) if c.is_numeric() || (c == '-' && self.peek().map_or(false, |ch| ch.is_numeric())) => {
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

        while self.current().map_or(false, |c| c.is_numeric() || c == '.') {
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
        while self.current().map_or(false, |c| {
            c.is_alphanumeric() || "+-*/<>=!?_-".contains(c)
        }) {
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
        while self.current().map_or(false, |c| c.is_whitespace() || c == ';') {
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
