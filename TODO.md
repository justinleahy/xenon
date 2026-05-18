# TODO

This checklist mirrors `GAME_ENGINE_DEVELOPMENT_PHASES.md`. Completed items are marked from repository evidence in code and docs.

## Phase 0: Vision, Scope, and Constraints

### Key Decisions

- [x] Choose the target game type.
- [x] Choose the target platforms.
- [x] Choose the rendering backend.
- [x] Choose the engine style.
- [x] Choose the tooling strategy.
- [x] Choose the scripting strategy.

### Deliverables

- [x] One-page engine charter.
- [x] Initial repository structure.
- [x] Rust toolchain policy.
- [x] Coding standards and contribution rules.
- [x] Initial dependency policy.

### Exit Criteria

- [x] The first demo target is clearly defined.
- [x] The target platforms and rendering strategy are chosen.
- [x] The first three phases can be implemented without reopening core direction debates.

## Phase 1: Foundation and Runtime Loop

### Core Work

- [x] Application lifecycle.
- [x] Window creation and event handling.
- [x] Fixed and variable timestep strategy.
- [x] Main loop ownership model.
- [x] Logging and panic reporting.
- [x] Configuration loading.
- [x] Basic error handling patterns.
- [x] Feature flags for optional subsystems.

### Deliverables

- [x] A runnable application shell.
- [x] Frame timing metrics.
- [x] Graceful startup and shutdown.
- [x] Minimal runtime configuration file.
- [ ] Headless mode, if server simulation or tests become a priority.

### Exit Criteria

- [x] The engine can run a stable empty frame loop.
- [x] The engine reports frame time and lifecycle events.
- [x] Window, event, update, and render responsibilities are separated cleanly.

## Phase 2: Rendering Core

### Core Work

- [x] GPU device and swapchain initialization.
- [x] Render pass abstraction.
- [x] Shader loading and compilation flow.
- [x] Mesh and buffer management.
- [x] Texture loading path.
- [x] Camera model.
- [x] Material representation.
- [x] Basic 2D draw path.
- [x] Debug grid or primitive rendering.

### Deliverables

- [x] Clear-screen render pass.
- [x] Triangle or sprite rendering.
- [x] Camera-controlled scene.
- [x] Texture rendering path.
- [x] Debug grid or primitive overlay.
- [x] Renderer documentation explaining resource lifetime and ownership.

### Exit Criteria

- [x] The engine renders a scene every frame without validation errors.
- [x] Renderer resources can be created, reused, and destroyed predictably.
- [x] The debug renderer can visualize basic spatial information.

## Phase 3: ECS, Scene, and Transform Model

### Core Work

- [x] Entity identity and lifetime.
- [x] Component storage.
- [x] System scheduling.
- [x] Flat transform model.
- [x] Scene loading and saving.
- [x] Prefab or archetype concept.
- [x] Runtime spawning and despawning.
- [x] Stable IDs for editor and asset references.

### Deliverables

- [x] Entities with transform components.
- [x] Renderable components connected to the renderer.
- [x] Scene serialization format.
- [x] Basic system scheduler.
- [x] Demo scene loaded from disk.

### Exit Criteria

- [x] A scene can be loaded, updated, rendered, and saved.
- [x] Entities can be spawned and removed at runtime.
- [x] Rendering does not depend on hard-coded demo objects.

## Phase 4: Asset Pipeline

### Core Work

- [x] Asset registry.
- [x] Asset handles.
- [ ] Loading, caching, and unloading.
- [ ] Hot reload for development.
- [ ] Import steps for textures, meshes, shaders, fonts, audio, and scenes.
- [ ] Asset dependency tracking.
- [x] Stable asset IDs.
- [ ] Build-time asset packaging.

### Deliverables

- [x] Asset manifest format.
- [x] Runtime asset manager.
- [ ] Hot reload for at least shaders and textures.
- [ ] Import command-line tool.
- [ ] Missing-asset fallback behavior.

### Exit Criteria

- [x] Assets can be referenced by stable handles rather than raw paths.
- [ ] A changed asset can update in a running debug build.
- [ ] Packaged assets can be loaded without depending on the source directory layout.

## Phase 5: Input, Cameras, and Interaction

### Core Work

- [x] Keyboard input.
- [ ] Mouse input.
- [ ] Controller input.
- [ ] Action mapping layer.
- [ ] Input contexts.
- [ ] Cursor capture and pointer locking.
- [x] Camera-following scene view.
- [ ] Text input support.
- [ ] Input recording hooks for tests and replays.

### Deliverables

- [ ] Configurable input bindings.
- [x] Runtime input state queries.
- [ ] Example free camera or 2D camera controller.
- [ ] Input debug overlay.

### Exit Criteria

- [ ] Gameplay code can ask for actions instead of raw keys.
- [ ] Bindings can be changed without recompiling.
- [ ] Input behavior is visible enough to debug quickly.

## Phase 6: Gameplay Framework and First Vertical Slice

### Core Work

- [x] Reset and quit flow.
- [ ] Full game state transitions.
- [ ] Basic UI flow.
- [ ] Save/load strategy, if relevant.
- [x] Collision or physics placeholder.
- [ ] Audio placeholder.
- [x] Entity spawning and gameplay rules.
- [x] Minimal content pipeline for the slice.

### Deliverables

- [ ] A playable demo with a start, loop, and end condition.
- [x] One real level or test arena.
- [ ] Debug controls for reset, pause, step, and reload.
- [ ] List of engine pain points discovered during gameplay implementation.

### Exit Criteria

- [x] The demo is playable from a clean checkout.
- [x] The engine supports gameplay without special-case demo code in core systems.
- [ ] The next engine priorities are based on observed friction, not speculation.

## Phase 7: Physics and Collision

### Core Work

- [x] Collider components.
- [ ] Broadphase and narrowphase collision.
- [ ] Trigger volumes.
- [ ] Raycasts and shape casts.
- [ ] Rigid body dynamics, if needed.
- [x] Character movement support.
- [ ] Collision layers and masks.
- [ ] Debug visualization.

### Deliverables

- [ ] Static and dynamic colliders.
- [ ] Collision events.
- [ ] Spatial query API.
- [ ] Physics debug view.
- [x] Deterministic fixed-step physics update.

### Exit Criteria

- [x] Gameplay code can query and react to collisions without depending on renderer state.
- [x] Physics runs at a stable fixed timestep.
- [ ] Collision behavior can be inspected visually.

## Phase 8: Audio

### Core Work

- [ ] Audio device initialization.
- [ ] Sound asset loading.
- [ ] One-shot and looping playback.
- [ ] Volume groups.
- [ ] Spatial audio, if needed.
- [ ] Streaming music.
- [ ] Debug controls for active sounds.

### Deliverables

- [ ] Sound effect playback.
- [ ] Music playback.
- [ ] Mixer groups.
- [ ] Audio asset handles.
- [ ] Runtime mute and volume controls.

### Exit Criteria

- [ ] The vertical slice can use audio entirely through engine APIs.
- [ ] Audio resources follow the same asset ownership model as other assets.
- [ ] Playback can be controlled and debugged at runtime.

## Phase 9: UI and Text

### Core Work

- [ ] Text shaping and rendering.
- [ ] Immediate-mode debug UI.
- [ ] Runtime/game UI model.
- [ ] Layout primitives.
- [ ] Input focus.
- [ ] Theming and style data.
- [ ] Localization-ready text identifiers, if needed.

### Deliverables

- [ ] Debug inspector overlay.
- [ ] Basic menu screen.
- [ ] Text rendering path.
- [ ] Runtime UI input handling.

### Exit Criteria

- [ ] The engine can display debug state and gameplay UI.
- [ ] UI input and gameplay input can coexist without conflict.
- [ ] Text is rendered through a reusable system, not one-off debug drawing.

## Phase 10: Editor and Tooling

### Core Work

- [ ] Entity hierarchy view.
- [ ] Component inspector.
- [ ] Scene save/load integration.
- [ ] Asset browser.
- [ ] Transform gizmos.
- [ ] Play/edit mode boundary.
- [ ] Undo and redo model.
- [ ] Editor-only components or metadata.

### Deliverables

- [ ] In-engine editor or companion editor application.
- [ ] Inspector for entities and components.
- [ ] Scene editing and saving.
- [ ] Asset browser.
- [ ] Basic undo and redo.

### Exit Criteria

- [ ] A simple scene can be built without editing raw scene files by hand.
- [ ] Runtime and editor state boundaries are explicit.
- [ ] Editor changes serialize back into the same scene format used by the runtime.

## Phase 11: Animation

### Core Work

- [ ] Sprite animation or skeletal animation.
- [ ] Animation clips.
- [ ] Animation state machines or blend trees.
- [ ] Event markers.
- [ ] Retargeting, if needed.
- [ ] Animation preview tooling.

### Deliverables

- [ ] Animation asset format.
- [ ] Runtime animation player.
- [ ] Animation-driven entity demo.
- [ ] Debug visualization for current clips and states.

### Exit Criteria

- [ ] Gameplay can trigger and observe animation state.
- [ ] Animation data is loaded through the asset pipeline.
- [ ] Animated entities work in the vertical slice without bespoke code.

## Phase 12: Performance, Memory, and Parallelism

### Core Work

- [ ] Frame profiler.
- [ ] Memory tracking.
- [ ] Job system or task scheduler.
- [ ] Parallel system execution where safe.
- [x] Renderer batching.
- [ ] Asset streaming.
- [ ] ECS query optimization.
- [ ] Allocation reduction in hot paths.

### Deliverables

- [ ] Profiling integration.
- [ ] Performance budgets for CPU, GPU, memory, and asset loading.
- [ ] Stress test scenes.
- [ ] Documented hot-path allocation rules.

### Exit Criteria

- [ ] The engine can identify where frame time is spent.
- [ ] Stress scenes expose performance limits.
- [ ] Hot systems avoid avoidable per-frame allocations.

## Phase 13: Packaging, Platform Support, and Distribution

### Core Work

- [ ] Build profiles.
- [ ] Asset packaging.
- [ ] Platform-specific window, filesystem, and input behavior.
- [ ] Crash reporting strategy.
- [ ] Version metadata.
- [ ] Release automation.
- [ ] Installer or archive packaging.

### Deliverables

- [ ] Release build command.
- [ ] Packaged demo.
- [ ] Platform smoke tests.
- [ ] Asset bundle validation.
- [ ] Basic release checklist.

### Exit Criteria

- [ ] A clean machine can run a packaged demo.
- [ ] Missing assets and startup failures produce useful errors.
- [ ] Release builds are reproducible.

## Phase 14: Stability, Testing, and Long-Term Maintenance

### Core Work

- [x] Unit tests for pure logic.
- [x] Integration-style tests for asset loading, scene loading, and serialization.
- [ ] Golden tests for asset import results.
- [ ] Render tests where practical.
- [ ] API stability rules.
- [ ] Deprecation policy.
- [ ] Fuzzing for parsers and importers.
- [ ] Documentation for engine contributors.

### Deliverables

- [x] Test suite grouped by subsystem.
- [ ] Benchmark suite for hot paths.
- [ ] Contributor guide.
- [x] Engine architecture overview.
- [ ] Release notes process.

### Exit Criteria

- [x] Core systems have tests that catch regressions.
- [ ] Engine APIs have clear ownership and compatibility expectations.
- [ ] New contributors can understand how to add a feature without reading the entire codebase.
