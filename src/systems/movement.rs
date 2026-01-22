use bevy::prelude::*;
use crate::components::player::Player;
use crate::constants::PLAYER_SPEED;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
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
        let iso_y = -(direction.x + direction.y) * 16.0;
        
        transform.translation.x += iso_x * PLAYER_SPEED * time.delta_secs();
        transform.translation.y += iso_y * PLAYER_SPEED * time.delta_secs();
    }
}


pub fn isometric_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    

    if keyboard.pressed(KeyCode::KeyW) {
        direction.x += 1.0;
        direction.y += 0.5;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.x -= 1.0;
        direction.y -= 0.5;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        direction.y += 0.5;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        direction.y -= 0.5;
    }
    
    let is_moving = direction.length() > 0.1;
    
    if is_moving {
        direction = direction.normalize();
        transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
    }
}