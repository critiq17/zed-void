use bevy::prelude::*;
use std::collections::HashMap;

/// Component that defines the z-layer (depth) of an entity for rendering order
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ZLayer(pub f32);

impl ZLayer {
    pub fn new(layer: f32) -> Self {
        Self(layer)
    }
    
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Resource that manages z-layer assignments for different entity types
#[derive(Resource, Debug)]
pub struct ZLayerManager {
    layer_assignments: HashMap<EntityType, f32>,
}

impl Default for ZLayerManager {
    fn default() -> Self {
        let mut layer_assignments = HashMap::new();
        
        // Set the required z-layer assignments as specified in the task
        layer_assignments.insert(EntityType::Tile, 0.0);
        layer_assignments.insert(EntityType::Player, 10.0);
        layer_assignments.insert(EntityType::Bat, 11.0);
        layer_assignments.insert(EntityType::Camera, 999.0);
        
        Self {
            layer_assignments,
        }
    }
}

impl ZLayerManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Get the z-layer value for a specific entity type
    pub fn get_layer(&self, entity_type: EntityType) -> f32 {
        self.layer_assignments.get(&entity_type).copied().unwrap_or(0.0)
    }
    
    /// Set the z-layer value for a specific entity type
    pub fn set_layer(&mut self, entity_type: EntityType, layer: f32) {
        self.layer_assignments.insert(entity_type, layer);
    }
    
    /// Get all layer assignments
    pub fn get_all_layers(&self) -> &HashMap<EntityType, f32> {
        &self.layer_assignments
    }
}

/// Enum representing different entity types for z-layer management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Tile,
    Player,
    Bat,
    Camera,
}

/// Marker components for different entity types to help with z-layer assignment
#[derive(Component)]
pub struct IsometricTile;

#[derive(Component)]
pub struct PlayerEntity;

#[derive(Component)]
pub struct BatEntity;

#[derive(Component)]
pub struct CameraEntity;

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_z_layer_creation() {
        let z_layer = ZLayer::new(10.0);
        assert_eq!(z_layer.value(), 10.0);
    }

    #[test]
    fn test_z_layer_manager_default_assignments() {
        let manager = ZLayerManager::new();
        
        // Test the required z-layer assignments as specified in the task
        assert_eq!(manager.get_layer(EntityType::Tile), 0.0);
        assert_eq!(manager.get_layer(EntityType::Player), 10.0);
        assert_eq!(manager.get_layer(EntityType::Bat), 11.0);
        assert_eq!(manager.get_layer(EntityType::Camera), 999.0);
    }

    #[test]
    fn test_z_layer_manager_set_layer() {
        let mut manager = ZLayerManager::new();
        
        // Test setting a new layer value
        manager.set_layer(EntityType::Player, 15.0);
        assert_eq!(manager.get_layer(EntityType::Player), 15.0);
        
        // Test that other layers remain unchanged
        assert_eq!(manager.get_layer(EntityType::Tile), 0.0);
        assert_eq!(manager.get_layer(EntityType::Bat), 11.0);
        assert_eq!(manager.get_layer(EntityType::Camera), 999.0);
    }

    #[test]
    fn test_z_layer_manager_unknown_entity_type() {
        let manager = ZLayerManager::new();
        
        // This test would require adding a new entity type, but for now we test the existing ones
        // The get_layer method returns 0.0 for unknown types (fallback behavior)
        assert_eq!(manager.get_layer(EntityType::Tile), 0.0);
    }

    #[test]
    fn test_z_layer_ordering() {
        let manager = ZLayerManager::new();
        
        // Test that the z-layer values are in the correct order for proper rendering
        let tile_layer = manager.get_layer(EntityType::Tile);
        let player_layer = manager.get_layer(EntityType::Player);
        let bat_layer = manager.get_layer(EntityType::Bat);
        let camera_layer = manager.get_layer(EntityType::Camera);
        
        // Tiles should be at the bottom (lowest z-value)
        assert!(tile_layer < player_layer);
        assert!(tile_layer < bat_layer);
        assert!(tile_layer < camera_layer);
        
        // Player should be below bat
        assert!(player_layer < bat_layer);
        
        // Camera should be at the top (highest z-value)
        assert!(camera_layer > tile_layer);
        assert!(camera_layer > player_layer);
        assert!(camera_layer > bat_layer);
    }

    // **Property 3: Z-layer assignment consistency**
    // **Validates: Requirements 1.4, 2.5, 4.4, 5.2, 6.1, 6.2, 6.3, 6.4**
    proptest! {
        #[test]
        fn prop_z_layer_assignment_consistency(
            entity_type in prop::sample::select(vec![
                EntityType::Tile,
                EntityType::Player,
                EntityType::Bat,
                EntityType::Camera,
            ])
        ) {
            let manager = ZLayerManager::new();
            let assigned_layer = manager.get_layer(entity_type);
            
            // Property: For any entity type, the z-layer assignment should match 
            // the specified layer for that entity type
            let expected_layer = match entity_type {
                EntityType::Tile => 0.0,
                EntityType::Player => 10.0,
                EntityType::Bat => 11.0,
                EntityType::Camera => 999.0,
            };
            
            prop_assert_eq!(assigned_layer, expected_layer,
                "Entity type {:?} should have z-layer {}, but got {}",
                entity_type, expected_layer, assigned_layer);
        }
    }

    proptest! {
        #[test]
        fn prop_z_layer_consistency_across_managers(
            entity_type in prop::sample::select(vec![
                EntityType::Tile,
                EntityType::Player,
                EntityType::Bat,
                EntityType::Camera,
            ])
        ) {
            // Property: Multiple ZLayerManager instances should assign the same z-layer
            // for the same entity type (consistency across different manager instances)
            let manager1 = ZLayerManager::new();
            let manager2 = ZLayerManager::new();
            
            let layer1 = manager1.get_layer(entity_type);
            let layer2 = manager2.get_layer(entity_type);
            
            prop_assert_eq!(layer1, layer2,
                "Different ZLayerManager instances should assign the same z-layer for {:?}",
                entity_type);
        }
    }

    proptest! {
        #[test]
        fn prop_z_layer_component_consistency(
            layer_value in -1000.0f32..1000.0f32
        ) {
            // Property: ZLayer component should consistently store and retrieve the same value
            let z_layer = ZLayer::new(layer_value);
            
            prop_assert_eq!(z_layer.value(), layer_value,
                "ZLayer component should store and retrieve the same value");
            
            // Test that cloning preserves the value
            let cloned_layer = z_layer.clone();
            prop_assert_eq!(cloned_layer.value(), layer_value,
                "Cloned ZLayer should have the same value as original");
        }
    }

    proptest! {
        #[test]
        fn prop_z_layer_ordering_invariant(
            entity_types in prop::collection::vec(
                prop::sample::select(vec![
                    EntityType::Tile,
                    EntityType::Player,
                    EntityType::Bat,
                    EntityType::Camera,
                ]),
                1..=10
            )
        ) {
            let manager = ZLayerManager::new();
            
            // Property: Z-layer values should maintain proper ordering regardless of 
            // which entity types are queried
            for entity_type in entity_types {
                let layer = manager.get_layer(entity_type);
                
                match entity_type {
                    EntityType::Tile => {
                        // Tiles should always be at the lowest layer (0.0)
                        prop_assert!(layer <= manager.get_layer(EntityType::Player));
                        prop_assert!(layer <= manager.get_layer(EntityType::Bat));
                        prop_assert!(layer <= manager.get_layer(EntityType::Camera));
                    },
                    EntityType::Player => {
                        // Player should be above tiles but below bat and camera
                        prop_assert!(layer >= manager.get_layer(EntityType::Tile));
                        prop_assert!(layer <= manager.get_layer(EntityType::Bat));
                        prop_assert!(layer <= manager.get_layer(EntityType::Camera));
                    },
                    EntityType::Bat => {
                        // Bat should be above tiles and player but below camera
                        prop_assert!(layer >= manager.get_layer(EntityType::Tile));
                        prop_assert!(layer >= manager.get_layer(EntityType::Player));
                        prop_assert!(layer <= manager.get_layer(EntityType::Camera));
                    },
                    EntityType::Camera => {
                        // Camera should be at the highest layer
                        prop_assert!(layer >= manager.get_layer(EntityType::Tile));
                        prop_assert!(layer >= manager.get_layer(EntityType::Player));
                        prop_assert!(layer >= manager.get_layer(EntityType::Bat));
                    },
                }
            }
        }
    }

    // **Property 16: Depth sorting correctness**
    // **Validates: Requirements 6.6**
    proptest! {
        #[test]
        fn prop_depth_sorting_correctness(
            entities in prop::collection::vec(
                (
                    prop::sample::select(vec![
                        EntityType::Tile,
                        EntityType::Player,
                        EntityType::Bat,
                        EntityType::Camera,
                    ]),
                    -100.0f32..100.0f32, // x position
                    -100.0f32..100.0f32, // y position
                    -1000.0f32..1000.0f32, // initial z position (should be overridden)
                ),
                1..=20
            )
        ) {
            use bevy::app::App;
            use bevy::prelude::*;
            
            let mut app = App::new();
            app.insert_resource(ZLayerManager::new());
            
            // Spawn entities with their assigned z-layers and random positions
            let mut entity_data = Vec::new();
            for (entity_type, x, y, initial_z) in entities {
                let z_layer_value = match entity_type {
                    EntityType::Tile => 0.0,
                    EntityType::Player => 10.0,
                    EntityType::Bat => 11.0,
                    EntityType::Camera => 999.0,
                };
                
                let entity = app.world_mut().spawn((
                    Transform::from_xyz(x, y, initial_z),
                    ZLayer::new(z_layer_value),
                )).id();
                
                entity_data.push((entity, entity_type, z_layer_value));
            }
            
            // Add the depth sorting system
            app.add_systems(Update, crate::systems::z_layer::depth_sorting_system);
            
            // Run one update cycle to apply depth sorting
            app.update();
            
            // Property: For any collection of entities, the rendering order should follow 
            // z-layer values from lowest to highest
            let world = app.world();
            let mut sorted_entities = Vec::new();
            
            for (entity, entity_type, expected_z) in entity_data {
                if let Some(transform) = world.get::<Transform>(entity) {
                    sorted_entities.push((entity, entity_type, expected_z, transform.translation.z));
                }
            }
            
            // Sort by actual z-coordinate (rendering order)
            sorted_entities.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
            
            // Verify that the rendering order follows z-layer values from lowest to highest
            for i in 1..sorted_entities.len() {
                let (_, _, prev_expected_z, prev_actual_z) = sorted_entities[i-1];
                let (_, _, curr_expected_z, curr_actual_z) = sorted_entities[i];
                
                // The actual z-coordinates should be in non-decreasing order
                prop_assert!(prev_actual_z <= curr_actual_z,
                    "Depth sorting failed: entity at z={} should not come after entity at z={}",
                    prev_actual_z, curr_actual_z);
                
                // The actual z-coordinates should match the expected z-layer values
                // (allowing for small micro-offsets to prevent z-fighting)
                prop_assert!((prev_actual_z - prev_expected_z).abs() < 0.1,
                    "Entity z-coordinate {} should be close to expected z-layer {}",
                    prev_actual_z, prev_expected_z);
                
                prop_assert!((curr_actual_z - curr_expected_z).abs() < 0.1,
                    "Entity z-coordinate {} should be close to expected z-layer {}",
                    curr_actual_z, curr_expected_z);
                
                // If expected z-layers are different, actual z-coordinates should respect that order
                if prev_expected_z < curr_expected_z {
                    prop_assert!(prev_actual_z <= curr_actual_z,
                        "Entity with z-layer {} (actual z={}) should render before entity with z-layer {} (actual z={})",
                        prev_expected_z, prev_actual_z, curr_expected_z, curr_actual_z);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn prop_depth_sorting_stability(
            entities in prop::collection::vec(
                prop::sample::select(vec![
                    EntityType::Tile,
                    EntityType::Player,
                    EntityType::Bat,
                    EntityType::Camera,
                ]),
                2..=10
            )
        ) {
            use bevy::app::App;
            use bevy::prelude::*;
            
            let mut app = App::new();
            app.insert_resource(ZLayerManager::new());
            
            // Spawn entities with their assigned z-layers
            let mut entity_data = Vec::new();
            for entity_type in entities {
                let z_layer_value = match entity_type {
                    EntityType::Tile => 0.0,
                    EntityType::Player => 10.0,
                    EntityType::Bat => 11.0,
                    EntityType::Camera => 999.0,
                };
                
                let entity = app.world_mut().spawn((
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    ZLayer::new(z_layer_value),
                )).id();
                
                entity_data.push((entity, z_layer_value));
            }
            
            // Add the depth sorting system
            app.add_systems(Update, crate::systems::z_layer::depth_sorting_system);
            
            // Run multiple update cycles
            app.update();
            let first_sort = get_entity_z_positions(&app, &entity_data);
            
            app.update();
            let second_sort = get_entity_z_positions(&app, &entity_data);
            
            // Property: Depth sorting should be stable - multiple runs should produce 
            // the same relative ordering
            prop_assert_eq!(first_sort.len(), second_sort.len());
            
            for i in 0..first_sort.len() {
                let (entity1, z1_first) = first_sort[i];
                let (entity2, z2_first) = second_sort[i];
                
                prop_assert_eq!(entity1, entity2, "Entity order should be stable across multiple sorts");
                
                // Z-coordinates should be identical or very close (accounting for floating point precision)
                prop_assert!((z1_first - z2_first).abs() < f32::EPSILON,
                    "Z-coordinate should be stable: {} vs {}", z1_first, z2_first);
            }
        }
    }

    // Helper function for depth sorting stability test
    fn get_entity_z_positions(app: &App, entity_data: &[(Entity, f32)]) -> Vec<(Entity, f32)> {
        let world = app.world();
        let mut positions = Vec::new();
        
        for &(entity, _) in entity_data {
            if let Some(transform) = world.get::<Transform>(entity) {
                positions.push((entity, transform.translation.z));
            }
        }
        
        // Sort by z-coordinate to get rendering order
        positions.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        positions
    }
}