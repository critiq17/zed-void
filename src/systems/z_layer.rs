use bevy::prelude::*;
use crate::components::z_layer::{ZLayer, ZLayerManager, EntityType, IsometricTile, PlayerEntity, BatEntity, CameraEntity};
use crate::components::player::Player;
use crate::components::combat::Bat;
use crate::components::world::Tile;

/// System that assigns z-layers to entities based on their type
pub fn assign_z_layers(
    mut commands: Commands,
    z_layer_manager: Res<ZLayerManager>,
    // Query for entities that need z-layer assignment
    tiles_query: Query<Entity, (With<Tile>, Without<ZLayer>)>,
    players_query: Query<Entity, (With<Player>, Without<ZLayer>)>,
    bats_query: Query<Entity, (With<Bat>, Without<ZLayer>)>,
    cameras_query: Query<Entity, (With<Camera>, Without<ZLayer>)>,
) {
    // Assign z-layers to tiles
    for entity in tiles_query.iter() {
        let z_layer = ZLayer::new(z_layer_manager.get_layer(EntityType::Tile));
        commands.entity(entity).insert((z_layer, IsometricTile));
    }
    
    // Assign z-layers to players
    for entity in players_query.iter() {
        let z_layer = ZLayer::new(z_layer_manager.get_layer(EntityType::Player));
        commands.entity(entity).insert((z_layer, PlayerEntity));
    }
    
    // Assign z-layers to bats
    for entity in bats_query.iter() {
        let z_layer = ZLayer::new(z_layer_manager.get_layer(EntityType::Bat));
        commands.entity(entity).insert((z_layer, BatEntity));
    }
    
    // Assign z-layers to cameras
    for entity in cameras_query.iter() {
        let z_layer = ZLayer::new(z_layer_manager.get_layer(EntityType::Camera));
        commands.entity(entity).insert((z_layer, CameraEntity));
    }
}

/// System that updates the Transform z-coordinate based on ZLayer component
pub fn update_transform_z_from_layer(
    mut query: Query<(&mut Transform, &ZLayer), Changed<ZLayer>>,
) {
    for (mut transform, z_layer) in query.iter_mut() {
        transform.translation.z = z_layer.value();
    }
}

/// System that ensures proper depth sorting by updating z-coordinates
/// This system runs after any z-layer changes to maintain rendering order
pub fn depth_sorting_system(
    mut query: Query<(&mut Transform, &ZLayer)>,
) {
    // Collect all entities with their z-layers
    let mut entities: Vec<_> = query.iter_mut().collect();
    
    // Sort by z-layer value to ensure proper rendering order
    entities.sort_by(|a, b| a.1.value().partial_cmp(&b.1.value()).unwrap_or(std::cmp::Ordering::Equal));
    
    // Update transform z-coordinates to match z-layer values
    for (mut transform, z_layer) in entities {
        transform.translation.z = z_layer.value();
    }
}

/// System that prevents z-fighting by applying micro-offsets when entities share the same z-layer
pub fn prevent_z_fighting(
    mut query: Query<(Entity, &mut Transform, &ZLayer)>,
) {
    let mut entities: Vec<_> = query.iter_mut().collect();
    
    // Group entities by z-layer value
    entities.sort_by(|a, b| a.2.value().partial_cmp(&b.2.value()).unwrap_or(std::cmp::Ordering::Equal));
    
    let mut current_z = f32::MIN;
    let mut offset_counter = 0;
    
    for (_entity, mut transform, z_layer) in entities {
        let base_z = z_layer.value();
        
        if (base_z - current_z).abs() < f32::EPSILON {
            // Same z-layer, apply micro-offset
            offset_counter += 1;
            transform.translation.z = base_z + (offset_counter as f32 * 0.001);
        } else {
            // New z-layer, reset counter
            current_z = base_z;
            offset_counter = 0;
            transform.translation.z = base_z;
        }
    }
}

/// System that validates z-layer assignments match the specification
pub fn validate_z_layer_assignments(
    z_layer_manager: Res<ZLayerManager>,
    tiles_query: Query<&ZLayer, With<IsometricTile>>,
    players_query: Query<&ZLayer, With<PlayerEntity>>,
    bats_query: Query<&ZLayer, With<BatEntity>>,
    cameras_query: Query<&ZLayer, With<CameraEntity>>,
) {
    // Validate tile z-layers
    for z_layer in tiles_query.iter() {
        if z_layer.value() != z_layer_manager.get_layer(EntityType::Tile) {
            warn!("Tile z-layer mismatch: expected {}, got {}", 
                  z_layer_manager.get_layer(EntityType::Tile), z_layer.value());
        }
    }
    
    // Validate player z-layers
    for z_layer in players_query.iter() {
        if z_layer.value() != z_layer_manager.get_layer(EntityType::Player) {
            warn!("Player z-layer mismatch: expected {}, got {}", 
                  z_layer_manager.get_layer(EntityType::Player), z_layer.value());
        }
    }
    
    // Validate bat z-layers
    for z_layer in bats_query.iter() {
        if z_layer.value() != z_layer_manager.get_layer(EntityType::Bat) {
            warn!("Bat z-layer mismatch: expected {}, got {}", 
                  z_layer_manager.get_layer(EntityType::Bat), z_layer.value());
        }
    }
    
    // Validate camera z-layers
    for z_layer in cameras_query.iter() {
        if z_layer.value() != z_layer_manager.get_layer(EntityType::Camera) {
            warn!("Camera z-layer mismatch: expected {}, got {}", 
                  z_layer_manager.get_layer(EntityType::Camera), z_layer.value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::world::TileType;
    use bevy::app::App;

    #[test]
    fn test_z_layer_assignment_system() {
        let mut app = App::new();
        app.insert_resource(ZLayerManager::new());
        
        // Spawn test entities
        let tile_entity = app.world_mut().spawn((
            Transform::default(),
            Tile::new(0, 0, TileType::GrassLight),
        )).id();
        
        let player_entity = app.world_mut().spawn((
            Transform::default(),
            Player,
        )).id();
        
        let bat_entity = app.world_mut().spawn((
            Transform::default(),
            Bat,
        )).id();
        
        let camera_entity = app.world_mut().spawn((
            Transform::default(),
            Camera2d,
        )).id();
        
        // Add the z-layer assignment system
        app.add_systems(Update, assign_z_layers);
        
        // Run one update cycle
        app.update();
        
        // Check that z-layers were assigned correctly
        let world = app.world();
        
        // Check tile z-layer
        if let Some(z_layer) = world.get::<ZLayer>(tile_entity) {
            assert_eq!(z_layer.value(), 0.0);
        } else {
            panic!("Tile entity should have ZLayer component");
        }
        
        // Check player z-layer
        if let Some(z_layer) = world.get::<ZLayer>(player_entity) {
            assert_eq!(z_layer.value(), 10.0);
        } else {
            panic!("Player entity should have ZLayer component");
        }
        
        // Check bat z-layer
        if let Some(z_layer) = world.get::<ZLayer>(bat_entity) {
            assert_eq!(z_layer.value(), 11.0);
        } else {
            panic!("Bat entity should have ZLayer component");
        }
        
        // Check camera z-layer
        if let Some(z_layer) = world.get::<ZLayer>(camera_entity) {
            assert_eq!(z_layer.value(), 999.0);
        } else {
            panic!("Camera entity should have ZLayer component");
        }
    }

    #[test]
    fn test_transform_z_update_system() {
        let mut app = App::new();
        
        // Spawn entity with transform and z-layer
        let entity = app.world_mut().spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            ZLayer::new(15.0),
        )).id();
        
        // Add the transform update system
        app.add_systems(Update, update_transform_z_from_layer);
        
        // Run one update cycle
        app.update();
        
        // Check that transform z was updated
        let world = app.world();
        if let Some(transform) = world.get::<Transform>(entity) {
            assert_eq!(transform.translation.z, 15.0);
        } else {
            panic!("Entity should have Transform component");
        }
    }

    #[test]
    fn test_depth_sorting_system() {
        let mut app = App::new();
        
        // Spawn entities with different z-layers
        let entity1 = app.world_mut().spawn((
            Transform::from_xyz(0.0, 0.0, 100.0), // Wrong z initially
            ZLayer::new(5.0),
        )).id();
        
        let entity2 = app.world_mut().spawn((
            Transform::from_xyz(0.0, 0.0, 0.0), // Wrong z initially
            ZLayer::new(15.0),
        )).id();
        
        // Add the depth sorting system
        app.add_systems(Update, depth_sorting_system);
        
        // Run one update cycle
        app.update();
        
        // Check that transforms were sorted correctly
        let world = app.world();
        
        if let Some(transform1) = world.get::<Transform>(entity1) {
            assert_eq!(transform1.translation.z, 5.0);
        }
        
        if let Some(transform2) = world.get::<Transform>(entity2) {
            assert_eq!(transform2.translation.z, 15.0);
        }
    }

    #[test]
    fn test_z_fighting_prevention() {
        let mut app = App::new();
        
        // Spawn entities with the same z-layer
        let entity1 = app.world_mut().spawn((
            Transform::from_xyz(0.0, 0.0, 10.0),
            ZLayer::new(10.0),
        )).id();
        
        let entity2 = app.world_mut().spawn((
            Transform::from_xyz(1.0, 0.0, 10.0),
            ZLayer::new(10.0),
        )).id();
        
        // Add the z-fighting prevention system
        app.add_systems(Update, prevent_z_fighting);
        
        // Run one update cycle
        app.update();
        
        // Check that micro-offsets were applied
        let world = app.world();
        
        let transform1 = world.get::<Transform>(entity1).unwrap();
        let transform2 = world.get::<Transform>(entity2).unwrap();
        
        // One should be at base z, the other should have a micro-offset
        assert!(transform1.translation.z != transform2.translation.z);
        assert!((transform1.translation.z - 10.0).abs() < 0.01);
        assert!((transform2.translation.z - 10.0).abs() < 0.01);
    }
}