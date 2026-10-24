//! 変数の環境 (名前と値の組. 外側の環境をたどる).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::scheme::value::Value;

/// 環境. 複製しても, 同じ環境を指します.
#[derive(Clone, Default)]
pub struct Env(Arc<Mutex<Frame>>);

#[derive(Default)]
struct Frame {
    vars: HashMap<String, Value>,
    parent: Option<Env>,
}

impl Env {
    /// 外側のない環境を作ります.
    ///
    /// # Returns
    ///
    /// 環境.
    pub fn new() -> Self {
        Self::default()
    }

    /// この環境を外側にする, 新しい環境を作ります.
    ///
    /// # Returns
    ///
    /// 環境.
    pub fn child(&self) -> Self {
        Env(Arc::new(Mutex::new(Frame {
            vars: HashMap::new(),
            parent: Some(self.clone()),
        })))
    }

    /// 変数の値を, 外側へたどって探します.
    ///
    /// # Arguments
    ///
    /// * `name` - 変数の名前.
    ///
    /// # Returns
    ///
    /// 値. なければ `None`.
    pub fn get(&self, name: &str) -> Option<Value> {
        let frame = self.0.lock().unwrap();
        match frame.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => frame.parent.as_ref()?.get(name),
        }
    }

    /// この環境に, 変数を作ります (あれば, 値を変えます).
    ///
    /// # Arguments
    ///
    /// * `name` - 変数の名前.
    /// * `value` - 値.
    pub fn define(&self, name: &str, value: Value) {
        self.0.lock().unwrap().vars.insert(name.to_string(), value);
    }

    /// 変数の値を変えます (外側へたどって探す).
    ///
    /// # Arguments
    ///
    /// * `name` - 変数の名前.
    /// * `value` - 値.
    ///
    /// # Returns
    ///
    /// 変数があれば `true`.
    pub fn set(&self, name: &str, value: Value) -> bool {
        let mut frame = self.0.lock().unwrap();
        if let Some(slot) = frame.vars.get_mut(name) {
            *slot = value;
            return true;
        }
        match &frame.parent {
            Some(parent) => parent.set(name, value),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_up_the_outer_environments() {
        let global = Env::new();
        global.define("a", Value::Number(1.0));
        let inner = global.child();
        inner.define("b", Value::Number(2.0));
        assert_eq!(inner.get("a"), Some(Value::Number(1.0)));
        assert_eq!(inner.get("b"), Some(Value::Number(2.0)));
        assert_eq!(global.get("b"), None);
    }

    #[test]
    fn set_changes_the_nearest_variable() {
        let global = Env::new();
        global.define("a", Value::Number(1.0));
        let inner = global.child();
        assert!(inner.set("a", Value::Number(3.0)));
        assert_eq!(global.get("a"), Some(Value::Number(3.0)));
        assert!(!inner.set("none", Value::Nil));
    }
}
