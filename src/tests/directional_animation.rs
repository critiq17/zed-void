use bevy::prelude::*;
use crate::components::animation::{DirectionalAnimation, AnimationState, Direction, AnimationIndices};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directional_animation_creation() {
        let anim = DirectionalAnimation::new_player();
        
        // Verify initial state
        assert_eq!(anim.current_state, AnimationState::Idle);
        assert_eq!(anim.current_direction, Direction::Down);
        
        // Verify animation indices are set correctly for 4x4 grid
        assert_eq!(anim.down_idle.first, 0);
        assert_eq!(anim.down_idle.last, 0);
        assert_eq!(anim.down_walking.first, 0);
        assert_eq!(anim.down_walking.last, 3);
        
        assert_eq!(anim.up_idle.first, 4);
        assert_eq!(anim.up_idle.last, 4);
        assert_eq!(anim.up_walking.first, 4);
        assert_eq!(anim.up_walking.last, 7);
        
        assert_eq!(anim.left_idle.first, 8);
        assert_eq!(anim.left_idle.last, 8);
        assert_eq!(anim.left_walking.first, 8);
        assert_eq!(anim.left_walking.last, 11);
        
        assert_eq!(anim.right_idle.first, 12);
        assert_eq!(anim.right_idle.last, 12);
        assert_eq!(anim.right_walking.first, 12);
        assert_eq!(anim.right_walking.last, 15);
    }

    #[test]
    fn test_get_current_animation_idle_states() {
        let mut anim = DirectionalAnimation::new_player();
        
        // Test idle animations for each direction
        anim.set_state_and_direction(AnimationState::Idle, Direction::Down);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 0);
        assert_eq!(current.last, 0);
        
        anim.set_state_and_direction(AnimationState::Idle, Direction::Up);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 4);
        assert_eq!(current.last, 4);
        
        anim.set_state_and_direction(AnimationState::Idle, Direction::Left);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 8);
        assert_eq!(current.last, 8);
        
        anim.set_state_and_direction(AnimationState::Idle, Direction::Right);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 12);
        assert_eq!(current.last, 12);
    }

    #[test]
    fn test_get_current_animation_walking_states() {
        let mut anim = DirectionalAnimation::new_player();
        
        // Test walking animations for each direction
        anim.set_state_and_direction(AnimationState::Walking, Direction::Down);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 0);
        assert_eq!(current.last, 3);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Up);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 4);
        assert_eq!(current.last, 7);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Left);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 8);
        assert_eq!(current.last, 11);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Right);
        let current = anim.get_current_animation();
        assert_eq!(current.first, 12);
        assert_eq!(current.last, 15);
    }

    #[test]
    fn test_animation_state_transitions() {
        let mut anim = DirectionalAnimation::new_player();
        
        // Test transition from idle to walking
        anim.set_state_and_direction(AnimationState::Idle, Direction::Right);
        assert_eq!(anim.current_state, AnimationState::Idle);
        assert_eq!(anim.current_direction, Direction::Right);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Right);
        assert_eq!(anim.current_state, AnimationState::Walking);
        assert_eq!(anim.current_direction, Direction::Right);
        
        // Test direction change while walking
        anim.set_state_and_direction(AnimationState::Walking, Direction::Left);
        assert_eq!(anim.current_state, AnimationState::Walking);
        assert_eq!(anim.current_direction, Direction::Left);
        
        // Test transition back to idle
        anim.set_state_and_direction(AnimationState::Idle, Direction::Left);
        assert_eq!(anim.current_state, AnimationState::Idle);
        assert_eq!(anim.current_direction, Direction::Left);
    }

    #[test]
    fn test_animation_indices_frame_count() {
        let indices_single = AnimationIndices { first: 0, last: 0 };
        assert_eq!(indices_single.frame_count(), 1);
        
        let indices_multi = AnimationIndices { first: 0, last: 3 };
        assert_eq!(indices_multi.frame_count(), 4);
        
        let indices_range = AnimationIndices { first: 8, last: 11 };
        assert_eq!(indices_range.frame_count(), 4);
    }

    #[test]
    fn test_directional_animation_requirements_2_3() {
        // **Validates: Requirements 2.3**
        // WHEN the player moves in any direction, THE Animation_System SHALL play the appropriate directional walk animation
        
        let mut anim = DirectionalAnimation::new_player();
        
        // Test that each direction maps to correct walking animation
        anim.set_state_and_direction(AnimationState::Walking, Direction::Up);
        let up_anim = anim.get_current_animation();
        assert_eq!(up_anim.first, 4);
        assert_eq!(up_anim.last, 7);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Down);
        let down_anim = anim.get_current_animation();
        assert_eq!(down_anim.first, 0);
        assert_eq!(down_anim.last, 3);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Left);
        let left_anim = anim.get_current_animation();
        assert_eq!(left_anim.first, 8);
        assert_eq!(left_anim.last, 11);
        
        anim.set_state_and_direction(AnimationState::Walking, Direction::Right);
        let right_anim = anim.get_current_animation();
        assert_eq!(right_anim.first, 12);
        assert_eq!(right_anim.last, 15);
    }

    #[test]
    fn test_directional_animation_requirements_2_4() {
        // **Validates: Requirements 2.4**
        // WHEN the player is idle, THE Animation_System SHALL display the appropriate idle frame
        
        let mut anim = DirectionalAnimation::new_player();
        
        // Test that idle state shows single frame for each direction
        anim.set_state_and_direction(AnimationState::Idle, Direction::Up);
        let up_idle = anim.get_current_animation();
        assert_eq!(up_idle.first, 4);
        assert_eq!(up_idle.last, 4);
        assert_eq!(up_idle.frame_count(), 1);
        
        anim.set_state_and_direction(AnimationState::Idle, Direction::Down);
        let down_idle = anim.get_current_animation();
        assert_eq!(down_idle.first, 0);
        assert_eq!(down_idle.last, 0);
        assert_eq!(down_idle.frame_count(), 1);
        
        anim.set_state_and_direction(AnimationState::Idle, Direction::Left);
        let left_idle = anim.get_current_animation();
        assert_eq!(left_idle.first, 8);
        assert_eq!(left_idle.last, 8);
        assert_eq!(left_idle.frame_count(), 1);
        
        anim.set_state_and_direction(AnimationState::Idle, Direction::Right);
        let right_idle = anim.get_current_animation();
        assert_eq!(right_idle.first, 12);
        assert_eq!(right_idle.last, 12);
        assert_eq!(right_idle.frame_count(), 1);
    }
}