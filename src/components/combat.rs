use bevy::prelude::*;

#[derive(Component)]
pub struct Bat;

#[derive(Component)]
pub struct Weapon {
    pub damage: f32,    
    pub range: f32, 
    pub attack_speed: f32, 
    pub knockback: f32,       
}

impl Weapon {
    pub fn bat() -> Self {
        Self {
            damage: 25.0,
            range: 70.0,
            attack_speed: 0.8,
            knockback: 50.0,
        }
    }
    
    pub fn axe() -> Self {
        Self {
            damage: 40.0,
            range: 60.0,
            attack_speed: 1.2,
            knockback: 30.0,
        }
    }
    
    pub fn knife() -> Self {
        Self {
            damage: 15.0,
            range: 40.0,
            attack_speed: 0.4,
            knockback: 10.0,
        }
    }
}


#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

impl AttackCooldown {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }
    
    pub fn can_attack(&self) -> bool {
        self.timer.finished()
    }
    
    pub fn reset(&mut self) {
        self.timer.reset();
    }
}


#[derive(Component)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2, 
}

impl Hitbox {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            offset: Vec2::ZERO,
        }
    }
    
    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }
    
    pub fn intersects(&self, self_pos: Vec2, other: &Hitbox, other_pos: Vec2) -> bool {
        let self_center = self_pos + self.offset;
        let other_center = other_pos + other.offset;
        
        let dx = (self_center.x - other_center.x).abs();
        let dy = (self_center.y - other_center.y).abs();
        
        dx < (self.width + other.width) / 2.0 &&
        dy < (self.height + other.height) / 2.0
    }
}