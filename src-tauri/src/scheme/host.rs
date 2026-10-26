//! 評価器から, エディタ (画面) を動かす窓口.
//!
//! 評価は Rust で行うので, エディタの操作 (`insert`, `point` など) は, 画面に頼んで, 返事を待ちます.
//! アプリでは, [`Bridge`] が, 頼みを画面へ送って (Tauri のイベント), 返事 (Tauri のコマンド) を待ちます.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Mutex};
use std::time::Duration;

use serde::Serialize;

use crate::scheme::value::Value;

/// ミニバッファの入力 (`read-string`) を待つ長さ (人が入力するので, 長くする).
pub const READ_TIMEOUT: Duration = Duration::from_secs(60 * 60);

/// エディタ. 操作の名前と引数を受け取って, 結果を返します.
pub trait Host: Sync {
    /// エディタを操作します.
    ///
    /// # Arguments
    ///
    /// * `op` - 操作の名前 (`point`, `insert` など).
    /// * `args` - 引数.
    ///
    /// # Returns
    ///
    /// 結果, または失敗の内容.
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String>;
}

/// エディタのないところ (テストや, 画面のない評価). 操作は, 全て失敗します.
pub struct NoHost;

impl Host for NoHost {
    fn call(&self, op: &str, _args: Vec<Value>) -> Result<Value, String> {
        Err(format!("No editor is attached ({})", op))
    }
}

/// 画面への頼み.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct EditorRequest {
    /// 返事と組にする番号.
    pub id: u64,
    /// 操作の名前.
    pub op: String,
    /// 引数.
    pub args: Vec<serde_json::Value>,
}

/// 画面に頼んで, 返事を待つ仕組み.
pub struct Bridge {
    /// 次の頼みに付ける番号.
    next_id: AtomicU64,
    /// 返事を待っている頼み (番号と, 返事を渡す先).
    pending: Mutex<HashMap<u64, mpsc::Sender<Result<Value, String>>>>,
    /// 返事を待つ長さ (過ぎたら, 失敗にする. `read-string` は [`READ_TIMEOUT`]).
    timeout: Duration,
}

impl Bridge {
    /// 作ります.
    ///
    /// # Arguments
    ///
    /// * `timeout` - 返事を待つ長さ (過ぎたら, 失敗にする).
    ///
    /// # Returns
    ///
    /// 仕組み.
    pub fn new(timeout: Duration) -> Self {
        Bridge {
            next_id: AtomicU64::new(1),
            pending: Mutex::new(HashMap::new()),
            timeout,
        }
    }

    /// 画面に頼んで, 返事を待ちます.
    ///
    /// # Arguments
    ///
    /// * `op` - 操作の名前.
    /// * `args` - 引数.
    /// * `send` - 頼みを画面へ送る関数.
    ///
    /// # Returns
    ///
    /// 返事, または失敗の内容.
    pub fn request(
        &self,
        op: &str,
        args: Vec<Value>,
        send: &dyn Fn(EditorRequest) -> Result<(), String>,
    ) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::channel();
        self.pending.lock().unwrap().insert(id, sender);
        let sent = send(EditorRequest {
            id,
            op: op.to_string(),
            args: args.iter().map(to_json).collect(),
        });
        let timeout = if op == "read-string" {
            READ_TIMEOUT
        } else {
            self.timeout
        };
        let answer = sent.and_then(|()| {
            receiver
                .recv_timeout(timeout)
                .unwrap_or_else(|_| Err(format!("The editor did not answer ({})", op)))
        });
        self.pending.lock().unwrap().remove(&id);
        answer
    }

    /// 画面からの返事を, 待っている頼みに渡します.
    ///
    /// # Arguments
    ///
    /// * `id` - 頼みの番号.
    /// * `value` - 結果.
    /// * `error` - 失敗の内容 (成功したときは `None`).
    ///
    /// # Returns
    ///
    /// 待っている頼みがあれば `true`.
    pub fn reply(&self, id: u64, value: serde_json::Value, error: Option<String>) -> bool {
        let Some(sender) = self.pending.lock().unwrap().remove(&id) else {
            return false;
        };
        // 待つのをやめていても (時間切れ), かまわない
        let _ = sender.send(match error {
            Some(e) => Err(e),
            None => Ok(from_json(value)),
        });
        true
    }
}

/// [`Bridge`] を使うエディタ.
pub struct BridgeHost<'a, F> {
    /// 返事を待つ仕組み.
    bridge: &'a Bridge,
    /// 頼みを画面へ送る関数.
    send: F,
}

impl<'a, F> BridgeHost<'a, F>
where
    F: Fn(EditorRequest) -> Result<(), String> + Sync,
{
    /// 作ります.
    ///
    /// # Arguments
    ///
    /// * `bridge` - 返事を待つ仕組み.
    /// * `send` - 頼みを画面へ送る関数.
    ///
    /// # Returns
    ///
    /// エディタ.
    pub fn new(bridge: &'a Bridge, send: F) -> Self {
        BridgeHost { bridge, send }
    }
}

impl<F> Host for BridgeHost<'_, F>
where
    F: Fn(EditorRequest) -> Result<(), String> + Sync,
{
    fn call(&self, op: &str, args: Vec<Value>) -> Result<Value, String> {
        self.bridge.request(op, args, &self.send)
    }
}

/// 値を, 画面へ送る JSON にします. 記号と手続きは, 文字列にします.
///
/// # Arguments
///
/// * `value` - 値.
///
/// # Returns
///
/// JSON.
pub fn to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Nil => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => serde_json::json!(n),
        Value::String(s) | Value::Symbol(s) => serde_json::Value::String(s.clone()),
        Value::List(items) => serde_json::Value::Array(items.iter().map(to_json).collect()),
        other => serde_json::Value::String(other.to_string()),
    }
}

/// 画面からの JSON を, 値にします.
///
/// # Arguments
///
/// * `json` - JSON.
///
/// # Returns
///
/// 値.
pub fn from_json(json: serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(f64::NAN)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(items) => Value::List(items.into_iter().map(from_json).collect()),
        object => Value::String(object.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn no_host_fails() {
        assert_eq!(
            NoHost.call("point", vec![]),
            Err("No editor is attached (point)".to_string())
        );
    }

    #[test]
    fn converts_values_to_json_and_back() {
        let value = Value::List(
            vec![
                Value::Nil,
                Value::Boolean(true),
                Value::Number(1.5),
                Value::String("a".to_string()),
                Value::Symbol("b".to_string()),
                Value::Function("car".to_string()),
            ]
            .into(),
        );
        let json = to_json(&value);
        assert_eq!(json, json!([null, true, 1.5, "a", "b", "#<procedure:car>"]));
        assert_eq!(
            from_json(json),
            Value::List(
                vec![
                    Value::Nil,
                    Value::Boolean(true),
                    Value::Number(1.5),
                    Value::String("a".to_string()),
                    Value::String("b".to_string()),
                    Value::String("#<procedure:car>".to_string()),
                ]
                .into()
            )
        );
        assert_eq!(
            from_json(json!({"a": 1})),
            Value::String("{\"a\":1}".to_string())
        );
    }

    #[test]
    fn bridge_passes_the_reply_to_the_request() {
        let bridge = Arc::new(Bridge::new(Duration::from_secs(5)));
        let answerer = Arc::clone(&bridge);
        let host = BridgeHost::new(&bridge, move |request: EditorRequest| {
            assert_eq!(request.op, "insert");
            assert_eq!(request.args, vec![json!("x")]);
            let answerer = Arc::clone(&answerer);
            std::thread::spawn(move || {
                // 待っている頼みに, 返事が届くまで送り直す
                while !answerer.reply(request.id, json!(3), None) {
                    std::thread::yield_now();
                }
            });
            Ok(())
        });
        assert_eq!(
            host.call("insert", vec![Value::String("x".to_string())]),
            Ok(Value::Number(3.0))
        );
    }

    #[test]
    fn bridge_passes_the_error_of_the_reply() {
        let bridge = Bridge::new(Duration::from_secs(5));
        let result = std::thread::scope(|scope| {
            scope.spawn(|| {
                while !bridge.reply(1, json!(null), Some("bad".to_string())) {
                    std::thread::yield_now();
                }
            });
            bridge.request("point", vec![], &|_| Ok(()))
        });
        assert_eq!(result, Err("bad".to_string()));
    }

    #[test]
    fn bridge_waits_longer_for_the_input() {
        // 返事を待つ長さが 0 でも, 入力 (read-string) は待つ
        let bridge = Bridge::new(Duration::ZERO);
        let result = std::thread::scope(|scope| {
            scope.spawn(|| {
                std::thread::sleep(Duration::from_millis(20));
                while !bridge.reply(1, json!("abc"), None) {
                    std::thread::yield_now();
                }
            });
            bridge.request("read-string", vec![], &|_| Ok(()))
        });
        assert_eq!(result, Ok(Value::String("abc".to_string())));
    }

    #[test]
    fn bridge_fails_when_sending_fails_or_nobody_answers() {
        let bridge = Bridge::new(Duration::from_millis(10));
        assert_eq!(
            bridge.request("point", vec![], &|_| Err("closed".to_string())),
            Err("closed".to_string())
        );
        assert_eq!(
            bridge.request("point", vec![], &|_| Ok(())),
            Err("The editor did not answer (point)".to_string())
        );
        // 時間切れの後の返事は, 捨てる
        assert!(!bridge.reply(2, json!(1), None));
    }
}
