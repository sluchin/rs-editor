//! Scheme の値 ([`Value`]) と, `lambda` で作った手続き ([`Lambda`]).

use std::fmt;
use std::sync::Arc;

use crate::scheme::env::Env;

/// Scheme の値.
#[derive(Debug, Clone)]
pub enum Value {
    /// 値がないこと (`insert` などの結果).
    Nil,
    /// 真偽値 (`#t` / `#f`).
    Boolean(bool),
    /// 数 (整数も小数も, 浮動小数点数で持つ).
    Number(f64),
    /// 記号 (変数の名前など).
    Symbol(String),
    /// 文字列.
    String(String),
    /// リスト (ドット対は扱わない).
    List(Vec<Value>),
    /// 組み込みの手続き (名前で引く).
    Function(String),
    /// `lambda` で作った手続き.
    Lambda(Arc<Lambda>),
}

/// `lambda` で作った手続き.
pub struct Lambda {
    /// `define` で付けた名前.
    pub name: Option<String>,
    /// 引数の名前.
    pub params: Vec<String>,
    /// 残りの引数を, リストで受ける名前 (`(a . rest)` の `rest`).
    pub rest: Option<String>,
    /// 本体.
    pub body: Vec<Value>,
    /// 作ったときの環境.
    pub env: Env,
}

impl fmt::Debug for Lambda {
    // 環境は, 自分を含むことがあるので, 出さない
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lambda({:?})", self.name)
    }
}

impl Value {
    /// 文字列は引用符を付けずに, ほかは [`Display`](fmt::Display) で表します (`display` と `format` の `%s`).
    ///
    /// # Returns
    ///
    /// 表した文字列.
    pub fn to_display(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            v => v.to_string(),
        }
    }

    /// 真かを返します (`#f` だけが偽).
    ///
    /// # Returns
    ///
    /// 真なら `true`.
    pub fn is_true(&self) -> bool {
        !matches!(self, Value::Boolean(false))
    }
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
            Value::String(s) => write!(f, "{:?}", s),
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
            Value::Lambda(lambda) => match &lambda.name {
                Some(name) => write!(f, "#<procedure:{}>", name),
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
            (Value::Function(a), Value::Function(b)) => a == b,
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
        assert_eq!(
            Value::String("a\"b\n".to_string()).to_string(),
            "\"a\\\"b\\n\""
        );
        assert_eq!(Value::String("hi".to_string()).to_display(), "hi");
        assert_eq!(Value::Number(1.0).to_display(), "1");
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
    fn displays_lambdas_with_and_without_names() {
        let lambda = |name: Option<&str>| {
            Value::Lambda(Arc::new(Lambda {
                name: name.map(str::to_string),
                params: vec![],
                rest: None,
                body: vec![],
                env: Env::new(),
            }))
        };
        assert_eq!(lambda(Some("f")).to_string(), "#<procedure:f>");
        assert_eq!(lambda(None).to_string(), "#<procedure>");
        assert_eq!(
            format!("{:?}", lambda(Some("f"))),
            "Lambda(Lambda(Some(\"f\")))"
        );
    }

    #[test]
    fn equality_holds_across_variants() {
        assert_eq!(Value::Number(1.0), Value::Number(1.0));
        assert_ne!(Value::Number(1.0), Value::Boolean(true));
        assert_eq!(
            Value::List(vec![Value::Number(1.0)]),
            Value::List(vec![Value::Number(1.0)])
        );
        assert_eq!(
            Value::Function("car".to_string()),
            Value::Function("car".to_string())
        );
    }

    #[test]
    fn only_false_is_false() {
        assert!(!Value::Boolean(false).is_true());
        assert!(Value::Nil.is_true());
        assert!(Value::List(vec![]).is_true());
    }
}
