use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct MapLayer {
    pub name: String,
    pub data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<MapLayer>,
}

pub fn load_map(path: &str) -> Result<TiledMap, String> {
    let content = fs::read_to_string(path)?;
    let map: TiledMap = serde_json::from_str(&content)?;
    Ok(map)
}
