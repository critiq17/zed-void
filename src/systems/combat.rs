use bevy::prelude::*;
use crate::components::combat::Bat;


pub fn bat_show_hide(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bat_query: Query<&mut Visibility, With<Bat>>,
) {
    let visible = mouse.pressed(MouseButton::Left);
    
    if let Ok(mut visibility) = bat_query.get_single_mut() {
        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}


pub fn bat_attack(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bat_query: Query<&mut Transform, With<Bat>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        
        if let Ok(mut bat_transform) = bat_query.get_single_mut() {
            bat_transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
            
        }
    }
    
   
    if mouse.just_released(MouseButton::Right) {
        if let Ok(mut bat_transform) = bat_query.get_single_mut() {
            bat_transform.rotation = Quat::IDENTITY;
        }
    }
}
/*
pub fn damage_system(
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.damage(event.amount);
            
            if !health.is_alive() {
                println!("💀 Entity {} ", event.target);
            }
        }
    }
}
*/