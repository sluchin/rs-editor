//! 組み込みの手続き (数, 真偽, リスト, 文字列, エディタの操作).
//!
//! エディタの位置は, Emacs と同じく 1 から数えます (`point-min` は 1).

use crate::error::Error;
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
    "null",
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
    "string=",
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
    "signal",
    "define-error",
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
    "princ",
    "prin1",
    "print",
    "terpri",
    "call-interactively",
    "command-execute",
    "global-set-key",
    "local-set-key",
    "set-buffer-directory",
    "pop-to-buffer-same-window",
    "buffer-file-name",
    "set-buffer-modified-p",
    "read-string",
    "read-file-name",
    "read-key",
    "execute-kbd-macro",
    "write-region",
    "files--set-buffer-text",
    "buffer-list",
    "buffer-modified-p",
    "kill-buffer",
    "kill-new",
    "completing-read",
    "display-buffer",
    "insert-file-contents",
    "highlight-regexp",
    "unhighlight-regexp",
    "outline-flag-region",
    "outline-invisible-p",
    "set-background-color",
    "set-foreground-color",
    "ding",
    "set-frame-font",
    "set-face-attribute",
    // ヘルプ (ほかは help.rs)
    "where-is-internal",
    "editor-command-names",
    // キーの割り当てと最近のキー (ターミナルのアプリが答える)
    "key-binding",
    "all-key-bindings",
    "recent-keys",
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
pub fn call(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, Error> {
    match name {
        "+" => Ok(Value::Number(numbers(name, &args)?.iter().sum())),
        "*" => Ok(Value::Number(numbers(name, &args)?.iter().product())),
        "-" => {
            let nums = numbers(name, &args)?;
            match nums.split_first() {
                None => Err("- expects at least one argument".to_string().into()),
                Some((first, [])) => Ok(Value::Number(-first)),
                Some((first, rest)) => Ok(Value::Number(rest.iter().fold(*first, |a, b| a - b))),
            }
        }
        "/" => {
            if args.len() < 2 {
                return Err("/ expects at least two arguments".to_string().into());
            }
            let nums = numbers(name, &args)?;
            if nums[1..].contains(&0.0) {
                return Err(Error::bare("arith-error"));
            }
            Ok(Value::Number(nums[1..].iter().fold(nums[0], |a, b| a / b)))
        }
        "=" | "<" | ">" | "<=" | ">=" => {
            let nums = numbers(name, &args)?;
            if nums.is_empty() {
                return Err(format!("{} expects at least one argument", name).into());
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
                None => Err(format!("{} expects at least one argument", name).into()),
                Some((first, rest)) => Ok(Value::Number(rest.iter().copied().fold(*first, pick))),
            }
        }
        "quotient" | "remainder" | "modulo" => {
            let [a, b] = fixed::<2>(name, args)?;
            let (a, b) = (number(name, &a)?, number(name, &b)?);
            if b == 0.0 {
                return Err(Error::bare("arith-error"));
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
        "not" | "null" | "boolean?" | "symbol?" | "string?" | "procedure?" | "null?" | "pair?"
        | "list?" => {
            let [v] = fixed::<1>(name, args)?;
            Ok(Value::Boolean(match name {
                "not" => !v.is_true(),
                // Emacs の null: nil (偽と空のリスト) だけ真
                "null" => !v.is_non_nil(),
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
                // Emacs と同じく, 空のリスト (と nil) の car と cdr は nil
                Value::List(_) | Value::Boolean(false) | Value::Nil => Ok(if name == "car" {
                    Value::Boolean(false)
                } else {
                    Value::List(Vec::new().into())
                }),
                other => Err(wrong_type("listp", &other)),
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
                .map_err(|_| format!("{} expects a list with enough elements", name).into())
        }
        "cons" => {
            let [head, tail] = fixed::<2>(name, args)?;
            match tail {
                // 配列の前の空きに置く (前へ足していくループが, 長さの 2 乗にならないように)
                Value::List(items) => Ok(Value::List(items.cons(head))),
                _ => Err("cons expects a list as the second argument"
                    .to_string()
                    .into()),
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
                .ok_or_else(|| format!("{}: index out of range", name).into())
        }
        "map" | "for-each" => {
            let Some((procedure, lists)) = args.split_first() else {
                return Err(format!("{} expects a procedure and lists", name).into());
            };
            let lists = lists
                .iter()
                .map(|v| list(name, v))
                .collect::<Result<Vec<_>, _>>()?;
            if lists.is_empty() {
                return Err(format!("{} expects a procedure and lists", name).into());
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
                .ok_or_else(|| Error::bare("end-of-file"))
        }
        "eval" => {
            // 一番外側の環境で評価する
            let [expr] = fixed::<1>(name, args)?;
            let global = machine.evaluator.global().clone();
            machine.eval(&expr, &global)
        }
        "apply" => {
            let Some((procedure, rest)) = args.split_first() else {
                return Err("apply expects a procedure and a list".to_string().into());
            };
            let Some((last, init)) = rest.split_last() else {
                return Err("apply expects a procedure and a list".to_string().into());
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
                    _ => return Err("assoc expects a list of lists".to_string().into()),
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
                return Err(wrong_type(list_predicate(name), &v));
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
                .ok_or_else(|| format!("{name}: invalid character {code}").into())
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
                return Err(format!("{name} expects an integer").into());
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
                return Err("substring expects 2 or 3 arguments".to_string().into());
            }
            let chars: Vec<char> = string(name, &args[0])?.chars().collect();
            let start = index(name, &args[1])?;
            let end = match args.get(2) {
                Some(end) => index(name, end)?,
                None => chars.len(),
            };
            if start > end || end > chars.len() {
                let end = args.get(2).cloned().unwrap_or(Value::Nil);
                return Err(Error::args_out_of_range(vec![
                    args[0].clone(),
                    args[1].clone(),
                    end,
                ]));
            }
            Ok(Value::String(chars[start..end].iter().collect()))
        }
        "string=?" | "string=" | "string<?" => {
            let [a, b] = fixed::<2>(name, args)?;
            let (a, b) = (string(name, &a)?, string(name, &b)?);
            Ok(Value::Boolean(if name != "string<?" {
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
            _ => Err("symbol->string expects a symbol".to_string().into()),
        },
        "string->symbol" => {
            let [s] = fixed::<1>(name, args)?;
            Ok(Value::Symbol(string(name, &s)?.to_string()))
        }
        "format" => Ok(Value::String(format(&args)?)),
        // (error "書式" 引数...): Emacs と同じく, 書式で作った文字を, 失敗の内容にする
        "error" => Err(Error::plain(format(&args)?)),
        "user-error" => Err(machine.make_error("user-error", vec![Value::String(format(&args)?)])),
        // (signal ERROR-SYMBOL DATA): 種類とデータのエラーを投げる. 種類が nil なら, DATA を (種類 . データ) として投げ直す
        "signal" => {
            let [kind, data] = fixed::<2>(name, args)?;
            let items = error_items(&data);
            Err(match kind {
                Value::Symbol(kind) => machine.make_error(&kind, items),
                // 種類が nil で, データが (種類 . データ) なら, 投げ直し
                nil if !nil.is_non_nil() && !items.is_empty() => {
                    machine.make_error(&items[0].to_display(), items[1..].to_vec())
                }
                other => return Err(wrong_type("symbolp", &other)),
            })
        }
        // (define-error NAME MESSAGE [PARENT]): 新しい種類を決める. PARENT (記号か記号のリスト) の条件も受ける
        "define-error" => {
            let (Some(Value::Symbol(kind)), Some(Value::String(description))) =
                (args.first(), args.get(1))
            else {
                return Err("define-error expects a name and a message".into());
            };
            let parents = match args.get(2) {
                Some(Value::Symbol(parent)) => vec![parent.clone()],
                Some(Value::List(items)) => items.iter().map(Value::to_display).collect(),
                _ => vec!["error".to_string()],
            };
            let mut conditions = vec![kind.clone()];
            for parent in &parents {
                let known = machine.make_error(parent, Vec::new());
                conditions.extend(known.conditions().iter().cloned());
            }
            conditions.dedup();
            machine
                .evaluator
                .define_error(kind, conditions, description);
            Ok(Value::Symbol(kind.clone()))
        }
        "scan-error-reason" | "error-message-string" => {
            // (condition-case の変数の) (種類 . データ) から, メッセージを取り出す
            let [value] = fixed::<1>(name, args)?;
            let (message, reason) = match &value {
                Value::List(items) if !items.is_empty() => {
                    let error = machine.make_error(&items[0].to_display(), items[1..].to_vec());
                    // scan-error の最初のデータが理由. 他の失敗は, そのメッセージ
                    let reason = match (error.kind(), error.data().first()) {
                        ("scan-error", Some(reason)) => reason.to_display(),
                        _ => error.message().to_string(),
                    };
                    (error.message().to_string(), reason)
                }
                other => (other.to_display(), other.to_display()),
            };
            Ok(Value::String(if name == "error-message-string" {
                message
            } else {
                reason
            }))
        }
        "string-search" => {
            if !(2..=3).contains(&args.len()) {
                return Err("string-search expects 2 or 3 arguments".to_string().into());
            }
            let needle = string(name, &args[0])?;
            let haystack = string(name, &args[1])?;
            let chars: Vec<char> = haystack.chars().collect();
            // 始めが範囲の外なら, Emacs と同じく (args-out-of-range START)
            let start = match args.get(2) {
                Some(Value::Number(n)) if *n < 0.0 || *n > chars.len() as f64 => {
                    return Err(Error::args_out_of_range(vec![args[2].clone()]));
                }
                Some(start) => index(name, start)?,
                None => 0,
            };
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
                _ => return Err("string-join expects 1 or 2 arguments".to_string().into()),
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
                        return Err("split-string expects a non-empty separator"
                            .to_string()
                            .into());
                    }
                    string(name, text)?
                        .split(separator)
                        .map(str::to_string)
                        .collect()
                }
                _ => return Err("split-string expects 1 or 2 arguments".to_string().into()),
            };
            Ok(Value::List(pieces.into_iter().map(Value::String).collect()))
        }
        // (buffer-name BUFFER): バッファの名前 (バッファは名前で表すので, 生きているバッファなら, 同じ名前. 無ければ #f)
        "buffer-name" if !args.is_empty() => {
            let [buffer] = fixed::<1>(name, args)?;
            let buffer = Value::String(string(name, &buffer)?.to_string());
            Ok(match machine.host.call("buffer-list", vec![])? {
                Value::List(names) if names.contains(&buffer) => buffer,
                _ => Value::Boolean(false),
            })
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
            machine.evaluator.log_message(&text);
            machine.host.call(name, vec![Value::String(text.clone())])?;
            Ok(Value::String(text))
        }
        "terpri" => {
            fixed::<0>(name, args)?;
            write_output(machine, "\n")?;
            Ok(Value::Boolean(true))
        }
        "princ" | "prin1" | "print" => {
            let [value] = fixed::<1>(name, args)?;
            let text = print_string(&value, name != "princ");
            write_output(
                machine,
                &if name == "print" {
                    format!("\n{text}\n")
                } else {
                    text
                },
            )?;
            Ok(value)
        }
        "call-interactively" | "command-execute" => match fixed::<1>(name, args)? {
            [Value::Symbol(command) | Value::String(command)] => machine.call_command(&command),
            _ => Err(format!("{} expects a command name", name).into()),
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
            _ => Err("global-set-key expects a key and a command name"
                .to_string()
                .into()),
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
            _ => Err("local-set-key expects a key and a command name"
                .to_string()
                .into()),
        },
        "set-buffer-directory" => {
            // (set-buffer-directory DIR): 今のバッファの `default-directory` を DIR にする (dired が使う)
            let [dir] = fixed::<1>(name, args)?;
            let dir = string(name, &dir)?.to_string();
            machine.host.call(name, vec![Value::String(dir)])?;
            Ok(Value::Nil)
        }
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
            // (highlight-regexp REGEXP [FACE [SUBEXP [JS]]]): FACE は記号か文字列, SUBEXP は括弧の番号,
            // JS が non-nil なら, REGEXP は Emacs の書き方でなく, 画面の JavaScript と同じ書き方
            if args.is_empty() || args.len() > 4 {
                return Err("highlight-regexp expects 1 to 4 arguments"
                    .to_string()
                    .into());
            }
            let regexp = string(name, &args[0])?.to_string();
            let face = match args.get(1) {
                Some(Value::Symbol(face)) | Some(Value::String(face)) => {
                    Value::String(face.clone())
                }
                Some(v) if v.is_non_nil() => {
                    return Err(format!("highlight-regexp: invalid face {}", v).into())
                }
                _ => Value::Nil,
            };
            let group = match args.get(2) {
                Some(v) if v.is_non_nil() => Value::Number(index(name, v)? as f64),
                _ => Value::Nil,
            };
            let mut call_args = vec![Value::String(regexp), face, group];
            if let Some(js) = args.get(3) {
                call_args.push(Value::Boolean(js.is_non_nil()));
            }
            machine.host.call(name, call_args)?;
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
                _ => {
                    return Err("outline-invisible-p expects 0 or 1 arguments"
                        .to_string()
                        .into())
                }
            };
            machine.host.call(name, vec![pos])
        }
        "unhighlight-regexp" => {
            // #t なら, 全て消す
            let [regexp] = fixed::<1>(name, args)?;
            match regexp {
                Value::String(_) | Value::Boolean(true) => machine.host.call(name, vec![regexp])?,
                _ => {
                    return Err("unhighlight-regexp expects a regexp or #t"
                        .to_string()
                        .into())
                }
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
                _ => return Err("display-buffer expects 1 or 2 arguments".to_string().into()),
            };
            machine.host.call(name, args)?;
            Ok(Value::Nil)
        }
        "insert-file-contents" => {
            // (insert-file-contents FILENAME [VISIT]): ファイルの名前は, 広げてから渡す
            let (path, rest) = match args.as_slice() {
                [path, rest @ ..] if rest.len() <= 1 => (path, rest),
                _ => {
                    return Err("insert-file-contents expects 1 or 2 arguments"
                        .to_string()
                        .into())
                }
            };
            let path = Value::String(machine.expand_file_name(string(name, path)?));
            machine.host.call(
                name,
                std::iter::once(path).chain(rest.iter().cloned()).collect(),
            )
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
                return Err("read-string expects 1 to 5 arguments".to_string().into());
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
                return Err("read-file-name expects 1 to 6 arguments".to_string().into());
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
            // (read-key [PROMPT [ALLOW-QUIT]]): 1 つのキーを読む ("y", " ", "Enter" など. キーの名前は, 画面のもの).
            // ALLOW-QUIT が真なら, C-g も, 失敗にせず, キー ("C-g") として返す (isearch が使う)
            if args.len() > 2 {
                return Err("read-key expects 0 to 2 arguments".to_string().into());
            }
            let prompt = optional_string(name, args.first())?;
            let mut host_args = vec![prompt];
            if args.get(1).is_some_and(Value::is_non_nil) {
                host_args.push("allow-quit".to_string());
            }
            Ok(Value::String(read_input(machine, name, host_args)?))
        }
        "execute-kbd-macro" => {
            // (execute-kbd-macro KEYS [COUNT]): キーの並びを, COUNT 回 (0 なら, 失敗するまで) 打ったことにする. 打つのは, 画面
            let [keys, rest @ ..] = args.as_slice() else {
                return Err("execute-kbd-macro expects 1 or 2 arguments"
                    .to_string()
                    .into());
            };
            let count = match rest {
                [] => Value::Number(1.0),
                [count] => count.clone(),
                _ => {
                    return Err("execute-kbd-macro expects 1 or 2 arguments"
                        .to_string()
                        .into())
                }
            };
            machine.host.call(name, vec![keys.clone(), count])?;
            Ok(Value::Nil)
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
        "write-region" => {
            // (write-region START END FILENAME [APPEND VISIT]): ファイルの名前 (と, VISIT の文字列) は, 広げてから渡す
            if !(3..=5).contains(&args.len()) {
                return Err("write-region expects 3 to 5 arguments".to_string().into());
            }
            let mut args = args;
            args[2] = Value::String(machine.expand_file_name(string(name, &args[2])?));
            if let Some(Value::String(visit)) = args.get(4) {
                args[4] = Value::String(machine.expand_file_name(visit));
            }
            machine.host.call(name, args)
        }
        "ding" => {
            // (ding): ベルを鳴らす. 鳴らせない画面なら, 何もしない
            let _ = machine.host.call(name, vec![]);
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
                return Err("where-is-internal expects a symbol".to_string().into());
            };
            machine.host.call(name, vec![Value::String(command)])
        }
        "key-binding" => {
            // キーの並び ("C-x C-f") の割り当て: コマンドの名前, 続きがあれば #t, 無ければ #f
            let [keys] = fixed::<1>(name, args)?;
            string(name, &keys)?;
            machine.host.call(name, vec![keys])
        }
        "all-key-bindings" | "recent-keys" => {
            fixed::<0>(name, args)?;
            machine.host.call(name, vec![])
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
            .unwrap_or_else(|| Err(format!("Unknown procedure: {}", name).into())),
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
) -> Result<Vec<Value>, Error> {
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
fn set_font(machine: &mut Machine, family: String, size: Option<f64>) -> Result<Value, Error> {
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
fn set_face_attribute(machine: &mut Machine, args: Vec<Value>) -> Result<Value, Error> {
    let name = "set-face-attribute";
    let [face, _frame, attributes @ ..] = args.as_slice() else {
        return Err("set-face-attribute expects a face, a frame and attributes"
            .to_string()
            .into());
    };
    if *face != Value::Symbol("default".to_string()) {
        return Err(format!("set-face-attribute: unsupported face {}", face).into());
    }
    if attributes.len() % 2 != 0 {
        return Err(
            "set-face-attribute expects pairs of an attribute and a value"
                .to_string()
                .into(),
        );
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
            (key, _) => {
                return Err(format!("set-face-attribute: unsupported attribute {}", key).into())
            }
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
fn completing_read(machine: &mut Machine, args: Vec<Value>) -> Result<Value, Error> {
    let name = "completing-read";
    if args.len() < 2 || args.len() > 8 {
        return Err("completing-read expects 2 to 8 arguments"
            .to_string()
            .into());
    }
    let prompt = string(name, &args[0])?.to_string();
    let Value::List(items) = &args[1] else {
        return Err("completing-read expects a list of candidates"
            .to_string()
            .into());
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
            other => return Err(format!("completing-read: not a candidate: {}", other).into()),
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
            other => return Err(format!("completing-read: unexpected input {}", other).into()),
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
fn read_input(machine: &mut Machine, name: &str, args: Vec<String>) -> Result<String, Error> {
    match machine
        .host
        .call(name, args.into_iter().map(Value::String).collect())?
    {
        Value::String(input) => Ok(input),
        other => Err(format!("{}: unexpected input {}", name, other).into()),
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
fn optional_string(name: &str, v: Option<&Value>) -> Result<String, Error> {
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
fn default_value(name: &str, v: Option<&Value>) -> Result<Option<String>, Error> {
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
pub(crate) fn fixed<const N: usize>(name: &str, args: Vec<Value>) -> Result<[Value; N], Error> {
    let count = args.len();
    args.try_into()
        .map_err(|_| arity_error(name, N, count).into())
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
pub(crate) fn number(name: &str, v: &Value) -> Result<f64, Error> {
    match v {
        Value::Number(n) => Ok(*n),
        _ => Err(wrong_type(number_predicate(name), v)),
    }
}

/// Emacs の `wrong-type-argument` のエラー.
///
/// # Arguments
///
/// * `predicate` - 合わなかった述語の名前 (`listp`, `stringp` など).
/// * `value` - 渡された値.
///
/// # Returns
///
/// エラー.
pub(crate) fn wrong_type(predicate: &str, value: &Value) -> Error {
    Error::wrong_type(predicate, value)
}

/// 手続きが数を求める述語の名前 (Emacs と同じ): 算術と比較は `number-or-marker-p`, `abs` は `numberp`, ほかは位置などの `integer-or-marker-p`.
fn number_predicate(name: &str) -> &'static str {
    match name {
        "+" | "-" | "*" | "/" | "<" | ">" | "<=" | ">=" | "=" | "min" | "max" | "zero?"
        | "zerop" | "modulo" | "mod" | "1+" | "1-" => "number-or-marker-p",
        "abs" => "numberp",
        _ => "integer-or-marker-p",
    }
}

/// 手続きがリストを求める述語の名前 (Emacs と同じ): リストだけを取るものは `listp`, 並び全体を取るものは `sequencep`.
fn list_predicate(name: &str) -> &'static str {
    match name {
        "member" | "memq" | "assoc" | "assq" | "apply" | "nth" | "car" | "cdr" => "listp",
        _ => "sequencep",
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
fn numbers(name: &str, args: &[Value]) -> Result<Vec<f64>, Error> {
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
pub(crate) fn index(_name: &str, v: &Value) -> Result<usize, Error> {
    match v {
        Value::Number(n) if *n >= 0.0 && n.fract() == 0.0 => Ok(*n as usize),
        _ => Err(wrong_type("integerp", v)),
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
pub(crate) fn string<'v>(_name: &str, v: &'v Value) -> Result<&'v str, Error> {
    match v {
        Value::String(s) => Ok(s),
        _ => Err(wrong_type("stringp", v)),
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
fn optional_buffer(name: &str, args: Vec<Value>) -> Result<Vec<Value>, Error> {
    match args.as_slice() {
        [] => Ok(vec![]),
        [buffer] => {
            string(name, buffer)?;
            Ok(args)
        }
        _ => Err(format!("{} expects 0 or 1 arguments", name).into()),
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
fn list<'v>(name: &str, v: &'v Value) -> Result<&'v [Value], Error> {
    match v {
        Value::List(items) => Ok(items),
        _ => Err(wrong_type(list_predicate(name), v)),
    }
}

/// 標準出力に書く (バッチモードは標準出力. 画面のエディタは, エコー領域).
fn write_output(machine: &mut Machine, text: &str) -> Result<(), Error> {
    machine
        .host
        .call("standard-output", vec![Value::String(text.to_string())])
        .map(|_| ())
}

/// 値の出力の文字 (`prin1` は文字列を引用符で囲み, `princ` は囲まない).
pub(crate) fn print_string(value: &Value, quoted: bool) -> String {
    match value {
        Value::String(s) if quoted => {
            format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
        }
        Value::String(s) => s.clone(),
        // Emacs の書き方: 偽と空のリストは nil, 真は t
        Value::Boolean(false) | Value::Nil => "nil".to_string(),
        Value::Boolean(true) => "t".to_string(),
        Value::List(items) if items.is_empty() => "nil".to_string(),
        Value::List(items) => {
            let parts: Vec<String> = items.iter().map(|v| print_string(v, quoted)).collect();
            format!("({})", parts.join(" "))
        }
        other => other.to_string(),
    }
}

/// `format` の文字列を作ります. `%s` (文字列は引用符なし), `%S` (書いたとおり), `%d` (整数), `%o`, `%x`, `%X`, `%%` を使えます.
/// `%` の後ろに, 旗 `-` (左に寄せる) と `0` (0 で埋める) と, 幅を付けられます (`%3d`, `%-4s`, `%03d`).
///
/// # Arguments
///
/// * `args` - 書式の文字列と, 埋め込む値.
///
/// # Returns
///
/// 作った文字列. 書式が文字列でない, 値が足りない, 使えない指定がある場合は [`Err`].
fn format(args: &[Value]) -> Result<String, Error> {
    let Some((template, values)) = args.split_first() else {
        return Err("format expects a format string".to_string().into());
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
        // 旗 (`-` は左に寄せる, `0` は 0 で埋める) と, 幅
        let (mut left, mut zero, mut width) = (false, false, 0usize);
        let mut directive = chars.next();
        while let Some(flag @ ('-' | '0')) = directive {
            if flag == '-' {
                left = true;
            } else {
                zero = true;
            }
            directive = chars.next();
        }
        while let Some(digit) = directive.and_then(|d| d.to_digit(10)) {
            width = width * 10 + digit as usize;
            directive = chars.next();
        }
        if directive == Some('%') {
            result.push('%');
            continue;
        }
        let value = values
            .next()
            .ok_or_else(|| "format: not enough arguments".to_string())?;
        let (text, numeric) = match directive {
            Some('s') => (print_string(value, false), false),
            Some('S') => (print_string(value, true), false),
            Some('d') => (format!("{}", number("format", value)?.trunc() as i64), true),
            Some('o') => (
                format!("{:o}", number("format", value)?.trunc() as i64),
                true,
            ),
            Some('x') => (
                format!("{:x}", number("format", value)?.trunc() as i64),
                true,
            ),
            Some('X') => (
                format!("{:X}", number("format", value)?.trunc() as i64),
                true,
            ),
            _ => return Err("format: invalid directive".to_string().into()),
        };
        let pad = width.saturating_sub(text.chars().count());
        if left {
            result.push_str(&text);
            result.extend(std::iter::repeat_n(' ', pad));
        } else if zero && numeric {
            // 符号の後ろに 0 を入れる
            let (sign, digits) = text.strip_prefix('-').map_or(("", &text[..]), |d| ("-", d));
            result.push_str(sign);
            result.extend(std::iter::repeat_n('0', pad));
            result.push_str(digits);
        } else {
            result.extend(std::iter::repeat_n(' ', pad));
            result.push_str(&text);
        }
    }
    Ok(result)
}

/// `signal` の第 2 引数 (データのリスト) の要素.
fn error_items(data: &Value) -> Vec<Value> {
    match data {
        Value::List(items) => items.to_vec(),
        Value::Boolean(false) | Value::Nil => Vec::new(),
        other => vec![other.clone()],
    }
}
