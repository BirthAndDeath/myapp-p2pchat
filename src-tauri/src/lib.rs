// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn send(message: &str) -> bool {
    //chat_core::ChatCore::sendmessage(message.to_string());
    true
}
/*app_data_dir()		数据库、配置
app_local_data_dir()	缓存、日志
app_config_dir()		用户配置
temp_dir() */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let core =
        //chat_core::ChatCore::try_init(&chat_core::CoreConfig::new("sqlite:///path/to/database.db"));
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        //.manage()
        .invoke_handler(tauri::generate_handler![send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
