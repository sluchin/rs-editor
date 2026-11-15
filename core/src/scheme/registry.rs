//! 組み込みの手続きの登録表.
//!
//! 手続きは, 名前と関数の組を, モジュールごとの `register` で 1 か所に書きます (`register!`).
//! 名前の一覧と呼び出しの割り当ては, この表から作るので, 手続きを足すときに直す場所は, 1 つです.
//! 全ての手続きを移すまでは, 移していない手続きの名前と呼び出しは, 従来のまま (`builtins::NAMES` と `builtins::call`) です.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::error::Error;
use crate::scheme::ert;
use crate::scheme::evaluator::Machine;
use crate::scheme::hooks;
use crate::scheme::value::Value;

/// 組み込みの手続きの関数の型. 評価と, 呼ばれた名前と, 引数を受け取ります.
/// 1 つの関数を複数の名前で使えるように, 名前も渡します (失敗の内容に使う).
pub type NativeFn = fn(&mut Machine, &str, Vec<Value>) -> Result<Value, Error>;

/// 名前から関数を引く表.
#[derive(Default)]
pub struct Registry {
    /// 名前ごとの関数.
    functions: HashMap<&'static str, NativeFn>,
}

impl Registry {
    /// 空の表を作ります.
    ///
    /// # Returns
    ///
    /// 空の表.
    pub fn new() -> Self {
        Self::default()
    }

    /// 手続きを登録します.
    ///
    /// # Arguments
    ///
    /// * `name` - 手続きの名前.
    /// * `function` - 関数.
    ///
    /// # Panics
    ///
    /// 同じ名前がすでに登録されているとき (名前の書き間違いを, 起動のときに見つけるため).
    pub fn register(&mut self, name: &'static str, function: NativeFn) {
        let previous = self.functions.insert(name, function);
        assert!(previous.is_none(), "duplicate builtin: {name}");
    }

    /// 名前から関数を引きます.
    ///
    /// # Arguments
    ///
    /// * `name` - 手続きの名前.
    ///
    /// # Returns
    ///
    /// 関数. 登録されていなければ [`None`].
    pub fn get(&self, name: &str) -> Option<NativeFn> {
        self.functions.get(name).copied()
    }

    /// 登録されている名前を返します (順序は決まっていない).
    ///
    /// # Returns
    ///
    /// 名前の列.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.functions.keys().copied()
    }
}

/// 手続きを, まとめて登録します.
///
/// ```ignore
/// register!(registry, "add-hook" => add_hook, "run-hooks" => run_hooks);
/// ```
#[macro_export]
macro_rules! register {
    ($registry:expr, $($name:literal => $function:path),+ $(,)?) => {
        $( $registry.register($name, $function); )+
    };
}

/// 全ての手続きを登録した表を返します (初めて呼ぶときに作る).
///
/// # Returns
///
/// 表.
pub fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut registry = Registry::new();
        ert::register(&mut registry);
        hooks::register(&mut registry);
        registry
    })
}
