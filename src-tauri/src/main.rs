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

#[tokio::main]
async fn main() {
    dotenv().ok();
    init::init_database().await.expect("初始化数据库失败");
    init_pool().await;
    init::migrate().await.expect("迁移失败");
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            cmd::add_comic,
            cmd::get_store_comic,
            cmd::comic_delete,
            cmd::read_comic,
            cmd::get_comic
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
