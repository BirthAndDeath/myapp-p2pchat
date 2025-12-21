// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Error;
use anyhow::{Context, Result};
fn main() -> Result<(), anyhow::Error> {
    let result = myapp_lib::run();
    match result {
        Err(e) => {}
        _ => {}
    };
    Ok(())
}
