use bevy::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum MapTileType {
    Empty,
    GrassLight,
    GrassDark, 
    Dirt,
    Stone,
    House,
}

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
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<MapLayer>,
}

#[derive(Component, Clone, Copy)]
pub struct Tile {
    pub grid_x: i32, 
    pub grid_y: i32,      
    pub tile_type: TileType,  
}

impl Tile {
    pub fn new(grid_x: i32, grid_y: i32, tile_type: TileType) -> Self {
        Self {
            grid_x,
            grid_y,
            tile_type,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    GrassLight,  
    GrassDark,   
    Dirt,       
    Stone,       
    Water,       
}

impl TileType {

    pub fn color(&self) -> Color {
        use crate::constants::colors;
        match self {
            TileType::GrassLight => colors::GRASS_LIGHT,
            TileType::GrassDark => colors::GRASS_DARK,
            TileType::Dirt => colors::DIRT,
            TileType::Stone => colors::STONE,
            TileType::Water => Color::srgb(0.2, 0.4, 0.8),
        }
    }
    
   
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Water => false, 
            _ => true,
        }
    }
}


#[derive(Component)]
pub struct Building {
    pub building_type: BuildingType,
    pub grid_x: i32,
    pub grid_y: i32,
    pub width: u32,   
    pub height: u32,  
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuildingType {
    House,
    Shed,
    Wall,
}


#[derive(Component)]
pub struct Furniture {
    pub furniture_type: FurnitureType,
    pub is_blocking: bool,  
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FurnitureType {
    Table,
    Chair,
    Bed,
    Container,  
}