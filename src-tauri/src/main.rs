// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::beatsaver::{fetch_maps, Filter};
use types::bs_map::MapDetail;

mod api;
mod types;

#[tauri::command]
async fn get_maps(
    query: &str,
    page: u32,
    sort_mode: &str,
    filters: Vec<Filter>,
    min_bpm: u32,
    max_bpm: u32,
    start_date: &str,
    end_date: &str,
    mut current_maps: Vec<MapDetail>,
) -> Result<Vec<MapDetail>, String> {
    let new_maps = fetch_maps(
        query, page, sort_mode, min_bpm, max_bpm, filters, start_date, end_date,
    )
    .await;

    match new_maps {
        Ok(mut maps) => {
            current_maps.append(&mut maps);
            Ok(current_maps)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_maps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
