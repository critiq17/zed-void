use bevy::prelude::*;
use crate::components::world::{Tile, TileType};
use crate::constants::{MAP_SIZE, TILE_SIZE, TILE_HEIGHT, z_index};

pub fn grid_to_screen(grid_x: i32, grid_y: i32) -> Vec2 {
    let x = (grid_x - grid_y) as f32 * TILE_SIZE / 2.0;
    let y = (grid_x + grid_y) as f32 * TILE_HEIGHT / 2.0;
    Vec2::new(x, y)
}

pub fn screen_to_grid(screen_x: f32, screen_y: f32) -> (i32, i32) {
    let grid_x = ((screen_x / (TILE_SIZE / 2.0)) + (screen_y / (TILE_HEIGHT / 2.0))) / 2.0;
    let grid_y = ((screen_y / (TILE_HEIGHT / 2.0)) - (screen_x / (TILE_SIZE / 2.0))) / 2.0;
    (grid_x.round() as i32, grid_y.round() as i32)
}


pub fn setup_isometric_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}

pub fn spawn_isometric_map(mut commands: Commands) {
    println!(" Generating map {}x{}...", MAP_SIZE * 2, MAP_SIZE * 2);
    
    let mut tile_count = 0;
    

    for grid_x in -MAP_SIZE..=MAP_SIZE {
        for grid_y in -MAP_SIZE..=MAP_SIZE {

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
                Transform::from_xyz(
                    screen_pos.x,
                    screen_pos.y,
                    z_index::GROUND, 
                ),
                Tile::new(grid_x, grid_y, tile_type),
            ));
            
            tile_count += 1;
        }
    }
    
    println!("Created {} tails!", tile_count);
}


#[allow(dead_code)]
pub fn _update_sprite_sorting(
    query: Query<&Transform, Without<Tile>>,
) {
    for transform in query.iter() {
        let _sort_y = -transform.translation.y;
        
    
    }
}


/*
pub fn _generate_random_tiles(grid_x: i32, grid_y: i32) -> TileType {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    

    let roll: f32 = rng.gen();
    
    if roll < 0.8 {
        if (grid_x + grid_y) % 2 == 0 {
            TileType::GrassLight
        } else {
            TileType::GrassDark
        }
    } else if roll < 0.95 {
        TileType::Dirt
    } else {
        TileType::Stone
    }
}
*/