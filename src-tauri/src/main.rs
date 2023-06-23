#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::beatsaver::{fetch_maps, Filter};
use ts_rs::TS;
use types::bs_map::MapDetail;

mod api;
mod types;

#[derive(TS, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    query: String,
    page: u32,
    sort_order: String,
    filters: Vec<Filter>,
    min_bpm: u32,
    max_bpm: u32,
    start_date: String,
    end_date: String,
}

#[tauri::command]
async fn get_maps(
    params: QueryParams,
    mut current_maps: Vec<MapDetail>,
) -> Result<Vec<MapDetail>, String> {
    let new_maps = fetch_maps(params).await;

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
