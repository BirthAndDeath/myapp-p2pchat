use anyhow::Result;
use std::fmt::Debug;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder};
use tauri::AppHandle;
use tauri_plugin_cli::CliExt;
mod port;
use tauri::Manager;

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
            let apphandle = app.handle();

            let quit_i = check(MenuItem::with_id(
                apphandle,
                "quit",
                "Quit",
                true,
                None::<&str>,
            ));
            let menu = check(Menu::with_items(apphandle, &[&quit_i]));

            let tray: tauri::tray::TrayIcon = check(
                TrayIconBuilder::new()
                    .menu(&menu)
                    .show_menu_on_left_click(true)
                    .build(apphandle),
            );
            let window = apphandle.get_webview_window("main");
            if let Some(window) = window {
                if cfg!(debug_assertions) {
                    if let Ok(url) = window.url() {
                        println!(">>> Tauri will load: {}", url);
                    }
                }
            } else {
                eprintln!("Error: Could not get main window");
            }
            match app.handle().cli().matches() {
                // `matches` here is a Struct with { args, subcommand }.
                // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
                // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
                Ok(matches) => {
                    #[cfg(debug_assertions)]
                    {
                        println!("matches=>{:?}", matches);
                        matches
                            .args
                            .iter()
                            .for_each(|(key, value)| println!("{}=>{:?}", key, value));
                    }
                    if let Some(arg_data) = matches.args.get("server") {
                        if &arg_data.value == true {
                            println!("server mode!");
                        }
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
        //.manage()
        .invoke_handler(tauri::generate_handler![greet, exit_app]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
