#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use serde::{Deserialize, Serialize};

mod scheme;
use scheme::evaluator::Evaluator;

#[derive(Serialize, Deserialize)]
struct EvalRequest {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct EvalResponse {
    result: String,
    error: Option<String>,
}

#[tauri::command]
fn eval_scheme(code: String, state: State<'_, Evaluator>) -> EvalResponse {
    match state.eval(&code) {
        Ok(result) => EvalResponse {
            result,
            error: None,
        },
        Err(e) => EvalResponse {
            result: String::new(),
            error: Some(e),
        },
    }
}

fn main() {
    let evaluator = Evaluator::new();

    tauri::Builder::default()
        .manage(evaluator)
        .invoke_handler(tauri::generate_handler![eval_scheme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
