# Contributing to Xenon

Xenon is a desktop-first Rust game engine project. Contributions should keep the engine moving toward a playable 2D survival demo while protecting the reusable engine boundary.

This repository is AI assisted but not AI generated. Main production code should be authored and reviewed by humans. AI assistance is appropriate for tests, documentation, review support, and development guidance.

## Project Rules

- Build from the current demo's needs before adding general engine abstractions.
- Keep reusable runtime, rendering, asset, and scene primitives in the engine crate.
- Keep survival-game rules, content, tuning, and prototype experiments in the sandbox crate until the abstraction is proven.
- Promote sandbox code into the engine only when it solves a reusable engine problem, not because it looks tidy.
- Validate engine direction with a running demo, not only with unit tests.
- Avoid unrelated refactors in feature changes. Make architectural changes explicit and reviewable.

## Rust Coding Standards

- Use Rust 2024 edition idioms and standard `rustfmt` formatting.
- Prefer small modules with clear ownership over broad framework layers.
- Keep public APIs narrow. Export types from the crate root only when they are intended for engine users.
- Use typed errors for reusable engine code. Application-level orchestration may use broader error handling when it improves clarity.
- Do not panic for recoverable runtime, asset, render, or configuration failures. Panics are acceptable in tests when they make the assertion clearer.
- Prefer explicit data flow over global mutable state.
- Keep fixed-step gameplay logic deterministic enough to reproduce bugs from the same inputs and configuration.
- Treat render, collision, spawn, and asset paths as performance-sensitive. Avoid avoidable per-frame allocation in hot loops.
- Use stable IDs for serialized scene or asset references instead of relying on load order.
- Keep comments focused on invariants, ownership, and non-obvious tradeoffs. Do not narrate simple code.

## Dependency Rules

- Add dependencies only when they remove meaningful implementation risk or complexity.
- For game-development crates, check Are We Game Yet first and prefer crates listed there.
- Prefer small, focused crates over broad frameworks while the engine boundaries are still forming.
- Document exceptions when a game-development dependency is not listed in the expected ecosystem category.
- Keep application-only dependencies out of the reusable engine crate unless the engine API truly needs them.

## Testing Rules

Before treating a change as ready, run:

```bash
cargo fmt --check
cargo test
```

Also run the sandbox when a change affects runtime behavior, rendering, input, assets, scene loading, or gameplay:

```bash
cargo run -p sandbox
```

Tests should cover the behavior being changed. Use focused unit tests for pure runtime, scene, asset, and gameplay rules. Use manual sandbox verification for windowing, rendering, and player-facing behavior that unit tests cannot prove.

## Contribution Workflow

1. Start with a small change that has one clear purpose.
2. Decide whether the change belongs in the engine crate or the sandbox crate before coding.
3. Add or update tests with the behavior change.
4. Update documentation when commands, project direction, public APIs, or contributor expectations change.
5. Update the changelog for published engine behavior.
6. Run the verification commands and include the results when asking for review.

Review should focus on correctness, architecture boundaries, test coverage, and whether the change keeps the playable demo moving forward.
