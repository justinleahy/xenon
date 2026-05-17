use super::{EnemyKind, GameCatalog, Scene};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneDefinition {
    pub player: PlayerSceneDefinition,
    #[serde(default)]
    pub enemies: Vec<EnemySpawnDefinition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerSceneDefinition {
    pub position: [f32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EnemySpawnDefinition {
    pub kind: EnemyKind,
    pub position: [f32; 2],
}

impl SceneDefinition {
    pub fn survivor_demo() -> Self {
        Self {
            player: PlayerSceneDefinition {
                position: [0.0, 0.0],
            },
            enemies: vec![EnemySpawnDefinition {
                kind: EnemyKind::Basic,
                position: [5.0, 5.0],
            }],
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let definition = toml::from_str(&contents)?;

        Ok(definition)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self)?;

        std::fs::write(path, contents)?;

        Ok(())
    }

    pub fn build_scene(&self, catalog: &GameCatalog) -> Scene {
        let mut scene = Scene::empty();

        if let Some(transform) = scene.transform_mut(scene.player) {
            transform.position = self.player.position;
        }

        for enemy in &self.enemies {
            scene.spawn_enemy(enemy.position, enemy.kind, catalog);
        }

        scene
    }

    pub fn from_scene(scene: &Scene) -> Self {
        let player_position = scene
            .transform(scene.player)
            .map(|transform| transform.position)
            .unwrap_or([0.0, 0.0]);

        let enemies = scene
            .enemies
            .iter()
            .filter_map(|entity| {
                let enemy = scene.enemy(*entity)?;
                let transform = scene.transform(*entity)?;

                Some(EnemySpawnDefinition {
                    kind: enemy.kind,
                    position: transform.position,
                })
            })
            .collect();

        Self {
            player: PlayerSceneDefinition {
                position: player_position,
            },
            enemies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_scene_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("xenon-scene-{name}-{nanos}.toml"))
    }

    fn demo_scene_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/demo_scene.toml")
    }

    #[test]
    fn test_load_from_file_loads_scene_definition() {
        let path = temp_scene_path("load");
        fs::write(
            &path,
            r#"
[player]
position = [1.0, 2.0]

[[enemies]]
kind = "basic"
position = [3.0, 4.0]
            "#,
        )
        .unwrap();

        let definition = SceneDefinition::load_from_file(&path).unwrap();

        assert_eq!(definition.player.position, [1.0, 2.0]);
        assert_eq!(
            definition.enemies,
            vec![EnemySpawnDefinition {
                kind: EnemyKind::Basic,
                position: [3.0, 4.0],
            }]
        );
    }

    #[test]
    fn test_save_to_file_writes_loadable_scene_definition() {
        let path = temp_scene_path("save");
        let definition = SceneDefinition::survivor_demo();

        definition.save_to_file(&path).unwrap();

        let loaded = SceneDefinition::load_from_file(&path).unwrap();

        assert_eq!(loaded, definition);
    }

    #[test]
    fn test_build_scene_applies_player_position_and_enemy_spawns() {
        let catalog = GameCatalog::default();
        let definition = SceneDefinition {
            player: PlayerSceneDefinition {
                position: [2.0, -1.0],
            },
            enemies: vec![EnemySpawnDefinition {
                kind: EnemyKind::Basic,
                position: [4.0, 5.0],
            }],
        };

        let scene = definition.build_scene(&catalog);

        assert_eq!(scene.transform(scene.player).unwrap().position, [2.0, -1.0]);
        assert_eq!(scene.enemies.len(), 1);
        assert_eq!(
            scene
                .transform(*scene.enemies.first().unwrap())
                .unwrap()
                .position,
            [4.0, 5.0]
        );
        assert_eq!(
            scene.enemy(*scene.enemies.first().unwrap()).unwrap().kind,
            EnemyKind::Basic
        );
    }

    #[test]
    fn test_load_demo_scene_file_builds_expected_scene() {
        let catalog = GameCatalog::default();
        let definition = SceneDefinition::load_from_file(demo_scene_path()).unwrap();

        let scene = definition.build_scene(&catalog);

        assert_eq!(scene.transform(scene.player).unwrap().position, [0.0, 0.0]);
        assert_eq!(scene.enemies.len(), 1);
        assert_eq!(
            scene.enemy(*scene.enemies.first().unwrap()).unwrap().kind,
            EnemyKind::Basic
        );
        assert_eq!(
            scene
                .transform(*scene.enemies.first().unwrap())
                .unwrap()
                .position,
            [5.0, 5.0]
        );
    }

    #[test]
    fn test_from_scene_captures_player_and_enemy_spawns() {
        let catalog = GameCatalog::default();
        let mut scene = Scene::empty();

        scene.transform_mut(scene.player).unwrap().position = [2.0, -1.0];
        scene.spawn_enemy([4.0, 5.0], EnemyKind::Basic, &catalog);

        let definition = SceneDefinition::from_scene(&scene);

        assert_eq!(
            definition,
            SceneDefinition {
                player: PlayerSceneDefinition {
                    position: [2.0, -1.0],
                },
                enemies: vec![EnemySpawnDefinition {
                    kind: EnemyKind::Basic,
                    position: [4.0, 5.0],
                }],
            }
        );
    }
}
