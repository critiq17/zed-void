use bevy::prelude::*;
use proptest::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test data structure representing different spritesheet configurations
    #[derive(Debug, Clone)]
    struct SpritesheetConfig {
        sprite_width: u32,
        sprite_height: u32,
        columns: u32,
        rows: u32,
        expected_frame_count: usize,
    }

    impl SpritesheetConfig {
        fn new(sprite_width: u32, sprite_height: u32, columns: u32, rows: u32) -> Self {
            Self {
                sprite_width,
                sprite_height,
                columns,
                rows,
                expected_frame_count: (columns * rows) as usize,
            }
        }
    }

    /// Known spritesheet configurations used in the game
    fn get_known_spritesheet_configs() -> Vec<SpritesheetConfig> {
        vec![
            // Player spritesheet: 4x4 grid with 32x32 sprites (16 frames)
            SpritesheetConfig::new(32, 32, 4, 4),
            // Tileset: 12x24 grid with 64x32 sprites (288 tiles)
            SpritesheetConfig::new(64, 32, 12, 24),
            // Bat spritesheet: 4x1 grid with 32x32 sprites (4 frames)
            SpritesheetConfig::new(32, 32, 4, 1),
        ]
    }

    #[test]
    fn test_player_spritesheet_parsing_accuracy() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(ImagePlugin::default())
            .init_asset::<TextureAtlasLayout>();

        let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
        
        // Test player spritesheet parsing (4x4 grid, 32x32 sprites)
        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),  // Individual sprite size
            4,                   // 4 columns
            4,                   // 4 rows  
            None,
            None,
        );
        let atlas_layout_handle = texture_atlases.add(atlas_layout);
        
        // Verify the layout was parsed correctly
        let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
        
        // Verify frame count matches expected 4x4 = 16 frames
        assert_eq!(layout.textures.len(), 16, 
            "Player spritesheet should have 16 frames for 4x4 grid");
        
        // Verify each frame has correct dimensions
        for (i, texture_rect) in layout.textures.iter().enumerate() {
            assert_eq!(texture_rect.width(), 32, 
                "Player spritesheet frame {} should be 32 pixels wide", i);
            assert_eq!(texture_rect.height(), 32, 
                "Player spritesheet frame {} should be 32 pixels tall", i);
        }
        
        // Verify grid layout is correct (frames should be arranged in 4x4 pattern)
        for row in 0..4 {
            for col in 0..4 {
                let frame_index = row * 4 + col;
                let texture_rect = &layout.textures[frame_index];
                
                // Expected position in the texture atlas
                let expected_x = col as f32 * 32.0;
                let expected_y = row as f32 * 32.0;
                
                assert_eq!(texture_rect.min.x, expected_x,
                    "Frame {} should be at x position {}", frame_index, expected_x);
                assert_eq!(texture_rect.min.y, expected_y,
                    "Frame {} should be at y position {}", frame_index, expected_y);
            }
        }
    }

    #[test]
    fn test_tileset_spritesheet_parsing_accuracy() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(ImagePlugin::default())
            .init_asset::<TextureAtlasLayout>();

        let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
        
        // Test tileset parsing (12x24 grid, 64x32 sprites)
        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(64, 32),  // Individual tile size
            12,                  // 12 columns
            24,                  // 24 rows  
            None,
            None,
        );
        let atlas_layout_handle = texture_atlases.add(atlas_layout);
        
        // Verify the layout was parsed correctly
        let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
        
        // Verify frame count matches expected 12x24 = 288 tiles
        assert_eq!(layout.textures.len(), 288, 
            "Tileset should have 288 tiles for 12x24 grid");
        
        // Verify each tile has correct dimensions
        for (i, texture_rect) in layout.textures.iter().enumerate() {
            assert_eq!(texture_rect.width(), 64, 
                "Tileset tile {} should be 64 pixels wide", i);
            assert_eq!(texture_rect.height(), 32, 
                "Tileset tile {} should be 32 pixels tall", i);
        }
        
        // Verify grid layout is correct for a few sample tiles
        let sample_indices = [0, 11, 12, 23, 276, 287]; // First, last of first row, first/last of second row, last two
        for &tile_index in &sample_indices {
            let texture_rect = &layout.textures[tile_index];
            
            let row = tile_index / 12;
            let col = tile_index % 12;
            
            let expected_x = col as f32 * 64.0;
            let expected_y = row as f32 * 32.0;
            
            assert_eq!(texture_rect.min.x, expected_x,
                "Tile {} should be at x position {}", tile_index, expected_x);
            assert_eq!(texture_rect.min.y, expected_y,
                "Tile {} should be at y position {}", tile_index, expected_y);
        }
    }

    #[test]
    fn test_bat_spritesheet_parsing_accuracy() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(ImagePlugin::default())
            .init_asset::<TextureAtlasLayout>();

        let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
        
        // Test bat spritesheet parsing (4x1 grid, 32x32 sprites)
        let atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),  // Individual sprite size
            4,                   // 4 columns
            1,                   // 1 row  
            None,
            None,
        );
        let atlas_layout_handle = texture_atlases.add(atlas_layout);
        
        // Verify the layout was parsed correctly
        let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
        
        // Verify frame count matches expected 4x1 = 4 frames
        assert_eq!(layout.textures.len(), 4, 
            "Bat spritesheet should have 4 frames for 4x1 grid");
        
        // Verify each frame has correct dimensions
        for (i, texture_rect) in layout.textures.iter().enumerate() {
            assert_eq!(texture_rect.width(), 32, 
                "Bat spritesheet frame {} should be 32 pixels wide", i);
            assert_eq!(texture_rect.height(), 32, 
                "Bat spritesheet frame {} should be 32 pixels tall", i);
        }
        
        // Verify horizontal layout (all frames in one row)
        for col in 0..4 {
            let texture_rect = &layout.textures[col];
            
            let expected_x = col as f32 * 32.0;
            let expected_y = 0.0; // Single row, so y is always 0
            
            assert_eq!(texture_rect.min.x, expected_x,
                "Bat frame {} should be at x position {}", col, expected_x);
            assert_eq!(texture_rect.min.y, expected_y,
                "Bat frame {} should be at y position {}", col, expected_y);
        }
    }

    // **Property 7: Spritesheet parsing accuracy**
    // **Validates: Requirements 2.6, 7.3**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_spritesheet_parsing_accuracy(
            sprite_width in 8u32..128u32,
            sprite_height in 8u32..128u32,
            columns in 1u32..16u32,
            rows in 1u32..16u32,
        ) {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins)
                .add_plugins(AssetPlugin::default())
                .add_plugins(ImagePlugin::default())
                .init_asset::<TextureAtlasLayout>();

            let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
            
            // Property: For any spritesheet file, the texture atlas layout should 
            // correctly parse the grid dimensions and frame count
            let atlas_layout = TextureAtlasLayout::from_grid(
                UVec2::new(sprite_width, sprite_height),
                columns,
                rows,
                None,
                None,
            );
            let atlas_layout_handle = texture_atlases.add(atlas_layout);
            
            // Verify the layout was created successfully
            let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
            
            // Property 1: Frame count should equal columns * rows
            let expected_frame_count = (columns * rows) as usize;
            prop_assert_eq!(layout.textures.len(), expected_frame_count,
                "Frame count should equal columns ({}) * rows ({}) = {}", 
                columns, rows, expected_frame_count);
            
            // Property 2: Each frame should have the specified dimensions
            for (i, texture_rect) in layout.textures.iter().enumerate() {
                prop_assert_eq!(texture_rect.width(), sprite_width,
                    "Frame {} should have width {}", i, sprite_width);
                prop_assert_eq!(texture_rect.height(), sprite_height,
                    "Frame {} should have height {}", i, sprite_height);
            }
            
            // Property 3: Frames should be arranged in correct grid pattern
            for frame_index in 0..expected_frame_count {
                let texture_rect = &layout.textures[frame_index];
                
                let row = frame_index / columns as usize;
                let col = frame_index % columns as usize;
                
                let expected_x = col as f32 * sprite_width as f32;
                let expected_y = row as f32 * sprite_height as f32;
                
                prop_assert_eq!(texture_rect.min.x, expected_x,
                    "Frame {} should be at x position {} (row {}, col {})", 
                    frame_index, expected_x, row, col);
                prop_assert_eq!(texture_rect.min.y, expected_y,
                    "Frame {} should be at y position {} (row {}, col {})", 
                    frame_index, expected_y, row, col);
            }
            
            // Property 4: Frame rectangles should not overlap
            for i in 0..layout.textures.len() {
                for j in (i + 1)..layout.textures.len() {
                    let rect1 = &layout.textures[i];
                    let rect2 = &layout.textures[j];
                    
                    // Check if rectangles overlap
                    let no_overlap = rect1.max.x <= rect2.min.x || 
                                   rect2.max.x <= rect1.min.x ||
                                   rect1.max.y <= rect2.min.y || 
                                   rect2.max.y <= rect1.min.y;
                    
                    prop_assert!(no_overlap,
                        "Frame {} and frame {} should not overlap", i, j);
                }
            }
            
            // Property 5: All frames should fit within the expected texture bounds
            let expected_texture_width = columns as f32 * sprite_width as f32;
            let expected_texture_height = rows as f32 * sprite_height as f32;
            
            for (i, texture_rect) in layout.textures.iter().enumerate() {
                prop_assert!(texture_rect.min.x >= 0.0,
                    "Frame {} min x should be non-negative", i);
                prop_assert!(texture_rect.min.y >= 0.0,
                    "Frame {} min y should be non-negative", i);
                prop_assert!(texture_rect.max.x <= expected_texture_width,
                    "Frame {} max x should be within texture width {}", i, expected_texture_width);
                prop_assert!(texture_rect.max.y <= expected_texture_height,
                    "Frame {} max y should be within texture height {}", i, expected_texture_height);
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_spritesheet_parsing_known_configurations(
            config_index in 0usize..3usize,
        ) {
            let configs = get_known_spritesheet_configs();
            let config = &configs[config_index];
            
            let mut app = App::new();
            app.add_plugins(MinimalPlugins)
                .add_plugins(AssetPlugin::default())
                .add_plugins(ImagePlugin::default())
                .init_asset::<TextureAtlasLayout>();

            let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
            
            // Property: Known spritesheet configurations should parse correctly
            let atlas_layout = TextureAtlasLayout::from_grid(
                UVec2::new(config.sprite_width, config.sprite_height),
                config.columns,
                config.rows,
                None,
                None,
            );
            let atlas_layout_handle = texture_atlases.add(atlas_layout);
            
            let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
            
            // Verify frame count matches expected
            prop_assert_eq!(layout.textures.len(), config.expected_frame_count,
                "Known config should have {} frames", config.expected_frame_count);
            
            // Verify all frames have correct dimensions
            for (i, texture_rect) in layout.textures.iter().enumerate() {
                prop_assert_eq!(texture_rect.width(), config.sprite_width,
                    "Known config frame {} should have width {}", i, config.sprite_width);
                prop_assert_eq!(texture_rect.height(), config.sprite_height,
                    "Known config frame {} should have height {}", i, config.sprite_height);
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_spritesheet_parsing_edge_cases(
            sprite_size in 1u32..4u32,
            grid_size in 1u32..4u32,
        ) {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins)
                .add_plugins(AssetPlugin::default())
                .add_plugins(ImagePlugin::default())
                .init_asset::<TextureAtlasLayout>();

            let mut texture_atlases = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>();
            
            // Property: Edge cases (small sprites, small grids) should parse correctly
            let atlas_layout = TextureAtlasLayout::from_grid(
                UVec2::new(sprite_size, sprite_size),
                grid_size,
                grid_size,
                None,
                None,
            );
            let atlas_layout_handle = texture_atlases.add(atlas_layout);
            
            let layout = texture_atlases.get(&atlas_layout_handle).unwrap();
            
            let expected_frame_count = (grid_size * grid_size) as usize;
            
            // Property: Even edge cases should have correct frame count
            prop_assert_eq!(layout.textures.len(), expected_frame_count,
                "Edge case should have {} frames", expected_frame_count);
            
            // Property: Even tiny sprites should have correct dimensions
            for (i, texture_rect) in layout.textures.iter().enumerate() {
                prop_assert_eq!(texture_rect.width(), sprite_size,
                    "Edge case frame {} should have width {}", i, sprite_size);
                prop_assert_eq!(texture_rect.height(), sprite_size,
                    "Edge case frame {} should have height {}", i, sprite_size);
            }
            
            // Property: Edge case frames should still be properly positioned
            for frame_index in 0..expected_frame_count {
                let texture_rect = &layout.textures[frame_index];
                
                let row = frame_index / grid_size as usize;
                let col = frame_index % grid_size as usize;
                
                let expected_x = col as f32 * sprite_size as f32;
                let expected_y = row as f32 * sprite_size as f32;
                
                prop_assert_eq!(texture_rect.min.x, expected_x,
                    "Edge case frame {} should be at x position {}", frame_index, expected_x);
                prop_assert_eq!(texture_rect.min.y, expected_y,
                    "Edge case frame {} should be at y position {}", frame_index, expected_y);
            }
        }
    }
}