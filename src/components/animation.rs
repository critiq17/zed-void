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

/// DirectionalAnimation component that manages animations for each direction and state
#[derive(Component, Clone)]
pub struct DirectionalAnimation {
    pub up_idle: AnimationIndices,
    pub up_walking: AnimationIndices,
    pub down_idle: AnimationIndices,
    pub down_walking: AnimationIndices,
    pub left_idle: AnimationIndices,
    pub left_walking: AnimationIndices,
    pub right_idle: AnimationIndices,
    pub right_walking: AnimationIndices,
    pub current_state: AnimationState,
    pub current_direction: Direction,
}

impl DirectionalAnimation {
    /// Create a new DirectionalAnimation with default 4x4 player spritesheet layout
    pub fn new_player() -> Self {
        Self {
            // Row 0: Down animations (idle frame 0, walk frames 0-3)
            down_idle: AnimationIndices { first: 0, last: 0 },
            down_walking: AnimationIndices { first: 0, last: 3 },
            
            // Row 1: Up animations (idle frame 4, walk frames 4-7)
            up_idle: AnimationIndices { first: 4, last: 4 },
            up_walking: AnimationIndices { first: 4, last: 7 },
            
            // Row 2: Left animations (idle frame 8, walk frames 8-11)
            left_idle: AnimationIndices { first: 8, last: 8 },
            left_walking: AnimationIndices { first: 8, last: 11 },
            
            // Row 3: Right animations (idle frame 12, walk frames 12-15)
            right_idle: AnimationIndices { first: 12, last: 12 },
            right_walking: AnimationIndices { first: 12, last: 15 },
            
            current_state: AnimationState::Idle,
            current_direction: Direction::Down,
        }
    }
    
    /// Get the current animation indices based on state and direction
    pub fn get_current_animation(&self) -> AnimationIndices {
        match (self.current_direction, self.current_state) {
            (Direction::Up, AnimationState::Idle) => self.up_idle,
            (Direction::Up, AnimationState::Walking) => self.up_walking,
            (Direction::Down, AnimationState::Idle) => self.down_idle,
            (Direction::Down, AnimationState::Walking) => self.down_walking,
            (Direction::Left, AnimationState::Idle) => self.left_idle,
            (Direction::Left, AnimationState::Walking) => self.left_walking,
            (Direction::Right, AnimationState::Idle) => self.right_idle,
            (Direction::Right, AnimationState::Walking) => self.right_walking,
        }
    }
    
    /// Update the animation state and direction
    pub fn set_state_and_direction(&mut self, state: AnimationState, direction: Direction) {
        self.current_state = state;
        self.current_direction = direction;
    }
}

/// Animation indices defining the start and end frames for an animation
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub fn frame_count(&self) -> usize {
        self.last - self.first + 1
    }
}

/// Animation state for entities
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
}

/// Animation timer component for frame timing
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

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
            // For a 4x4 grid with 16 frames:
            // Row 0: frames 0-3 (idle and walk down)
            // Row 1: frames 4-7 (walk up)  
            // Row 2: frames 8-11 (walk left)
            // Row 3: frames 12-15 (walk right)
            AnimationType::Idle => (0, 0),          // First frame (idle)
            AnimationType::WalkDown => (0, 3),      // Row 0: frames 0-3
            AnimationType::WalkUp => (4, 7),        // Row 1: frames 4-7
            AnimationType::WalkLeft => (8, 11),     // Row 2: frames 8-11
            AnimationType::WalkRight => (12, 15),   // Row 3: frames 12-15
            AnimationType::Attack => (0, 3),        // Use walk down frames for attack for now
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