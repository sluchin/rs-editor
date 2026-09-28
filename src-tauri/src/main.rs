#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::State;

mod scheme;
use scheme::evaluator::Evaluator;

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

#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file_content(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn path_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}

#[tauri::command]
fn home_dir() -> Result<String, String> {
    std::env::var("HOME").map_err(|e| e.to_string())
}

fn main() {
    let evaluator = Evaluator::new();

    tauri::Builder::default()
        .manage(evaluator)
        .invoke_handler(tauri::generate_handler![
            eval_scheme,
            read_file_content,
            write_file_content,
            path_exists,
            home_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
