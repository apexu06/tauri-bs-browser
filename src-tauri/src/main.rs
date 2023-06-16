// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::beatsaver::fetch_maps;
use types::bs_map::Map;

mod api;
mod types;

#[tauri::command]
async fn get_maps(
    query: &str,
    page: u32,
    sort_mode: &str,
    ranked: bool,
    qualified: bool,
    curated: bool,
    mut current_maps: Vec<Map>,
) -> Result<Vec<Map>, String> {
    let new_maps = fetch_maps(query, page, sort_mode, ranked, qualified, curated).await;

    match new_maps {
        Ok(mut maps) => {
            if qualified {
                for map in maps.iter_mut() {
                    map.qualified = true;
                }
            }
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
