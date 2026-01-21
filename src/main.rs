use bevy::prelude::*;


mod components;
mod systems;
mod rendering;
mod constants;

use components::player::Player;
use components::combat::Bat;
use components::world::Tile;

use systems::movement::player_movement;
use systems::combat::{bat_show_hide, bat_attack};
use systems::camera::camera_follow;


use rendering::isometric::{setup_isometric_camera, spawn_isometric_map};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zed Void".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (
            setup_isometric_camera,  
            spawn_isometric_map,   
            setup_player,          
        ))

        .add_systems(Update, (
            player_movement,
            bat_show_hide,
            bat_attack,
            camera_follow,
        ))
        .run();
}

fn setup_player(mut commands: Commands) {
    let player_entity = commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        Player,
    )).id();

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.5, 0.2),
                custom_size: Some(Vec2::new(64.0, 24.0)),
                ..default()
            },
            Transform::from_xyz(20.0, 0.0, 1.0),
            Visibility::Hidden,
            Bat,
        ));
    });
    
    println!("Player created");
}