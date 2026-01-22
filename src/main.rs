use bevy::prelude::*;

mod components;
mod systems;
mod rendering;
mod constants;

use components::player::Player;
use components::combat::Bat;
use components::animation::{AnimationController, Direction};

use systems::movement::player_movement;
use systems::combat::{bat_show_hide, bat_attack};
use systems::camera::camera_follow;
use systems::animation::animate_sprites;

use rendering::isometric::{setup_isometric_camera, spawn_map_from_json};

use constants::ANIMATION_SPEED;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Zed Void - День 5-7: Анимация".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())  
        )
        .add_systems(Startup, (
            setup_isometric_camera,
            spawn_map_from_json,
            setup_player,
        ))
        .add_systems(Update, (
            player_movement,
            animate_sprites,  
            bat_show_hide,
            bat_attack,
            camera_follow,
        ))
        .run();
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/player.png");
    
    let player_entity = commands.spawn((
        Sprite {
            image: texture_handle,  
            custom_size: Some(Vec2::new(32.0, 32.0)),  
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        Player,
        AnimationController::new(ANIMATION_SPEED),  
        Direction::Down, 
    )).id();
    

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.5, 0.2),
                custom_size: Some(Vec2::new(48.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(20.0, 0.0, 1.0),
            Visibility::Hidden,
            Bat,
        ));
    });
    
    println!("Player created with animation");
}

/*
fn setup_player_with_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Загружаем спрайтшит
    let texture_handle = asset_server.load("sprites/player_spritesheet.png");
    
    // Создаем атлас (например, 4 кадра по горизонтали, 5 рядов)
    let atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),  // Размер одного кадра
        4,                   // Колонок (кадров в ряду)
        5,                   // Рядов (разные анимации)
        None,
        None,
    );
    let atlas_layout_handle = texture_atlases.add(atlas_layout);
    
    let player_entity = commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout_handle,
                index: 0,  // Начальный кадр
            }),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        Player,
        AnimationController::new(ANIMATION_SPEED),
        Direction::Down,
    )).id();
    
    // Бита
    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.5, 0.2),
                custom_size: Some(Vec2::new(48.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(20.0, 0.0, 1.0),
            Visibility::Hidden,
            Bat,
        ));
    });
}
*/