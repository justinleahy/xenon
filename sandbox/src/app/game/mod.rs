pub mod catalog;
pub mod combat;
pub mod components;
pub mod enemies;
pub mod entity;
pub mod player;
pub mod scene;
pub mod scene_definition;
pub mod systems;

#[cfg(test)]
pub mod test_helpers;

pub use catalog::{EnemyKind, GameCatalog, WeaponKind};
pub use combat::CombatState;
pub use enemies::EnemyState;
pub use player::{PlayerController, PlayerProgression};
pub use scene::Scene;
pub use scene_definition::{EnemySpawnDefinition, PlayerSceneDefinition, SceneDefinition};
pub use systems::GameSystems;
