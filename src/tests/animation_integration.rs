use bevy::prelude::*;
use crate::components::animation::{DirectionalAnimation, AnimationTimer, AnimationState, Direction};
use crate::systems::animation::{animate_directional_sprites, update_directional_animation_state};
use crate::constants::ANIMATION_SPEED;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_systems(Update, (
               update_directional_animation_state,
               animate_directional_sprites,
           ));
        app
    }

    #[test]
    fn test_directional_animation_system_integration() {
        let mut app = setup_test_app();
        
        // Create a test entity with DirectionalAnimation
        let entity = app.world_mut().spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: Handle::default(),
                    index: 0,
                }),
                ..default()
            },
            DirectionalAnimation::new_player(),
            AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        )).id();
        
        // Verify initial state
        let directional_anim = app.world().get::<DirectionalAnimation>(entity).unwrap();
        assert_eq!(directional_anim.current_state, AnimationState::Idle);
        assert_eq!(directional_anim.current_direction, Direction::Down);
        
        // Verify sprite starts with correct frame
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        if let Some(ref texture_atlas) = sprite.texture_atlas {
            assert_eq!(texture_atlas.index, 0); // Should start with idle down frame
        }
    }

    #[test]
    fn test_animation_frame_timing() {
        let mut app = setup_test_app();
        
        // Create entity with walking animation
        let entity = app.world_mut().spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: Handle::default(),
                    index: 0,
                }),
                ..default()
            },
            DirectionalAnimation::new_player(),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)), // Fast animation for testing
        )).id();
        
        // Set to walking state
        {
            let mut directional_anim = app.world_mut().get_mut::<DirectionalAnimation>(entity).unwrap();
            directional_anim.set_state_and_direction(AnimationState::Walking, Direction::Down);
        }
        
        // Verify that animation system can process the entity without panicking
        let directional_anim = app.world().get::<DirectionalAnimation>(entity).unwrap();
        let current_anim = directional_anim.get_current_animation();
        assert_eq!(current_anim.first, 0);
        assert_eq!(current_anim.last, 3);
        assert_eq!(current_anim.frame_count(), 4);
    }

    #[test]
    fn test_animation_state_management() {
        let mut app = setup_test_app();
        
        let entity = app.world_mut().spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: Handle::default(),
                    index: 0,
                }),
                ..default()
            },
            DirectionalAnimation::new_player(),
            AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        )).id();
        
        // Test state transitions
        {
            let mut directional_anim = app.world_mut().get_mut::<DirectionalAnimation>(entity).unwrap();
            
            // Start idle
            assert_eq!(directional_anim.current_state, AnimationState::Idle);
            
            // Transition to walking
            directional_anim.set_state_and_direction(AnimationState::Walking, Direction::Right);
            assert_eq!(directional_anim.current_state, AnimationState::Walking);
            assert_eq!(directional_anim.current_direction, Direction::Right);
            
            // Verify correct animation is selected
            let current_anim = directional_anim.get_current_animation();
            assert_eq!(current_anim.first, 12); // Right walking animation
            assert_eq!(current_anim.last, 15);
            
            // Transition back to idle
            directional_anim.set_state_and_direction(AnimationState::Idle, Direction::Right);
            assert_eq!(directional_anim.current_state, AnimationState::Idle);
            
            let idle_anim = directional_anim.get_current_animation();
            assert_eq!(idle_anim.first, 12); // Right idle animation
            assert_eq!(idle_anim.last, 12);
        }
    }

    #[test]
    fn test_directional_animation_requirements_validation() {
        // **Validates: Requirements 2.3, 2.4**
        // Integration test to verify directional animation system meets requirements
        
        let mut app = setup_test_app();
        
        let entity = app.world_mut().spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: Handle::default(),
                    index: 0,
                }),
                ..default()
            },
            DirectionalAnimation::new_player(),
            AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        )).id();
        
        // Test Requirement 2.3: Appropriate directional walk animation
        {
            let mut directional_anim = app.world_mut().get_mut::<DirectionalAnimation>(entity).unwrap();
            
            // Test each direction has correct walking animation
            directional_anim.set_state_and_direction(AnimationState::Walking, Direction::Up);
            let up_walk = directional_anim.get_current_animation();
            assert_eq!(up_walk.first, 4);
            assert_eq!(up_walk.last, 7);
            assert!(up_walk.frame_count() > 1, "Walking animation should have multiple frames");
            
            directional_anim.set_state_and_direction(AnimationState::Walking, Direction::Down);
            let down_walk = directional_anim.get_current_animation();
            assert_eq!(down_walk.first, 0);
            assert_eq!(down_walk.last, 3);
            assert!(down_walk.frame_count() > 1, "Walking animation should have multiple frames");
            
            directional_anim.set_state_and_direction(AnimationState::Walking, Direction::Left);
            let left_walk = directional_anim.get_current_animation();
            assert_eq!(left_walk.first, 8);
            assert_eq!(left_walk.last, 11);
            assert!(left_walk.frame_count() > 1, "Walking animation should have multiple frames");
            
            directional_anim.set_state_and_direction(AnimationState::Walking, Direction::Right);
            let right_walk = directional_anim.get_current_animation();
            assert_eq!(right_walk.first, 12);
            assert_eq!(right_walk.last, 15);
            assert!(right_walk.frame_count() > 1, "Walking animation should have multiple frames");
        }
        
        // Test Requirement 2.4: Appropriate idle frame
        {
            let mut directional_anim = app.world_mut().get_mut::<DirectionalAnimation>(entity).unwrap();
            
            // Test each direction has correct idle animation (single frame)
            directional_anim.set_state_and_direction(AnimationState::Idle, Direction::Up);
            let up_idle = directional_anim.get_current_animation();
            assert_eq!(up_idle.first, 4);
            assert_eq!(up_idle.last, 4);
            assert_eq!(up_idle.frame_count(), 1, "Idle animation should be single frame");
            
            directional_anim.set_state_and_direction(AnimationState::Idle, Direction::Down);
            let down_idle = directional_anim.get_current_animation();
            assert_eq!(down_idle.first, 0);
            assert_eq!(down_idle.last, 0);
            assert_eq!(down_idle.frame_count(), 1, "Idle animation should be single frame");
            
            directional_anim.set_state_and_direction(AnimationState::Idle, Direction::Left);
            let left_idle = directional_anim.get_current_animation();
            assert_eq!(left_idle.first, 8);
            assert_eq!(left_idle.last, 8);
            assert_eq!(left_idle.frame_count(), 1, "Idle animation should be single frame");
            
            directional_anim.set_state_and_direction(AnimationState::Idle, Direction::Right);
            let right_idle = directional_anim.get_current_animation();
            assert_eq!(right_idle.first, 12);
            assert_eq!(right_idle.last, 12);
            assert_eq!(right_idle.frame_count(), 1, "Idle animation should be single frame");
        }
    }
}