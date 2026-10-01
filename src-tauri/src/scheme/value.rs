use crate::scheme::evaluator::Interp;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// 環境フレーム. 親フレームへの参照を持つ.
pub struct Frame {
    pub vars: HashMap<String, Value>,
    pub parent: Option<Env>,
}

pub type Env = Arc<Mutex<Frame>>;

pub fn new_env(parent: Option<Env>) -> Env {
    Arc::new(Mutex::new(Frame {
        vars: HashMap::new(),
        parent,
    }))
}

pub type BuiltinFn = fn(&mut Interp, Vec<Value>) -> Result<Value, String>;

/// ユーザ定義の手続き (クロージャ).
pub struct Lambda {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub rest: Option<String>,
    pub body: Vec<Value>,
    pub env: Env,
}

#[derive(Clone)]
pub enum Value {
    Nil,
    /// 値を返さない式 (define など). REPL には表示されない.
    Void,
    Boolean(bool),
    Number(f64),
    Symbol(String),
    String(String),
    List(Vec<Value>),
    Function(String, BuiltinFn),
    Lambda(Arc<Lambda>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    pub fn is_true(&self) -> bool {
        !matches!(self, Value::Boolean(false))
    }

    /// `display` 用の文字列表現 (文字列に引用符を付けない).
    pub fn display_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::List(items) => {
                let parts: Vec<String> = items.iter().map(|v| v.display_string()).collect();
                format!("({})", parts.join(" "))
            }
            v => v.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Void => Ok(()),
            Value::Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Symbol(s) => write!(f, "{}", s),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Function(name, _) => write!(f, "#<procedure:{}>", name),
            Value::Lambda(l) => match &l.name {
                Some(n) => write!(f, "#<procedure:{}>", n),
                None => write!(f, "#<procedure>"),
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Void, Value::Void) => true,
            (Value::Function(a, _), Value::Function(b, _)) => a == b,
            (Value::Lambda(a), Value::Lambda(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_integer_numbers_without_decimal_point() {
        assert_eq!(Value::Number(42.0).to_string(), "42");
    }

    #[test]
    fn displays_fractional_numbers() {
        assert_eq!(Value::Number(3.5).to_string(), "3.5");
    }

    #[test]
    fn displays_booleans() {
        assert_eq!(Value::Boolean(true).to_string(), "#t");
        assert_eq!(Value::Boolean(false).to_string(), "#f");
    }

    #[test]
    fn displays_strings_with_quotes() {
        assert_eq!(Value::String("hi".to_string()).to_string(), "\"hi\"");
    }

    #[test]
    fn displays_lists_with_spaces() {
        let list = Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        assert_eq!(list.to_string(), "(1 2 3)");
    }

    #[test]
    fn displays_nested_lists() {
        let list = Value::List(vec![
            Value::Symbol("a".to_string()),
            Value::List(vec![Value::Number(1.0), Value::Number(2.0)]),
        ]);
        assert_eq!(list.to_string(), "(a (1 2))");
    }

    #[test]
    fn equality_holds_across_variants() {
        assert_eq!(Value::Number(1.0), Value::Number(1.0));
        assert_ne!(Value::Number(1.0), Value::Boolean(true));
        assert_eq!(
            Value::List(vec![Value::Number(1.0)]),
            Value::List(vec![Value::Number(1.0)])
        );
    }
}
