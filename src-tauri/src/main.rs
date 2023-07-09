// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;

use dotenv::dotenv;
use crate::dao::init;
use crate::dao::init::init_pool;

pub mod cmd;
pub mod services;
pub mod dao;
pub mod common;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_pool().await;
    init::init_database().await.expect("初始化数据库失败");
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            cmd::add_comic
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
