# Changelog

All notable changes to Xenon are documented here.

This project follows semantic versioning for published crates.

## [xenon-engine 0.1.0] - 2026-05-18

Initial crates.io release of `xenon-engine`.

### Added

- Runtime configuration loading with validation.
- Window lifecycle event types.
- Frame timing, FPS sampling, and fixed-timestep utilities.
- `wgpu` renderer with clear-pass and instanced 2D sprite rendering.
- Render scene primitives for cameras and sprites.
- Reusable scene primitives: `EntityId`, `SceneObjectId`, `Transform`, and `Sprite`.
- Structured runtime and render error types.

### Notes

- The `sandbox` crate remains unpublished and acts as the reference prototype for validating engine behavior.
- Gameplay-specific systems, content definitions, combat, enemies, pickups, and progression remain in `sandbox`.
