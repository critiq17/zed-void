use bevy::prelude::*;
use crate::components::animation::{
    AnimationController, AnimationType, DirectionalAnimation, AnimationTimer,
    AnimationState
};

/// System for animating sprites using the new DirectionalAnimation component
pub fn animate_directional_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut DirectionalAnimation,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
) {
    for (directional_anim, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            let current_animation = directional_anim.get_current_animation();

            if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                let frame_count = current_animation.frame_count();
                if frame_count > 1 {
                    // Cycle through animation frames
                    let current_frame = texture_atlas.index;
                    let relative_frame = if current_frame >= current_animation.first && current_frame <= current_animation.last {
                        current_frame - current_animation.first
                    } else {
                        0
                    };

                    let next_relative_frame = (relative_frame + 1) % frame_count;
                    texture_atlas.index = current_animation.first + next_relative_frame;
                } else {
                    // Single frame animation (idle)
                    texture_atlas.index = current_animation.first;
                }
            }
        }
    }
}

/// Legacy system for backward compatibility with old AnimationController
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationController, &mut Sprite)>,
    _atlas_handles: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut controller, mut sprite) in query.iter_mut() {
        controller.frame_timer.tick(time.delta());

        if controller.frame_timer.just_finished() {
            let (start, end) = controller.current_animation.frame_range();
            let frame_count = end - start + 1;

            controller.current_frame = (controller.current_frame + 1) % frame_count;

            // Update TextureAtlas index to display the correct frame
            if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                let absolute_frame = start + controller.current_frame;
                texture_atlas.index = absolute_frame;
            }
        }
    }
}

/// System to update DirectionalAnimation based on movement
pub fn update_directional_animation_from_movement(
    mut query: Query<&mut DirectionalAnimation>,
    movement_query: Query<&crate::components::animation::Direction, With<crate::components::player::Player>>,
) {
    if let Ok(direction) = movement_query.get_single() {
        for mut directional_anim in query.iter_mut() {
            // Determine if the entity is moving (this would need to be passed from movement system)
            // For now, we'll assume walking state when direction changes
            directional_anim.set_state_and_direction(AnimationState::Walking, *direction);
        }
    }
}

/// Helper function for updating animation from movement (legacy support)
pub fn update_animation_from_movement(
    controller: &mut AnimationController,
    is_moving: bool,
    direction_x: f32,
    direction_y: f32,
) {
    if !is_moving {
        controller.set_animation(AnimationType::Idle);
        return;
    }

    let animation = if direction_x.abs() > direction_y.abs() {
        if direction_x > 0.0 {
            AnimationType::WalkRight
        } else {
            AnimationType::WalkLeft
        }
    } else if direction_y > 0.0 {
        AnimationType::WalkUp
    } else {
        AnimationType::WalkDown
    };

    controller.set_animation(animation);
}

/// System to update DirectionalAnimation state based on movement input
pub fn update_directional_animation_state(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut DirectionalAnimation>,
) {
    let mut direction = Vec2::ZERO;

    // Check movement input
    if keyboard.pressed(KeyCode::KeyW) {
        direction.x += 1.0;
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.x -= 1.0;
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        direction.y -= 1.0;
    }

    let is_moving = direction.length() > 0.1;

    for mut directional_anim in query.iter_mut() {
        if is_moving {
            // Determine direction based on input
            let anim_direction = if direction.x.abs() > direction.y.abs() {
                if direction.x > 0.0 {
                    crate::components::animation::Direction::Right
                } else {
                    crate::components::animation::Direction::Left
                }
            } else if direction.y > 0.0 {
                crate::components::animation::Direction::Up
            } else {
                crate::components::animation::Direction::Down
            };

            directional_anim.set_state_and_direction(AnimationState::Walking, anim_direction);
        } else {
            // Keep current direction but set to idle
            let current_direction = directional_anim.current_direction;
            directional_anim.set_state_and_direction(AnimationState::Idle, current_direction);
        }
    }
}
