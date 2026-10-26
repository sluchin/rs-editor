//! Scheme の値 ([`Value`]) と, `lambda` で作った手続き ([`Lambda`]).

use std::cell::UnsafeCell;
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::scheme::env::Env;

/// Scheme の値.
#[derive(Debug, Clone)]
pub enum Value {
    /// 値がないこと (`insert` などの結果).
    Nil,
    /// 真偽値 (`#t` / `#f`).
    Boolean(bool),
    /// 数 (整数も小数も, 浮動小数点数で持つ).
    Number(f64),
    /// 記号 (変数の名前など).
    Symbol(String),
    /// 文字列.
    String(String),
    /// リスト (ドット対は扱わない). 写すときは, 要素を写さず, 参照を増やすだけ (評価のたびに, 式の木を写さないように).
    List(List),
    /// 組み込みの手続き (名前で引く).
    Function(String),
    /// `lambda` で作った手続き.
    Lambda(Arc<Lambda>),
}

/// リストの要素を入れる配列 (リストどうしで共有する). 前に空きを持ち, `cons` は, 空きに要素を置きます.
///
/// - `front` より後ろのマスは, 値が入っていて, もう変えない.
/// - `front` より前のマスは, 空き. `cons` は, 比較交換で `front` を 1 つ前へ動かして, 空きを 1 マス取ってから, 値を入れる.
///   取れるのは, リストの始めが `front` の所にあるとき (同じ配列に, まだ誰も前へ足していないとき) だけ.
struct Block {
    /// マス.
    slots: Box<[UnsafeCell<MaybeUninit<Value>>]>,
    /// 値の入った, 一番前のマスの位置.
    front: AtomicUsize,
}

// SAFETY: 値の入ったマスは変えない (読むだけ). 空きのマスに書くのは, 比較交換でそのマスを取った 1 人だけで,
// そのマスを含むリストは, 書いた後で作る. Value は Send と Sync.
unsafe impl Send for Block {}
// SAFETY: 上と同じ.
unsafe impl Sync for Block {}

impl Block {
    /// 要素から, 前に空きを持つ配列を作ります.
    ///
    /// # Arguments
    ///
    /// * `spare` - 前の空きの数.
    /// * `items` - 要素.
    ///
    /// # Returns
    ///
    /// 配列.
    fn new(spare: usize, items: impl IntoIterator<Item = Value>) -> Block {
        let slots: Box<[_]> = std::iter::repeat_with(|| UnsafeCell::new(MaybeUninit::uninit()))
            .take(spare)
            .chain(
                items
                    .into_iter()
                    .map(|v| UnsafeCell::new(MaybeUninit::new(v))),
            )
            .collect();
        Block {
            slots,
            front: AtomicUsize::new(spare),
        }
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        let front = *self.front.get_mut();
        for slot in &mut self.slots[front..] {
            // SAFETY: front より後ろのマスには, 値が入っている
            unsafe { slot.get_mut().assume_init_drop() };
        }
    }
}

/// リストの要素. 要素の配列を共有し, その中の始めの位置からを, リストとして見せます.
/// 写すとき, `cdr` (始めの要素を除いたリスト) と, `cons` (前に要素を足したリスト) は,
/// 配列を写さず, 参照を増やすだけです (`cons` は, ならして). 要素は, スライス (`&[Value]`) として読めます.
#[derive(Clone)]
pub struct List {
    /// 要素の配列 (ほかのリストと共有する).
    block: Arc<Block>,
    /// リストの始めの要素の, 配列での位置.
    start: usize,
}

impl List {
    /// 始めの要素を除いたリスト (`cdr`) を返します. 配列は写しません.
    ///
    /// # Returns
    ///
    /// 残りのリスト. 空のリストなら, 空のリスト.
    pub fn tail(&self) -> List {
        self.drop_front(1)
    }

    /// 始めの `n` 個の要素を除いたリストを返します. 配列は写しません.
    ///
    /// # Arguments
    ///
    /// * `n` - 除く数 (長さを超えたら, 空のリスト).
    ///
    /// # Returns
    ///
    /// 残りのリスト.
    pub fn drop_front(&self, n: usize) -> List {
        List {
            block: self.block.clone(),
            start: (self.start + n).min(self.block.slots.len()),
        }
    }

    /// 前に要素を足したリスト (`cons`) を返します. 配列の前の空きを取れれば, 配列は写しません.
    /// 取れなければ (空きが無いか, 同じ配列に, 誰かが前へ足していれば), 長さと同じだけの空きを持つ配列に写します.
    ///
    /// # Arguments
    ///
    /// * `head` - 足す要素.
    ///
    /// # Returns
    ///
    /// 足したリスト.
    pub fn cons(&self, head: Value) -> List {
        let block = &self.block;
        if self.start > 0
            && block
                .front
                .compare_exchange(
                    self.start,
                    self.start - 1,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
        {
            // SAFETY: 比較交換で, このマスを取った (ほかには, 誰も書かず, 読まない)
            unsafe { (*block.slots[self.start - 1].get()).write(head) };
            return List {
                block: block.clone(),
                start: self.start - 1,
            };
        }
        let spare = self.len().max(MIN_SPARE);
        List {
            block: Arc::new(Block::new(
                spare,
                std::iter::once(head).chain(self.iter().cloned()),
            )),
            start: spare,
        }
    }
}

/// `cons` で写すときに, 前に取っておく空きの, 少なくともの数.
const MIN_SPARE: usize = 4;

impl std::ops::Deref for List {
    type Target = [Value];

    fn deref(&self) -> &[Value] {
        let slots = &self.block.slots[self.start..];
        // SAFETY: start より後ろのマスには, 値が入っていて, もう変えない.
        // UnsafeCell<MaybeUninit<Value>> と Value は, 同じ並び
        unsafe { std::slice::from_raw_parts(slots.as_ptr().cast::<Value>(), slots.len()) }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl From<Vec<Value>> for List {
    fn from(items: Vec<Value>) -> Self {
        items.into_iter().collect()
    }
}

impl From<&[Value]> for List {
    fn from(items: &[Value]) -> Self {
        items.iter().cloned().collect()
    }
}

impl<const N: usize> From<[Value; N]> for List {
    fn from(items: [Value; N]) -> Self {
        items.into_iter().collect()
    }
}

impl FromIterator<Value> for List {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        List {
            block: Arc::new(Block::new(0, iter)),
            start: 0,
        }
    }
}

/// `lambda` で作った手続き.
pub struct Lambda {
    /// `define` で付けた名前.
    pub name: Option<String>,
    /// 引数の名前.
    pub params: Vec<String>,
    /// 残りの引数を, リストで受ける名前 (`(a . rest)` の `rest`).
    pub rest: Option<String>,
    /// 本体.
    pub body: Vec<Value>,
    /// 作ったときの環境.
    pub env: Env,
}

impl fmt::Debug for Lambda {
    // 環境は, 自分を含むことがあるので, 出さない
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lambda({:?})", self.name)
    }
}

impl Value {
    /// 文字列は引用符を付けずに, ほかは [`Display`](fmt::Display) で表します (`display` と `format` の `%s`).
    ///
    /// # Returns
    ///
    /// 表した文字列.
    pub fn to_display(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            v => v.to_string(),
        }
    }

    /// 真かを返します (`#f` だけが偽).
    ///
    /// # Returns
    ///
    /// 真なら `true`.
    pub fn is_true(&self) -> bool {
        !matches!(self, Value::Boolean(false))
    }

    /// Emacs の non-nil かを返します (`#f`, 値が無いこと, 空のリストだけが nil). 省ける引数に使う.
    ///
    /// # Returns
    ///
    /// non-nil なら `true`.
    pub fn is_non_nil(&self) -> bool {
        match self {
            Value::Boolean(false) | Value::Nil => false,
            Value::List(items) => !items.is_empty(),
            _ => true,
        }
    }
}

/// i64 で表せる数の境 (2^63). これより小さい整数は, `-0` を `0` と書くように, i64 で書く.
const I64_LIMIT: f64 = 9_223_372_036_854_775_808.0;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Number(n) => {
                // i64 に収まる整数だけ, 整数で書く (大きな数を i64 にすると, 値が変わる)
                if n.fract() == 0.0 && n.abs() < I64_LIMIT {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Symbol(s) => write!(f, "{}", s),
            Value::String(s) => write!(f, "{:?}", s),
            Value::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Function(name) => write!(f, "#<procedure:{}>", name),
            Value::Lambda(lambda) => match &lambda.name {
                Some(name) => write!(f, "#<procedure:{}>", name),
                None => write!(f, "#<procedure>"),
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::Lambda(a), Value::Lambda(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 数のリストを作る.
    fn numbers(items: &[f64]) -> List {
        items.iter().map(|n| Value::Number(*n)).collect()
    }

    #[test]
    fn debug_shows_the_list_elements() {
        assert_eq!(
            format!("{:?}", Value::List(numbers(&[1.0, 2.0]).tail())),
            "List([Number(2.0)])"
        );
    }

    #[test]
    fn values_are_send_and_sync() {
        fn assert<T: Send + Sync>() {}
        assert::<Value>();
    }

    #[test]
    fn cons_shares_the_block_only_once() {
        let a = numbers(&[1.0, 2.0]);
        // 空きが無ければ写す. 写した配列は, 前に空きを持つ
        let b = a.cons(Value::Number(0.0));
        assert!(!Arc::ptr_eq(&a.block, &b.block));
        // 前へ足していくと, 同じ配列の空きを使う
        let c = b.cons(Value::Number(-1.0));
        assert!(Arc::ptr_eq(&b.block, &c.block));
        // 同じリストに, もう一度足すと (空きは取られている), 写す
        let d = b.cons(Value::Number(9.0));
        assert!(!Arc::ptr_eq(&b.block, &d.block));
        // cdr で作ったリストの前は, 取られているので, 写す
        let e = c.tail().cons(Value::Number(7.0));
        assert!(!Arc::ptr_eq(&c.block, &e.block));
        assert_eq!(*a, *numbers(&[1.0, 2.0]));
        assert_eq!(*b, *numbers(&[0.0, 1.0, 2.0]));
        assert_eq!(*c, *numbers(&[-1.0, 0.0, 1.0, 2.0]));
        assert_eq!(*d, *numbers(&[9.0, 0.0, 1.0, 2.0]));
        assert_eq!(*e, *numbers(&[7.0, 0.0, 1.0, 2.0]));
        // 空きを使い切ったら, また写す
        let mut list = List::from(vec![]);
        for i in 0..100 {
            list = list.cons(Value::Number(i as f64));
        }
        assert_eq!(list.len(), 100);
        assert_eq!(list[0], Value::Number(99.0));
        assert_eq!(list[99], Value::Number(0.0));
        assert_eq!(list.drop_front(200).len(), 0);
    }

    #[test]
    fn cons_from_threads_claims_the_slot_once() {
        let base = numbers(&[1.0]).cons(Value::Number(0.0));
        let results: Vec<List> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..8)
                .map(|i| {
                    let base = base.clone();
                    scope.spawn(move || base.cons(Value::Number(10.0 + i as f64)))
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for (i, list) in results.iter().enumerate() {
            assert_eq!(**list, *numbers(&[10.0 + i as f64, 0.0, 1.0]));
        }
        // 同じ配列の空きを取れたのは, 1 つだけ
        let shared = results
            .iter()
            .filter(|l| Arc::ptr_eq(&l.block, &base.block))
            .count();
        assert_eq!(shared, 1);
    }

    #[test]
    fn lists_drop_each_element_once() {
        let lambda = Arc::new(Lambda {
            name: None,
            params: vec![],
            rest: None,
            body: vec![],
            env: Env::new(),
        });
        {
            let mut list = List::from(vec![Value::Lambda(lambda.clone())]);
            for _ in 0..10 {
                list = list.cons(Value::Lambda(lambda.clone()));
            }
            let forked = list.tail().cons(Value::Lambda(lambda.clone()));
            assert_eq!(Arc::strong_count(&lambda), 1 + 11 + 11);
            drop(forked);
            assert_eq!(Arc::strong_count(&lambda), 1 + 11);
        }
        assert_eq!(Arc::strong_count(&lambda), 1);
    }

    #[test]
    fn displays_integer_numbers_without_decimal_point() {
        assert_eq!(Value::Number(42.0).to_string(), "42");
    }

    #[test]
    fn displays_large_numbers_without_overflow() {
        // i64 に収まる整数は, 整数で書く (-0 は 0)
        assert_eq!(
            Value::Number(-9.223372036854775e18).to_string(),
            "-9223372036854774784"
        );
        assert_eq!(Value::Number(-0.0).to_string(), "0");
        // 収まらない数は, i64 にしない (飽和して, 値が変わらないように)
        assert_eq!(Value::Number(1e19).to_string(), "10000000000000000000");
        assert_eq!(Value::Number(-1e19).to_string(), "-10000000000000000000");
        assert_eq!(Value::Number(f64::INFINITY).to_string(), "inf");
        assert_eq!(Value::Number(f64::NAN).to_string(), "NaN");
    }

    #[test]
    fn displays_fractional_numbers() {
        assert_eq!(Value::Number(3.5).to_string(), "3.5");
    }

    #[test]
    fn displays_booleans() {
        assert_eq!(Value::Boolean(true).to_string(), "#t");
        assert_eq!(Value::Boolean(false).to_string(), "#f");
    }

    #[test]
    fn displays_strings_with_quotes() {
        assert_eq!(Value::String("hi".to_string()).to_string(), "\"hi\"");
        assert_eq!(
            Value::String("a\"b\n".to_string()).to_string(),
            "\"a\\\"b\\n\""
        );
        assert_eq!(Value::String("hi".to_string()).to_display(), "hi");
        assert_eq!(Value::Number(1.0).to_display(), "1");
    }

    #[test]
    fn displays_lists_with_spaces() {
        let list =
            Value::List(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)].into());
        assert_eq!(list.to_string(), "(1 2 3)");
    }

    #[test]
    fn displays_nested_lists() {
        let list = Value::List(
            vec![
                Value::Symbol("a".to_string()),
                Value::List(vec![Value::Number(1.0), Value::Number(2.0)].into()),
            ]
            .into(),
        );
        assert_eq!(list.to_string(), "(a (1 2))");
    }

    #[test]
    fn displays_lambdas_with_and_without_names() {
        let lambda = |name: Option<&str>| {
            Value::Lambda(Arc::new(Lambda {
                name: name.map(str::to_string),
                params: vec![],
                rest: None,
                body: vec![],
                env: Env::new(),
            }))
        };
        assert_eq!(lambda(Some("f")).to_string(), "#<procedure:f>");
        assert_eq!(lambda(None).to_string(), "#<procedure>");
        assert_eq!(
            format!("{:?}", lambda(Some("f"))),
            "Lambda(Lambda(Some(\"f\")))"
        );
    }

    #[test]
    fn equality_holds_across_variants() {
        assert_eq!(Value::Number(1.0), Value::Number(1.0));
        assert_ne!(Value::Number(1.0), Value::Boolean(true));
        assert_eq!(
            Value::List(vec![Value::Number(1.0)].into()),
            Value::List(vec![Value::Number(1.0)].into())
        );
        assert_eq!(
            Value::Function("car".to_string()),
            Value::Function("car".to_string())
        );
    }

    #[test]
    fn only_false_is_false() {
        assert!(!Value::Boolean(false).is_true());
        assert!(Value::Nil.is_true());
        assert!(Value::List(vec![].into()).is_true());
    }
}
