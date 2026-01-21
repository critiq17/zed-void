use bevy::prelude::*;
use crate::components::player::Player;
use crate::constants::PLAYER_SPEED;


pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,  
    time: Res<Time>,                      
    mut query: Query<&mut Transform, With<Player>>, 
) {
   
    let mut transform = query.single_mut();

    let mut direction = Vec2::ZERO;
    

    if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    if direction.length() > 0.1 {

        direction = direction.normalize();
        
        
        transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
    }
}


pub fn _isometric_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
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
    
    if direction.length() > 0.1 {
        direction = direction.normalize();
        transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
    }
}