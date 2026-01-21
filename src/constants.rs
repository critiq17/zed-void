pub const MAP_SIZE: i32 = 25;          
pub const TILE_SIZE: f32 = 64.0;        
pub const TILE_HEIGHT: f32 = 32.0;


pub const PLAYER_SPEED: f32 = 200.0;    
pub const PLAYER_SIZE: (f32, f32) = (32.0, 48.0); 


pub const BAT_SIZE: (f32, f32) = (64.0, 24.0);    
pub const BAT_DAMAGE: f32 = 25.0;                  

pub const CAMERA_Z: f32 = 5.0;        
pub const CAMERA_SCALE: f32 = 1.0;     


pub mod colors {
    use bevy::prelude::Color;
    
    pub const GRASS_LIGHT: Color = Color::srgb(0.3, 0.7, 0.3);
    pub const GRASS_DARK: Color = Color::srgb(0.2, 0.6, 0.2);
    pub const DIRT: Color = Color::srgb(0.6, 0.5, 0.3);
    pub const STONE: Color = Color::srgb(0.5, 0.5, 0.5);
    
    pub const PLAYER: Color = Color::srgb(0.0, 0.8, 0.0);
    pub const BAT: Color = Color::srgb(0.7, 0.5, 0.2);
}


pub mod z_index {
    pub const GROUND: f32 = 0.0;        
    pub const ITEMS: f32 = 5.0;        
    pub const PLAYER: f32 = 10.0;      
    pub const ENEMIES: f32 = 10.0;     
    pub const EFFECTS: f32 = 15.0;      
    pub const UI: f32 = 100.0;          
}