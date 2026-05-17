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
- Basic entity IDs, transforms, health, enemies, projectiles, and gameplay systems.
- Sandbox gameplay loop with player movement, enemy spawning/chasing, automatic projectile firing, projectile hits, and reset.

The project is currently in Phase 3: entity, scene, and gameplay-system structure. The current entity/component model is intentionally small and direct; it is being shaped by the sandbox before committing to a larger ECS abstraction.

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

The sandbox loads runtime settings from `sandbox/config/runtime.toml`.

## Verify Changes

Run formatting and tests before treating a change as complete:

```bash
cargo fmt --check
cargo test
```

The current test suite covers runtime configuration, frame timing, fixed timestep behavior, and focused gameplay-system behavior in the sandbox.

## Development Notes

- Keep reusable engine behavior in `engine`.
- Keep prototype-specific gameplay in `sandbox` until an engine abstraction proves itself.
- Prefer small, testable gameplay systems over adding a broad ECS or plugin layer too early.
- Validate each phase with a running demo, not only unit tests.
