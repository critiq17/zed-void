use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use crate::components::world::{Tile, TileType};
use crate::components::z_layer::{ZLayerManager, EntityType};


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

#[derive(Clone)]
pub struct TilesetHandle {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Clone)]
pub struct Tilesets {
    pub tileset1: TilesetHandle,
    pub tileset2: TilesetHandle,
}

pub fn grid_to_screen(grid_x: i32, grid_y: i32) -> Vec2 {
    let x = (grid_x - grid_y) as f32 * 32.0;
    let y = (grid_x + grid_y) as f32 * 16.0;
    Vec2::new(x, -y)
}

pub fn screen_to_grid(screen_x: f32, screen_y: f32) -> (i32, i32) {
    let screen_y = -screen_y;
    let grid_x = ((screen_x / 32.0) + (screen_y / 16.0)) / 2.0;
    let grid_y = ((screen_y / 16.0) - (screen_x / 32.0)) / 2.0;
    (grid_x.round() as i32, grid_y.round() as i32)
}

pub fn setup_isometric_camera(
    mut commands: Commands,
    z_layer_manager: Res<ZLayerManager>
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            0.0,
            0.0,
            z_layer_manager.get_layer(EntityType::Camera)
        ),
    ));
    println!("Camera created!");
}

pub fn load_tileset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture1 = asset_server.load("isometric_tiles.png");
    let layout1 = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        26,  // columns
        25,  // rows (650 / 26 = 25)
        None,
        None,
    );

    // ТАЙЛСЕТ 2: firstgid=651, columns=24, tilecount=576
    let texture2 = asset_server.load("isometric_tiles.png");
    let layout2 = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        24,  // columns
        24,  // rows (576 / 24 = 24)
        None,
        None,
    );

    commands.insert_resource(Tilesets {
        tileset1: TilesetHandle {
            texture: texture1,
            layout: texture_atlases.add(layout1),
        },
        tileset2: TilesetHandle {
            texture: texture2,
            layout: texture_atlases.add(layout2),
        },
    });

    println!("Tilesets loaded");
}

pub fn spawn_map_from_json(
    mut commands: Commands,
    tilesets: Res<Tilesets>,
    z_layer_manager: Res<ZLayerManager>,
) {
    match load_map("assets/world_map_v1.json") {
        Ok(map) => {
            println!("Map loaded {}x{}", map.width, map.height);
            render_tiled_map(&map, &mut commands, &tilesets, &z_layer_manager);
        }
        Err(e) => {
            println!(" Error loading map {}", e);
            println!(" Generating test map...");
            spawn_procedural_map(commands, tilesets.clone(), z_layer_manager);
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

fn render_tiled_map(
    map: &TiledMap,
    commands: &mut Commands,
    tilesets: &Tilesets,
    z_layer_manager: &ZLayerManager
) {
    let mut tile_count = 0;

    let offset_x = -(map.width as i32) / 2;
    let offset_y = -(map.height as i32) / 2;

    for layer in &map.layers {
        println!("  Rendering layer: {}", layer.name);

        for (index, &tile_id) in layer.data.iter().enumerate() {
            if tile_id == 0 { continue; }

            let x = (index as u32 % layer.width) as i32 + offset_x;
            let y = (index as u32 / layer.width) as i32 + offset_y;

            let screen_pos = grid_to_screen(x, y);

            let (tileset, atlas_index, tile_type) = if tile_id < 651 {
               let atlas_idx = (tile_id - 1) as usize;
                (&tilesets.tileset1, atlas_idx, TileType::GrassDark)
            } else {
               let atlas_idx = (tile_id - 651) as usize;
                let tile_type = match tile_id {
                    853 => TileType::GrassLight,
                    1187 => TileType::Water,
                    809 => TileType::Dirt,
                    995 | 994 => TileType::Stone,
                    _ => TileType::GrassDark,
                };
                (&tilesets.tileset2, atlas_idx, tile_type)
            };

            let base_z = z_layer_manager.get_layer(EntityType::Tile);
            let z = base_z + (y as f32 * 0.01);

            commands.spawn((
                Sprite {
                    image: tileset.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: tileset.layout.clone(),
                        index: atlas_index,
                    }),
                    custom_size: Some(Vec2::new(64.0, 32.0)),
                    ..default()
                },
                Transform::from_xyz(screen_pos.x, screen_pos.y, z),
                Tile::new(x, y, tile_type),
            ));

            tile_count += 1;
        }
    }

    println!(" Created {} tiles!", tile_count);
}

fn spawn_procedural_map(
    mut commands: Commands,
    tilesets: Tilesets,
    z_layer_manager: Res<ZLayerManager>
) {
    let map_size: i32 = 15;
    let mut tile_count = 0;

    for grid_x in -map_size..=map_size {
        for grid_y in -map_size..=map_size {
            let (tile_id, tile_type) = if (grid_x.abs() + grid_y.abs()) < 5 {
                (1187, TileType::Water)
            } else {
                (853, TileType::GrassLight)
            };

            let atlas_index = (tile_id - 651) as usize;
            let screen_pos = grid_to_screen(grid_x, grid_y);
            let z = z_layer_manager.get_layer(EntityType::Tile) + (grid_y as f32 * 0.01);

            commands.spawn((
                Sprite {
                    image: tilesets.tileset2.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: tilesets.tileset2.layout.clone(),
                        index: atlas_index,
                    }),
                    custom_size: Some(Vec2::new(64.0, 32.0)),
                    ..default()
                },
                Transform::from_xyz(screen_pos.x, screen_pos.y, z),
                Tile::new(grid_x, grid_y, tile_type),
            ));

            tile_count += 1;
        }
    }

    println!(" Generated {} tiles", tile_count);
}
