use anyhow::Result;
use std::fmt::Debug;

use tauri::AppHandle;

mod port;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn exit_app(app: AppHandle) {
    app.exit(0);
}
pub fn check<T, E: Debug>(r: Result<T, E>) -> T {
    match r {
        Ok(r) => {
            return r;
        }
        Err(e) => {
            eprintln!("Failed : {:?}", e);
            panic!()
        }
    }
}

/// 启动 Tauri 应用程序的主函数
///

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), anyhow::Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            print!("setting up");
            Ok(())
        })
        //.manage()
        .invoke_handler(tauri::generate_handler![greet, exit_app]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
