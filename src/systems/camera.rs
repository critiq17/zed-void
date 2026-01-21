use bevy::prelude::*;
use crate::components::player::Player;

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {

    let player_transform = player_query.single();
    

    let mut camera_transform = camera_query.single_mut();
    

    camera_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        camera_transform.translation.z, 
    );
}


pub fn _smooth_camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    

    let target = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        camera_transform.translation.z,
    );
    
  
    let smoothness = 5.0; 
    camera_transform.translation = camera_transform.translation.lerp(
        target,
        smoothness * time.delta_secs()
    );
}