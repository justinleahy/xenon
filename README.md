# Xenon

Xenon is a desktop-first Rust game engine project. The current demo target is a small Vampire Survivors-style 2D survival arena built in the `sandbox` crate while reusable runtime and rendering code grows in the `engine` crate.

This codebase is AI assisted but NOT AI generated. All main code will be written by humans. AI will only assist with test code, guiding the development process, and updating documentation.

## Current Status

Xenon has moved past the initial runtime shell into an early playable prototype.

Implemented so far:

- Runtime configuration loading.
- Window lifecycle and event handling.
- Fixed timestep simulation.
- Frame clock and FPS logging.
- Keyboard input for movement, reset, and quit.
- `wgpu` clear pass and instanced 2D quad rendering.
- Camera-following sprite scene.
- Debug grid and player health bar.
- Basic entity IDs, component storage, and scheduled gameplay systems.
- Data-driven rendering through `Transform` and `Sprite` components.
- Demo scene loading from `sandbox/config/demo_scene.toml`.
- Stable scene-object IDs for serialized scene enemies.
- Catalog-driven enemy archetypes for basic, fast, and tank enemies.
- Starting demo scene uses basic, fast, and tank enemy archetypes.
- Gun-style weapon archetypes for pistol, SMG, and shotgun behavior.
- Scene-defined active weapon sets with independent per-weapon cooldowns.
- Circle-collider based projectile hits and enemy contact damage.
- Enemy health, projectile damage, death drops, pickups, and player experience collection.
- Sandbox gameplay loop with player movement, enemy spawning/chasing, automatic projectile firing, pickup collection, and reset.

The project is currently in Phase 3: entity, scene, and gameplay-system structure. The current entity/component model is intentionally small and direct; it is being shaped by the sandbox before committing to a larger ECS abstraction or upgrade system.

## Repository Shape

- `engine` contains reusable engine code: runtime config, lifecycle types, frame timing, fixed timestep, and rendering.
- `sandbox` contains the first playable prototype and game-specific systems.

Longer-term direction:

- [Engine charter](ENGINE_CHARTER.md)
- [Development phases](GAME_ENGINE_DEVELOPMENT_PHASES.md)

## Run the Sandbox

```bash
cargo run -p sandbox
```

The sandbox opens a desktop window and runs the current survival prototype.

Controls:

- Move: `WASD` or arrow keys
- Reset simulation: `R`
- Quit: `Q` or close the window

The sandbox loads runtime settings from `sandbox/config/runtime.toml` and the demo scene from `sandbox/config/demo_scene.toml`.

## Verify Changes

Run formatting and tests before treating a change as complete:

```bash
cargo fmt --check
cargo test
```

The current test suite covers runtime configuration, frame timing, fixed timestep behavior, scene/component invariants, scene definition loading/saving, render-scene conversion, combat, enemies, player movement, pickups, player progression, and game-system scheduling.

## Development Notes

- Keep reusable engine behavior in `engine`.
- Keep prototype-specific gameplay in `sandbox` until an engine abstraction proves itself.
- Prefer small, testable gameplay systems over adding a broad ECS or plugin layer too early.
- Validate each phase with a running demo, not only unit tests.
