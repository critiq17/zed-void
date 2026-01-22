use bevy::prelude::*;
use crate::components::animation::{AnimationController, AnimationType};

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationController, &mut Sprite)>,
    atlas_handles: Query<&Handle<TextureAtlasLayout>>,
) {
    for (mut controller, mut _sprite) in query.iter_mut() {
        controller.frame_timer.tick(time.delta());
        
        if controller.frame_timer.just_finished() {
            let (start, end) = controller.current_animation.frame_range();
            let frame_count = end - start + 1;

            controller.current_frame = (controller.current_frame + 1) % frame_count;
            
            // TODO: Here will be update TextureAtlas index
            // when we will add TextureAlias 
        }
    }
}

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