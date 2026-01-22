use bevy::prelude::*;

mod components;
mod systems;
mod rendering;
mod constants;

#[cfg(test)]
mod tests;

use components::player::Player;
use components::combat::Bat;
use components::animation::{Direction, DirectionalAnimation, AnimationTimer};
use components::z_layer::ZLayerManager;

use systems::movement::player_movement;
use systems::combat::{bat_show_hide, bat_attack};
use systems::camera::camera_follow;
use systems::animation::{animate_sprites, animate_directional_sprites, update_directional_animation_state};
use systems::z_layer::{assign_z_layers, update_transform_z_from_layer, depth_sorting_system, prevent_z_fighting};

use rendering::isometric::{setup_isometric_camera, load_tileset, spawn_map_from_json};

use constants::ANIMATION_SPEED;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Zed Void".to_string(),
                    resolution: (1920.0, 1080.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .insert_resource(ZLayerManager::new())
        .add_systems(Startup, (
            setup_isometric_camera,
            load_tileset,
        ).chain())
        .add_systems(Startup, (
            spawn_map_from_json,
            setup_player,
        ).after(load_tileset))
        .add_systems(Update, (
            // Z-layer management systems (run first to ensure proper layering)
            assign_z_layers,
            update_transform_z_from_layer,
            depth_sorting_system,
            prevent_z_fighting,
        ).chain().before(player_movement))
        .add_systems(Update, (
            player_movement,
            update_directional_animation_state,
            animate_directional_sprites,
            animate_sprites, // Keep for backward compatibility with bat
            bat_show_hide,
            bat_attack,
            camera_follow,
        ))
        .run();
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    z_layer_manager: Res<ZLayerManager>,
) {
    // Load player spritesheet as 4x4 grid with 16 frames
    let texture_handle = asset_server.load("sprites/player_spritesheet.png");
    
    let atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),  // Individual sprite size
        4,                   // 4 columns
        4,                   // 4 rows  
        None,
        None,
    );
    let atlas_layout_handle = texture_atlases.add(atlas_layout);
    
    let player_entity = commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(48.0, 48.0)), 
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, z_layer_manager.get_layer(components::z_layer::EntityType::Player)),
        Player,
        DirectionalAnimation::new_player(),
        AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        Direction::Down,
    )).id();
    

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.5, 0.2),
                custom_size: Some(Vec2::new(48.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(30.0, 0.0, z_layer_manager.get_layer(components::z_layer::EntityType::Bat)),
            Visibility::Hidden,
            Bat,
        ));
    });
    
    println!("Player created with directional animations!");
}