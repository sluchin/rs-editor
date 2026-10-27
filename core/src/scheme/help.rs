//! ヘルプの組み込みの手続き (`documentation`, `help-function-arglist`, `mapatoms` など).
//!
//! docstring は, 変数 `help-language` の言語の翻訳があれば, 翻訳を返します ([`po`]).
//! 元の docstring (英語) は, 引数 RAW を真にすると返します.

use crate::scheme::builtins::fixed;
use crate::scheme::evaluator::Machine;
use crate::scheme::po;
use crate::scheme::value::Value;

/// ヘルプの組み込みの手続きの名前.
pub const NAMES: &[&str] = &[
    "documentation",
    "documentation-property",
    "help-function-arglist",
    "commandp",
    "fboundp",
    "symbol-file",
    "mapatoms",
];

/// 手続きか, 手続きの名前の記号から, 手続きと, その名前を返します.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `function` - 手続きか, 名前の記号.
///
/// # Returns
///
/// 手続きと, 名前 (無名の手続きは [`None`]). 記号に値が無ければ [`Err`].
fn resolve(machine: &Machine, function: Value) -> Result<(Value, Option<String>), String> {
    match function {
        Value::Symbol(symbol) => {
            let value = machine
                .evaluator
                .global()
                .get(&symbol)
                .ok_or_else(|| format!("Symbol's function definition is void: {}", symbol))?;
            Ok((value, Some(symbol)))
        }
        Value::Lambda(ref lambda) => {
            let name = lambda.name.clone();
            Ok((function, name))
        }
        Value::Function(ref builtin) => {
            let name = Some(builtin.clone());
            Ok((function, name))
        }
        other => Ok((other, None)),
    }
}

/// docstring を, `help-language` の言語に訳します (翻訳が無いか, RAW なら, そのまま).
///
/// # Arguments
///
/// * `machine` - 評価 (変数 `help-language` を見る).
/// * `context` - `function:名前` か `variable:名前` (名前が無ければ [`None`]).
/// * `doc` - 元の docstring.
/// * `raw` - 訳さないか.
///
/// # Returns
///
/// docstring.
fn translate(machine: &Machine, context: Option<String>, doc: &str, raw: bool) -> Value {
    let language = match machine.evaluator.global().get("help-language") {
        Some(Value::String(language)) => language,
        _ => String::new(),
    };
    let translated = context
        .filter(|_| !raw)
        .and_then(|context| po::translate(&language, &context, doc));
    Value::String(translated.unwrap_or(doc).to_string())
}

/// 省ける, 真偽の引数 (省くか, `#f` か, nil なら偽).
///
/// # Arguments
///
/// * `v` - 引数.
///
/// # Returns
///
/// 真偽.
fn flag(v: Option<&Value>) -> bool {
    v.is_some_and(Value::is_non_nil)
}

/// ヘルプの組み込みの手続きを呼びます.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 手続きの名前 ([`NAMES`] の 1 つ).
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub(crate) fn call(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "documentation" => {
            // (documentation FUNCTION &optional RAW). docstring が無ければ #f
            let (function, raw) = match args.as_slice() {
                [function] => (function.clone(), false),
                [function, raw] => (function.clone(), flag(Some(raw))),
                _ => return Err("documentation expects 1 or 2 arguments".to_string()),
            };
            match resolve(machine, function)? {
                (Value::Lambda(lambda), name) => {
                    Ok(lambda.docstring().map_or(Value::Boolean(false), |doc| {
                        translate(machine, name.map(|n| format!("function:{}", n)), doc, raw)
                    }))
                }
                (Value::Function(_), _) => Ok(Value::Boolean(false)),
                (other, _) => Err(format!("Invalid function: {}", other)),
            }
        }
        "documentation-property" => {
            // (documentation-property SYMBOL PROP &optional RAW). 変数の docstring だけ (variable-documentation)
            let (symbol, property, raw) = match args.as_slice() {
                [symbol, property] => (symbol, property, false),
                [symbol, property, raw] => (symbol, property, flag(Some(raw))),
                _ => return Err("documentation-property expects 2 or 3 arguments".to_string()),
            };
            match (symbol, property) {
                (Value::Symbol(symbol), Value::Symbol(property)) => Ok(
                    match (
                        property.as_str(),
                        machine.evaluator.variable_documentation(symbol),
                    ) {
                        ("variable-documentation", Some(doc)) => {
                            translate(machine, Some(format!("variable:{}", symbol)), &doc, raw)
                        }
                        _ => Value::Boolean(false),
                    },
                ),
                _ => Err("documentation-property expects symbols".to_string()),
            }
        }
        "help-function-arglist" => {
            // 引数の名前の記号のリスト (残りの引数は &rest の後). 組み込みの手続きは, 分からないので #t
            let [function] = fixed::<1>(name, args)?;
            match resolve(machine, function)? {
                (Value::Lambda(lambda), _) => {
                    let mut params: Vec<Value> =
                        lambda.params.iter().cloned().map(Value::Symbol).collect();
                    if let Some(rest) = &lambda.rest {
                        params.push(Value::Symbol("&rest".to_string()));
                        params.push(Value::Symbol(rest.clone()));
                    }
                    Ok(Value::List(params.into()))
                }
                (Value::Function(_), _) => Ok(Value::Boolean(true)),
                (other, _) => Err(format!("Invalid function: {}", other)),
            }
        }
        "commandp" => {
            // Lisp で作ったコマンド (define-command) か. 画面のコマンドは, editor-command-names で見る
            let [function] = fixed::<1>(name, args)?;
            let name = match function {
                Value::Symbol(symbol) => Some(symbol),
                Value::Lambda(lambda) => lambda.name.clone(),
                _ => None,
            };
            Ok(Value::Boolean(name.is_some_and(|name| {
                machine.evaluator.commands().contains(&name)
            })))
        }
        "fboundp" => {
            // 一番外側の環境で, 手続きの名前か
            let [symbol] = fixed::<1>(name, args)?;
            let Value::Symbol(symbol) = symbol else {
                return Err("fboundp expects a symbol".to_string());
            };
            Ok(Value::Boolean(matches!(
                machine.evaluator.global().get(&symbol),
                Some(Value::Lambda(_) | Value::Function(_))
            )))
        }
        "symbol-file" => {
            // 定義を読んだファイル. 分からなければ #f
            let symbol = match args.as_slice() {
                [Value::Symbol(symbol)] | [Value::Symbol(symbol), _] => symbol,
                _ => return Err("symbol-file expects a symbol".to_string()),
            };
            Ok(machine
                .evaluator
                .symbol_file(symbol)
                .map_or(Value::Boolean(false), Value::String))
        }
        _ => {
            // mapatoms: 一番外側の環境の全ての名前 (記号) で, 手続きを呼ぶ (名前の順)
            let [function] = fixed::<1>(name, args)?;
            for symbol in machine.evaluator.global().names() {
                machine.apply(function.clone(), vec![Value::Symbol(symbol)])?;
            }
            Ok(Value::Nil)
        }
    }
}
