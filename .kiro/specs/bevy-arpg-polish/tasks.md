# Implementation Plan: Bevy ARPG Polish

## Overview

This implementation plan transforms a basic Bevy isometric game into a production-quality ARPG by fixing critical rendering bugs, implementing proper sprite animations, correcting movement mechanics, and establishing robust z-layering. The approach focuses on incremental fixes with comprehensive testing to ensure Steam-quality presentation.

## Tasks

- [ ] 1. Fix core rendering and z-layering system
  - [x] 1.1 Implement proper z-layer management system
    - Create ZLayer component and ZLayerManager resource
    - Implement z-layer assignment for all entity types (tiles=0, player=10, bat=11, camera=999)
    - Add depth sorting system for proper rendering order
    - _Requirements: 1.4, 2.5, 4.4, 5.2, 6.1, 6.2, 6.3, 6.4, 6.6_

  - [x] 1.2 Write property test for z-layer assignment consistency
    - **Property 3: Z-layer assignment consistency**
    - **Validates: Requirements 1.4, 2.5, 4.4, 5.2, 6.1, 6.2, 6.3, 6.4**

  - [x] 1.3 Write property test for depth sorting correctness
    - **Property 16: Depth sorting correctness**
    - **Validates: Requirements 6.6**

- [ ] 2. Fix isometric map rendering system
  - [x] 2.1 Implement correct isometric projection and tile alignment
    - Fix diamond-shaped isometric projection calculations
    - Correct tile positioning to eliminate gaps and overlaps
    - Implement proper tileset indexing from assets/isometric_tiles.png
    - _Requirements: 1.1, 1.2, 1.3_

  - [x] 2.2 Write property test for isometric projection consistency
    - **Property 1: Isometric projection consistency**
    - **Validates: Requirements 1.1, 1.2**

  - [x] 2.3 Write property test for sprite indexing correctness
    - **Property 2: Sprite indexing correctness**
    - **Validates: Requirements 1.3**

- [x] 3. Checkpoint - Verify map rendering fixes
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. Fix player sprite visibility and animation system
  - [x] 4.1 Implement player spritesheet loading and parsing
    - Load sprites/player_spritesheet.png as 4x4 grid with 16 frames
    - Create texture atlas layout for player animations
    - Replace white square with proper player sprite
    - _Requirements: 2.1, 2.2, 2.6_

  - [x] 4.2 Implement directional animation system
    - Create DirectionalAnimation component with up/down/left/right animations
    - Implement animation state management (idle/walking)
    - Add animation frame timing and transitions
    - _Requirements: 2.3, 2.4_
    
  - [x] 4.3 Write property test for player sprite visibility
    - **Property 4: Player sprite visibility**
    - **Validates: Requirements 2.2**

  - [ ] 4.4 Write property test for spritesheet parsing accuracy
    - **Property 7: Spritesheet parsing accuracy**
    - **Validates: Requirements 2.6, 7.3**

  - [~] 4.5 Write property test for directional animation mapping
    - **Property 5: Directional animation mapping**
    - **Validates: Requirements 2.3**

  - [~] 4.6 Write property test for animation state transitions
    - **Property 6: Animation state transitions**
    - **Validates: Requirements 2.4, 4.2, 4.3, 7.2**

- [ ] 5. Fix movement system precision and responsiveness
  - [~] 5.1 Implement precise movement calculations
    - Set movement speed to exactly 64 pixels per second
    - Implement proper delta_time integration for frame-rate independence
    - Add grid snapping to isometric coordinates
    - Implement immediate input responsiveness
    - _Requirements: 3.1, 3.2, 3.3, 3.5_

  - [~] 5.2 Write property test for movement speed precision
    - **Property 8: Movement speed precision**
    - **Validates: Requirements 3.1**

  - [~] 5.3 Write property test for grid snapping consistency
    - **Property 9: Grid snapping consistency**
    - **Validates: Requirements 3.2**

  - [~] 5.4 Write property test for frame-rate independence
    - **Property 10: Frame-rate independence**
    - **Validates: Requirements 3.3, 7.4**

  - [~] 5.5 Write property test for input responsiveness
    - **Property 11: Input responsiveness**
    - **Validates: Requirements 3.5**

- [~] 6. Checkpoint - Verify player system fixes
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 7. Fix bat enemy sprite and animation system
  - [~] 7.1 Implement bat enemy spritesheet loading
    - Load sprites/bat.png as 4-frame horizontal strip
    - Replace colored rectangle with proper bat sprite
    - Set up bat as child entity of player
    - _Requirements: 4.1, 4.5_

  - [~] 7.2 Implement bat animation system
    - Create attack and idle animations using 4-frame spritesheet
    - Implement combat state detection and animation switching
    - _Requirements: 4.2, 4.3_

  - [~] 7.3 Write property test for enemy sprite visibility
    - **Property 12: Enemy sprite visibility**
    - **Validates: Requirements 4.1**

  - [~] 7.4 Write property test for parent-child relationship maintenance
    - **Property 13: Parent-child relationship maintenance**
    - **Validates: Requirements 4.5**

- [ ] 8. Implement smooth camera following system
  - [~] 8.1 Create camera following behavior
    - Implement linear interpolation-based camera following
    - Set initial camera position centered on player
    - Ensure smooth tracking during player movement
    - _Requirements: 5.1, 5.4, 5.5_

  - [~] 8.2 Write property test for camera following behavior
    - **Property 14: Camera following behavior**
    - **Validates: Requirements 5.1**

  - [~] 8.3 Write property test for camera centering consistency
    - **Property 15: Camera centering consistency**
    - **Validates: Requirements 5.5**

  - [~] 8.4 Write unit test for camera initialization
    - Test initial camera positioning at game start
    - _Requirements: 5.4_

- [ ] 9. Implement comprehensive animation timing system
  - [~] 9.1 Create unified animation timing system
    - Implement consistent frame rates across all entities
    - Add animation state persistence for invisible entities
    - Ensure smooth transitions between animation states
    - _Requirements: 7.1, 7.4, 7.5_

  - [~] 9.2 Write property test for animation timing consistency
    - **Property 17: Animation timing consistency**
    - **Validates: Requirements 7.1**

  - [~] 9.3 Write property test for animation state persistence
    - **Property 18: Animation state persistence**
    - **Validates: Requirements 7.5**

- [ ] 10. Implement robust asset loading and error handling
  - [~] 10.1 Create comprehensive asset loading system
    - Implement error handling for missing or invalid sprite files
    - Add fallback textures for failed asset loads
    - Create detailed error logging and recovery mechanisms
    - _Requirements: 8.3_

  - [~] 10.2 Write property test for asset loading reliability
    - **Property 19: Asset loading reliability**
    - **Validates: Requirements 8.3**

  - [~] 10.3 Write unit tests for error handling scenarios
    - Test missing file handling
    - Test invalid format handling
    - Test memory exhaustion scenarios
    - _Requirements: 8.3_

- [ ] 11. Final integration and performance optimization
  - [~] 11.1 Wire all systems together
    - Integrate all fixed systems into main game loop
    - Ensure proper system ordering and dependencies
    - Verify 60 FPS performance target
    - _Requirements: 8.1, 8.5_

  - [~] 11.2 Write integration tests for system coordination
    - Test interactions between movement, animation, and camera systems
    - Test z-layer management across all entity types
    - _Requirements: 8.5_

- [~] 12. Final checkpoint - Comprehensive testing
  - Ensure all tests pass, ask the user if questions arise.
  - Verify Steam-quality visual presentation
  - Confirm all critical bugs are resolved

## Notes

- All tasks are required for comprehensive implementation from start
- Each task references specific requirements for traceability
- Property tests validate universal correctness properties with minimum 100 iterations
- Unit tests validate specific examples and error conditions
- Checkpoints ensure incremental validation and user feedback opportunities
- All sprite assets are assumed to be available in the specified paths
- The implementation uses Bevy's ECS architecture with proper component separation