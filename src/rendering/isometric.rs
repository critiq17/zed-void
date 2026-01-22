use bevy::prelude::*;
use serde::Deserialize;
use std::fs;
use crate::components::world::{Tile, TileType};
use crate::constants::{TILE_SIZE, TILE_HEIGHT};
use crate::constants::z_index::GROUND;

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
    let x = (grid_x - grid_y) as f32 * TILE_SIZE / 2.0;
    let y = (grid_x + grid_y) as f32 * TILE_HEIGHT / 2.0;
    Vec2::new(x, y)
}

pub fn setup_isometric_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}

pub fn spawn_isometric_map(mut commands: Commands) {
    println!("Generating procedural map...");
    
    let map_size = 25;
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
                Transform::from_xyz(screen_pos.x, screen_pos.y, GROUND),
                Tile::new(grid_x, grid_y, tile_type),
            ));
            tile_count += 1;
        }
    }
    println!("Created {} tiles!", tile_count);
}

pub fn spawn_map_from_json(mut commands: Commands) {
    match load_map("assets/world_map_v1.json") {
        Ok(map) => {
            println!("Map loaded: {}x{}", map.width, map.height);
            render_tiled_map(&map, &mut commands);
        }
        Err(e) => {
            println!("Map error: {}. Using procedural.", e);
            spawn_isometric_map(commands);  // ← ФИКС: убрал &mut
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
    for layer in &map.layers {
        println!("  Layer: {}", layer.name);
        
        for (i, &tile_id) in layer.data.iter().enumerate() {
            if tile_id == 0 { continue; }
            
            let x = (i as u32 % layer.width) as i32;
            let y = (i as u32 / layer.width) as i32;
            let screen_pos = grid_to_screen(x, y);
            
         
            let color = match tile_id {
                1..=49 => TileType::GrassLight.color(),    
                50     => TileType::GrassDark.color(),      
                51     => TileType::GrassLight.color(),
                52     => TileType::Stone.color(),      
                53..=60 => TileType::GrassDark.color(),
                61..=76 => TileType::Dirt.color(),      
                77     => Color::srgb(0.1, 0.3, 0.8),   
                78..=90 => TileType::Stone.color(),     
                91..=104 => Color::srgb(0.6, 0.4, 0.2),  
                105    => Color::srgb(0.4, 0.2, 0.1),  
                _ => TileType::GrassDark.color(),
            };

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(screen_pos.x, screen_pos.y, GROUND),
                Tile::new(x, y, TileType::GrassLight),
            ));
        }
    }
}
