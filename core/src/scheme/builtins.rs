//! 組み込みの手続き (数, 真偽, リスト, 文字列, エディタの操作).
//!
//! エディタの位置は, Emacs と同じく 1 から数えます (`point-min` は 1).

use crate::scheme::editor_ops;
use crate::scheme::ert;
use crate::scheme::evaluator::{KeyBinding, Machine};
use crate::scheme::help;
use crate::scheme::hooks;
use crate::scheme::keys;
use crate::scheme::parser::Parser;
use crate::scheme::system;
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
    "eq",
    "equal",
    "symbol?",
    "string?",
    "procedure?",
    // リスト
    "car",
    "cdr",
    "caar",
    "cadr",
    "cdar",
    "cddr",
    "caddr",
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
    "read",
    "eval",
    "assoc",
    "member",
    "sort",
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
    "error",
    "user-error",
    "error-message-string",
    "scan-error-reason",
    "string-search",
    "string-prefix?",
    "string-suffix?",
    "string-trim",
    "string-join",
    "split-string",
    "string-width",
    "string-to-char",
    "char-to-string",
    "string->utf8",
    "even?",
    "odd?",
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
    "local-set-key",
    "pop-to-buffer-same-window",
    "buffer-file-name",
    "set-buffer-modified-p",
    "read-string",
    "read-file-name",
    "read-key",
    "files--write-buffer",
    "files--set-buffer-text",
    "files--revert-dired",
    "buffer-list",
    "buffer-modified-p",
    "kill-buffer",
    "kill-new",
    "completing-read",
    "display-buffer",
    "find-file-noselect",
    "highlight-regexp",
    "unhighlight-regexp",
    "outline-flag-region",
    "outline-invisible-p",
    "set-background-color",
    "set-foreground-color",
    "set-frame-font",
    "set-face-attribute",
    // ヘルプ (ほかは help.rs)
    "where-is-internal",
    "editor-command-names",
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
        // eq と equal は, Emacs Lisp の名前 (ERT のテストで使う)
        "eq?" | "eqv?" | "equal?" | "eq" | "equal" => {
            let [a, b] = fixed::<2>(name, args)?;
            Ok(Value::Boolean(a == b))
        }
        "car" | "cdr" => {
            let [v] = fixed::<1>(name, args)?;
            match v {
                Value::List(items) if !items.is_empty() => Ok(if name == "car" {
                    items[0].clone()
                } else {
                    // 配列は写さない (cdr でたどるループが, 長さの 2 乗にならないように)
                    Value::List(items.tail())
                }),
                _ => Err(format!("{} expects a non-empty list", name)),
            }
        }
        "caar" | "cadr" | "cdar" | "cddr" | "caddr" => {
            let [v] = fixed::<1>(name, args)?;
            // 名前の a と d を, 後ろから順に, car と cdr にする
            name[1..name.len() - 1]
                .chars()
                .rev()
                .try_fold(v, |v, c| {
                    call(machine, if c == 'a' { "car" } else { "cdr" }, vec![v])
                })
                .map_err(|_| format!("{} expects a list with enough elements", name))
        }
        "cons" => {
            let [head, tail] = fixed::<2>(name, args)?;
            match tail {
                // 配列の前の空きに置く (前へ足していくループが, 長さの 2 乗にならないように)
                Value::List(items) => Ok(Value::List(items.cons(head))),
                _ => Err("cons expects a list as the second argument".to_string()),
            }
        }
        "list" => Ok(Value::List(args.into())),
        "length" => {
            let [v] = fixed::<1>(name, args)?;
            Ok(Value::Number(list(name, &v)?.len() as f64))
        }
        "append" => {
            let mut result = Vec::new();
            for v in &args {
                result.extend(list(name, v)?.iter().cloned());
            }
            Ok(Value::List(result.into()))
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
                Value::List(results.into())
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
            Ok(Value::List(results.into()))
        }
        "read" => {
            // 文字列の始めの式を読む (Emacs の, 文字列を渡した read)
            let [text] = fixed::<1>(name, args)?;
            Parser::new(string(name, &text)?)
                .parse_first()?
                .ok_or_else(|| "End of file during parsing".to_string())
        }
        "eval" => {
            // 一番外側の環境で評価する
            let [expr] = fixed::<1>(name, args)?;
            let global = machine.evaluator.global().clone();
            machine.eval(&expr, &global)
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
        "sort" => {
            let [v, procedure] = fixed::<2>(name, args)?;
            let items = list(name, &v)?.to_vec();
            Ok(Value::List(merge_sort(machine, &procedure, items)?.into()))
        }
        "member" => {
            let [key, v] = fixed::<2>(name, args)?;
            let Value::List(items) = v else {
                return Err(format!("{} expects lists", name));
            };
            Ok(match items.iter().position(|item| *item == key) {
                Some(i) => Value::List(items.drop_front(i)),
                None => Value::Boolean(false),
            })
        }
        "string-width" => {
            let [s] = fixed::<1>(name, args)?;
            Ok(Value::Number(string_width(string(name, &s)?) as f64))
        }
        "string-to-char" => {
            // 最初の文字の番号 (空の文字列は 0. Emacs と同じ)
            let [s] = fixed::<1>(name, args)?;
            Ok(Value::Number(
                string(name, &s)?.chars().next().map_or(0, |c| c as u32) as f64,
            ))
        }
        "char-to-string" => {
            let [code] = fixed::<1>(name, args)?;
            let code = number(name, &code)?;
            char::from_u32(code as u32)
                .map(|c| Value::String(c.to_string()))
                .ok_or_else(|| format!("{name}: invalid character {code}"))
        }
        "string->utf8" => {
            // UTF-8 のバイト (数のリスト)
            let [s] = fixed::<1>(name, args)?;
            Ok(Value::List(
                string(name, &s)?
                    .bytes()
                    .map(|b| Value::Number(f64::from(b)))
                    .collect(),
            ))
        }
        "even?" | "odd?" => {
            let [n] = fixed::<1>(name, args)?;
            let n = number(name, &n)?;
            if n.fract() != 0.0 {
                return Err(format!("{name} expects an integer"));
            }
            Ok(Value::Boolean((n as i64 % 2 == 0) == (name == "even?")))
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
        // (error "書式" 引数...): Emacs と同じく, 書式で作った文字を, 失敗の内容にする
        "error" | "user-error" => Err(format(&args)?),
        "scan-error-reason" | "error-message-string" => {
            // (condition-case の変数の) (error "メッセージ") か, メッセージの文字列から, メッセージを取り出す
            let [value] = fixed::<1>(name, args)?;
            let message = match &value {
                Value::List(items) => items[1].to_display(),
                other => other.to_display(),
            };
            if name == "error-message-string" {
                return Ok(Value::String(message));
            }
            // 'Scan error: "理由", 3, 4' から理由を取り出す (Emacs の scan-error の最初のデータ). 他の失敗は, そのメッセージ
            let reason = message
                .strip_prefix("Scan error: \"")
                .and_then(|rest| rest.split_once('"'))
                .map_or(message.clone(), |(reason, _)| reason.to_string());
            Ok(Value::String(reason))
        }
        "string-search" => {
            if !(2..=3).contains(&args.len()) {
                return Err("string-search expects 2 or 3 arguments".to_string());
            }
            let needle = string(name, &args[0])?;
            let haystack = string(name, &args[1])?;
            let start = match args.get(2) {
                Some(start) => index(name, start)?,
                None => 0,
            };
            let chars: Vec<char> = haystack.chars().collect();
            if start > chars.len() {
                return Err("string-search: index out of range".to_string());
            }
            let rest: String = chars[start..].iter().collect();
            Ok(match rest.find(needle) {
                Some(found) => Value::Number((start + rest[..found].chars().count()) as f64),
                None => Value::Boolean(false),
            })
        }
        "string-prefix?" | "string-suffix?" => {
            let [affix, text] = fixed::<2>(name, args)?;
            let (affix, text) = (string(name, &affix)?, string(name, &text)?);
            Ok(Value::Boolean(if name == "string-prefix?" {
                text.starts_with(affix)
            } else {
                text.ends_with(affix)
            }))
        }
        "string-trim" => {
            let [text] = fixed::<1>(name, args)?;
            Ok(Value::String(string(name, &text)?.trim().to_string()))
        }
        "string-join" => {
            let (items, separator) = match args.as_slice() {
                [items] => (items, ""),
                [items, separator] => (items, string(name, separator)?),
                _ => return Err("string-join expects 1 or 2 arguments".to_string()),
            };
            let items = list(name, items)?
                .iter()
                .map(|item| string(name, item))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::String(items.join(separator)))
        }
        "split-string" => {
            let pieces: Vec<String> = match args.as_slice() {
                [text] => string(name, text)?
                    .split_whitespace()
                    .map(str::to_string)
                    .collect(),
                [text, separator] => {
                    let separator = string(name, separator)?;
                    if separator.is_empty() {
                        return Err("split-string expects a non-empty separator".to_string());
                    }
                    string(name, text)?
                        .split(separator)
                        .map(str::to_string)
                        .collect()
                }
                _ => return Err("split-string expects 1 or 2 arguments".to_string()),
            };
            Ok(Value::List(pieces.into_iter().map(Value::String).collect()))
        }
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
        "local-set-key" => match fixed::<2>(name, args)? {
            [Value::String(key), Value::Symbol(command) | Value::String(command)] => {
                let key = keys::to_codemirror(&key)?;
                machine.host.call(
                    name,
                    vec![Value::String(key), Value::String(command.clone())],
                )?;
                Ok(Value::Symbol(command))
            }
            _ => Err("local-set-key expects a key and a command name".to_string()),
        },
        "pop-to-buffer-same-window" | "set-background-color" | "set-foreground-color" => {
            let [text] = fixed::<1>(name, args)?;
            string(name, &text)?;
            machine.host.call(name, vec![text])?;
            Ok(Value::Nil)
        }
        "buffer-file-name" => {
            // バッファの名前を省くと, 今のバッファ
            let args = optional_buffer(name, args)?;
            Ok(match machine.host.call(name, args)? {
                Value::String(path) => Value::String(path),
                _ => Value::Boolean(false),
            })
        }
        "buffer-modified-p" | "kill-buffer" => {
            let args = optional_buffer(name, args)?;
            Ok(Value::Boolean(machine.host.call(name, args)?.is_true()))
        }
        "buffer-list" => {
            fixed::<0>(name, args)?;
            machine.host.call(name, vec![])
        }
        "kill-new" => {
            let [text] = fixed::<1>(name, args)?;
            string(name, &text)?;
            machine.host.call(name, vec![text])?;
            Ok(Value::Nil)
        }
        "highlight-regexp" => {
            // (highlight-regexp REGEXP [FACE [SUBEXP]]): FACE は記号か文字列, SUBEXP は括弧の番号
            if args.is_empty() || args.len() > 3 {
                return Err("highlight-regexp expects 1 to 3 arguments".to_string());
            }
            let regexp = string(name, &args[0])?.to_string();
            let face = match args.get(1) {
                Some(Value::Symbol(face)) | Some(Value::String(face)) => {
                    Value::String(face.clone())
                }
                Some(v) if v.is_non_nil() => {
                    return Err(format!("highlight-regexp: invalid face {}", v))
                }
                _ => Value::Nil,
            };
            let group = match args.get(2) {
                Some(v) if v.is_non_nil() => Value::Number(index(name, v)? as f64),
                _ => Value::Nil,
            };
            machine
                .host
                .call(name, vec![Value::String(regexp), face, group])?;
            Ok(Value::Nil)
        }
        "outline-flag-region" => {
            // (outline-flag-region FROM TO FLAG): FLAG が non-nil なら隠す, nil なら出す
            let [from, to, flag] = fixed::<3>(name, args)?;
            number(name, &from)?;
            number(name, &to)?;
            machine
                .host
                .call(name, vec![from, to, Value::Boolean(flag.is_non_nil())])?;
            Ok(Value::Nil)
        }
        "outline-invisible-p" => {
            // 省くと, カーソルの位置
            let pos = match args.as_slice() {
                [] => machine.host.call("point", vec![])?,
                [pos] => {
                    number(name, pos)?;
                    pos.clone()
                }
                _ => return Err("outline-invisible-p expects 0 or 1 arguments".to_string()),
            };
            machine.host.call(name, vec![pos])
        }
        "unhighlight-regexp" => {
            // #t なら, 全て消す
            let [regexp] = fixed::<1>(name, args)?;
            match regexp {
                Value::String(_) | Value::Boolean(true) => machine.host.call(name, vec![regexp])?,
                _ => return Err("unhighlight-regexp expects a regexp or #t".to_string()),
            };
            Ok(Value::Nil)
        }
        "completing-read" => completing_read(machine, args),
        "display-buffer" => {
            // 行 (1 から) を渡すと, そのウィンドウのカーソルを, その行に置く
            let args = match args.as_slice() {
                [buffer] => vec![Value::String(string(name, buffer)?.to_string())],
                [buffer, line] => {
                    number(name, line)?;
                    vec![
                        Value::String(string(name, buffer)?.to_string()),
                        line.clone(),
                    ]
                }
                _ => return Err("display-buffer expects 1 or 2 arguments".to_string()),
            };
            machine.host.call(name, args)?;
            Ok(Value::Nil)
        }
        "find-file-noselect" => {
            let [path] = fixed::<1>(name, args)?;
            let path = machine.expand_file_name(string(name, &path)?);
            machine.host.call(name, vec![Value::String(path)])
        }
        "set-buffer-modified-p" => {
            let [flag] = fixed::<1>(name, args)?;
            machine
                .host
                .call(name, vec![Value::Boolean(flag.is_true())])?;
            Ok(flag)
        }
        "read-string" => {
            // (read-string PROMPT [INITIAL-INPUT [HISTORY [DEFAULT-VALUE]]])
            if args.is_empty() || args.len() > 5 {
                return Err("read-string expects 1 to 5 arguments".to_string());
            }
            let prompt = string(name, &args[0])?;
            let initial = optional_string(name, args.get(1))?;
            let default = default_value(name, args.get(3))?;
            let input = read_input(machine, name, vec![prompt.to_string(), initial])?;
            Ok(Value::String(or_default(input, default)))
        }
        "read-file-name" => {
            // (read-file-name PROMPT [DIR [DEFAULT-FILENAME [MUSTMATCH [INITIAL [PREDICATE]]]]])
            // DIR を省くと, 画面が今のバッファのディレクトリを入れる. 入力を変えなければ, DEFAULT-FILENAME
            if args.is_empty() || args.len() > 6 {
                return Err("read-file-name expects 1 to 6 arguments".to_string());
            }
            let prompt = string(name, &args[0])?;
            let dir = optional_string(name, args.get(1))?;
            let default = default_value(name, args.get(2))?;
            let input = read_input(machine, name, vec![prompt.to_string(), dir.clone()])?;
            Ok(Value::String(match default {
                Some(default) if input.is_empty() || input == dir => default,
                _ => input,
            }))
        }
        "read-key" => {
            // (read-key [PROMPT]): 1 つのキーを読む ("y", " ", "Enter" など. キーの名前は, 画面のもの)
            if args.len() > 1 {
                return Err("read-key expects 0 or 1 arguments".to_string());
            }
            let prompt = optional_string(name, args.first())?;
            Ok(Value::String(read_input(machine, name, vec![prompt])?))
        }
        "files--set-buffer-text" => {
            // (files--set-buffer-text TEXT MODIFIED): 今のバッファの本文を置き換える (カーソルは同じ文字の所に残り, 元に戻せる)
            let [text, modified] = fixed::<2>(name, args)?;
            string(name, &text)?;
            machine
                .host
                .call(name, vec![text, Value::Boolean(modified.is_true())])?;
            Ok(Value::Nil)
        }
        "files--revert-dired" => {
            // (files--revert-dired): 今のバッファが dired なら, 一覧を作り直して #t. dired でなければ #f
            fixed::<0>(name, args)?;
            machine.host.call(name, vec![])
        }
        "files--write-buffer" => {
            // (files--write-buffer FILENAME [BUFFER]): バッファ (省くと, 今のバッファ) を, ファイルに書く
            let (path, buffer) = match args.as_slice() {
                [path] => (path, None),
                [path, buffer] => (path, Some(string(name, buffer)?.to_string())),
                _ => return Err("files--write-buffer expects 1 or 2 arguments".to_string()),
            };
            let path = machine.expand_file_name(string(name, path)?);
            let args = std::iter::once(path)
                .chain(buffer)
                .map(Value::String)
                .collect();
            machine.host.call(name, args)?;
            Ok(Value::Nil)
        }
        "set-frame-font" => {
            let [font] = fixed::<1>(name, args)?;
            let (family, size) = parse_font(string(name, &font)?);
            set_font(machine, family, size)
        }
        "set-face-attribute" => set_face_attribute(machine, args),
        _ if ert::NAMES.contains(&name) => ert::call(machine, name, args),
        "where-is-internal" => {
            // 割り当てたキー (Emacs の書き方. 画面のキーと, Lisp のキー)
            let [command] = fixed::<1>(name, args)?;
            let Value::Symbol(command) = command else {
                return Err("where-is-internal expects a symbol".to_string());
            };
            machine.host.call(name, vec![Value::String(command)])
        }
        "editor-command-names" => {
            // 画面 (TypeScript) で書いたコマンドの名前
            fixed::<0>(name, args)?;
            machine.host.call(name, vec![])
        }
        _ if editor_ops::NAMES.contains(&name) => editor_ops::call(machine, name, args),
        _ if hooks::NAMES.contains(&name) => hooks::call(machine, name, args),
        _ if help::NAMES.contains(&name) => help::call(machine, name, args),
        _ => system::call(machine, name, args)
            .unwrap_or_else(|| Err(format!("Unknown procedure: {}", name))),
    }
}

/// リストを, 安定に並べ替えます (マージソート).
///
/// # Arguments
///
/// * `machine` - 評価 (比べる手続きを呼ぶ).
/// * `less` - 比べる手続き (1 つ目が前なら, 真を返す).
/// * `items` - リストの要素.
///
/// # Returns
///
/// 並べ替えた要素, または, 比べる手続きの失敗の内容.
fn merge_sort(
    machine: &mut Machine,
    less: &Value,
    mut items: Vec<Value>,
) -> Result<Vec<Value>, String> {
    if items.len() <= 1 {
        return Ok(items);
    }
    let right = items.split_off(items.len() / 2);
    let mut left = merge_sort(machine, less, items)?.into_iter().peekable();
    let mut right = merge_sort(machine, less, right)?.into_iter().peekable();
    let mut merged = Vec::new();
    while let (Some(a), Some(b)) = (left.peek(), right.peek()) {
        // 右が左より前のときだけ, 右を先にする (同じなら, 元の順)
        if machine
            .apply(less.clone(), vec![b.clone(), a.clone()])?
            .is_true()
        {
            merged.extend(right.next());
        } else {
            merged.extend(left.next());
        }
    }
    merged.extend(left);
    merged.extend(right);
    Ok(merged)
}

/// Emacs のフォントの名前 (`DejaVu Sans Mono-14`) を, ファミリーと大きさ (ポイント) に分けます.
///
/// # Arguments
///
/// * `font` - フォントの名前 (最後の `-` の後が数なら, 大きさ).
///
/// # Returns
///
/// ファミリーと大きさ (大きさが無ければ `None`).
pub fn parse_font(font: &str) -> (String, Option<f64>) {
    if let Some((family, size)) = font.rsplit_once('-') {
        if let Ok(size) = size.trim().parse::<f64>() {
            return (family.trim().to_string(), Some(size));
        }
    }
    (font.trim().to_string(), None)
}

/// エディタのフォントを変えます.
///
/// # Arguments
///
/// * `machine` - 評価 (エディタに頼む).
/// * `family` - ファミリー (空なら, 変えない).
/// * `size` - 大きさ (ポイント. `None` なら, 変えない).
///
/// # Returns
///
/// `nil`, または失敗の内容.
fn set_font(machine: &mut Machine, family: String, size: Option<f64>) -> Result<Value, String> {
    let family = if family.is_empty() {
        Value::Nil
    } else {
        Value::String(family)
    };
    let size = size.map_or(Value::Nil, Value::Number);
    machine.host.call("set-font", vec![family, size])?;
    Ok(Value::Nil)
}

/// `(set-face-attribute 'default #f :family "Monospace" :height 140)` を行います.
/// 変えられるのは, `default` の face の, `:family`, `:height` (1/10 ポイント), `:foreground`, `:background` です.
///
/// # Arguments
///
/// * `machine` - 評価 (エディタに頼む).
/// * `args` - face, フレーム (使わない), 属性と値の組.
///
/// # Returns
///
/// `nil`, または失敗の内容.
fn set_face_attribute(machine: &mut Machine, args: Vec<Value>) -> Result<Value, String> {
    let name = "set-face-attribute";
    let [face, _frame, attributes @ ..] = args.as_slice() else {
        return Err("set-face-attribute expects a face, a frame and attributes".to_string());
    };
    if *face != Value::Symbol("default".to_string()) {
        return Err(format!("set-face-attribute: unsupported face {}", face));
    }
    if attributes.len() % 2 != 0 {
        return Err("set-face-attribute expects pairs of an attribute and a value".to_string());
    }
    let (mut family, mut size) = (String::new(), None);
    for pair in attributes.chunks(2) {
        match (&pair[0], &pair[1]) {
            (Value::Symbol(key), value) if key == ":family" => {
                family = string(name, value)?.to_string()
            }
            (Value::Symbol(key), value) if key == ":height" => {
                size = Some(number(name, value)? / 10.0)
            }
            (Value::Symbol(key), value) if key == ":foreground" || key == ":background" => {
                string(name, value)?;
                let op = if key == ":foreground" {
                    "set-foreground-color"
                } else {
                    "set-background-color"
                };
                machine.host.call(op, vec![value.clone()])?;
            }
            (key, _) => return Err(format!("set-face-attribute: unsupported attribute {}", key)),
        }
    }
    if family.is_empty() && size.is_none() {
        return Ok(Value::Nil);
    }
    set_font(machine, family, size)
}

/// 文字の表示の幅 (Emacs の `char-width`). 全角の文字 (東アジアの幅の広い文字と, 絵文字) は 2, タブは 8, ほかは 1.
///
/// # Arguments
///
/// * `c` - 文字.
///
/// # Returns
///
/// 幅.
pub fn char_width(c: char) -> usize {
    const WIDE: &[(u32, u32)] = &[
        (0x1100, 0x115f),
        (0x2e80, 0x303e),
        (0x3041, 0x33ff),
        (0x3400, 0x4dbf),
        (0x4e00, 0x9fff),
        (0xa000, 0xa4cf),
        (0xac00, 0xd7a3),
        (0xf900, 0xfaff),
        (0xfe30, 0xfe4f),
        (0xff00, 0xff60),
        (0xffe0, 0xffe6),
        (0x1f300, 0x1f64f),
        (0x1f900, 0x1f9ff),
        (0x20000, 0x3fffd),
    ];
    let code = c as u32;
    if c == '\t' {
        8
    } else if WIDE.iter().any(|(lo, hi)| (*lo..=*hi).contains(&code)) {
        2
    } else {
        1
    }
}

/// 文字列の表示の幅 (Emacs の `string-width`).
///
/// # Arguments
///
/// * `s` - 文字列.
///
/// # Returns
///
/// 幅 (文字の幅の和).
pub fn string_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

/// `(completing-read PROMPT COLLECTION [PREDICATE [REQUIRE-MATCH [INITIAL-INPUT [HIST [DEF]]]]])`.
/// 候補を Tab で補完しながら, 入力してもらいます (Emacs と同じ引数).
///
/// - COLLECTION は, 文字列, 記号, リスト (`car` を候補にする. alist) のリスト.
/// - PREDICATE が non-nil なら, 候補の要素を渡して, 真のものだけを候補にする.
/// - REQUIRE-MATCH が non-nil なら, 候補に無い入力は, 聞き直す (空の入力は, 認める).
/// - 入力が空で, DEF (文字列か, 文字列のリスト. リストなら最初) があれば, DEF を返す.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `args` - 引数.
///
/// # Returns
///
/// 入力. 引数が誤っているか, やめたら [`Err`].
fn completing_read(machine: &mut Machine, args: Vec<Value>) -> Result<Value, String> {
    let name = "completing-read";
    if args.len() < 2 || args.len() > 8 {
        return Err("completing-read expects 2 to 8 arguments".to_string());
    }
    let prompt = string(name, &args[0])?.to_string();
    let Value::List(items) = &args[1] else {
        return Err("completing-read expects a list of candidates".to_string());
    };
    let predicate = args.get(2).filter(|p| p.is_non_nil()).cloned();
    let require_match = args.get(3).is_some_and(Value::is_non_nil);
    let initial = optional_string(name, args.get(4))?;
    let default = default_value(name, args.get(6))?;
    let mut candidates = Vec::new();
    for item in items.iter() {
        let label = match item {
            Value::List(pair) if !pair.is_empty() => &pair[0],
            other => other,
        };
        let label = match label {
            Value::String(text) | Value::Symbol(text) => text.clone(),
            other => return Err(format!("completing-read: not a candidate: {}", other)),
        };
        if let Some(predicate) = &predicate {
            if !machine
                .apply(predicate.clone(), vec![item.clone()])?
                .is_non_nil()
            {
                continue;
            }
        }
        candidates.push(label);
    }
    let collection: Value = Value::List(
        candidates
            .iter()
            .map(|c| Value::String(c.clone()))
            .collect(),
    );
    let mut ask = (prompt.clone(), initial);
    loop {
        let input = match machine.host.call(
            name,
            vec![
                Value::String(ask.0.clone()),
                collection.clone(),
                Value::String(ask.1.clone()),
            ],
        )? {
            Value::String(input) => input,
            other => return Err(format!("completing-read: unexpected input {}", other)),
        };
        if !require_match || input.is_empty() || candidates.contains(&input) {
            return Ok(Value::String(or_default(input, default)));
        }
        // 候補に無い (Emacs は [No match] と出して, 入力を続けさせる)
        ask = (format!("[No match] {}", prompt), input);
    }
}

/// ミニバッファで, 入力してもらいます (`read-string` の, エディタへの頼み).
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 操作の名前.
/// * `args` - 引数 (文字列).
///
/// # Returns
///
/// 入力. やめたら [`Err`].
fn read_input(machine: &mut Machine, name: &str, args: Vec<String>) -> Result<String, String> {
    match machine
        .host
        .call(name, args.into_iter().map(Value::String).collect())?
    {
        Value::String(input) => Ok(input),
        other => Err(format!("{}: unexpected input {}", name, other)),
    }
}

/// 省ける, 文字列の引数を返します (省くか, nil なら空).
///
/// # Arguments
///
/// * `name` - 手続きの名前.
/// * `v` - 引数.
///
/// # Returns
///
/// 文字列. 文字列でも nil でもなければ [`Err`].
fn optional_string(name: &str, v: Option<&Value>) -> Result<String, String> {
    match v {
        Some(v) if v.is_non_nil() => string(name, v).map(str::to_string),
        _ => Ok(String::new()),
    }
}

/// 既定の値の引数 (Emacs の DEFAULT-VALUE, DEF) を返します. 文字列のリストなら, 最初.
///
/// # Arguments
///
/// * `name` - 手続きの名前.
/// * `v` - 引数.
///
/// # Returns
///
/// 既定の値 (省くか, nil なら `None`).
fn default_value(name: &str, v: Option<&Value>) -> Result<Option<String>, String> {
    match v {
        Some(Value::List(items)) if !items.is_empty() => {
            string(name, &items[0]).map(|s| Some(s.to_string()))
        }
        Some(v) if v.is_non_nil() => string(name, v).map(|s| Some(s.to_string())),
        _ => Ok(None),
    }
}

/// 入力が空で, 既定の値があれば, 既定の値を返します.
///
/// # Arguments
///
/// * `input` - 入力.
/// * `default` - 既定の値.
///
/// # Returns
///
/// 入力, または既定の値.
fn or_default(input: String, default: Option<String>) -> String {
    match default {
        Some(default) if input.is_empty() => default,
        _ => input,
    }
}

/// 引数の数を確かめて, 配列にします.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 引数.
///
/// # Returns
///
/// `N` 個の引数の配列. 数が違う場合は [`Err`].
pub(crate) fn fixed<const N: usize>(name: &str, args: Vec<Value>) -> Result<[Value; N], String> {
    let count = args.len();
    args.try_into().map_err(|_| arity_error(name, N, count))
}

/// 引数の数が違うときの, 失敗の内容を作ります.
///
/// # Arguments
///
/// * `name` - 手続きの名前.
/// * `expected` - 求める引数の数.
/// * `given` - 渡された引数の数.
///
/// # Returns
///
/// 失敗の内容.
fn arity_error(name: &str, expected: usize, given: usize) -> String {
    let plural = if expected == 1 { "" } else { "s" };
    format!(
        "{} expects {} argument{}, given {}",
        name, expected, plural, given
    )
}

/// 数を取り出します.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `v` - 値.
///
/// # Returns
///
/// 数. 数でない場合は [`Err`].
pub(crate) fn number(name: &str, v: &Value) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        _ => Err(format!("{} expects numbers", name)),
    }
}

/// 全ての引数を, 数として取り出します.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 引数.
///
/// # Returns
///
/// 数 (引数の順). 数でない引数がある場合は [`Err`].
fn numbers(name: &str, args: &[Value]) -> Result<Vec<f64>, String> {
    args.iter().map(|v| number(name, v)).collect()
}

/// 0 以上の整数 (添え字) を取り出します.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `v` - 値.
///
/// # Returns
///
/// 添え字. 0 以上の整数でない場合は [`Err`].
pub(crate) fn index(name: &str, v: &Value) -> Result<usize, String> {
    match v {
        Value::Number(n) if *n >= 0.0 && n.fract() == 0.0 => Ok(*n as usize),
        _ => Err(format!("{} expects a non-negative integer index", name)),
    }
}

/// 文字列を取り出します.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `v` - 値.
///
/// # Returns
///
/// 文字列. 文字列でない場合は [`Err`].
pub(crate) fn string<'v>(name: &str, v: &'v Value) -> Result<&'v str, String> {
    match v {
        Value::String(s) => Ok(s),
        _ => Err(format!("{} expects strings", name)),
    }
}

/// 省ける, バッファの名前の引数を返します (Emacs の, BUFFER を省ける手続き).
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `args` - 引数 (無いか, バッファの名前).
///
/// # Returns
///
/// エディタに渡す引数. 多すぎるか, 名前が文字列でなければ [`Err`].
fn optional_buffer(name: &str, args: Vec<Value>) -> Result<Vec<Value>, String> {
    match args.as_slice() {
        [] => Ok(vec![]),
        [buffer] => {
            string(name, buffer)?;
            Ok(args)
        }
        _ => Err(format!("{} expects 0 or 1 arguments", name)),
    }
}

/// リストを取り出します.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に書く).
/// * `v` - 値.
///
/// # Returns
///
/// リストの要素. リストでない場合は [`Err`].
fn list<'v>(name: &str, v: &'v Value) -> Result<&'v [Value], String> {
    match v {
        Value::List(items) => Ok(items),
        _ => Err(format!("{} expects lists", name)),
    }
}

/// `format` の文字列を作ります. `%s` (文字列は引用符なし), `%S` (書いたとおり), `%d` (整数), `%%` を使えます.
///
/// # Arguments
///
/// * `args` - 書式の文字列と, 埋め込む値.
///
/// # Returns
///
/// 作った文字列. 書式が文字列でない, 値が足りない, 使えない指定がある場合は [`Err`].
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
            Some('o') => result.push_str(&format!("{:o}", number("format", value)?.trunc() as i64)),
            Some('x') => result.push_str(&format!("{:x}", number("format", value)?.trunc() as i64)),
            Some('X') => result.push_str(&format!("{:X}", number("format", value)?.trunc() as i64)),
            _ => return Err("format: invalid directive".to_string()),
        }
    }
    Ok(result)
}
