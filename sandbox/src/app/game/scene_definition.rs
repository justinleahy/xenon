use super::{EnemyKind, GameCatalog, Scene, SceneObjectId, WeaponKind};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneDefinition {
    pub player: PlayerSceneDefinition,
    #[serde(default)]
    pub enemies: Vec<EnemySpawnDefinition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSceneDefinition {
    pub position: [f32; 2],
    #[serde(default = "default_player_weapons")]
    pub weapons: Vec<WeaponKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnemySpawnDefinition {
    pub id: SceneObjectId,
    pub kind: EnemyKind,
    pub position: [f32; 2],
}

impl SceneDefinition {
    pub fn survivor_demo() -> Self {
        Self {
            player: PlayerSceneDefinition {
                position: [0.0, 0.0],
                weapons: vec![WeaponKind::Pistol, WeaponKind::Smg],
            },
            enemies: vec![
                EnemySpawnDefinition {
                    id: SceneObjectId::new("enemy.starting_basic"),
                    kind: EnemyKind::Basic,
                    position: [5.0, 5.0],
                },
                EnemySpawnDefinition {
                    id: SceneObjectId::new("enemy.starting_fast"),
                    kind: EnemyKind::Fast,
                    position: [-6.0, 4.0],
                },
                EnemySpawnDefinition {
                    id: SceneObjectId::new("enemy.starting_tank"),
                    kind: EnemyKind::Tank,
                    position: [8.0, -5.0],
                },
            ],
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
            let entity = scene.spawn_enemy(enemy.position, enemy.kind, catalog);
            scene.set_scene_object_id(entity, enemy.id.clone());
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
                    id: scene
                        .scene_object_id(*entity)
                        .cloned()
                        .unwrap_or_else(|| SceneObjectId::new(format!("enemy.{}", entity.0))),
                    kind: enemy.kind,
                    position: transform.position,
                })
            })
            .collect();

        Self {
            player: PlayerSceneDefinition {
                position: player_position,
                weapons: default_player_weapons(),
            },
            enemies,
        }
    }
}

fn default_player_weapons() -> Vec<WeaponKind> {
    vec![WeaponKind::Pistol]
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

    fn assert_enemy_spawn(
        scene: &Scene,
        index: usize,
        id: &str,
        kind: EnemyKind,
        position: [f32; 2],
    ) {
        let enemy = scene.enemies[index];

        assert_eq!(scene.enemy(enemy).unwrap().kind, kind);
        assert_eq!(scene.scene_object_id(enemy), Some(&SceneObjectId::new(id)));
        assert_eq!(scene.transform(enemy).unwrap().position, position);
    }

    fn default_weapons() -> Vec<WeaponKind> {
        vec![WeaponKind::Pistol]
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
id = "enemy.test_basic"
kind = "basic"
position = [3.0, 4.0]
            "#,
        )
        .unwrap();

        let definition = SceneDefinition::load_from_file(&path).unwrap();

        assert_eq!(definition.player.position, [1.0, 2.0]);
        assert_eq!(definition.player.weapons, default_weapons());
        assert_eq!(
            definition.enemies,
            vec![EnemySpawnDefinition {
                id: SceneObjectId::new("enemy.test_basic"),
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
                weapons: vec![WeaponKind::Pistol, WeaponKind::Smg],
            },
            enemies: vec![EnemySpawnDefinition {
                id: SceneObjectId::new("enemy.test_basic"),
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
        assert_eq!(
            scene.scene_object_id(*scene.enemies.first().unwrap()),
            Some(&SceneObjectId::new("enemy.test_basic"))
        );
    }

    #[test]
    fn test_load_demo_scene_file_builds_expected_scene() {
        let catalog = GameCatalog::default();
        let definition = SceneDefinition::load_from_file(demo_scene_path()).unwrap();

        let scene = definition.build_scene(&catalog);

        assert_eq!(
            definition.player.weapons,
            vec![WeaponKind::Pistol, WeaponKind::Smg]
        );
        assert_eq!(scene.transform(scene.player).unwrap().position, [0.0, 0.0]);
        assert_eq!(scene.enemies.len(), 3);
        assert_enemy_spawn(
            &scene,
            0,
            "enemy.starting_basic",
            EnemyKind::Basic,
            [5.0, 5.0],
        );
        assert_enemy_spawn(
            &scene,
            1,
            "enemy.starting_fast",
            EnemyKind::Fast,
            [-6.0, 4.0],
        );
        assert_enemy_spawn(
            &scene,
            2,
            "enemy.starting_tank",
            EnemyKind::Tank,
            [8.0, -5.0],
        );
    }

    #[test]
    fn test_from_scene_captures_player_and_enemy_spawns() {
        let catalog = GameCatalog::default();
        let mut scene = Scene::empty();

        scene.transform_mut(scene.player).unwrap().position = [2.0, -1.0];
        let enemy = scene.spawn_enemy([4.0, 5.0], EnemyKind::Basic, &catalog);
        scene.set_scene_object_id(enemy, SceneObjectId::new("enemy.saved_basic"));

        let definition = SceneDefinition::from_scene(&scene);

        assert_eq!(
            definition,
            SceneDefinition {
                player: PlayerSceneDefinition {
                    position: [2.0, -1.0],
                    weapons: default_weapons(),
                },
                enemies: vec![EnemySpawnDefinition {
                    id: SceneObjectId::new("enemy.saved_basic"),
                    kind: EnemyKind::Basic,
                    position: [4.0, 5.0],
                }],
            }
        );
    }
}
