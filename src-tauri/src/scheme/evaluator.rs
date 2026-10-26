//! Scheme の評価器. 定義 (`define`) は, アプリが持ち続ける評価器の環境に覚えます.
//!
//! - 特別な形: `quote` (`'`), `if`, `define`, `set!`, `lambda`, `begin`, `let` (名前付きも), `let*`, `letrec`,
//!   `cond`, `case`, `and`, `or`, `when`, `unless`, `unwind-protect`, `define-command`.
//! - テスト ([`ert`]): `ert-deftest`, `should`, `should-not`, `should-error`, `skip-unless`.
//! - 組み込みの手続きは [`builtins`].
//! - 末尾の呼び出しは, 深さを使いません. それ以外の入れ子は, [`MAX_DEPTH`] までです.

use std::sync::{Arc, Mutex, MutexGuard};

use serde::{Deserialize, Serialize};

use crate::scheme::builtins;
use crate::scheme::env::Env;
use crate::scheme::ert::{self, ErtTest};
use crate::scheme::hooks;
use crate::scheme::host::{Host, NoHost};
use crate::scheme::parser::Parser;
use crate::scheme::system;
use crate::scheme::value::{Lambda, Value};

/// 評価の入れ子の深さの上限.
pub const MAX_DEPTH: usize = 10_000;

/// 評価に使うスレッドのスタックの大きさ ([`MAX_DEPTH`] まで入れ子にできるように, 大きくする).
const STACK_SIZE: usize = 256 * 1024 * 1024;

/// Lisp で割り当てたキー (`global-set-key`).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyBinding {
    /// キー (画面のエディタ (CodeMirror) の書き方).
    pub key: String,
    /// コマンドの名前.
    pub command: String,
}

/// 評価器. 定義と, Lisp で作ったコマンドとキーの割り当てを, 覚えています.
pub struct Evaluator {
    /// 一番外側の環境 (組み込みの手続きと, `define` した変数).
    global: Env,
    /// Lisp で作ったコマンドの名前 (作った順).
    commands: Mutex<Vec<String>>,
    /// Lisp で割り当てたキー (割り当てた順).
    keys: Mutex<Vec<KeyBinding>>,
    /// 評価中は, 持ち続けるロック (評価は, 1 度に 1 つ).
    running: Mutex<()>,
    /// `provide` した機能の名前 (`require` で, 2 度読まない).
    features: Mutex<Vec<String>>,
    /// `ert-deftest` で定義したテスト (定義した順).
    tests: Mutex<Vec<ErtTest>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    /// 組み込みの手続きを定義した評価器を作ります.
    ///
    /// # Returns
    ///
    /// 評価器.
    pub fn new() -> Self {
        let global = Env::new();
        for name in builtins::NAMES
            .iter()
            .chain(system::NAMES)
            .chain(ert::NAMES)
            .chain(hooks::NAMES)
        {
            global.define(name, Value::Function(name.to_string()));
        }
        // load と require が, ファイルを探すディレクトリ
        global.define(
            "load-path",
            Value::List(
                system::DEFAULT_LOAD_PATH
                    .iter()
                    .map(|dir| Value::String(dir.to_string()))
                    .collect(),
            ),
        );
        // 読んだ初期設定ファイル (読む前と, 無いときは #f)
        global.define("user-init-file", Value::Boolean(false));
        // エディタが実行するフック (find-file-hook, after-save-hook)
        for hook in hooks::EDITOR_HOOKS {
            global.define(hook, Value::List(Vec::new().into()));
        }
        let evaluator = Evaluator {
            global,
            commands: Mutex::new(Vec::new()),
            keys: Mutex::new(Vec::new()),
            running: Mutex::new(()),
            // テスト (ERT) は, require しなくても使える
            features: Mutex::new(vec!["ert".to_string()]),
            tests: Mutex::new(Vec::new()),
        };
        // M-x ert
        let mut machine = Machine::new(&evaluator, &NoHost);
        for expr in ert::prelude() {
            machine
                .eval(&expr, &evaluator.global)
                .expect("the ERT prelude evaluates");
        }
        evaluator
    }

    /// コードを評価します (エディタの操作は, 失敗します).
    ///
    /// # Arguments
    ///
    /// * `code` - コード.
    ///
    /// # Returns
    ///
    /// 式ごとの結果 (改行で区切る), または失敗の内容.
    pub fn eval(&self, code: &str) -> Result<String, String> {
        self.eval_with(code, &NoHost)
    }

    /// コードを評価します.
    ///
    /// # Arguments
    ///
    /// * `code` - コード.
    /// * `host` - エディタ.
    ///
    /// # Returns
    ///
    /// 式ごとの結果 (改行で区切る), または失敗の内容.
    pub fn eval_with(&self, code: &str, host: &dyn Host) -> Result<String, String> {
        // 読むのも, 大きなスタックのスレッドで (パーサは, 入れ子を再帰で読むため)
        self.run(host, |machine| {
            let values = Parser::new(code).parse()?;
            let mut results = Vec::new();
            for value in &values {
                results.push(machine.eval(value, &self.global)?.to_string());
            }
            Ok(results.join("\n"))
        })
    }

    /// Lisp で作ったコマンド (`define-command`) を実行します.
    ///
    /// # Arguments
    ///
    /// * `name` - コマンドの名前.
    /// * `host` - エディタ.
    ///
    /// # Returns
    ///
    /// 結果, または失敗の内容.
    pub fn run_command(&self, name: &str, host: &dyn Host) -> Result<String, String> {
        if !self.is_command(name) {
            return Err(format!("Undefined command: {}", name));
        }
        self.run(host, |machine| {
            machine.call_command(name).map(|value| value.to_string())
        })
    }

    /// Lisp で作ったコマンドの名前を返します.
    ///
    /// # Returns
    ///
    /// 名前 (作った順).
    pub fn commands(&self) -> Vec<String> {
        self.commands.lock().unwrap().clone()
    }

    /// Lisp で割り当てたキーを返します.
    ///
    /// # Returns
    ///
    /// 割り当て (割り当てた順).
    pub fn key_bindings(&self) -> Vec<KeyBinding> {
        self.keys.lock().unwrap().clone()
    }

    /// 評価で作って, まだ放していない環境の数を返します (評価のあとに, 環境を放したかを確かめる).
    ///
    /// # Returns
    ///
    /// 環境の数.
    pub fn live_frames(&self) -> usize {
        self.global.live_frames()
    }

    /// 手続きのある, エディタが実行するフック (`find-file-hook` など) の名前を返します.
    ///
    /// # Returns
    ///
    /// 名前.
    pub fn active_hooks(&self) -> Vec<String> {
        hooks::active(&self.global)
    }

    /// 一番外側の環境を返します (`load` で使う).
    ///
    /// # Returns
    ///
    /// 環境.
    pub(crate) fn global(&self) -> &Env {
        &self.global
    }

    /// 機能を覚えます (Emacs の `provide`. 覚えていれば, 何もしない).
    ///
    /// # Arguments
    ///
    /// * `feature` - 機能の名前.
    pub(crate) fn provide(&self, feature: &str) {
        let mut features = self.features.lock().unwrap();
        if !features.iter().any(|f| f == feature) {
            features.push(feature.to_string());
        }
    }

    /// 機能を `provide` したかを返します.
    ///
    /// # Arguments
    ///
    /// * `feature` - 機能の名前.
    ///
    /// # Returns
    ///
    /// `provide` していれば `true`.
    pub(crate) fn featurep(&self, feature: &str) -> bool {
        self.features.lock().unwrap().iter().any(|f| f == feature)
    }

    /// `ert-deftest` で定義したテストを返します.
    ///
    /// # Returns
    ///
    /// テスト (定義した順. 書き換えられる).
    pub(crate) fn tests(&self) -> MutexGuard<'_, Vec<ErtTest>> {
        self.tests.lock().unwrap()
    }

    /// コマンドとして覚えます (覚えていれば, 何もしない).
    ///
    /// # Arguments
    ///
    /// * `name` - コマンドの名前.
    pub(crate) fn add_command(&self, name: &str) {
        let mut commands = self.commands.lock().unwrap();
        if !commands.iter().any(|c| c == name) {
            commands.push(name.to_string());
        }
    }

    /// Lisp で作ったコマンドかを返します.
    ///
    /// # Arguments
    ///
    /// * `name` - コマンドの名前.
    ///
    /// # Returns
    ///
    /// Lisp で作ったコマンドなら `true`.
    pub(crate) fn is_command(&self, name: &str) -> bool {
        self.commands.lock().unwrap().iter().any(|c| c == name)
    }

    /// キーを割り当てます (同じキーの割り当ては, 置き換える).
    ///
    /// # Arguments
    ///
    /// * `binding` - 割り当て.
    pub(crate) fn set_key(&self, binding: KeyBinding) {
        let mut keys = self.keys.lock().unwrap();
        keys.retain(|k| k.key != binding.key);
        keys.push(binding);
    }

    /// 大きなスタックのスレッドで, 評価します. 前の評価が終わるまで待ちます.
    ///
    /// # Arguments
    ///
    /// * `host` - エディタ.
    /// * `task` - 評価する処理.
    ///
    /// # Returns
    ///
    /// `task` の結果. スレッドを作れないか, 評価が panic した場合は [`Err`].
    fn run<T: Send>(
        &self,
        host: &dyn Host,
        task: impl FnOnce(&mut Machine) -> Result<T, String> + Send,
    ) -> Result<T, String> {
        // 前の評価が失敗 (panic) していても, 続ける
        let _running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn_scoped(scope, || {
                    let result = task(&mut Machine::new(self, host));
                    // 評価で作った, 環境と手続きの輪を切る (放すと深く入れ子になるので, 大きなスタックで)
                    self.global.collect();
                    result
                })
                .map_err(|e| e.to_string())?
                .join()
                .unwrap_or_else(|_| Err("The evaluator crashed".to_string()))
        })
    }
}

/// 評価の 1 歩の結果.
enum Step {
    /// 値が決まった.
    Done(Value),
    /// この式を, この環境で評価した値が, 結果 (末尾の呼び出し).
    Tail(Value, Env),
}

/// 1 回の評価 (入れ子の深さと, エディタを持つ).
pub(crate) struct Machine<'a> {
    /// 評価器 (定義とコマンドとキーを覚えている).
    pub(crate) evaluator: &'a Evaluator,
    /// エディタ (エディタの操作を頼む先).
    pub(crate) host: &'a dyn Host,
    /// 今の評価の入れ子の深さ ([`MAX_DEPTH`] まで).
    depth: usize,
    /// 読み込んでいるファイル (`load`) のディレクトリ (入れ子の順. 相対パスの基).
    pub(crate) load_dirs: Vec<std::path::PathBuf>,
}

impl<'a> Machine<'a> {
    /// 入れ子の深さが 0 の, 1 回の評価を作ります.
    ///
    /// # Arguments
    ///
    /// * `evaluator` - 評価器.
    /// * `host` - エディタ.
    ///
    /// # Returns
    ///
    /// 1 回の評価.
    fn new(evaluator: &'a Evaluator, host: &'a dyn Host) -> Self {
        Machine {
            evaluator,
            host,
            depth: 0,
            load_dirs: Vec::new(),
        }
    }

    /// パスを, 絶対パスにします. 相対パスは, 読み込んでいるファイル (`load`) のディレクトリ, またはホームディレクトリから数えます.
    ///
    /// # Arguments
    ///
    /// * `path` - パス (`~` も使える).
    ///
    /// # Returns
    ///
    /// 絶対パス.
    pub(crate) fn expand_file_name(&self, path: &str) -> String {
        let home = dirs::home_dir().unwrap_or_default();
        let base = self.load_dirs.last().unwrap_or(&home);
        system::expand_file_name(path, base, &home)
            .to_string_lossy()
            .into_owned()
    }

    /// 式を評価します.
    ///
    /// # Arguments
    ///
    /// * `expr` - 式.
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 値. 失敗した場合 (入れ子が深すぎる場合も) は, その内容の [`Err`].
    pub(crate) fn eval(&mut self, expr: &Value, env: &Env) -> Result<Value, String> {
        // リストでない式は, その場で値にする (式を写さない)
        if !matches!(expr, Value::List(_)) {
            return eval_atom(expr, env);
        }
        if self.depth >= MAX_DEPTH {
            return Err("Too deep recursion".to_string());
        }
        self.depth += 1;
        let result = self.eval_tail(expr.clone(), env.clone());
        self.depth -= 1;
        result
    }

    /// 式を評価します. 末尾の式は, ここで繰り返して評価します (深さを使わない).
    ///
    /// # Arguments
    ///
    /// * `expr` - 式.
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 値, または失敗の内容.
    fn eval_tail(&mut self, mut expr: Value, mut env: Env) -> Result<Value, String> {
        loop {
            let items = match &expr {
                Value::List(items) if !items.is_empty() => items.clone(),
                atom => return eval_atom(atom, &env),
            };
            let special = match &items[0] {
                Value::Symbol(op) => self.special_form(op, &items[1..], &env)?,
                _ => None,
            };
            let step = match special {
                Some(step) => step,
                None => {
                    let procedure = self.eval(&items[0], &env)?;
                    let args = self.eval_all(&items[1..], &env)?;
                    self.apply_step(procedure, args)?
                }
            };
            match step {
                Step::Done(value) => return Ok(value),
                Step::Tail(next, next_env) => {
                    expr = next;
                    env = next_env;
                }
            }
        }
    }

    /// 式を順に評価して, 値を並べます.
    ///
    /// # Arguments
    ///
    /// * `exprs` - 式.
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 値 (式の順). 1 つでも失敗した場合は, その内容の [`Err`].
    fn eval_all(&mut self, exprs: &[Value], env: &Env) -> Result<Vec<Value>, String> {
        exprs.iter().map(|e| self.eval(e, env)).collect()
    }

    /// 手続きを呼びます.
    ///
    /// # Arguments
    ///
    /// * `procedure` - 手続き.
    /// * `args` - 引数.
    ///
    /// # Returns
    ///
    /// 結果, または失敗の内容.
    pub(crate) fn apply(&mut self, procedure: Value, args: Vec<Value>) -> Result<Value, String> {
        match self.apply_step(procedure, args)? {
            Step::Done(value) => Ok(value),
            Step::Tail(expr, env) => self.eval(&expr, &env),
        }
    }

    /// 手続きを呼びます. `lambda` の本体の最後の式は, 評価せずに返します.
    ///
    /// # Arguments
    ///
    /// * `procedure` - 手続き.
    /// * `args` - 引数.
    ///
    /// # Returns
    ///
    /// 評価の 1 歩の結果. 手続きでない値の場合や, 引数の数が違う場合は [`Err`].
    fn apply_step(&mut self, procedure: Value, args: Vec<Value>) -> Result<Step, String> {
        match procedure {
            Value::Lambda(lambda) => {
                let env = bind(&lambda, args)?;
                self.sequence(&lambda.body, &env)
            }
            Value::Function(name) => builtins::call(self, &name, args).map(Step::Done),
            other => Err(format!("Not a procedure: {}", other)),
        }
    }

    /// コマンドを実行します. Lisp で作ったコマンドは, ここで呼び, ほかは, エディタに頼みます.
    ///
    /// # Arguments
    ///
    /// * `name` - コマンドの名前.
    ///
    /// # Returns
    ///
    /// 結果, または失敗の内容.
    pub(crate) fn call_command(&mut self, name: &str) -> Result<Value, String> {
        if self.evaluator.is_command(name) {
            let procedure = self
                .evaluator
                .global
                .get(name)
                .ok_or_else(|| format!("Unbound variable: {}", name))?;
            return self.apply(procedure, vec![]);
        }
        self.host
            .call("command", vec![Value::String(name.to_string())])
    }

    /// 式を順に評価します. 最後の式は, 評価せずに返します (空なら `nil`).
    ///
    /// # Arguments
    ///
    /// * `body` - 式.
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 評価の 1 歩の結果, または失敗の内容.
    fn sequence(&mut self, body: &[Value], env: &Env) -> Result<Step, String> {
        match body.split_last() {
            None => Ok(Step::Done(Value::Nil)),
            Some((last, init)) => {
                for expr in init {
                    self.eval(expr, env)?;
                }
                Ok(Step::Tail(last.clone(), env.clone()))
            }
        }
    }

    /// 特別な形を評価します.
    ///
    /// # Arguments
    ///
    /// * `op` - 形の名前 (`if` など).
    /// * `args` - 形の名前の後の, 評価していない式.
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 評価の 1 歩の結果. 特別な形でなければ `None`. 書き方が誤っている場合は [`Err`].
    fn special_form(
        &mut self,
        op: &str,
        args: &[Value],
        env: &Env,
    ) -> Result<Option<Step>, String> {
        let step = match op {
            "quote" => match args {
                [value] => Step::Done(value.clone()),
                _ => return Err("quote expects 1 argument".to_string()),
            },
            "if" => match args {
                [test, then, rest @ ..] if rest.len() <= 1 => {
                    if self.eval(test, env)?.is_true() {
                        Step::Tail(then.clone(), env.clone())
                    } else {
                        match rest.first() {
                            Some(otherwise) => Step::Tail(otherwise.clone(), env.clone()),
                            None => Step::Done(Value::Nil),
                        }
                    }
                }
                _ => return Err("if expects 2 or 3 arguments".to_string()),
            },
            "define" => Step::Done(Value::Symbol(self.define(args, env)?)),
            "define-command" => {
                let name = self.define(args, env)?;
                match env.get(&name) {
                    Some(Value::Lambda(_) | Value::Function(_)) => {
                        self.evaluator.add_command(&name);
                        Step::Done(Value::Symbol(name))
                    }
                    _ => return Err("define-command expects a procedure".to_string()),
                }
            }
            "set!" => match args {
                [Value::Symbol(name), expr] => {
                    let value = self.eval(expr, env)?;
                    if !env.set(name, value) {
                        return Err(format!("Unbound variable: {}", name));
                    }
                    Step::Done(Value::Nil)
                }
                _ => return Err("set! expects a name and a value".to_string()),
            },
            "lambda" => match args {
                [params, body @ ..] => Step::Done(make_lambda(None, params, body, env)?),
                _ => return Err("lambda expects parameters".to_string()),
            },
            "begin" => self.sequence(args, env)?,
            "let" => match args {
                [Value::Symbol(name), bindings, body @ ..] => {
                    self.named_let(name, bindings, body, env)?
                }
                [bindings, body @ ..] => {
                    let pairs = parse_bindings("let", bindings)?;
                    let inner = env.child();
                    for (name, expr) in pairs {
                        let value = self.eval(&expr, env)?;
                        inner.define(&name, value);
                    }
                    self.sequence(body, &inner)?
                }
                _ => return Err("let expects bindings".to_string()),
            },
            "let*" | "letrec" | "letrec*" => match args {
                [bindings, body @ ..] => {
                    let pairs = parse_bindings(op, bindings)?;
                    let inner = env.child();
                    // letrec では, 初めの式から, 全ての名前が見える
                    if op != "let*" {
                        for (name, _) in &pairs {
                            inner.define(name, Value::Nil);
                        }
                    }
                    for (name, expr) in pairs {
                        let value = self.eval(&expr, &inner)?;
                        inner.define(&name, name_lambda(value, &name));
                    }
                    self.sequence(body, &inner)?
                }
                _ => return Err(format!("{} expects bindings", op)),
            },
            "cond" => self.cond(args, env)?,
            "case" => match args {
                [key, clauses @ ..] => {
                    let key = self.eval(key, env)?;
                    self.case(&key, clauses, env)?
                }
                _ => return Err("case expects a key".to_string()),
            },
            "and" => match args.split_last() {
                None => Step::Done(Value::Boolean(true)),
                Some((last, init)) => {
                    for expr in init {
                        let value = self.eval(expr, env)?;
                        if !value.is_true() {
                            return Ok(Some(Step::Done(value)));
                        }
                    }
                    Step::Tail(last.clone(), env.clone())
                }
            },
            "or" => match args.split_last() {
                None => Step::Done(Value::Boolean(false)),
                Some((last, init)) => {
                    for expr in init {
                        let value = self.eval(expr, env)?;
                        if value.is_true() {
                            return Ok(Some(Step::Done(value)));
                        }
                    }
                    Step::Tail(last.clone(), env.clone())
                }
            },
            "when" | "unless" => match args {
                [test, body @ ..] => {
                    if self.eval(test, env)?.is_true() == (op == "when") {
                        self.sequence(body, env)?
                    } else {
                        Step::Done(Value::Nil)
                    }
                }
                _ => return Err(format!("{} expects a test", op)),
            },
            "unwind-protect" => match args {
                [body, cleanup @ ..] => {
                    // 本体が失敗しても, 後始末を評価する
                    let result = self.eval(body, env);
                    for expr in cleanup {
                        self.eval(expr, env)?;
                    }
                    Step::Done(result?)
                }
                _ => return Err("unwind-protect expects a body form".to_string()),
            },
            _ => return Ok(ert::special_form(self, op, args, env)?.map(Step::Done)),
        };
        Ok(Some(step))
    }

    /// `define` (と `define-command`) を評価して, 定義した名前を返します.
    ///
    /// # Arguments
    ///
    /// * `args` - `define` の後の式 (名前と値, または `(名前 引数...)` と本体).
    /// * `env` - 定義する環境.
    ///
    /// # Returns
    ///
    /// 定義した名前, または失敗の内容.
    fn define(&mut self, args: &[Value], env: &Env) -> Result<String, String> {
        match args {
            [Value::Symbol(name), rest @ ..] if rest.len() <= 1 => {
                let value = match rest.first() {
                    Some(expr) => self.eval(expr, env)?,
                    None => Value::Nil,
                };
                env.define(name, name_lambda(value, name));
                Ok(name.clone())
            }
            [Value::List(signature), body @ ..] => match signature.split_first() {
                Some((Value::Symbol(name), params)) => {
                    let lambda = make_lambda(Some(name), &Value::List(params.into()), body, env)?;
                    env.define(name, lambda);
                    Ok(name.clone())
                }
                _ => Err("define expects a name".to_string()),
            },
            _ => Err("define expects a name and a value".to_string()),
        }
    }

    /// 名前付きの `let` (`(let loop ((i 0)) ...)`) を評価します.
    ///
    /// # Arguments
    ///
    /// * `name` - 名前 (本体から呼べる手続きの名前).
    /// * `bindings` - 束縛 (`((i 0))`).
    /// * `body` - 本体.
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 評価の 1 歩の結果, または失敗の内容.
    fn named_let(
        &mut self,
        name: &str,
        bindings: &Value,
        body: &[Value],
        env: &Env,
    ) -> Result<Step, String> {
        let pairs = parse_bindings("let", bindings)?;
        let (names, exprs): (Vec<Value>, Vec<Value>) = pairs
            .into_iter()
            .map(|(name, expr)| (Value::Symbol(name), expr))
            .unzip();
        let inner = env.child();
        let lambda = make_lambda(Some(name), &Value::List(names.into()), body, &inner)?;
        inner.define(name, lambda.clone());
        let args = self.eval_all(&exprs, env)?;
        self.apply_step(lambda, args)
    }

    /// `cond` を評価します.
    ///
    /// # Arguments
    ///
    /// * `clauses` - 節 (`(test body...)`. `else` も使える).
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 評価の 1 歩の結果 (当てはまる節が無ければ `nil`), または失敗の内容.
    fn cond(&mut self, clauses: &[Value], env: &Env) -> Result<Step, String> {
        for clause in clauses {
            let Value::List(items) = clause else {
                return Err("cond expects clauses".to_string());
            };
            let Some((test, body)) = items.split_first() else {
                return Err("cond expects clauses".to_string());
            };
            if *test == Value::Symbol("else".to_string()) {
                return self.sequence(body, env);
            }
            let value = self.eval(test, env)?;
            if value.is_true() {
                return match body {
                    [] => Ok(Step::Done(value)),
                    _ => self.sequence(body, env),
                };
            }
        }
        Ok(Step::Done(Value::Nil))
    }

    /// `case` を評価します.
    ///
    /// # Arguments
    ///
    /// * `key` - 比べる値 (評価した値).
    /// * `clauses` - 節 (`((data...) body...)`. `else` も使える).
    /// * `env` - 評価する環境.
    ///
    /// # Returns
    ///
    /// 評価の 1 歩の結果 (当てはまる節が無ければ `nil`), または失敗の内容.
    fn case(&mut self, key: &Value, clauses: &[Value], env: &Env) -> Result<Step, String> {
        for clause in clauses {
            match clause {
                Value::List(items) => match items.split_first() {
                    Some((Value::Symbol(word), body)) if word == "else" => {
                        return self.sequence(body, env)
                    }
                    Some((Value::List(data), body)) => {
                        if data.contains(key) {
                            return self.sequence(body, env);
                        }
                    }
                    _ => return Err("case expects clauses".to_string()),
                },
                _ => return Err("case expects clauses".to_string()),
            }
        }
        Ok(Step::Done(Value::Nil))
    }
}

/// `lambda` を作ります.
///
/// # Arguments
///
/// * `name` - 名前 (無名なら [`None`]).
/// * `params` - 引数のリスト (`(a b)`, `(a . rest)`, または残りの全てを受ける記号).
/// * `body` - 本体.
/// * `env` - 作ったときの環境.
///
/// # Returns
///
/// 手続き. 引数のリストが誤っている場合は [`Err`].
pub(crate) fn make_lambda(
    name: Option<&str>,
    params: &Value,
    body: &[Value],
    env: &Env,
) -> Result<Value, String> {
    let (names, rest) = match params {
        Value::Symbol(rest) => (vec![], Some(rest.clone())),
        Value::List(items) => {
            let mut names = Vec::new();
            let mut rest = None;
            let mut iter = items.iter();
            while let Some(item) = iter.next() {
                match item {
                    Value::Symbol(dot) if dot == "." => match (iter.next(), iter.next()) {
                        (Some(Value::Symbol(last)), None) => rest = Some(last.clone()),
                        _ => return Err("Invalid parameter list".to_string()),
                    },
                    Value::Symbol(param) => names.push(param.clone()),
                    _ => return Err("Invalid parameter list".to_string()),
                }
            }
            (names, rest)
        }
        _ => return Err("Invalid parameter list".to_string()),
    };
    Ok(Value::Lambda(Arc::new(Lambda {
        name: name.map(str::to_string),
        params: names,
        rest,
        body: body.to_vec(),
        env: env.clone(),
    })))
}

/// リストでない式 (と, 空のリスト) を評価します. 記号は変数の値 (`:` で始まる記号は, そのまま), ほかは, それ自身です.
///
/// # Arguments
///
/// * `expr` - 式.
/// * `env` - 評価する環境.
///
/// # Returns
///
/// 値. 変数が無ければ [`Err`].
fn eval_atom(expr: &Value, env: &Env) -> Result<Value, String> {
    match expr {
        Value::Symbol(name) if !name.starts_with(':') => env
            .get(name)
            .ok_or_else(|| format!("Unbound variable: {}", name)),
        other => Ok(other.clone()),
    }
}

/// 名前のない `lambda` に, 名前を付けます (表示のため). ほかの値は, そのまま返します.
///
/// # Arguments
///
/// * `value` - 値.
/// * `name` - 名前.
///
/// # Returns
///
/// 名前を付けた値.
fn name_lambda(value: Value, name: &str) -> Value {
    match value {
        Value::Lambda(lambda) if lambda.name.is_none() => Value::Lambda(Arc::new(Lambda {
            name: Some(name.to_string()),
            params: lambda.params.clone(),
            rest: lambda.rest.clone(),
            body: lambda.body.clone(),
            env: lambda.env.clone(),
        })),
        other => other,
    }
}

/// 引数を, `lambda` の環境に結び付けます.
///
/// # Arguments
///
/// * `lambda` - 手続き.
/// * `args` - 引数.
///
/// # Returns
///
/// 引数を定義した, 本体を評価する環境. 引数の数が違う場合は [`Err`].
fn bind(lambda: &Lambda, args: Vec<Value>) -> Result<Env, String> {
    let count = lambda.params.len();
    let fits = match lambda.rest {
        Some(_) => args.len() >= count,
        None => args.len() == count,
    };
    if !fits {
        let name = lambda.name.as_deref().unwrap_or("#<procedure>");
        let at_least = if lambda.rest.is_some() {
            "at least "
        } else {
            ""
        };
        return Err(format!(
            "{} expects {}{} arguments, given {}",
            name,
            at_least,
            count,
            args.len()
        ));
    }
    let env = lambda.env.child();
    let mut args = args.into_iter();
    for param in &lambda.params {
        // 数は, 上で確かめた
        env.define(param, args.next().unwrap_or(Value::Nil));
    }
    if let Some(rest) = &lambda.rest {
        env.define(rest, Value::List(args.collect()));
    }
    Ok(env)
}

/// `let` の束縛 (`((a 1) (b 2))`) を読みます.
///
/// # Arguments
///
/// * `form` - 形の名前 (失敗の内容に書く).
/// * `bindings` - 束縛.
///
/// # Returns
///
/// 名前と式の組 (書いた順). 書き方が誤っている場合は [`Err`].
fn parse_bindings(form: &str, bindings: &Value) -> Result<Vec<(String, Value)>, String> {
    let invalid = || format!("{} expects bindings", form);
    let Value::List(items) = bindings else {
        return Err(invalid());
    };
    items
        .iter()
        .map(|item| match item {
            Value::List(pair) => match &**pair {
                [Value::Symbol(name), expr] => Ok((name.clone(), expr.clone())),
                _ => Err(invalid()),
            },
            _ => Err(invalid()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_addition() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(+ 1 2 3)").unwrap(), "6");
    }

    #[test]
    fn evaluates_subtraction() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(- 10 3)").unwrap(), "7");
    }

    #[test]
    fn evaluates_unary_minus_as_negation() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(- 5)").unwrap(), "-5");
    }

    #[test]
    fn evaluates_multiplication() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(* 2 3 4)").unwrap(), "24");
    }

    #[test]
    fn evaluates_division() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(/ 20 4)").unwrap(), "5");
    }

    #[test]
    fn division_by_zero_is_an_error() {
        let eval = Evaluator::new();
        assert!(eval.eval("(/ 1 0)").is_err());
    }

    #[test]
    fn evaluates_nested_expressions() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(+ 1 (* 2 3))").unwrap(), "7");
    }

    #[test]
    fn evaluates_quote() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(quote (a b c))").unwrap(), "(a b c)");
        assert_eq!(eval.eval("'(a b c)").unwrap(), "(a b c)");
    }

    #[test]
    fn evaluates_list() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(list 1 2 3)").unwrap(), "(1 2 3)");
    }

    #[test]
    fn evaluates_multiple_top_level_forms_line_by_line() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(+ 1 1)\n(+ 2 2)").unwrap(), "2\n4");
    }

    #[test]
    fn empty_input_evaluates_to_empty_string() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("").unwrap(), "");
    }

    #[test]
    fn unknown_operator_is_an_error() {
        let eval = Evaluator::new();
        assert_eq!(
            eval.eval("(unknown-op 1 2)").unwrap_err(),
            "Unbound variable: unknown-op"
        );
    }

    #[test]
    fn wrong_argument_type_is_an_error() {
        let eval = Evaluator::new();
        assert!(eval.eval(r#"(+ 1 "two")"#).is_err());
    }

    #[test]
    fn unknown_builtin_is_an_error() {
        // 組み込みの手続きの値は, 名前の一覧からしか作らないので, 評価では起きない
        let evaluator = Evaluator::new();
        let mut machine = Machine::new(&evaluator, &NoHost);
        assert_eq!(
            builtins::call(&mut machine, "none", vec![]),
            Err("Unknown procedure: none".to_string())
        );
    }

    #[test]
    fn remembers_definitions_between_evaluations() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(define x 10)").unwrap(), "x");
        assert_eq!(eval.eval("(* x 2)").unwrap(), "20");
    }
}
