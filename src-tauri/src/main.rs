// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::Result;
fn main() -> Result<(), anyhow::Error> {
    let result = myapp_lib::run();
    match result {
        Err(e) => {
            eprintln!("Application error: {}", e);
            return Err(e);
        }
        _ => {}
    };
    Ok(())
}
