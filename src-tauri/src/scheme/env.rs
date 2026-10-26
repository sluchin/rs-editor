//! 変数の環境 (名前と値の組. 外側の環境をたどる).
//!
//! 名前付き `let`, `letrec`, 内側の `define` は, 環境と手続き ([`Lambda`](crate::scheme::value::Lambda))
//! が互いを指す輪を作り, 参照の数では放せません. そのため, 一番外側の環境は, その下に作った環境を覚え
//! ([`Registry`]), 評価を終えるたびに, 一番外側からたどれない環境の変数を消して, 輪を切ります ([`Env::collect`]).

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, Weak};

use crate::scheme::value::Value;

/// 環境. 複製しても, 同じ環境を指します.
#[derive(Clone, Default)]
pub struct Env(Arc<Mutex<Frame>>);

/// 環境の中身 (1 段分).
#[derive(Default)]
struct Frame {
    /// この段の変数 (名前と値).
    vars: HashMap<String, Value>,
    /// 外側の環境. 一番外側では [`None`].
    parent: Option<Env>,
    /// 一番外側の環境の下に作った環境 (一番外側と, その下の全ての環境で, 同じものを指す).
    registry: Arc<Mutex<Registry>>,
}

/// 一番外側の環境の下に作った環境 (放したものは, ときどき除く).
#[derive(Default)]
struct Registry {
    /// 作った環境 (放したかもしれない).
    frames: Vec<Weak<Mutex<Frame>>>,
    /// 放した環境を除く, 覚えている数 (覚えている数に合わせて, 倍にしていく).
    prune_at: usize,
}

/// 放した環境を除く, 覚えている数の始め.
const PRUNE_AT: usize = 1024;

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
        let registry = self.0.lock().unwrap().registry.clone();
        let env = Env(Arc::new(Mutex::new(Frame {
            vars: HashMap::new(),
            parent: Some(self.clone()),
            registry: registry.clone(),
        })));
        let mut registry = registry.lock().unwrap();
        // 放した環境は, 覚えている数が倍になるたびに除く (長い評価でも, 増え続けないように)
        if registry.frames.len() >= registry.prune_at.max(PRUNE_AT) {
            registry.frames.retain(|frame| frame.strong_count() > 0);
            registry.prune_at = registry.frames.len() * 2;
        }
        registry.frames.push(Arc::downgrade(&env.0));
        env
    }

    /// この環境 (一番外側の環境) からたどれない, その下に作った環境の変数を消して, 環境と手続きの輪を切ります.
    /// 評価を終えたときに呼びます (評価のあいだは, 評価器だけが持つ環境があるため).
    pub fn collect(&self) {
        // 一番外側から, 変数の値 (手続きの環境と, リストの中) と, 外側の環境をたどる
        let mut reached: HashSet<*const Mutex<Frame>> = HashSet::new();
        let mut pending = vec![self.clone()];
        while let Some(env) = pending.pop() {
            if !reached.insert(Arc::as_ptr(&env.0)) {
                continue;
            }
            let frame = env.0.lock().unwrap();
            for value in frame.vars.values() {
                push_envs(value, &mut pending);
            }
            pending.extend(frame.parent.clone());
        }
        let registry = self.0.lock().unwrap().registry.clone();
        let frames = std::mem::take(&mut registry.lock().unwrap().frames);
        let mut kept = Vec::new();
        for weak in frames {
            let Some(frame) = weak.upgrade() else {
                continue;
            };
            if reached.contains(&Arc::as_ptr(&frame)) {
                kept.push(weak);
            } else {
                // 消した値は, 環境のロックを外してから放す (放す環境が, ロックを取るため)
                let vars = std::mem::take(&mut frame.lock().unwrap().vars);
                drop(vars);
            }
        }
        let mut registry = registry.lock().unwrap();
        registry.prune_at = kept.len() * 2;
        registry.frames = kept;
    }

    /// 一番外側の環境の下に作って, まだ放していない環境の数を返します.
    ///
    /// # Returns
    ///
    /// 環境の数.
    pub fn live_frames(&self) -> usize {
        let registry = self.0.lock().unwrap().registry.clone();
        let registry = registry.lock().unwrap();
        registry
            .frames
            .iter()
            .filter(|frame| frame.strong_count() > 0)
            .count()
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

    /// この段の変数の名前を返します (外側の環境は, たどらない).
    ///
    /// # Returns
    ///
    /// 名前 (名前の順).
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.0.lock().unwrap().vars.keys().cloned().collect();
        names.sort();
        names
    }

    /// 同じ環境か (複製したものも, 同じ環境).
    ///
    /// # Arguments
    ///
    /// * `other` - 比べる環境.
    ///
    /// # Returns
    ///
    /// 同じ環境なら `true`.
    pub fn same(&self, other: &Env) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
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

/// 値から指す環境 (手続きの環境と, リストの中の手続きの環境) を, 加えます.
///
/// # Arguments
///
/// * `value` - 値.
/// * `envs` - 環境を加える先.
fn push_envs(value: &Value, envs: &mut Vec<Env>) {
    match value {
        Value::Lambda(lambda) => envs.push(lambda.env.clone()),
        Value::List(items) => {
            for item in items.iter() {
                push_envs(item, envs);
            }
        }
        _ => {}
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

    #[test]
    fn collect_breaks_the_cycles_that_cannot_be_reached() {
        use crate::scheme::value::Lambda;
        let global = Env::new();
        let lambda = |env: &Env| {
            Value::Lambda(Arc::new(Lambda {
                name: None,
                params: Vec::new(),
                rest: None,
                body: Vec::new(),
                env: env.clone(),
            }))
        };
        // 環境と手続きの輪 (一番外側から, たどれない)
        let lost = global.child();
        lost.define("f", lambda(&lost));
        let weak = Arc::downgrade(&lost.0);
        drop(lost);
        // 一番外側から, リストの中の手続きでたどれる輪
        let kept = global.child();
        kept.define("g", lambda(&kept));
        global.define("h", Value::List(vec![lambda(&kept), Value::Nil].into()));
        drop(kept);
        assert_eq!(global.live_frames(), 2);

        global.collect();
        assert!(weak.upgrade().is_none());
        assert_eq!(global.live_frames(), 1);
        global.define("h", Value::Nil);
        global.collect();
        assert_eq!(global.live_frames(), 0);
    }

    #[test]
    fn child_forgets_the_released_frames() {
        let global = Env::new();
        for _ in 0..PRUNE_AT * 3 {
            global.child();
        }
        let registry = global.0.lock().unwrap().registry.clone();
        assert!(registry.lock().unwrap().frames.len() <= PRUNE_AT + 1);
        assert_eq!(global.live_frames(), 0);
    }
}
