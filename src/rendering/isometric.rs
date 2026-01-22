use bevy::prelude::*;
use serde::Deserialize;
use std::fs;
use crate::components::world::{Tile, TileType};
use crate::constants::{TILE_SIZE, TILE_HEIGHT, z_index};

#[derive(Deserialize)]
struct MapLayer {
    name: String,
    data: Vec<u32>,
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
struct TiledMap {
    width: u32,
    height: u32,
    layers: Vec<MapLayer>,
}

pub fn grid_to_screen(grid_x: i32, grid_y: i32) -> Vec2 {
    // Изометрическая формула
    let x = (grid_x - grid_y) as f32 * TILE_SIZE / 2.0;
    let y = -(grid_x + grid_y) as f32 * TILE_HEIGHT / 4.0;  
    Vec2::new(x, y)
}

pub fn screen_to_grid(screen_x: f32, screen_y: f32) -> (i32, i32) {
    let grid_x = ((screen_x / (TILE_SIZE / 2.0)) - (screen_y / (TILE_HEIGHT / 4.0))) / 2.0;
    let grid_y = (-(screen_y / (TILE_HEIGHT / 4.0)) - (screen_x / (TILE_SIZE / 2.0))) / 2.0;
    (grid_x.round() as i32, grid_y.round() as i32)
}

pub fn setup_isometric_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 999.0),
    ));
    
    println!("Camera created!");
}

// ============================================
// СИСТЕМА: ЗАГРУЗКА КАРТЫ ИЗ JSON
// ============================================
pub fn spawn_map_from_json(mut commands: Commands) {
    match load_map("assets/world_map_v1.json") {
        Ok(map) => {
            println!(" Map loaded: {}x{}", map.width, map.height);
            render_tiled_map(&map, &mut commands);
        }
        Err(e) => {
            println!(" Error loading map: {}", e);
            println!(" Generating map...");
            spawn_procedural_map(commands);
        }
    }
}

fn load_map(path: &str) -> Result<TiledMap, String> {
    match fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(map) => Ok(map),
            Err(e) => Err(format!("JSON error: {}", e)),
        },
        Err(e) => Err(format!("File not found: {}", e)),
    }
}

fn render_tiled_map(map: &TiledMap, commands: &mut Commands) {
    let mut tile_count = 0;
    
    // Центрируем карту
    let offset_x = -(map.width as i32) / 2;
    let offset_y = -(map.height as i32) / 2;
    
    for layer in &map.layers {
        println!("  Layer : {}", layer.name);
        
        for (index, &tile_id) in layer.data.iter().enumerate() {
            if tile_id == 0 { continue; }
            
            let x = (index as u32 % layer.width) as i32 + offset_x;
            let y = (index as u32 / layer.width) as i32 + offset_y;

            let screen_pos = grid_to_screen(x, y);
            
            let (color, z, tile_type) = get_tile_properties(tile_id);
        
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(screen_pos.x, screen_pos.y, z),
                Tile::new(x, y, tile_type),
            ));
            
            tile_count += 1;
        }
    }
    
    println!("Created {} tails!", tile_count);
}

fn get_tile_properties(tile_id: u32) -> (Color, f32, TileType) {
    use crate::constants::colors;
    
    match tile_id {
        1..=49 | 51 | 53..=60 => (
            colors::GRASS_LIGHT,
            z_index::GROUND,
            TileType::GrassLight,
        ),
        50 => (
            colors::GRASS_DARK,
            z_index::GROUND,
            TileType::GrassDark,
        ),

        52 | 78..=90 => (
            colors::ROAD,
            z_index::ROADS,
            TileType::Stone,
        ),
        

        61..=76 => (
            colors::DIRT,
            z_index::GROUND,
            TileType::Dirt,
        ),
        
        77 => (
            colors::WATER,
            z_index::WATER,
            TileType::Water,
        ),
        
        91..=104 => (
            colors::HOUSE_WALL,
            z_index::BUILDINGS,
            TileType::Stone,
        ),
        105 => (
            colors::HOUSE_ROOF,
            z_index::BUILDINGS,
            TileType::Stone,
        ),
    
        _ => (
            colors::GRASS_DARK,
            z_index::GROUND,
            TileType::GrassDark,
        ),
    }
}

fn spawn_procedural_map(mut commands: Commands) {
    let map_size = 30;
    let mut tile_count = 0;
    
    for grid_x in -map_size..=map_size {
        for grid_y in -map_size..=map_size {
            let tile_type = if (grid_x + grid_y) % 2 == 0 {
                TileType::GrassLight
            } else {
                TileType::GrassDark
            };
            
            let screen_pos = grid_to_screen(grid_x, grid_y);
            
            commands.spawn((
                Sprite {
                    color: tile_type.color(),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(screen_pos.x, screen_pos.y, z_index::GROUND),
                Tile::new(grid_x, grid_y, tile_type),
            ));
            
            tile_count += 1;
        }
    }
    
    println!("{} tails", tile_count);
}