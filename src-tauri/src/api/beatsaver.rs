use std::error::Error;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::bs_map::{MapDetail, Maps};

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub active: bool,
}

pub async fn fetch_maps(
    query: &str,
    page_index: u32,
    sort_order: &str,
    min_bpm: u32,
    max_bpm: u32,
    filters: Vec<Filter>,
) -> Result<Vec<MapDetail>, Box<dyn Error>> {
    let mut query_string = format!(
        "https://api.beatsaver.com/search/text/{}?q={}&sortOrder={}&minBpm={}&maxBpm={}",
        page_index, query, sort_order, min_bpm, max_bpm
    )
    .to_string();

    for filter in filters.iter() {
        match filter.active {
            true => {
                query_string =
                    query_string + &format!("&{}={}", filter.name, filter.active).to_string();
            }
            false => {}
        }
    }

    let resp: Maps = reqwest::get(query_string).await?.json().await?;

    Ok(resp.docs)
}

pub async fn fetch_map_details(id: &String) -> Result<MapDetail, Box<dyn Error>> {
    let resp: MapDetail = reqwest::get(format!("https://api.beatsaver.com/maps/id/{}", id))
        .await?
        .json()
        .await?;
    Ok(resp)
}
