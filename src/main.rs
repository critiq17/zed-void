use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bat;

#[derive(Component)]
struct Tile {
    x: i32,
    y: i32
}

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
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, bat_show_hide, bat_attack, camera_follow))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));

    let player_entity = commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
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

    let map_size = 25;
    let tile_size = 32.0;
    
    for x in -map_size..=map_size {
        for y in -map_size..=map_size {
            let color = if (x + y) % 2 == 0 {
                Color::srgb(0.2, 0.6, 0.2)
            } else {
                Color::srgb(0.1, 0.4, 0.1)
            };
            
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * tile_size,
                    y as f32 * tile_size,
                    0.0
                ),
                Tile { x: x as i32, y: y as i32 }
            ));
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    let speed = 200.0;
    let mut direction = Vec2::ZERO;
    
    if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }
    
    if direction.length() > 0.1 {
        direction = direction.normalize();
        transform.translation += direction.extend(0.0) * speed * time.delta_secs();
    }
}

fn bat_show_hide(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bat_query: Query<&mut Visibility, With<Bat>>,
) {
    let visible = mouse.pressed(MouseButton::Left);
    if let Ok(mut visibility) = bat_query.get_single_mut() {
        *visibility = if visible { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn bat_attack(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bat_query: Query<&mut Transform, With<Bat>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        println!("💥 УДАР БИТОЙ!");
        
        if let Ok(mut bat_transform) = bat_query.get_single_mut() {
            bat_transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    
    camera_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        5.0
    );
}
