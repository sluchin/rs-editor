//! 組み込みの手続き (数, 真偽, リスト, 文字列, エディタの操作).
//!
//! エディタの位置は, Emacs と同じく 1 から数えます (`point-min` は 1).

use crate::scheme::evaluator::{KeyBinding, Machine};
use crate::scheme::keys;
use crate::scheme::value::Value;

/// 組み込みの手続きの名前.
pub const NAMES: &[&str] = &[
    // 数
    "+",
    "-",
    "*",
    "/",
    "=",
    "<",
    ">",
    "<=",
    ">=",
    "abs",
    "min",
    "max",
    "quotient",
    "remainder",
    "modulo",
    "number?",
    "integer?",
    "zero?",
    // 真偽と比較
    "not",
    "boolean?",
    "eq?",
    "eqv?",
    "equal?",
    "symbol?",
    "string?",
    "procedure?",
    // リスト
    "car",
    "cdr",
    "cons",
    "list",
    "length",
    "append",
    "reverse",
    "list-ref",
    "null?",
    "pair?",
    "list?",
    "map",
    "for-each",
    "filter",
    "apply",
    "assoc",
    "member",
    // 文字列
    "string-length",
    "string-append",
    "substring",
    "string=?",
    "string<?",
    "string-upcase",
    "string-downcase",
    "number->string",
    "string->number",
    "symbol->string",
    "string->symbol",
    "format",
    // エディタ
    "point",
    "point-min",
    "point-max",
    "goto-char",
    "insert",
    "delete-region",
    "buffer-string",
    "buffer-substring",
    "buffer-name",
    "message",
    "call-interactively",
    "command-execute",
    "global-set-key",
];

/// 組み込みの手続きを呼びます.
///
/// # Arguments
///
/// * `machine` - 評価 (手続きを受け取る手続きと, エディタの操作に使う).
/// * `name` - 手続きの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub(crate) fn call(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "+" => Ok(Value::Number(numbers(name, &args)?.iter().sum())),
        "*" => Ok(Value::Number(numbers(name, &args)?.iter().product())),
        "-" => {
            let nums = numbers(name, &args)?;
            match nums.split_first() {
                None => Err("- expects at least one argument".to_string()),
                Some((first, [])) => Ok(Value::Number(-first)),
                Some((first, rest)) => Ok(Value::Number(rest.iter().fold(*first, |a, b| a - b))),
            }
        }
        "/" => {
            if args.len() < 2 {
                return Err("/ expects at least two arguments".to_string());
            }
            let nums = numbers(name, &args)?;
            if nums[1..].contains(&0.0) {
                return Err("/ expects non-zero numbers".to_string());
            }
            Ok(Value::Number(nums[1..].iter().fold(nums[0], |a, b| a / b)))
        }
        "=" | "<" | ">" | "<=" | ">=" => {
            let nums = numbers(name, &args)?;
            if nums.is_empty() {
                return Err(format!("{} expects at least one argument", name));
            }
            let holds = |a: f64, b: f64| match name {
                "=" => a == b,
                "<" => a < b,
                ">" => a > b,
                "<=" => a <= b,
                _ => a >= b,
            };
            Ok(Value::Boolean(nums.windows(2).all(|w| holds(w[0], w[1]))))
        }
        "abs" => {
            let [n] = fixed::<1>(name, args)?;
            Ok(Value::Number(number(name, &n)?.abs()))
        }
        "min" | "max" => {
            let nums = numbers(name, &args)?;
            let pick = if name == "min" { f64::min } else { f64::max };
            match nums.split_first() {
                None => Err(format!("{} expects at least one argument", name)),
                Some((first, rest)) => Ok(Value::Number(rest.iter().copied().fold(*first, pick))),
            }
        }
        "quotient" | "remainder" | "modulo" => {
            let [a, b] = fixed::<2>(name, args)?;
            let (a, b) = (number(name, &a)?, number(name, &b)?);
            if b == 0.0 {
                return Err(format!("{}: division by zero", name));
            }
            Ok(Value::Number(match name {
                "quotient" => (a / b).trunc(),
                "remainder" => a % b,
                _ => a - b * (a / b).floor(),
            }))
        }
        "number?" | "integer?" | "zero?" => {
            let [v] = fixed::<1>(name, args)?;
            Ok(Value::Boolean(match (name, &v) {
                ("number?", v) => matches!(v, Value::Number(_)),
                ("integer?", v) => matches!(v, Value::Number(n) if n.fract() == 0.0),
                (_, v) => number(name, v)? == 0.0,
            }))
        }
        "not" | "boolean?" | "symbol?" | "string?" | "procedure?" | "null?" | "pair?" | "list?" => {
            let [v] = fixed::<1>(name, args)?;
            Ok(Value::Boolean(match name {
                "not" => !v.is_true(),
                "boolean?" => matches!(v, Value::Boolean(_)),
                "symbol?" => matches!(v, Value::Symbol(_)),
                "string?" => matches!(v, Value::String(_)),
                "procedure?" => matches!(v, Value::Function(_) | Value::Lambda(_)),
                "null?" => matches!(&v, Value::List(items) if items.is_empty()),
                "pair?" => matches!(&v, Value::List(items) if !items.is_empty()),
                _ => matches!(v, Value::List(_)),
            }))
        }
        "eq?" | "eqv?" | "equal?" => {
            let [a, b] = fixed::<2>(name, args)?;
            Ok(Value::Boolean(a == b))
        }
        "car" | "cdr" => {
            let [v] = fixed::<1>(name, args)?;
            match v {
                Value::List(mut items) if !items.is_empty() => Ok(if name == "car" {
                    items.swap_remove(0)
                } else {
                    Value::List(items.split_off(1))
                }),
                _ => Err(format!("{} expects a non-empty list", name)),
            }
        }
        "cons" => {
            let [head, tail] = fixed::<2>(name, args)?;
            match tail {
                Value::List(mut items) => {
                    items.insert(0, head);
                    Ok(Value::List(items))
                }
                _ => Err("cons expects a list as the second argument".to_string()),
            }
        }
        "list" => Ok(Value::List(args)),
        "length" => {
            let [v] = fixed::<1>(name, args)?;
            Ok(Value::Number(list(name, &v)?.len() as f64))
        }
        "append" => {
            let mut result = Vec::new();
            for v in &args {
                result.extend(list(name, v)?.iter().cloned());
            }
            Ok(Value::List(result))
        }
        "reverse" => {
            let [v] = fixed::<1>(name, args)?;
            Ok(Value::List(list(name, &v)?.iter().rev().cloned().collect()))
        }
        "list-ref" => {
            let [v, k] = fixed::<2>(name, args)?;
            let index = index(name, &k)?;
            list(name, &v)?
                .get(index)
                .cloned()
                .ok_or_else(|| format!("{}: index out of range", name))
        }
        "map" | "for-each" => {
            let Some((procedure, lists)) = args.split_first() else {
                return Err(format!("{} expects a procedure and lists", name));
            };
            let lists = lists
                .iter()
                .map(|v| list(name, v))
                .collect::<Result<Vec<_>, _>>()?;
            if lists.is_empty() {
                return Err(format!("{} expects a procedure and lists", name));
            }
            let count = lists.iter().map(|l| l.len()).min().unwrap_or(0);
            let mut results = Vec::new();
            for i in 0..count {
                let call_args = lists.iter().map(|l| l[i].clone()).collect();
                results.push(machine.apply(procedure.clone(), call_args)?);
            }
            Ok(if name == "map" {
                Value::List(results)
            } else {
                Value::Nil
            })
        }
        "filter" => {
            let [procedure, v] = fixed::<2>(name, args)?;
            let mut results = Vec::new();
            for item in list(name, &v)? {
                if machine
                    .apply(procedure.clone(), vec![item.clone()])?
                    .is_true()
                {
                    results.push(item.clone());
                }
            }
            Ok(Value::List(results))
        }
        "apply" => {
            let Some((procedure, rest)) = args.split_first() else {
                return Err("apply expects a procedure and a list".to_string());
            };
            let Some((last, init)) = rest.split_last() else {
                return Err("apply expects a procedure and a list".to_string());
            };
            let mut call_args = init.to_vec();
            call_args.extend(list(name, last)?.iter().cloned());
            machine.apply(procedure.clone(), call_args)
        }
        "assoc" => {
            let [key, v] = fixed::<2>(name, args)?;
            for item in list(name, &v)? {
                match item {
                    Value::List(pair) if !pair.is_empty() => {
                        if pair[0] == key {
                            return Ok(item.clone());
                        }
                    }
                    _ => return Err("assoc expects a list of lists".to_string()),
                }
            }
            Ok(Value::Boolean(false))
        }
        "member" => {
            let [key, v] = fixed::<2>(name, args)?;
            let items = list(name, &v)?;
            Ok(match items.iter().position(|item| *item == key) {
                Some(i) => Value::List(items[i..].to_vec()),
                None => Value::Boolean(false),
            })
        }
        "string-length" => {
            let [s] = fixed::<1>(name, args)?;
            Ok(Value::Number(string(name, &s)?.chars().count() as f64))
        }
        "string-append" => {
            let mut result = String::new();
            for v in &args {
                result.push_str(string(name, v)?);
            }
            Ok(Value::String(result))
        }
        "substring" => {
            if !(2..=3).contains(&args.len()) {
                return Err("substring expects 2 or 3 arguments".to_string());
            }
            let chars: Vec<char> = string(name, &args[0])?.chars().collect();
            let start = index(name, &args[1])?;
            let end = match args.get(2) {
                Some(end) => index(name, end)?,
                None => chars.len(),
            };
            if start > end || end > chars.len() {
                return Err("substring: index out of range".to_string());
            }
            Ok(Value::String(chars[start..end].iter().collect()))
        }
        "string=?" | "string<?" => {
            let [a, b] = fixed::<2>(name, args)?;
            let (a, b) = (string(name, &a)?, string(name, &b)?);
            Ok(Value::Boolean(if name == "string=?" {
                a == b
            } else {
                a < b
            }))
        }
        "string-upcase" | "string-downcase" => {
            let [s] = fixed::<1>(name, args)?;
            let s = string(name, &s)?;
            Ok(Value::String(if name == "string-upcase" {
                s.to_uppercase()
            } else {
                s.to_lowercase()
            }))
        }
        "number->string" => {
            let [n] = fixed::<1>(name, args)?;
            Ok(Value::String(Value::Number(number(name, &n)?).to_string()))
        }
        "string->number" => {
            let [s] = fixed::<1>(name, args)?;
            Ok(match string(name, &s)?.trim().parse::<f64>() {
                Ok(n) => Value::Number(n),
                Err(_) => Value::Boolean(false),
            })
        }
        "symbol->string" => match fixed::<1>(name, args)? {
            [Value::Symbol(s)] => Ok(Value::String(s)),
            _ => Err("symbol->string expects a symbol".to_string()),
        },
        "string->symbol" => {
            let [s] = fixed::<1>(name, args)?;
            Ok(Value::Symbol(string(name, &s)?.to_string()))
        }
        "format" => Ok(Value::String(format(&args)?)),
        "point" | "point-min" | "point-max" | "buffer-string" | "buffer-name" => {
            fixed::<0>(name, args)?;
            machine.host.call(name, vec![])
        }
        "goto-char" => {
            let [pos] = fixed::<1>(name, args)?;
            number(name, &pos)?;
            machine.host.call(name, vec![pos])
        }
        "delete-region" | "buffer-substring" => {
            let [from, to] = fixed::<2>(name, args)?;
            number(name, &from)?;
            number(name, &to)?;
            machine.host.call(name, vec![from, to])
        }
        "insert" => {
            for v in &args {
                string(name, v)?;
            }
            machine.host.call(name, args)?;
            Ok(Value::Nil)
        }
        "message" => {
            let text = format(&args)?;
            machine.host.call(name, vec![Value::String(text.clone())])?;
            Ok(Value::String(text))
        }
        "call-interactively" | "command-execute" => match fixed::<1>(name, args)? {
            [Value::Symbol(command) | Value::String(command)] => machine.call_command(&command),
            _ => Err(format!("{} expects a command name", name)),
        },
        "global-set-key" => match fixed::<2>(name, args)? {
            [Value::String(key), Value::Symbol(command) | Value::String(command)] => {
                let key = keys::to_codemirror(&key)?;
                machine.evaluator.set_key(KeyBinding {
                    key,
                    command: command.clone(),
                });
                Ok(Value::Symbol(command))
            }
            _ => Err("global-set-key expects a key and a command name".to_string()),
        },
        _ => Err(format!("Unknown procedure: {}", name)),
    }
}

/// 引数の数を確かめて, 配列にします.
fn fixed<const N: usize>(name: &str, args: Vec<Value>) -> Result<[Value; N], String> {
    let count = args.len();
    args.try_into().map_err(|_| arity_error(name, N, count))
}

/// 引数の数が違うときの, 失敗の内容を作ります.
fn arity_error(name: &str, expected: usize, given: usize) -> String {
    let plural = if expected == 1 { "" } else { "s" };
    format!(
        "{} expects {} argument{}, given {}",
        name, expected, plural, given
    )
}

/// 数を取り出します.
fn number(name: &str, v: &Value) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        _ => Err(format!("{} expects numbers", name)),
    }
}

/// 全ての引数を, 数として取り出します.
fn numbers(name: &str, args: &[Value]) -> Result<Vec<f64>, String> {
    args.iter().map(|v| number(name, v)).collect()
}

/// 0 以上の整数 (添え字) を取り出します.
fn index(name: &str, v: &Value) -> Result<usize, String> {
    match v {
        Value::Number(n) if *n >= 0.0 && n.fract() == 0.0 => Ok(*n as usize),
        _ => Err(format!("{} expects a non-negative integer index", name)),
    }
}

/// 文字列を取り出します.
fn string<'v>(name: &str, v: &'v Value) -> Result<&'v str, String> {
    match v {
        Value::String(s) => Ok(s),
        _ => Err(format!("{} expects strings", name)),
    }
}

/// リストを取り出します.
fn list<'v>(name: &str, v: &'v Value) -> Result<&'v Vec<Value>, String> {
    match v {
        Value::List(items) => Ok(items),
        _ => Err(format!("{} expects lists", name)),
    }
}

/// `format` の文字列を作ります. `%s` (文字列は引用符なし), `%S` (書いたとおり), `%d` (整数), `%%` を使えます.
fn format(args: &[Value]) -> Result<String, String> {
    let Some((template, values)) = args.split_first() else {
        return Err("format expects a format string".to_string());
    };
    let template = string("format", template)?;
    let mut values = values.iter();
    let mut result = String::new();
    let mut chars = template.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            result.push(c);
            continue;
        }
        let directive = chars.next();
        if directive == Some('%') {
            result.push('%');
            continue;
        }
        let value = values
            .next()
            .ok_or_else(|| "format: not enough arguments".to_string())?;
        match directive {
            Some('s') => result.push_str(&value.to_display()),
            Some('S') => result.push_str(&value.to_string()),
            Some('d') => result.push_str(&format!("{}", number("format", value)?.trunc() as i64)),
            _ => return Err("format: invalid directive".to_string()),
        }
    }
    Ok(result)
}
