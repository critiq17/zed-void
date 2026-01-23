use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::world::{Tile, TileType};
use crate::constants::PLAYER_SPEED;
use crate::rendering::isometric::screen_to_grid;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    tile_query: Query<&Tile>, 
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    

    if keyboard.pressed(KeyCode::KeyW) {
        direction.x += 1.0; 
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.x -= 1.0; 
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;  
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;  
        direction.y -= 1.0;
    }

    let is_moving = direction.length() > 0.1;
    
    if is_moving {
        direction = direction.normalize();
        
     
        let iso_x = (direction.x - direction.y) * 32.0;
        let iso_y = (direction.x + direction.y) * 16.0;
        
        let new_x = transform.translation.x + iso_x * PLAYER_SPEED * time.delta_secs();
        let new_y = transform.translation.y - iso_y * PLAYER_SPEED * time.delta_secs();

        let (grid_x, grid_y) = screen_to_grid(new_x, new_y);
        
        let mut can_move = true;
        
  
        for tile in tile_query.iter() {
            if tile.grid_x == grid_x && tile.grid_y == grid_y {
                if !tile.tile_type.is_walkable() {
                    can_move = false;
                    break;
                }
            }
        }
        
 
        if can_move {
            transform.translation.x = new_x;
            transform.translation.y = new_y;
        }
    }
}