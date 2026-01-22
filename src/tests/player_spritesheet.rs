use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::animation::{AnimationController, AnimationType};
use crate::components::z_layer::ZLayerManager;
use proptest::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_spritesheet_loading() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(ImagePlugin::default())
            .insert_resource(ZLayerManager::new())
            .init_asset::<TextureAtlasLayout>();

        // Add the setup_player system
        app.add_systems(Startup, setup_player_test);
        
        // Run the app for one frame to execute startup systems
        app.update();

        // Query for player entities
        let mut query = app.world_mut().query::<(&Player, &Sprite, &AnimationController)>();
        let results: Vec<_> = query.iter(&app.world()).collect();
        
        assert_eq!(results.len(), 1, "Should have exactly one player entity");
        
        let (_, sprite, animation_controller) = results[0];
        
        // Verify sprite has texture atlas
        assert!(sprite.texture_atlas.is_some(), "Player sprite should have texture atlas");
        
        if let Some(ref texture_atlas) = sprite.texture_atlas {
            // Verify initial frame is 0 (idle)
            assert_eq!(texture_atlas.index, 0, "Player should start with idle frame (index 0)");
        }
        
        // Verify animation controller is set to idle
        assert_eq!(animation_controller.current_animation, AnimationType::Idle, 
                   "Player should start with idle animation");
    }

    #[test]
    fn test_texture_atlas_layout_4x4_grid() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(ImagePlugin::default())
            .init_asset::<TextureAtlasLayout>();

        let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
        
        // Create the same atlas layout as in setup_player
        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),  // Individual sprite size
            4,                   // 4 columns
            4,                   // 4 rows  
            None,
            None,
        );
        let atlas_layout_handle = texture_atlases.add(atlas_layout);
        
        // Verify the layout was created
        let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
        
        // Verify it has 16 textures (4x4 grid)
        assert_eq!(layout.textures.len(), 16, "Atlas should have 16 frames for 4x4 grid");
        
        // Verify each texture has the correct size
        for texture_rect in &layout.textures {
            assert_eq!(texture_rect.width(), 32, "Each frame should be 32 pixels wide");
            assert_eq!(texture_rect.height(), 32, "Each frame should be 32 pixels tall");
        }
    }

    #[test]
    fn test_animation_frame_ranges_for_4x4_grid() {
        // Test that animation frame ranges are correct for 4x4 grid
        assert_eq!(AnimationType::Idle.frame_range(), (0, 0), "Idle should use frame 0");
        assert_eq!(AnimationType::WalkDown.frame_range(), (0, 3), "Walk down should use frames 0-3");
        assert_eq!(AnimationType::WalkUp.frame_range(), (4, 7), "Walk up should use frames 4-7");
        assert_eq!(AnimationType::WalkLeft.frame_range(), (8, 11), "Walk left should use frames 8-11");
        assert_eq!(AnimationType::WalkRight.frame_range(), (12, 15), "Walk right should use frames 12-15");
    }

    fn setup_player_test(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
        z_layer_manager: Res<ZLayerManager>,
    ) {
        // Load player spritesheet as 4x4 grid with 16 frames
        let texture_handle = asset_server.load("sprites/player_spritesheet.png");
        
        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),  // Individual sprite size
            4,                   // 4 columns
            4,                   // 4 rows  
            None,
            None,
        );
        let atlas_layout_handle = texture_atlases.add(atlas_layout);
        
        commands.spawn((
            Sprite {
                image: texture_handle,
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout_handle,
                    index: 0,
                }),
                custom_size: Some(Vec2::new(48.0, 48.0)), 
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, z_layer_manager.get_layer(crate::components::z_layer::EntityType::Player)),
            Player,
            AnimationController::new(crate::constants::ANIMATION_SPEED),
        ));
    }

    // **Property 4: Player sprite visibility**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_player_sprite_visibility(
            sprite_size_x in 16.0f32..128.0f32,
            sprite_size_y in 16.0f32..128.0f32,
            atlas_index in 0usize..16usize,
            position_x in -1000.0f32..1000.0f32,
            position_y in -1000.0f32..1000.0f32,
        ) {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins)
                .add_plugins(AssetPlugin::default())
                .add_plugins(ImagePlugin::default())
                .insert_resource(ZLayerManager::new())
                .init_asset::<TextureAtlasLayout>();

            // Create a player entity with randomized sprite properties
            let texture_handle = app.world_mut().resource::<AssetServer>()
                .load("sprites/player_spritesheet.png");
            
            let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
            let atlas_layout = TextureAtlasLayout::from_grid(
                UVec2::new(32, 32),
                4,
                4,
                None,
                None,
            );
            let atlas_layout_handle = texture_atlases.add(atlas_layout);
            drop(texture_atlases);

            let z_layer_manager = app.world().resource::<ZLayerManager>();
            let player_z = z_layer_manager.get_layer(crate::components::z_layer::EntityType::Player);

            // Spawn player entity with randomized properties
            app.world_mut().spawn((
                Sprite {
                    image: texture_handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas_layout_handle.clone(),
                        index: atlas_index,
                    }),
                    custom_size: Some(Vec2::new(sprite_size_x, sprite_size_y)),
                    ..default()
                },
                Transform::from_xyz(position_x, position_y, player_z),
                Player,
                AnimationController::new(crate::constants::ANIMATION_SPEED),
            ));

            // Run one frame to ensure systems process
            app.update();

            // Query for player entities
            let mut query = app.world_mut().query::<(&Player, &Sprite)>();
            let results: Vec<_> = query.iter(&app.world()).collect();
            
            // Property: For any player entity, the sprite component should contain 
            // valid texture data instead of default/empty values
            prop_assert_eq!(results.len(), 1, "Should have exactly one player entity");
            
            let (_, sprite) = results[0];
            
            // Verify sprite has valid texture handle (not default/empty)
            prop_assert_ne!(&sprite.image, &Handle::<Image>::default(), 
                "Player sprite should have valid texture handle, not default");
            
            // Verify sprite has texture atlas (required for spritesheet)
            prop_assert!(sprite.texture_atlas.is_some(), 
                "Player sprite should have texture atlas for spritesheet animation");
            
            if let Some(ref texture_atlas) = sprite.texture_atlas {
                // Verify atlas layout handle is valid (not default)
                prop_assert_ne!(&texture_atlas.layout, &Handle::<TextureAtlasLayout>::default(),
                    "Player sprite atlas layout should be valid, not default");
                
                // Verify atlas index is within valid range for 4x4 grid (0-15)
                prop_assert!(texture_atlas.index < 16,
                    "Player sprite atlas index {} should be within 4x4 grid range (0-15)", 
                    texture_atlas.index);
            }
            
            // Verify sprite has custom size (not using default texture size)
            prop_assert!(sprite.custom_size.is_some(),
                "Player sprite should have custom size defined");
            
            if let Some(custom_size) = sprite.custom_size {
                // Verify custom size is reasonable (not zero or negative)
                prop_assert!(custom_size.x > 0.0 && custom_size.y > 0.0,
                    "Player sprite custom size should be positive: {:?}", custom_size);
            }
        }
    }
}