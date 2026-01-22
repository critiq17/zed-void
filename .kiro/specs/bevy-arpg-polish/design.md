# Design Document

## Overview

This design document outlines the architecture and implementation approach for fixing critical bugs and polishing a Bevy isometric ARPG game. The current implementation suffers from rendering issues, invisible player sprites, incorrect movement mechanics, basic enemy visuals, non-functional camera following, and z-layering problems.

The solution involves restructuring the rendering pipeline, implementing proper sprite animation systems, fixing movement calculations, upgrading enemy visuals, adding camera following behavior, and establishing a robust z-layering system to achieve Steam-quality presentation.

## Architecture

### System Architecture

The game follows Bevy's Entity Component System (ECS) architecture with the following key systems:

1. **Rendering Systems**
   - Map rendering with isometric projection
   - Sprite rendering with proper z-ordering
   - Animation frame management

2. **Game Logic Systems**
   - Player movement with grid snapping
   - Enemy behavior and positioning
   - Camera following mechanics

3. **Asset Management Systems**
   - Spritesheet loading and parsing
   - Texture atlas management
   - Animation data organization

### Component Architecture

```rust
// Core Components
struct Player;
struct BatEnemy;
struct IsometricTile;

// Animation Components
struct AnimationIndices {
    first: usize,
    last: usize,
}

struct AnimationTimer(Timer);

struct DirectionalAnimation {
    up: AnimationIndices,
    down: AnimationIndices,
    left: AnimationIndices,
    right: AnimationIndices,
}

// Movement Components
struct GridPosition {
    x: i32,
    y: i32,
}

struct MovementSpeed(f32);

// Rendering Components
struct ZLayer(f32);
```

## Components and Interfaces

### Map Rendering Component

The map rendering system handles isometric tile display with proper alignment and z-ordering:

```rust
struct IsometricMap {
    tiles: Vec<Vec<TileType>>,
    tile_size: Vec2,
    offset: Vec2,
}

struct TileRenderer {
    tileset_handle: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}
```

### Player Animation Component

The player system manages sprite visibility, animation states, and directional movement:

```rust
struct PlayerAnimator {
    current_direction: Direction,
    current_state: AnimationState,
    spritesheet: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

enum Direction {
    Up, Down, Left, Right
}

enum AnimationState {
    Idle, Walking
}
```

### Movement System Interface

The movement system provides grid-snapped movement with proper timing:

```rust
struct MovementController {
    speed: f32, // pixels per second
    target_position: Vec3,
    is_moving: bool,
}

struct GridSnapper {
    grid_size: f32,
}
```

### Camera Following Interface

The camera system implements smooth following behavior:

```rust
struct CameraFollower {
    target: Entity,
    follow_speed: f32,
    offset: Vec3,
}
```

### Z-Layer Management Interface

The z-layer system ensures proper depth sorting:

```rust
struct ZLayerManager {
    layer_assignments: HashMap<EntityType, f32>,
}

enum EntityType {
    Tile,
    Player,
    Enemy,
    Camera,
}
```

## Data Models

### Spritesheet Data Model

```rust
struct SpritesheetData {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    grid_size: UVec2,
    frame_count: usize,
}

struct PlayerSpritesheet {
    data: SpritesheetData,
    animations: DirectionalAnimations,
}

struct DirectionalAnimations {
    idle_down: AnimationIndices,
    walk_down: AnimationIndices,
    idle_up: AnimationIndices,
    walk_up: AnimationIndices,
    idle_left: AnimationIndices,
    walk_left: AnimationIndices,
    idle_right: AnimationIndices,
    walk_right: AnimationIndices,
}
```

### Map Data Model

```rust
struct MapData {
    width: usize,
    height: usize,
    tiles: Vec<Vec<TileId>>,
    tileset: Handle<Image>,
}

struct TileId(usize);

struct IsometricProjection {
    tile_width: f32,
    tile_height: f32,
    depth_offset: f32,
}
```

### Animation Data Model

```rust
struct AnimationClip {
    indices: AnimationIndices,
    duration: f32,
    repeat: bool,
}

struct AnimationState {
    current_clip: AnimationClip,
    timer: Timer,
    is_playing: bool,
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

After analyzing the acceptance criteria, I've identified several key properties that can be tested to ensure system correctness. Here are the consolidated properties after removing redundancy:

### Property 1: Isometric projection consistency
*For any* map data and tile coordinates, the rendered position should follow the diamond isometric projection formula consistently
**Validates: Requirements 1.1, 1.2**

### Property 2: Sprite indexing correctness
*For any* valid tile ID, the tileset indexing should map to the correct sprite region without errors
**Validates: Requirements 1.3**

### Property 3: Z-layer assignment consistency
*For any* entity type, the z-layer assignment should match the specified layer for that entity type (tiles=0, player=10, bat=11, camera=999)
**Validates: Requirements 1.4, 2.5, 4.4, 5.2, 6.1, 6.2, 6.3, 6.4**

### Property 4: Player sprite visibility
*For any* player entity, the sprite component should contain valid texture data instead of default/empty values
**Validates: Requirements 2.2**

### Property 5: Directional animation mapping
*For any* movement direction, the animation system should activate the corresponding directional animation clip
**Validates: Requirements 2.3**

### Property 6: Animation state transitions
*For any* animation state change, the system should transition to the correct animation without frame skipping
**Validates: Requirements 2.4, 4.2, 4.3, 7.2**

### Property 7: Spritesheet parsing accuracy
*For any* spritesheet file, the texture atlas layout should correctly parse the grid dimensions and frame count
**Validates: Requirements 2.6, 7.3**

### Property 8: Movement speed precision
*For any* movement input over a given time period, the actual distance moved should equal 64 pixels per second times the elapsed time
**Validates: Requirements 3.1**

### Property 9: Grid snapping consistency
*For any* movement operation, the final position should align to valid grid coordinates
**Validates: Requirements 3.2**

### Property 10: Frame-rate independence
*For any* delta time value, movement and animation calculations should produce consistent results regardless of frame rate
**Validates: Requirements 3.3, 7.4**

### Property 11: Input responsiveness
*For any* input state change, the corresponding system behavior should update immediately
**Validates: Requirements 3.5**

### Property 12: Enemy sprite visibility
*For any* bat enemy entity, the sprite component should contain valid texture data from the bat spritesheet
**Validates: Requirements 4.1**

### Property 13: Parent-child relationship maintenance
*For any* player movement, the bat enemy should maintain its relative position as a child entity
**Validates: Requirements 4.5**

### Property 14: Camera following behavior
*For any* player position change, the camera position should update using linear interpolation toward the player
**Validates: Requirements 5.1**

### Property 15: Camera centering consistency
*For any* player position, the camera should maintain centering on the player regardless of movement state
**Validates: Requirements 5.5**

### Property 16: Depth sorting correctness
*For any* collection of entities, the rendering order should follow z-layer values from lowest to highest
**Validates: Requirements 6.6**

### Property 17: Animation timing consistency
*For any* set of animated entities, all animation timers should advance at the same rate
**Validates: Requirements 7.1**

### Property 18: Animation state persistence
*For any* entity visibility change, the animation state should continue tracking regardless of visibility
**Validates: Requirements 7.5**

### Property 19: Asset loading reliability
*For any* sprite asset, the loading process should complete without errors and produce valid handles
**Validates: Requirements 8.3**

## Error Handling

The system implements comprehensive error handling across all major components:

### Asset Loading Errors
- **Missing Files**: When sprite files are not found, the system logs detailed error messages and falls back to placeholder textures
- **Invalid Formats**: When image files are corrupted or in unsupported formats, the system reports specific format errors
- **Memory Issues**: When textures are too large, the system provides memory usage warnings and optimization suggestions

### Animation System Errors
- **Invalid Frame Indices**: When animation indices exceed spritesheet bounds, the system clamps to valid ranges and logs warnings
- **Missing Animation Data**: When required animations are not defined, the system falls back to default idle animations
- **Timer Overflow**: When animation timers exceed expected ranges, the system resets timers and continues playback

### Movement System Errors
- **Grid Boundary Violations**: When movement would exceed map boundaries, the system clamps position to valid coordinates
- **Invalid Input States**: When conflicting movement inputs are detected, the system prioritizes the most recent input
- **Physics Integration Issues**: When delta time values are invalid or extreme, the system uses fallback timing values

### Rendering System Errors
- **Z-Fighting Detection**: When multiple entities occupy the same z-layer, the system applies micro-offsets to prevent artifacts
- **Viewport Overflow**: When entities are positioned outside the renderable area, the system culls them efficiently
- **Texture Memory Exhaustion**: When GPU memory is insufficient, the system reduces texture quality and reports performance warnings

## Testing Strategy

The testing approach combines unit testing for specific behaviors with property-based testing for universal correctness guarantees.

### Unit Testing Focus
Unit tests validate specific examples, edge cases, and integration points:

- **Asset Loading**: Test loading of specific sprite files and verify correct texture handles
- **Animation Transitions**: Test specific state changes (idle to walking, combat to idle)
- **Grid Positioning**: Test specific coordinate transformations and boundary conditions
- **Camera Initialization**: Test initial camera positioning and setup
- **Error Conditions**: Test specific error scenarios and recovery behaviors

### Property-Based Testing Configuration
Property tests verify universal properties across randomized inputs using the `proptest` crate:

- **Minimum 100 iterations** per property test to ensure comprehensive coverage
- **Randomized inputs** including map data, entity positions, animation states, and timing values
- **Invariant verification** across all generated test cases
- **Tag format**: **Feature: bevy-arpg-polish, Property {number}: {property_text}**

### Test Coverage Requirements
- All 19 correctness properties must be implemented as property-based tests
- Critical path unit tests for asset loading, initialization, and error handling
- Integration tests for system interactions and data flow
- Performance benchmarks for 60 FPS maintenance and memory usage

### Testing Tools and Framework
- **Bevy Testing**: Built-in Bevy test utilities for ECS system testing
- **Proptest**: Property-based testing framework for Rust
- **Criterion**: Performance benchmarking for frame rate and memory analysis
- **Mock Assets**: Test doubles for sprite files and texture data during testing
```