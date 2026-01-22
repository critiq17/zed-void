use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationController {
    pub current_animation: AnimationType,
    pub frame_timer: Timer,
    pub current_frame: usize,
}

impl AnimationController {
    pub fn new(animation_speed: f32) -> Self {
        Self {
            current_animation: AnimationType::Idle,
            frame_timer: Timer::from_seconds(animation_speed, TimerMode::Repeating),
            current_frame: 0,
        }
    }
    
    pub fn set_animation(&mut self, animation: AnimationType) {
        if self.current_animation != animation {
            self.current_animation = animation;
            self.current_frame = 0;
            self.frame_timer.reset();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationType {
    Idle,        
    WalkDown,    
    WalkUp,      
    WalkLeft,    
    WalkRight,  
    Attack,    
}

impl AnimationType {
    pub fn frame_range(&self) -> (usize, usize) {
        match self {
            AnimationType::Idle => (0, 0),          // 1 кадр
            AnimationType::WalkDown => (0, 3),      // 4 кадра (ряд 1)
            AnimationType::WalkUp => (4, 7),        // 4 кадра (ряд 2)
            AnimationType::WalkLeft => (8, 11),     // 4 кадра (ряд 3)
            AnimationType::WalkRight => (12, 15),   // 4 кадра (ряд 4)
            AnimationType::Attack => (16, 19),      // 4 кадра (ряд 5)
        }
    }
    
    pub fn frame_count(&self) -> usize {
        let (start, end) = self.frame_range();
        end - start + 1
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_movement(dx: f32, dy: f32) -> Option<Self> {
        if dx.abs() > dy.abs() {
            if dx > 0.0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else if dy.abs() > 0.1 {
            if dy > 0.0 {
                Some(Direction::Up)
            } else {
                Some(Direction::Down)
            }
        } else {
            None
        }
    }
    
    pub fn to_animation(&self) -> AnimationType {
        match self {
            Direction::Up => AnimationType::WalkUp,
            Direction::Down => AnimationType::WalkDown,
            Direction::Left => AnimationType::WalkLeft,
            Direction::Right => AnimationType::WalkRight,
        }
    }
}