pub const MAP_SIZE: i32 = 50;

pub const TILE_SIZE: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 64.0;

pub const PLAYER_SPEED: f32 = 5.0;
pub const PLAYER_SIZE: (f32, f32) = (32.0, 48.0);

pub const ANIMATION_SPEED: f32 = 0.12;

pub const BAT_SIZE: (f32, f32) = (48.0, 16.0);
pub const BAT_DAMAGE: f32 = 25.0;

pub const CAMERA_Z: f32 = 999.0;
pub const CAMERA_SCALE: f32 = 1.0;

pub mod colors {
    use bevy::prelude::Color;


    pub const GRASS_LIGHT: Color = Color::srgb(0.45, 0.75, 0.35);
    pub const GRASS_DARK: Color = Color::srgb(0.35, 0.65, 0.3);

    pub const DIRT: Color = Color::srgb(0.55, 0.45, 0.3);
    pub const STONE: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const ROAD: Color = Color::srgb(0.45, 0.45, 0.4);

    pub const WATER: Color = Color::srgb(0.2, 0.5, 0.8);
    pub const WATER_DEEP: Color = Color::srgb(0.1, 0.4, 0.7);

    pub const HOUSE_WALL: Color = Color::srgb(0.6, 0.45, 0.3);
    pub const HOUSE_ROOF: Color = Color::srgb(0.45, 0.3, 0.2);
    pub const HOUSE_DOOR: Color = Color::srgb(0.35, 0.2, 0.1);

    pub const TREE_TRUNK: Color = Color::srgb(0.35, 0.25, 0.15);
    pub const TREE_LEAVES: Color = Color::srgb(0.2, 0.5, 0.2);

    pub const PLAYER: Color = Color::WHITE;
    pub const BAT: Color = Color::srgb(0.7, 0.5, 0.2);
}

pub mod z_index {
    pub const GROUND: f32 = 0.0;
    pub const WATER: f32 = 0.0;
    pub const ROADS: f32 = 1.0;
    pub const PLAYER: f32 = 100.0;
    pub const ENEMIES: f32 = 100.0;
    pub const BUILDINGS: f32 = 150.0;
}
