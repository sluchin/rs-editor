//! ロックの補助: 評価が panic しても、エディタを続けられるようにします.

use std::sync::{Mutex, MutexGuard, PoisonError};

/// `Mutex` を、持ったまま panic したスレッド (poisoned) があっても取れるようにします.
///
/// Scheme の評価は別のスレッドで動かし、panic は `Evaluator` が捕まえて失敗にします.
/// そのとき持っていたロックが poisoned のままだと、`lock().unwrap()` が以後ずっと panic して、エディタが止まります.
/// Emacs はエラーのあとも続けるので、ここでも続けます (途中で止まった更新は、そのまま残る).
pub trait LockExt<T> {
    /// ロックを取ります. poisoned でも、中身をそのまま使います.
    fn lock_recover(&self) -> MutexGuard<'_, T>;
}

impl<T> LockExt<T> for Mutex<T> {
    fn lock_recover(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
