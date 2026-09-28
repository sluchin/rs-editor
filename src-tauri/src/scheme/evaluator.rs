use crate::scheme::parser::Parser;
use crate::scheme::value::Value;
use std::sync::Mutex;

pub struct Evaluator {
    vars: Mutex<std::collections::HashMap<String, Value>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            vars: Mutex::new(std::collections::HashMap::new()),
        }
    }

    pub fn eval(&self, code: &str) -> Result<String, String> {
        let mut parser = Parser::new(code);
        let values = parser.parse()?;

        if values.is_empty() {
            return Ok(String::new());
        }

        let mut results = Vec::new();
        for value in values {
            let result = self.eval_value(&value)?;
            results.push(result.to_string());
        }

        Ok(results.join("\n"))
    }

    fn eval_value(&self, value: &Value) -> Result<Value, String> {
        match value {
            Value::List(items) if !items.is_empty() => {
                match &items[0] {
                    Value::Symbol(op) => self.eval_builtin(op, &items[1..]),
                    _ => Err("First element must be a symbol".to_string()),
                }
            }
            v => Ok(v.clone()),
        }
    }

    fn eval_builtin(&self, op: &str, args: &[Value]) -> Result<Value, String> {
        match op {
            "+" => {
                let mut sum = 0.0;
                for arg in args {
                    let val = self.eval_value(arg)?;
                    match val {
                        Value::Number(n) => sum += n,
                        _ => return Err("+ expects numbers".to_string()),
                    }
                }
                Ok(Value::Number(sum))
            }
            "-" => {
                if args.is_empty() {
                    return Err("- expects at least one argument".to_string());
                }
                let first = self.eval_value(&args[0])?;
                let mut result = match first {
                    Value::Number(n) => n,
                    _ => return Err("- expects numbers".to_string()),
                };

                if args.len() == 1 {
                    return Ok(Value::Number(-result));
                }

                for arg in &args[1..] {
                    let val = self.eval_value(arg)?;
                    match val {
                        Value::Number(n) => result -= n,
                        _ => return Err("- expects numbers".to_string()),
                    }
                }
                Ok(Value::Number(result))
            }
            "*" => {
                let mut product = 1.0;
                for arg in args {
                    let val = self.eval_value(arg)?;
                    match val {
                        Value::Number(n) => product *= n,
                        _ => return Err("* expects numbers".to_string()),
                    }
                }
                Ok(Value::Number(product))
            }
            "/" => {
                if args.len() < 2 {
                    return Err("/ expects at least two arguments".to_string());
                }
                let first = self.eval_value(&args[0])?;
                let mut result = match first {
                    Value::Number(n) => n,
                    _ => return Err("/ expects numbers".to_string()),
                };

                for arg in &args[1..] {
                    let val = self.eval_value(arg)?;
                    match val {
                        Value::Number(n) if n != 0.0 => result /= n,
                        _ => return Err("/ expects non-zero numbers".to_string()),
                    }
                }
                Ok(Value::Number(result))
            }
            "quote" => {
                if args.len() != 1 {
                    return Err("quote expects 1 argument".to_string());
                }
                Ok(args[0].clone())
            }
            "list" => {
                let mut list_items = Vec::new();
                for arg in args {
                    list_items.push(self.eval_value(arg)?);
                }
                Ok(Value::List(list_items))
            }
            _ => Err(format!("Unknown operator: {}", op)),
        }
    }
}
