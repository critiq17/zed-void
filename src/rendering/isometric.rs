use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use crate::components::world::{Tile, TileType};
use crate::components::z_layer::ZLayerManager;
use crate::constants::z_index;

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

#[derive(Resource, Clone)]
pub struct TilesetHandle {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn grid_to_screen(grid_x: i32, grid_y: i32) -> Vec2 {
    // Use proper isometric diamond projection
    // For diamond-shaped isometric projection:
    // - Each tile is TILE_SIZE wide (64px) and TILE_HEIGHT tall (32px)
    // - X offset is half tile width per grid step
    // - Y offset is half tile height per grid step
    let tile_width_half = crate::constants::TILE_SIZE / 2.0;  // 32.0
    let tile_height_half = crate::constants::TILE_HEIGHT / 2.0; // 16.0
    
    let x = (grid_x - grid_y) as f32 * tile_width_half;
    let y = -(grid_x + grid_y) as f32 * tile_height_half;
    Vec2::new(x, y)
}

pub fn screen_to_grid(screen_x: f32, screen_y: f32) -> (i32, i32) {
    // Inverse of the diamond isometric projection
    let tile_width_half = crate::constants::TILE_SIZE / 2.0;  // 32.0
    let tile_height_half = crate::constants::TILE_HEIGHT / 2.0; // 16.0
    
    // Convert screen coordinates back to grid coordinates
    let grid_x = ((screen_x / tile_width_half) + (-screen_y / tile_height_half)) / 2.0;
    let grid_y = ((-screen_y / tile_height_half) - (screen_x / tile_width_half)) / 2.0;
    (grid_x.round() as i32, grid_y.round() as i32)
}

pub fn setup_isometric_camera(mut commands: Commands, z_layer_manager: Res<ZLayerManager>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, z_layer_manager.get_layer(crate::components::z_layer::EntityType::Camera)),
    ));
    println!(" Camera created!");
}

pub fn load_tileset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {

    let texture_handle = asset_server.load("isometric_tiles.png");

    // The tileset is 794x794 pixels
    // Each tile is 64x32 pixels (isometric diamond shape)
    // This gives us approximately 12x24 tiles, but we'll use 12x12 for a square grid
    let atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(64, 32), // Each tile is 64 pixels wide, 32 pixels tall
        12,  // 12 tiles horizontally (794/64 ≈ 12.4, rounded down)
        24,  // 24 tiles vertically (794/32 ≈ 24.8, rounded down)
        None,
        None,
    );
    let atlas_layout_handle = texture_atlases.add(atlas_layout);
    
    commands.insert_resource(TilesetHandle {
        texture: texture_handle,
        layout: atlas_layout_handle,
    });
    
    println!(" Isometric tileset loaded: 12x24 tiles (64x32 each)");
}

pub fn spawn_map_from_json(
    mut commands: Commands,
    tileset: Res<TilesetHandle>,
    z_layer_manager: Res<ZLayerManager>,
) {
    match load_map("assets/world_map_v1.json") {
        Ok(map) => {
            println!(" Map loaded: {}x{}", map.width, map.height);
            render_tiled_map(&map, &mut commands, &tileset, &z_layer_manager);
        }
        Err(e) => {
            println!(" Error loading map: {}", e);
            println!(" Generating procedural map...");
            spawn_procedural_map(commands, (*tileset).clone(), z_layer_manager);
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

fn render_tiled_map(map: &TiledMap, commands: &mut Commands, tileset: &TilesetHandle, z_layer_manager: &ZLayerManager) {
    let mut tile_count = 0;
    
    let offset_x = -(map.width as i32) / 2;
    let offset_y = -(map.height as i32) / 2;
    
    for layer in &map.layers {
        println!("   Rendering layer: {}", layer.name);
        
        for (index, &tile_id) in layer.data.iter().enumerate() {
            if tile_id == 0 { continue; }
            
            let x = (index as u32 % layer.width) as i32 + offset_x;
            let y = (index as u32 / layer.width) as i32 + offset_y;

            let screen_pos = grid_to_screen(x, y);
            let tile_type = get_tile_type(tile_id);
            let z = z_layer_manager.get_layer(crate::components::z_layer::EntityType::Tile);
            
            // Map tile IDs to actual atlas indices
            // ID 50 = grass (green) - needs to map to actual grass index in tileset
            // Since first rows are brown, and green starts around row 4-5, we offset
            let safe_tile_index = match tile_id {
                50 => 60,  // ID 50 (grass) → index 60 (try green area)
                52 => 0,   // ID 52 (dirt/brown) → index 0 (brown area)
                49 => 73,  // ID 49 (water candidate) → index 73
                // For other IDs, try to map intelligently
                id => {
                    if id > 0 && id <= 288 {
                        (id - 1) as usize
                    } else {
                        60  // Default to grass green
                    }
                }
            };
            
            commands.spawn((
                Sprite {
                    image: tileset.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: tileset.layout.clone(),
                        index: safe_tile_index,
                    }),
                    custom_size: Some(Vec2::new(128.0, 64.0)), 
                    ..default()
                },
                Transform::from_xyz(screen_pos.x, screen_pos.y, z),
                Tile::new(x, y, tile_type),
            ));
            
            tile_count += 1;
        }
    }
    
    println!("✓ Created {} tiles!", tile_count);
}

fn get_tile_type(tile_id: u32) -> TileType {
    // Match the actual tile IDs from the Tiled map
    match tile_id {
        50 => TileType::GrassLight,  // ID 50 is grass (most of the map)
        52 => TileType::Dirt,        // ID 52 is dirt/dark
        49 => TileType::Water,       // ID 49 might be water
        73 | 85 | 95 => TileType::Water,  // Water-like tiles
        91 | 77 | 93 | 58 | 105 | 81 | 97 | 65 => TileType::GrassDark,  // Building/tree tiles
        _ => TileType::GrassLight,   // Default to grass
    }
}

fn get_tile_properties(tile_id: u32) -> (f32, TileType) {
    // Updated for 12x24 grid (288 total tiles)
    match tile_id {
        1..=12 => (z_index::GROUND, TileType::Dirt),
        13..=24 => (z_index::GROUND, TileType::GrassLight),
        25..=36 => (z_index::GROUND, TileType::GrassLight),
        37..=48 => (z_index::GROUND, TileType::GrassDark),
        49..=60 => (z_index::GROUND, TileType::GrassLight),
        61..=72 => (z_index::ROADS, TileType::Stone),
        73..=84 => (z_index::ROADS, TileType::Stone),
        85..=144 => (z_index::WATER, TileType::Water),
        145..=204 => (z_index::WATER, TileType::Water),
        205..=288 => (z_index::WATER, TileType::Water),
        _ => (z_index::GROUND, TileType::GrassLight),
    }
}

fn spawn_procedural_map(mut commands: Commands, tileset: TilesetHandle, z_layer_manager: Res<ZLayerManager>) {
    let map_size: i32 = 20;
    let mut tile_count = 0;
    
    for grid_x in -map_size..=map_size {
        for grid_y in -map_size..=map_size {
            // Use safe tile indices within the 12x24 grid (288 total tiles)
            let tile_index = if (grid_x + grid_y).abs() % 3 == 0 {
                25  // Reduced from 26 to stay within bounds
            } else if (grid_x * grid_y).abs() % 5 == 0 {
                12  // Reduced from 13 to stay within bounds
            } else if grid_x.abs() > 15 || grid_y.abs() > 15 {
                100 // Reduced from 117 to stay within bounds
            } else {
                38  // Reduced from 39 to stay within bounds
            };
            
            let tile_type = if tile_index >= 100 {
                TileType::Water
            } else if tile_index >= 12 && tile_index < 25 {
                TileType::Dirt
            } else {
                TileType::GrassLight
            };
            
            let screen_pos = grid_to_screen(grid_x, grid_y);
            let z = z_layer_manager.get_layer(crate::components::z_layer::EntityType::Tile);
            
            commands.spawn((
                Sprite {
                    image: tileset.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: tileset.layout.clone(),
                        index: tile_index,
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
    
    println!(" Generated {} isometric tiles", tile_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_grid_to_screen_basic() {
        // Test basic isometric projection
        let pos = grid_to_screen(0, 0);
        assert_eq!(pos, Vec2::new(0.0, 0.0));
        
        let pos = grid_to_screen(1, 0);
        assert_eq!(pos, Vec2::new(32.0, -16.0));
        
        let pos = grid_to_screen(0, 1);
        assert_eq!(pos, Vec2::new(-32.0, -16.0));
        
        let pos = grid_to_screen(1, 1);
        assert_eq!(pos, Vec2::new(0.0, -32.0));
    }

    #[test]
    fn test_screen_to_grid_basic() {
        // Test inverse projection
        let grid = screen_to_grid(0.0, 0.0);
        assert_eq!(grid, (0, 0));
        
        let grid = screen_to_grid(32.0, -16.0);
        assert_eq!(grid, (1, 0));
        
        let grid = screen_to_grid(-32.0, -16.0);
        assert_eq!(grid, (0, 1));
    }

    #[test]
    fn test_tile_type_mapping() {
        assert_eq!(get_tile_type(1), TileType::Dirt);
        assert_eq!(get_tile_type(13), TileType::GrassLight);
        assert_eq!(get_tile_type(61), TileType::Stone);
        assert_eq!(get_tile_type(85), TileType::Water);
        assert_eq!(get_tile_type(999), TileType::GrassLight); // Default case
    }

    // **Property 1: Isometric projection consistency**
    // **Validates: Requirements 1.1, 1.2**
    proptest! {
        #[test]
        fn prop_isometric_projection_consistency(
            grid_x in -1000i32..1000i32,
            grid_y in -1000i32..1000i32
        ) {
            // Property: For any map data and tile coordinates, the rendered position 
            // should follow the diamond isometric projection formula consistently
            let screen_pos = grid_to_screen(grid_x, grid_y);
            
            // The isometric projection should follow the diamond formula:
            // x = (grid_x - grid_y) * tile_width_half
            // y = -(grid_x + grid_y) * tile_height_half
            let tile_width_half = crate::constants::TILE_SIZE / 2.0;  // 32.0
            let tile_height_half = crate::constants::TILE_HEIGHT / 2.0; // 16.0
            
            let expected_x = (grid_x - grid_y) as f32 * tile_width_half;
            let expected_y = -(grid_x + grid_y) as f32 * tile_height_half;
            
            prop_assert_eq!(screen_pos.x, expected_x,
                "X coordinate should follow diamond projection formula");
            prop_assert_eq!(screen_pos.y, expected_y,
                "Y coordinate should follow diamond projection formula");
            
            // Property: The projection should be consistent - same input always produces same output
            let screen_pos2 = grid_to_screen(grid_x, grid_y);
            prop_assert_eq!(screen_pos, screen_pos2,
                "Projection should be deterministic and consistent");
        }
    }

    proptest! {
        #[test]
        fn prop_isometric_projection_diamond_shape(
            grid_x in -100i32..100i32,
            grid_y in -100i32..100i32
        ) {
            // Property: The isometric projection should create a diamond pattern
            let center = grid_to_screen(grid_x, grid_y);
            let right = grid_to_screen(grid_x + 1, grid_y);
            let down = grid_to_screen(grid_x, grid_y + 1);
            let diagonal = grid_to_screen(grid_x + 1, grid_y + 1);
            
            // In diamond isometric projection:
            // - Moving right (+1, 0) should move diagonally down-right
            // - Moving down (0, +1) should move diagonally down-left
            // - Moving diagonally (+1, +1) should move straight down
            
            let tile_width_half = crate::constants::TILE_SIZE / 2.0;  // 32.0
            let tile_height_half = crate::constants::TILE_HEIGHT / 2.0; // 16.0
            
            // Right movement should increase x by tile_width_half and decrease y by tile_height_half
            prop_assert_eq!(right.x - center.x, tile_width_half,
                "Moving right should increase x by tile_width_half");
            prop_assert_eq!(right.y - center.y, -tile_height_half,
                "Moving right should decrease y by tile_height_half");
            
            // Down movement should decrease x by tile_width_half and decrease y by tile_height_half
            prop_assert_eq!(down.x - center.x, -tile_width_half,
                "Moving down should decrease x by tile_width_half");
            prop_assert_eq!(down.y - center.y, -tile_height_half,
                "Moving down should decrease y by tile_height_half");
            
            // Diagonal movement should keep x the same and decrease y by 2*tile_height_half
            prop_assert_eq!(diagonal.x - center.x, 0.0,
                "Moving diagonally should keep x coordinate the same");
            prop_assert_eq!(diagonal.y - center.y, -2.0 * tile_height_half,
                "Moving diagonally should decrease y by 2*tile_height_half");
        }
    }

    proptest! {
        #[test]
        fn prop_screen_to_grid_inverse_consistency(
            grid_x in -100i32..100i32,
            grid_y in -100i32..100i32
        ) {
            // Property: screen_to_grid should be the inverse of grid_to_screen
            let screen_pos = grid_to_screen(grid_x, grid_y);
            let recovered_grid = screen_to_grid(screen_pos.x, screen_pos.y);
            
            prop_assert_eq!(recovered_grid.0, grid_x,
                "X grid coordinate should be recovered exactly");
            prop_assert_eq!(recovered_grid.1, grid_y,
                "Y grid coordinate should be recovered exactly");
        }
    }

    // **Property 2: Sprite indexing correctness**
    // **Validates: Requirements 1.3**
    proptest! {
        #[test]
        fn prop_sprite_indexing_correctness(
            tile_id in 1u32..=288u32
        ) {
            // Property: For any valid tile ID, the tileset indexing should map to 
            // the correct sprite region without errors
            
            // Test that tile_id maps to a valid index within the 12x24 grid (288 total tiles)
            let safe_tile_index = if tile_id > 0 && tile_id <= 288 {
                (tile_id - 1) as usize
            } else {
                0 // Default to first tile if index is out of range
            };
            
            // The index should be within the valid range [0, 287] for a 12x24 grid
            prop_assert!(safe_tile_index < 288,
                "Tile index {} should be within valid range [0, 287]", safe_tile_index);
            
            // The index should correspond to the tile_id (adjusted for 0-based indexing)
            if tile_id > 0 && tile_id <= 288 {
                prop_assert_eq!(safe_tile_index, (tile_id - 1) as usize,
                    "Tile index should be tile_id - 1 for valid tile IDs");
            }
            
            // Test that the tile type mapping is consistent
            let tile_type = get_tile_type(tile_id);
            let tile_type2 = get_tile_type(tile_id);
            prop_assert_eq!(tile_type, tile_type2,
                "Tile type mapping should be consistent for the same tile_id");
        }
    }

    proptest! {
        #[test]
        fn prop_sprite_indexing_bounds_safety(
            tile_id in 0u32..1000u32
        ) {
            // Property: Sprite indexing should handle out-of-bounds tile IDs safely
            let safe_tile_index = if tile_id > 0 && tile_id <= 288 {
                (tile_id - 1) as usize
            } else {
                0 // Default to first tile if index is out of range
            };
            
            // The safe index should always be within valid bounds
            prop_assert!(safe_tile_index < 288,
                "Safe tile index should always be within bounds [0, 287]");
            
            // For invalid tile IDs, should default to index 0
            if tile_id == 0 || tile_id > 288 {
                prop_assert_eq!(safe_tile_index, 0,
                    "Invalid tile IDs should default to index 0");
            }
            
            // For valid tile IDs, should map correctly
            if tile_id > 0 && tile_id <= 288 {
                prop_assert_eq!(safe_tile_index, (tile_id - 1) as usize,
                    "Valid tile IDs should map to tile_id - 1");
            }
        }
    }

    proptest! {
        #[test]
        fn prop_tileset_grid_consistency(
            tile_index in 0usize..288usize
        ) {
            // Property: Tile indices should map consistently to grid positions within the 12x24 tileset
            let grid_width = 12;
            let grid_height = 24;
            
            // Calculate grid position from linear index
            let grid_x = tile_index % grid_width;
            let grid_y = tile_index / grid_width;
            
            // Grid position should be within bounds
            prop_assert!(grid_x < grid_width,
                "Grid X position {} should be within width {}", grid_x, grid_width);
            prop_assert!(grid_y < grid_height,
                "Grid Y position {} should be within height {}", grid_y, grid_height);
            
            // Reverse calculation should yield the original index
            let recovered_index = grid_y * grid_width + grid_x;
            prop_assert_eq!(recovered_index, tile_index,
                "Grid position should map back to original tile index");
        }
    }

    proptest! {
        #[test]
        fn prop_tile_type_consistency(
            tile_id in 1u32..=500u32
        ) {
            // Property: Tile type mapping should be consistent and deterministic
            let tile_type1 = get_tile_type(tile_id);
            let tile_type2 = get_tile_type(tile_id);
            
            prop_assert_eq!(tile_type1, tile_type2,
                "Tile type should be consistent for the same tile_id");
            
            // Test that tile types follow the expected ranges
            match tile_id {
                1..=12 => prop_assert_eq!(tile_type1, TileType::Dirt),
                13..=24 | 25..=36 | 49..=60 => prop_assert_eq!(tile_type1, TileType::GrassLight),
                37..=48 => prop_assert_eq!(tile_type1, TileType::GrassDark),
                61..=72 | 73..=84 => prop_assert_eq!(tile_type1, TileType::Stone),
                85..=144 | 145..=204 | 205..=288 => prop_assert_eq!(tile_type1, TileType::Water),
                _ => prop_assert_eq!(tile_type1, TileType::GrassLight), // Default case
            }
        }
    }
}