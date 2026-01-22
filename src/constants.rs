pub const MAP_SIZE: i32 = 50;       

pub const TILE_SIZE: f32 = 64.0;       
pub const TILE_HEIGHT: f32 = 32.0;     

pub const PLAYER_SPEED: f32 = 200.0;  
pub const PLAYER_SIZE: (f32, f32) = (48.0, 48.0);

pub const ANIMATION_SPEED: f32 = 0.12;  

pub const BAT_SIZE: (f32, f32) = (48.0, 16.0);
pub const BAT_DAMAGE: f32 = 25.0;

// camera
pub const CAMERA_Z: f32 = 999.0;     
pub const CAMERA_SCALE: f32 = 1.0;     

// colors for map 
pub mod colors {
    use bevy::prelude::Color;
    
    pub const GRASS_LIGHT: Color = Color::srgb(0.5, 0.75, 0.4);
    pub const GRASS_DARK: Color = Color::srgb(0.35, 0.65, 0.3);
    
    pub const DIRT: Color = Color::srgb(0.65, 0.5, 0.35);
    pub const STONE: Color = Color::srgb(0.55, 0.55, 0.55);
    pub const ROAD: Color = Color::srgb(0.5, 0.5, 0.45);
    
    pub const WATER: Color = Color::srgb(0.3, 0.6, 0.85);
    pub const WATER_DEEP: Color = Color::srgb(0.15, 0.45, 0.75);
    
    pub const HOUSE_WALL: Color = Color::srgb(0.7, 0.5, 0.3);
    pub const HOUSE_ROOF: Color = Color::srgb(0.5, 0.3, 0.2);
    pub const HOUSE_DOOR: Color = Color::srgb(0.4, 0.2, 0.1);
    
    pub const TREE_TRUNK: Color = Color::srgb(0.4, 0.3, 0.2);
    pub const TREE_LEAVES: Color = Color::srgb(0.2, 0.5, 0.2);
    
    pub const PLAYER: Color = Color::WHITE;  
    pub const BAT: Color = Color::srgb(0.7, 0.5, 0.2);
}

// z indexes для правильной отрисовки
pub mod z_index {
    pub const WATER: f32 = 0.0;      
    pub const GROUND: f32 = 1.0;     
    pub const ROADS: f32 = 2.0;      
    pub const ITEMS: f32 = 5.0;      
    pub const PLAYER: f32 = 10.0;    
    pub const ENEMIES: f32 = 10.0;   
    pub const BUILDINGS: f32 = 15.0;    
    pub const EFFECTS: f32 = 20.0;      
    pub const UI: f32 = 100.0;          
}