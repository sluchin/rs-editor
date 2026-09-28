use std::fmt;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    Symbol(String),
    String(String),
    List(Vec<Value>),
    Function(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Number(n) => {
                if n.fract() == 0.0 {
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
            Value::Function(name) => write!(f, "#<procedure:{}>", name),
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
