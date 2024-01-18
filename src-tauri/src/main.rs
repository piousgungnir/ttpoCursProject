// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod solve_expr_rpc;
use solve_expr_rpc::rpc_solve_expr;

use tauri::generate_context;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn solve_expr(expression: &str) -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(rpc_solve_expr(expression));
    match result {
        Ok(result) => {
            format!("{}", result)
        }
        Err(err) => {
            format!("Error: {}", err)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![solve_expr])
        .run(generate_context!())
        .expect("error while running tauri application");
}
