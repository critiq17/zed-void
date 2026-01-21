use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
    
    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

#[derive(Component)]
pub struct Hunger {
    pub current: f32, 
    pub max: f32,
    pub drain_rate: f32,  
}

impl Hunger {
    pub fn new(max: f32, drain_rate: f32) -> Self {
        Self {
            current: max,
            max,
            drain_rate,
        }
    }
    
    pub fn is_starving(&self) -> bool {
        self.current <= 0.0
    }
    
    pub fn update(&mut self, delta: f32) {
        self.current = (self.current - self.drain_rate * delta).max(0.0);
    }
}

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32, 
}

impl Stamina {
    pub fn new(max: f32, regen_rate: f32) -> Self {
        Self {
            current: max,
            max,
            regen_rate,
        }
    }
    
    pub fn can_sprint(&self) -> bool {
        self.current > 20.0 
    }
    
    pub fn use_stamina(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn regenerate(&mut self, delta: f32) {
        self.current = (self.current + self.regen_rate * delta).min(self.max);
    }
}