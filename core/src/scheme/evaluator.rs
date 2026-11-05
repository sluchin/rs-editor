//! Scheme の評価器. 定義 (`define`) は, アプリが持ち続ける評価器の環境に覚えます.
//!
//! - 特別な形: `quote` (`'`), `if`, `define`, `set!`, `lambda`, `begin`, `let` (名前付きも), `let*`, `letrec`,
//!   `cond`, `case`, `and`, `or`, `when`, `unless`, `unwind-protect`, `define-command`.
//! - テスト ([`ert`]): `ert-deftest`, `should`, `should-not`, `should-error`, `skip-unless`.
//! - 組み込みの手続きは [`builtins`].
//! - 末尾の呼び出しは, 深さを使いません. それ以外の入れ子は, [`MAX_DEPTH`] までです.

use crate::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use serde::{Deserialize, Serialize};

use crate::scheme::builtins;
use crate::scheme::editor_ops;
use crate::scheme::env::Env;
use crate::scheme::ert::{self, ErtTest};
use crate::scheme::help;
use crate::scheme::hooks;
use crate::scheme::host::{Host, NoHost};
use crate::scheme::loadup;

/// `message` の記録の行数の上限.
const MESSAGE_LOG_MAX: usize = 1000;
use crate::scheme::parser::Parser;
use crate::scheme::system;
use crate::scheme::value::{Lambda, Value};
use crate::sync::LockExt;

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

thread_local! {
    /// このスレッドが, 評価の最中か.
    static IN_EVALUATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
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
    /// 変数の docstring (`(define name value "doc")`. Emacs の `variable-documentation`).
    variable_docs: Mutex<HashMap<String, String>>,
    /// 名前を定義したファイル (読み込んでいるファイルの, 一番外側の `define`. Emacs の `symbol-file`).
    symbol_files: Mutex<HashMap<String, String>>,
    /// `message` が出した文 (Emacs の `*Messages*`). 古いものから.
    message_log: Mutex<Vec<String>>,
    /// 相対パスの基のディレクトリ (Emacs の `default-directory`. 無ければ, ホームディレクトリ).
    default_directory: Mutex<Option<std::path::PathBuf>>,
    /// `define-error` で決めた種類 (名前, 受ける条件と, 説明).
    error_kinds: Mutex<HashMap<String, (Vec<String>, String)>>,
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
            .chain(help::NAMES)
            .chain(editor_ops::NAMES)
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
        // browse-url で URL を開くプログラム
        global.define(
            "browse-url-generic-program",
            Value::String(system::DEFAULT_BROWSER.to_string()),
        );
        // 探すとき, 大文字と小文字を区別しないか (Emacs の case-fold-search)
        global.define("case-fold-search", Value::Boolean(true));
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
            variable_docs: Mutex::new(HashMap::new()),
            symbol_files: Mutex::new(HashMap::new()),
            message_log: Mutex::new(Vec::new()),
            default_directory: Mutex::new(None),
            error_kinds: Mutex::new(HashMap::new()),
        };
        // M-x ert
        let mut machine = Machine::new(&evaluator, &NoHost);
        for expr in ert::prelude() {
            machine
                .eval(&expr, &evaluator.global)
                .expect("the ERT prelude evaluates");
        }
        // 起動のときに読む Lisp (find-file などのコマンド)
        for (file, code) in loadup::FILES {
            machine.load_files.push(file.to_string());
            for expr in Parser::new(code)
                .parse()
                .unwrap_or_else(|e| panic!("{}: {}", file, e))
            {
                machine
                    .eval(&expr, &evaluator.global)
                    .unwrap_or_else(|e| panic!("{}: {}", file, e));
            }
            machine.load_files.pop();
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
    pub fn eval(&self, code: &str) -> Result<String, Error> {
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
    pub fn eval_with(&self, code: &str, host: &dyn Host) -> Result<String, Error> {
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

    /// エディタの基本のコマンド ([`loadup::EDITOR_FILES`]) を読みます (定義したファイルも覚える).
    ///
    /// # Arguments
    ///
    /// * `host` - エディタ.
    ///
    /// # Errors
    ///
    /// 読めないとき.
    pub fn load_editor_files(&self, host: &dyn Host) -> Result<(), Error> {
        for (file, code) in loadup::EDITOR_FILES {
            self.run(host, |machine| {
                machine.load_files.push(file.to_string());
                let result = Parser::new(code)
                    .parse()
                    .map_err(Error::from)
                    .and_then(|values| {
                        values
                            .iter()
                            .try_for_each(|value| machine.eval(value, &self.global).map(|_| ()))
                    });
                machine.load_files.pop();
                result
            })?;
        }
        Ok(())
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
    pub fn run_command(&self, name: &str, host: &dyn Host) -> Result<String, Error> {
        if !self.is_command(name) {
            return Err(format!("Undefined command: {}", name).into());
        }
        self.run(host, |machine| {
            machine.call_command(name).map(|value| value.to_string())
        })
    }

    /// 相対パスの基のディレクトリを決めます (バッチモードは, 起動したディレクトリ. 決めなければ, ホームディレクトリ).
    ///
    /// # Arguments
    ///
    /// * `dir` - ディレクトリ.
    pub fn set_default_directory(&self, dir: std::path::PathBuf) {
        *self.default_directory.lock_recover() = Some(dir);
    }

    /// `define-error`: 種類の, 受ける条件と説明を決めます.
    ///
    /// # Arguments
    ///
    /// * `kind` - 種類の名前.
    /// * `conditions` - 受ける条件 (種類が先).
    /// * `description` - 説明 (エラーの文の始め).
    pub(crate) fn define_error(&self, kind: &str, conditions: Vec<String>, description: &str) {
        self.error_kinds
            .lock_recover()
            .insert(kind.to_string(), (conditions, description.to_string()));
    }

    /// Lisp で作ったコマンドの名前を返します.
    ///
    /// # Returns
    ///
    /// 名前 (作った順).
    pub fn commands(&self) -> Vec<String> {
        self.commands.lock_recover().clone()
    }

    /// Lisp で割り当てたキーを返します.
    ///
    /// # Returns
    ///
    /// 割り当て (割り当てた順).
    pub fn key_bindings(&self) -> Vec<KeyBinding> {
        self.keys.lock_recover().clone()
    }

    /// 評価で作って, まだ放していない環境の数を返します (評価のあとに, 環境を放したかを確かめる).
    ///
    /// # Returns
    ///
    /// 環境の数.
    pub fn live_frames(&self) -> usize {
        self.global.live_frames()
    }

    /// 一番外側の環境を返します (`load` で使う).
    ///
    /// # Returns
    ///
    /// 環境.
    pub fn global(&self) -> &Env {
        &self.global
    }

    /// 変数の docstring を返します (Emacs の `(documentation-property name 'variable-documentation)`).
    ///
    /// # Arguments
    ///
    /// * `name` - 変数の名前.
    ///
    /// # Returns
    ///
    /// docstring. 無ければ [`None`].
    pub(crate) fn variable_documentation(&self, name: &str) -> Option<String> {
        self.variable_docs.lock_recover().get(name).cloned()
    }

    /// `message` が出した文を覚えます (空の文は覚えない. 1000 行を超えたら古いものを捨てる).
    ///
    /// # Arguments
    ///
    /// * `text` - 出した文.
    pub(crate) fn log_message(&self, text: &str) {
        if text.is_empty() {
            return;
        }
        let mut log = self.message_log.lock_recover();
        log.push(text.to_string());
        if log.len() > MESSAGE_LOG_MAX {
            log.remove(0);
        }
    }

    /// `message` が出した文を返します (古いものから).
    pub(crate) fn message_log(&self) -> Vec<String> {
        self.message_log.lock_recover().clone()
    }

    /// 名前を定義したファイルを返します.
    ///
    /// # Arguments
    ///
    /// * `name` - 名前.
    ///
    /// # Returns
    ///
    /// ファイル (起動のときに読む Lisp は `lisp/` からのパス, `load` したものは絶対パス). 分からなければ [`None`].
    pub(crate) fn symbol_file(&self, name: &str) -> Option<String> {
        self.symbol_files.lock_recover().get(name).cloned()
    }

    /// 機能を覚えます (Emacs の `provide`. 覚えていれば, 何もしない).
    ///
    /// # Arguments
    ///
    /// * `feature` - 機能の名前.
    pub(crate) fn provide(&self, feature: &str) {
        let mut features = self.features.lock_recover();
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
        self.features.lock_recover().iter().any(|f| f == feature)
    }

    /// `ert-deftest` で定義したテストを返します.
    ///
    /// # Returns
    ///
    /// テスト (定義した順. 書き換えられる).
    pub(crate) fn tests(&self) -> MutexGuard<'_, Vec<ErtTest>> {
        self.tests.lock_recover()
    }

    /// コマンドとして覚えます (覚えていれば, 何もしない).
    ///
    /// # Arguments
    ///
    /// * `name` - コマンドの名前.
    pub(crate) fn add_command(&self, name: &str) {
        let mut commands = self.commands.lock_recover();
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
        self.commands.lock_recover().iter().any(|c| c == name)
    }

    /// キーを割り当てます (同じキーの割り当ては, 置き換える).
    ///
    /// # Arguments
    ///
    /// * `binding` - 割り当て.
    pub(crate) fn set_key(&self, binding: KeyBinding) {
        let mut keys = self.keys.lock_recover();
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
        task: impl FnOnce(&mut Machine) -> Result<T, Error> + Send,
    ) -> Result<T, Error> {
        // 評価の中から呼ばれたとき (`execute-kbd-macro` が打ったキーのコマンドなど) は, ロックも新しいスレッドも要らない
        if IN_EVALUATION.with(std::cell::Cell::get) {
            return task(&mut Machine::new(self, host));
        }
        // 前の評価が失敗 (panic) していても, 続ける
        let _running = self.running.lock_recover();
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn_scoped(scope, || {
                    IN_EVALUATION.with(|flag| flag.set(true));
                    let result = task(&mut Machine::new(self, host));
                    // 評価で作った, 環境と手続きの輪を切る (放すと深く入れ子になるので, 大きなスタックで)
                    self.global.collect();
                    result
                })
                .map_err(|e| e.to_string())?
                .join()
                .unwrap_or_else(|_| Err("The evaluator crashed".to_string().into()))
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
pub struct Machine<'a> {
    /// 評価器 (定義とコマンドとキーを覚えている).
    pub(crate) evaluator: &'a Evaluator,
    /// エディタ (エディタの操作を頼む先).
    pub(crate) host: &'a dyn Host,
    /// 今の評価の入れ子の深さ ([`MAX_DEPTH`] まで).
    depth: usize,
    /// 読み込んでいるファイル (`load`) のディレクトリ (入れ子の順. 相対パスの基).
    pub(crate) load_dirs: Vec<std::path::PathBuf>,
    /// 読み込んでいるファイル (入れ子の順. 定義したファイルを覚える).
    pub(crate) load_files: Vec<String>,
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
    pub fn new(evaluator: &'a Evaluator, host: &'a dyn Host) -> Self {
        Machine {
            evaluator,
            host,
            depth: 0,
            load_dirs: Vec::new(),
            load_files: Vec::new(),
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
    /// 種類とデータからエラーを作ります (`define-error` で決めた種類は, その条件と説明を使う).
    pub(crate) fn make_error(&self, kind: &str, data: Vec<Value>) -> Error {
        let defined = self.evaluator.error_kinds.lock_recover().get(kind).cloned();
        match defined {
            Some((conditions, description)) => {
                Error::signal(kind, data, conditions, Some(&description))
            }
            None => Error::new(kind, data),
        }
    }

    /// 絶対パス.
    pub(crate) fn expand_file_name(&self, path: &str) -> String {
        let home = dirs::home_dir().unwrap_or_default();
        let default = self.evaluator.default_directory.lock_recover().clone();
        let base = self
            .load_dirs
            .last()
            .cloned()
            .or(default)
            .unwrap_or_else(|| home.clone());
        system::expand_file_name(path, &base, &home)
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
    pub(crate) fn eval(&mut self, expr: &Value, env: &Env) -> Result<Value, Error> {
        // リストでない式は, その場で値にする (式を写さない)
        if !matches!(expr, Value::List(_)) {
            return self.atom(expr, env);
        }
        if self.depth >= MAX_DEPTH {
            return Err("Too deep recursion".to_string().into());
        }
        self.depth += 1;
        let result = self.eval_tail(expr.clone(), env.clone());
        self.depth -= 1;
        result
    }

    /// リストでない式の値. 変数 `major-mode` は, 定義が無ければ, 今のバッファの構文の方言から決める (Emacs の名前で).
    fn atom(&self, expr: &Value, env: &Env) -> Result<Value, Error> {
        let value = eval_atom(expr, env);
        match (&value, expr) {
            (Err(_), Value::Symbol(name)) if name == "major-mode" => {
                self.host.call("major-mode-name", vec![])
            }
            _ => value,
        }
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
    fn eval_tail(&mut self, mut expr: Value, mut env: Env) -> Result<Value, Error> {
        loop {
            let items = match &expr {
                Value::List(items) if !items.is_empty() => items.clone(),
                atom => return self.atom(atom, &env),
            };
            let special = match &items[0] {
                Value::Symbol(op) => self.special_form(op, &items[1..], &env)?,
                _ => None,
            };
            let step = match special {
                Some(step) => step,
                None => {
                    // 呼ぶ手続きの記号が無いのは, void-function
                    if let Value::Symbol(name) = &items[0] {
                        if env.get(name).is_none() {
                            return Err(Error::void_function(name));
                        }
                    }
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
    fn eval_all(&mut self, exprs: &[Value], env: &Env) -> Result<Vec<Value>, Error> {
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
    pub(crate) fn apply(&mut self, procedure: Value, args: Vec<Value>) -> Result<Value, Error> {
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
    fn apply_step(&mut self, procedure: Value, args: Vec<Value>) -> Result<Step, Error> {
        match procedure {
            Value::Lambda(lambda) => {
                let env = bind(&lambda, args)?;
                self.sequence(&lambda.body, &env)
            }
            Value::Function(name) => builtins::call(self, &name, args).map(Step::Done),
            other => Err(format!("Not a procedure: {}", other).into()),
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
    pub(crate) fn call_command(&mut self, name: &str) -> Result<Value, Error> {
        if self.evaluator.is_command(name) {
            let procedure = self
                .evaluator
                .global
                .get(name)
                .ok_or_else(|| Error::void_variable(name))?;
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
    fn sequence(&mut self, body: &[Value], env: &Env) -> Result<Step, Error> {
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

    /// 今のバッファ (と, カーソル) を覚えて本体を評価し, 終わったら戻します
    /// (`save-current-buffer`, `save-excursion`, `with-current-buffer`). 本体が失敗しても戻す.
    fn save_buffer_state(&mut self, op: &str, args: &[Value], env: &Env) -> Result<Value, Error> {
        let (target, body) = match (op, args) {
            ("with-current-buffer", [buffer, body @ ..]) => (Some(buffer), body),
            ("with-current-buffer", _) => {
                return Err("with-current-buffer expects a buffer".to_string().into())
            }
            _ => (None, args),
        };
        let saved = self.host.call("current-buffer", vec![])?;
        // save-excursion は, 位置を追うマーカーで覚える (画面に頼むときは、マーカーが無いので、位置の数で覚える)
        let point = if op == "save-excursion" {
            Some(match self.host.call("point-marker", vec![]) {
                Ok(id) => (true, id),
                Err(_) => (false, self.host.call("point", vec![])?),
            })
        } else {
            None
        };
        let outcome = match target {
            Some(expr) => self
                .eval(expr, env)
                .and_then(|name| self.host.call("set-buffer", vec![name]))
                .and_then(|_| self.eval_body(body, env)),
            None => self.eval_body(body, env),
        };
        // 本体の間に, 戻すバッファを消したときは, 戻さない
        if self.host.call("set-buffer", vec![saved]).is_ok() {
            if let Some((is_marker, saved)) = point {
                let target = if is_marker {
                    let position = self.host.call("marker-position", vec![saved.clone()])?;
                    self.host.call("marker-release", vec![saved])?;
                    position
                } else {
                    saved
                };
                self.host.call("goto-char", vec![target])?;
            }
        }
        outcome
    }

    /// 今の絞り込みを覚えて本体を評価し, 終わったら戻します (`save-restriction`). 本体が失敗しても戻す.
    /// 絞り込みの無い画面 (Tauri) では, 覚えずに評価する.
    fn save_restriction(&mut self, body: &[Value], env: &Env) -> Result<Value, Error> {
        let saved = self.host.call("current-buffer", vec![])?;
        let id = self.host.call("restriction-save", vec![]).ok();
        let outcome = self.eval_body(body, env);
        if let Some(id) = id {
            // 覚えたバッファで戻してから, 本体の終わりのバッファに戻る (消したバッファは, 戻さない)
            let current = self.host.call("current-buffer", vec![])?;
            if self.host.call("set-buffer", vec![saved]).is_ok() {
                self.host.call("restriction-restore", vec![id])?;
                self.host.call("set-buffer", vec![current])?;
            }
        }
        outcome
    }

    /// 式を順に評価して, 最後の値を返します.
    fn eval_body(&mut self, body: &[Value], env: &Env) -> Result<Value, Error> {
        let mut value = Value::Nil;
        for expr in body {
            value = self.eval(expr, env)?;
        }
        Ok(value)
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
    fn special_form(&mut self, op: &str, args: &[Value], env: &Env) -> Result<Option<Step>, Error> {
        let step = match op {
            "quote" => match args {
                [value] => Step::Done(value.clone()),
                _ => return Err("quote expects 1 argument".to_string().into()),
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
                _ => return Err("if expects 2 or 3 arguments".to_string().into()),
            },
            "define" => Step::Done(Value::Symbol(self.define(args, env)?)),
            "define-command" => {
                let name = self.define(args, env)?;
                match env.get(&name) {
                    Some(Value::Lambda(_) | Value::Function(_)) => {
                        self.evaluator.add_command(&name);
                        Step::Done(Value::Symbol(name))
                    }
                    _ => return Err("define-command expects a procedure".to_string().into()),
                }
            }
            "set!" => match args {
                [Value::Symbol(name), expr] => {
                    let value = self.eval(expr, env)?;
                    if !env.set(name, value) {
                        return Err(Error::void_variable(name));
                    }
                    Step::Done(Value::Nil)
                }
                _ => return Err("set! expects a name and a value".to_string().into()),
            },
            "lambda" => match args {
                [params, body @ ..] => Step::Done(make_lambda(None, params, body, env)?),
                _ => return Err("lambda expects parameters".to_string().into()),
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
                _ => return Err("let expects bindings".to_string().into()),
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
                _ => return Err(format!("{} expects bindings", op).into()),
            },
            "cond" => self.cond(args, env)?,
            "case" => match args {
                [key, clauses @ ..] => {
                    let key = self.eval(key, env)?;
                    self.case(&key, clauses, env)?
                }
                _ => return Err("case expects a key".to_string().into()),
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
                _ => return Err(format!("{} expects a test", op).into()),
            },
            "while" => match args {
                // (while TEST BODY...): TEST が真の間, BODY を繰り返す (Emacs Lisp の while). 値は偽
                [test, body @ ..] => {
                    while self.eval(test, env)?.is_true() {
                        for form in body {
                            self.eval(form, env)?;
                        }
                    }
                    Step::Done(Value::Nil)
                }
                _ => return Err("while expects a test".to_string().into()),
            },
            "condition-case" => match args {
                [var @ (Value::Symbol(_) | Value::Boolean(false)), body, handlers @ ..] => {
                    match self.eval(body, env) {
                        Ok(value) => Step::Done(value),
                        Err(error) => {
                            let handler = handlers.iter().find_map(|handler| match handler {
                                Value::List(items) => {
                                    handles(&items[0], &error).then(|| items[1..].to_vec())
                                }
                                _ => None,
                            });
                            let Some(body) = handler else {
                                return Err(error);
                            };
                            // 変数には, (種類 . データ) を束ねる. 変数が nil (`(condition-case nil ...)`) なら, 束ねない
                            let inner = env.child();
                            if let Value::Symbol(var) = var {
                                inner.define(var, error.to_value());
                            }
                            self.sequence(&body, &inner)?
                        }
                    }
                }
                _ => {
                    return Err("condition-case expects a variable and a body form"
                        .to_string()
                        .into())
                }
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
                _ => return Err("unwind-protect expects a body form".to_string().into()),
            },
            "save-restriction" => Step::Done(self.save_restriction(args, env)?),
            "save-current-buffer" | "save-excursion" | "with-current-buffer" => {
                Step::Done(self.save_buffer_state(op, args, env)?)
            }
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
    fn define(&mut self, args: &[Value], env: &Env) -> Result<String, Error> {
        let name = self.define_value(args, env)?;
        // 読み込んでいるファイルの, 一番外側の定義なら, ファイルを覚える
        if let Some(file) = self.load_files.last() {
            if env.same(&self.evaluator.global) {
                self.evaluator
                    .symbol_files
                    .lock_recover()
                    .insert(name.clone(), file.clone());
            }
        }
        Ok(name)
    }

    /// `define` の値を, 環境に入れます ([`Machine::define`] の中身).
    ///
    /// # Arguments
    ///
    /// * `args` - `define` の後の式.
    /// * `env` - 定義する環境.
    ///
    /// # Returns
    ///
    /// 定義した名前, または失敗の内容.
    fn define_value(&mut self, args: &[Value], env: &Env) -> Result<String, Error> {
        match args {
            [Value::Symbol(name), rest @ ..] if rest.len() <= 2 => {
                // 3 つ目は, docstring (Emacs の defvar と同じ)
                if let Some(doc) = rest.get(1) {
                    let Value::String(doc) = doc else {
                        return Err("define expects a docstring after the value"
                            .to_string()
                            .into());
                    };
                    self.evaluator
                        .variable_docs
                        .lock_recover()
                        .insert(name.clone(), doc.clone());
                }
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
                _ => Err("define expects a name".to_string().into()),
            },
            _ => Err("define expects a name and a value".to_string().into()),
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
    ) -> Result<Step, Error> {
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
    fn cond(&mut self, clauses: &[Value], env: &Env) -> Result<Step, Error> {
        for clause in clauses {
            let Value::List(items) = clause else {
                return Err("cond expects clauses".to_string().into());
            };
            let Some((test, body)) = items.split_first() else {
                return Err("cond expects clauses".to_string().into());
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
    fn case(&mut self, key: &Value, clauses: &[Value], env: &Env) -> Result<Step, Error> {
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
                    _ => return Err("case expects clauses".to_string().into()),
                },
                _ => return Err("case expects clauses".to_string().into()),
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
) -> Result<Value, Error> {
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
                        _ => return Err("Invalid parameter list".to_string().into()),
                    },
                    Value::Symbol(param) => names.push(param.clone()),
                    _ => return Err("Invalid parameter list".to_string().into()),
                }
            }
            (names, rest)
        }
        _ => return Err("Invalid parameter list".to_string().into()),
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
fn eval_atom(expr: &Value, env: &Env) -> Result<Value, Error> {
    match expr {
        Value::Symbol(name) if !name.starts_with(':') => {
            env.get(name).ok_or_else(|| Error::void_variable(name))
        }
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
fn bind(lambda: &Lambda, args: Vec<Value>) -> Result<Env, Error> {
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
        )
        .into());
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
fn parse_bindings(form: &str, bindings: &Value) -> Result<Vec<(String, Value)>, Error> {
    let invalid = || format!("{} expects bindings", form);
    let Value::List(items) = bindings else {
        return Err(invalid().into());
    };
    items
        .iter()
        .map(|item| match item {
            Value::List(pair) => match &**pair {
                [Value::Symbol(name), expr] => Ok((name.clone(), expr.clone())),
                _ => Err(invalid().into()),
            },
            _ => Err(invalid().into()),
        })
        .collect()
}

/// `condition-case` の条件 (記号か記号のリスト) が, エラーを受けるか (エラーの `error-conditions` で決める).
pub(crate) fn handles(conditions: &Value, error: &Error) -> bool {
    match conditions {
        Value::Symbol(name) => error.matches(name),
        Value::Boolean(true) => true,
        Value::List(items) => items.iter().any(|v| handles(v, error)),
        _ => false,
    }
}
