# Rust Game Engine Charter

## Purpose

This charter defines the starting direction for the engine. It is meant to keep early development focused while leaving room to revise decisions after the first playable demo exposes real needs.

The immediate goal is not to build a complete general-purpose engine. The immediate goal is to build a small Rust engine that can run a playable vertical slice with real rendering, assets, input, debugging, and iteration.

## Initial Product Shape

The engine starts as a desktop-first, code-first game engine written in Rust. Its first target game is a Vampire Survivors-style 2D horde survival roguelite. It should support that game well before it tries to support many genres, many platforms, or a full editor.

The first version should make it easy to:

- Open a window and run a stable frame loop.
- Render a 2D sprite-based scene.
- Load assets through engine-managed handles.
- Process configurable input.
- Represent gameplay state with entities and components.
- Spawn, update, and despawn large numbers of simple enemies, projectiles, pickups, and effects.
- Run timed waves, auto-attacks, collision checks, experience pickup, leveling, and upgrade choices.
- Inspect runtime state with debug tools.
- Package and run a small demo outside the development environment.

## First Demo Target

The first demo should be a small Vampire Survivors-style survival arena.

This is a good starter target because it exercises the important engine systems without requiring a large content pipeline. It needs player movement, automatic weapons, many enemies, collision, pickups, progression choices, audio, UI, entity spawning, scene loading, and debug controls.

Minimum demo loop:

1. Launch the game.
2. Load one survival arena scene.
3. Move a player character.
4. Spawn enemies over time from wave data.
5. Fire at least one automatic weapon without manual aiming.
6. Detect player, enemy, projectile, and pickup overlaps.
7. Drop experience pickups from defeated enemies.
8. Trigger a level-up choice after enough experience is collected.
9. Apply one upgrade that changes gameplay.
10. Play at least one sound effect.
11. Show timer, health, experience, level, and kill count.
12. Restart the demo without restarting the process.

The demo should be mechanically inspired by Vampire Survivors, but it should use original names, characters, art, sounds, weapon identities, enemy designs, and tuning.

## Genre Requirements

The first game target creates specific engine pressure. These requirements should shape early architecture:

- Efficient sprite rendering with batching.
- Stable fixed-step gameplay updates.
- Cheap entity spawning and despawning.
- Broadphase collision or spatial partitioning for many simple objects.
- Data-driven enemy waves.
- Data-driven weapons and upgrades.
- Runtime tuning for spawn rates, health, damage, cooldowns, and movement speed.
- Debug views for hitboxes, spawn regions, entity counts, and frame time.
- UI that can show constantly changing survival state.
- Deterministic-enough simulation hooks for replaying bugs during development.

## Target Platforms

Initial platform:

- Desktop Linux.

Near-term platforms:

- Windows.
- macOS.

Deferred platforms:

- Web.
- Mobile.
- Console.

Desktop-first keeps the first implementation practical. Web, mobile, and console support should wait until the runtime, renderer, asset model, and input model have stabilized.

## Rendering Direction

The initial renderer should use `wgpu`.

Reasons:

- It supports modern GPU APIs through one Rust-facing abstraction.
- It gives a path to Vulkan, Metal, DirectX, and WebGPU.
- It is widely used in Rust graphics projects.
- It avoids committing the engine to a single native graphics API too early.

The renderer should begin with a simple 2D sprite path and clear batching rules before introducing a full render graph. A render graph should be added only after multiple passes and dependencies make the simpler design painful.

## Engine Architecture

The engine should start as ECS-first because the first game target involves many simple runtime entities.

Early gameplay state should be represented with:

- Entities for object identity.
- Components for data.
- Systems for behavior.
- Resources for global engine state.

The early ECS design should handle common survivor-game entities directly: player, enemies, weapons, projectiles, pickups, damage events, health, timers, waves, and temporary effects.

The ECS should not leak into every subsystem blindly. Rendering, audio, assets, and physics should expose engine APIs that gameplay systems can use without owning those subsystems directly.

## Tooling Strategy

The first tools should be runtime debug tools, not a full editor.

Initial tooling:

- Logging.
- Frame timing.
- Debug overlay.
- Entity and component inspection.
- Input state inspection.
- Renderer debug primitives.
- Hitbox and pickup-radius overlays.
- Wave, spawn-rate, and active-entity counters.
- Weapon cooldown and damage inspection.
- Hot reload for shaders and selected assets.

Deferred tooling:

- Full scene editor.
- Visual scripting.
- Node-based material editor.
- Complex asset browser.

This keeps iteration fast without turning the editor into the first major product.

## Scripting Strategy

The first version should be Rust-only.

Scripting should be deferred until the engine has at least one playable demo and a clearer understanding of what needs to be scriptable. If scripting becomes necessary, evaluate Lua, Rhai, and WASM against actual demo requirements.

## Dependency Policy

Use focused crates for infrastructure and domain problems where building from scratch would slow down the engine without improving its design.

Are We Game Yet should be the first source for game-development package choices. Before adding a game-dev dependency, check the relevant ecosystem category on `https://arewegameyet.rs` and prefer crates listed there. If a needed dependency is not listed there, treat it as an explicit exception and document why it is still being added.

Initial acceptable dependencies:

- `winit` for windowing and events, selected from the Are We Game Yet windowing category.
- `wgpu` for graphics, selected from the Are We Game Yet rendering categories.
- `glam` for math.
- `tracing` for logs.
- `serde` for configuration and data files.
- `thiserror` for typed library errors.
- `anyhow` or `eyre` for application-level errors.
- `egui` for early debug UI, if needed.

General Rust infrastructure crates, such as logging, serialization, and error-handling crates, may come from the broader Rust ecosystem when Are We Game Yet does not have a matching category. These should still be small, focused, and justified by the subsystem using them.

Dependencies should be added when they remove meaningful risk or complexity. Avoid large framework dependencies until the engine's own boundaries are clearer.

## Repository Shape

Start with a small workspace:

- `engine` for reusable engine library code.
- `sandbox` for the first playable demo and experiments.
- `tools` for asset importers and command-line utilities when needed.

The first implementation should keep the engine and demo separate enough that demo-specific code does not become engine architecture by accident.

## Non-Goals

These are intentionally out of scope for the first playable slice:

- Multiplayer networking.
- Console support.
- Mobile support.
- Full editor.
- Custom physics engine.
- Custom scripting language.
- Advanced renderer features.
- Exact replication of Vampire Survivors assets, names, characters, sounds, maps, weapons, or proprietary tuning.
- Procedural map generation.
- Meta-progression across runs.
- Dozens of weapons and upgrade evolutions.
- Terrain tools.
- Plugin marketplace.
- Large asset pipeline.

Deferring these keeps the early engine focused on proving the core runtime.

## Phase 1 Definition of Done

The first implementation phase is complete when the engine can:

- Open a desktop window.
- Run a stable event loop.
- Separate event handling, update, and render responsibilities.
- Track frame timing.
- Log startup, shutdown, and frame-loop errors.
- Load a small runtime configuration file.
- Exit cleanly.

The first rendered frame can be a clear color. Rendering real geometry belongs to the next phase.

## First Week Plan

1. Create the Rust workspace.
2. Add the engine and sandbox crates.
3. Add `winit`, `tracing`, and basic error handling.
4. Implement the application lifecycle.
5. Open a window.
6. Run an empty update/render loop.
7. Print frame timing to logs.
8. Add a small runtime configuration file.
9. Add a placeholder survivor-game state with timer, player position, and restart flow.
10. Document how to run the sandbox.

## Review Triggers

Revisit this charter when one of these happens:

- The first playable demo needs a feature that contradicts an initial decision.
- A dependency starts owning too much of the engine design.
- Platform support changes from desktop-first.
- The engine needs editor workflows before the runtime is stable.
- The renderer needs multiple passes with explicit dependencies.
- The survivor-style demo cannot maintain stable frame time with hundreds of active entities.
