use anyhow::Result;
use std::fmt::Debug;

use appcore::*;
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
#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), anyhow::Error> {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init());
    //mobile only
    #[cfg(mobile)]
    {
        builder = builder.plugin(tauri_plugin_barcode_scanner::init());
    }
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_cli::init());
    }

    builder
        .setup(|app| {
            print!("setting up");
            let apphandle = app.handle().clone();
            //Desktop only
            #[cfg(desktop)]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::TrayIconBuilder;
                use tauri::Manager;
                use tauri_plugin_cli::CliExt;
                let quit_i = check(MenuItem::with_id(
                    &apphandle,
                    "quit",
                    "Quit",
                    true,
                    None::<&str>,
                ));
                let show_i = check(MenuItem::with_id(
                    &apphandle,
                    "show",
                    "Show",
                    true,
                    None::<&str>,
                ));
                let menu = check(Menu::with_items(&apphandle, &[&quit_i, &show_i])); //窗口上侧菜单

                let _tray: tauri::tray::TrayIcon = check(
                    TrayIconBuilder::new()
                        .on_menu_event(|apphandle, event| match event.id.as_ref() {
                            "quit" => {
                                println!("quit menu item was clicked");
                                apphandle.exit(0);
                            }
                            "show" => {
                                println!("show menu item was clicked");
                                let _ = apphandle.get_webview_window("main").unwrap().show();
                            }
                            _ => {
                                println!("menu item {:?} not handled", event.id);
                            }
                        })
                        .menu(&menu)
                        .show_menu_on_left_click(true)
                        .build(&apphandle), //系统托盘
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
                    //命令行参数解析段 todo 简单自己托管sever服务：群和接受信息服务
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
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, exit_app]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
