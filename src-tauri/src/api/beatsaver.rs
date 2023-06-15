use std::error::Error;

use crate::types::bs_map::{Map, Maps};

pub async fn fetch_maps(
    query: &str,
    page_index: u32,
    sortOrder: &str,
) -> Result<Vec<Map>, Box<dyn Error>> {
    let resp: Maps = reqwest::get(format!(
        "https://api.beatsaver.com/search/text/{}?q={}&sortOrder={}",
        page_index, query, sortOrder
    ))
    .await?
    .json()
    .await?;

    Ok(resp.docs)
}

pub async fn fetch_map_details(id: &String) -> Result<Map, Box<dyn Error>> {
    let resp: Map = reqwest::get(format!("https://api.beatsaver.com/maps/id/{}", id))
        .await?
        .json()
        .await?;
    Ok(resp)
}
