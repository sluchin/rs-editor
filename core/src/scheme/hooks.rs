//! Emacs のフック (`add-hook`, `run-hooks` など).
//!
//! フックは, 手続き (または手続きの名前の記号) のリストを値に持つ変数です.
//! エディタは, ファイルを開いたとき (`find-file-hook`) と, 保存したとき (`after-save-hook`) に,
//! そのバッファで, フックを実行します ([`EDITOR_HOOKS`]).

use crate::scheme::builtins::fixed;
use crate::scheme::env::Env;
use crate::scheme::evaluator::Machine;
use crate::scheme::value::Value;

/// フックの組み込みの手続きの名前.
pub const NAMES: &[&str] = &["add-hook", "remove-hook", "run-hooks", "run-hook-with-args"];

/// エディタが実行するフック (評価器は, 始めに空のリストで定義する).
pub const EDITOR_HOOKS: &[&str] = &["find-file-hook", "after-save-hook"];

/// フックの手続きを返します.
///
/// # Arguments
///
/// * `global` - 一番外側の環境.
/// * `hook` - フックの名前.
///
/// # Returns
///
/// 手続き (記号のまま). 変数が無いか, `#f` か, 空なら, 空.
fn functions(global: &Env, hook: &str) -> Vec<Value> {
    match global.get(hook) {
        None | Some(Value::Boolean(false)) | Some(Value::Nil) => Vec::new(),
        Some(Value::List(items)) => items.to_vec(),
        Some(function) => vec![function],
    }
}

/// 値が入っているフックの名前を返します (エディタは, このフックだけを実行する).
///
/// # Arguments
///
/// * `global` - 一番外側の環境.
///
/// # Returns
///
/// [`EDITOR_HOOKS`] のうち, 手続きのあるもの.
pub(crate) fn active(global: &Env) -> Vec<String> {
    EDITOR_HOOKS
        .iter()
        .filter(|hook| !functions(global, hook).is_empty())
        .map(|hook| hook.to_string())
        .collect()
}

/// フックの名前 (記号) を取り出します.
///
/// # Arguments
///
/// * `name` - 手続きの名前 (失敗の内容に使う).
/// * `hook` - 値.
///
/// # Returns
///
/// 名前. 記号でなければ [`Err`].
fn hook_name(name: &str, hook: &Value) -> Result<String, String> {
    match hook {
        Value::Symbol(hook) => Ok(hook.clone()),
        other => Err(format!("{} expects a hook symbol, given {}", name, other)),
    }
}

/// フックの手続きを, 順に呼びます.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `hook` - フックの名前.
/// * `args` - 引数.
///
/// # Returns
///
/// 成功すれば `Ok`. 手続きが失敗すれば, その内容.
fn run(machine: &mut Machine, hook: &str, args: &[Value]) -> Result<(), String> {
    for function in functions(machine.evaluator.global(), hook) {
        // 記号は, その名前の手続き (呼ぶときに引く. 後で定義し直しても, 新しいものを呼ぶ)
        let procedure = match &function {
            Value::Symbol(name) => machine
                .evaluator
                .global()
                .get(name)
                .ok_or_else(|| format!("Unbound variable: {}", name))?,
            other => other.clone(),
        };
        machine.apply(procedure, args.to_vec())?;
    }
    Ok(())
}

/// フックの組み込みの手続きを呼びます.
///
/// # Arguments
///
/// * `machine` - 評価.
/// * `name` - 手続きの名前 ([`NAMES`] のどれか).
/// * `args` - 引数.
///
/// # Returns
///
/// 結果, または失敗の内容.
pub(crate) fn call(machine: &mut Machine, name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        // (add-hook 'hook f [append]): 無ければ, 前 (append なら後ろ) に足す
        "add-hook" => {
            let (hook, function, append) = match args.as_slice() {
                [hook, function] => (hook, function, false),
                [hook, function, append, ..] if args.len() <= 4 => {
                    (hook, function, append.is_non_nil())
                }
                _ => return Err("add-hook expects 2 to 4 arguments".to_string()),
            };
            let hook = hook_name(name, hook)?;
            let mut functions = functions(machine.evaluator.global(), &hook);
            if !functions.contains(function) {
                if append {
                    functions.push(function.clone());
                } else {
                    functions.insert(0, function.clone());
                }
            }
            machine
                .evaluator
                .global()
                .define(&hook, Value::List(functions.into()));
            Ok(Value::Nil)
        }
        "remove-hook" => {
            let [hook, function] = fixed::<2>(name, args)?;
            let hook = hook_name(name, &hook)?;
            let mut functions = functions(machine.evaluator.global(), &hook);
            functions.retain(|f| *f != function);
            machine
                .evaluator
                .global()
                .define(&hook, Value::List(functions.into()));
            Ok(Value::Nil)
        }
        "run-hooks" => {
            for hook in &args {
                run(machine, &hook_name(name, hook)?, &[])?;
            }
            Ok(Value::Nil)
        }
        _ => {
            // run-hook-with-args
            let Some((hook, rest)) = args.split_first() else {
                return Err("run-hook-with-args expects a hook".to_string());
            };
            run(machine, &hook_name(name, hook)?, rest)?;
            Ok(Value::Nil)
        }
    }
}
