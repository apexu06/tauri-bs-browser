// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::beatsaver::fetch_maps;
use types::bs_map::Map;

mod api;
mod types;

#[tauri::command]
async fn get_maps(query: &str, page: u32) -> Result<Vec<Map>, String> {
    let maps = fetch_maps(query, page).await;

    match maps {
        Ok(maps) => Ok(maps),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_maps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
