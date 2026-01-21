use bevy::prelude::*;

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
        .add_systems(Update, camera_controls)
        .run();
}

#[derive(Component)]
struct TestSprite;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        TestSprite,
    ));
}

fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let speed = 300.0;
    
    if keyboard.pressed(KeyCode::KeyW) {
        camera_transform.translation.y += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera_transform.translation.y -= speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        camera_transform.translation.x -= speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera_transform.translation.x += speed * time.delta_secs();
    }
}
