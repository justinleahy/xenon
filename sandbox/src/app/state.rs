use super::game::{
    CombatState, EnemyState, GameCatalog, PlayerController, PlayerProgression, Scene,
    SceneDefinition,
};
use super::input::InputState;
use std::path::Path;
use tracing::info;

pub struct SandboxState {
    pub simulation_time_secs: f64,
    pub fixed_updates: u64,
    pub scene: Scene,
    pub initial_scene_definition: SceneDefinition,
    pub combat_state: CombatState,
    pub enemy_state: EnemyState,
    pub player_controller: PlayerController,
    pub player_progression: PlayerProgression,
    pub game_catalog: GameCatalog,
}

impl Default for SandboxState {
    fn default() -> Self {
        let game_catalog = GameCatalog::default();
        let scene_definition = SceneDefinition::survivor_demo();

        Self::new(scene_definition, game_catalog)
    }
}

impl SandboxState {
    pub fn new(scene_definition: SceneDefinition, game_catalog: GameCatalog) -> Self {
        Self {
            simulation_time_secs: 0.0,
            fixed_updates: 0,
            scene: scene_definition.build_scene(&game_catalog),
            initial_scene_definition: scene_definition,
            combat_state: CombatState::default(),
            enemy_state: EnemyState::default(),
            player_controller: PlayerController::default(),
            player_progression: PlayerProgression::default(),
            game_catalog,
        }
    }

    pub fn load_from_scene_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let game_catalog = GameCatalog::default();
        let scene_definition = SceneDefinition::load_from_file(path)?;

        Ok(Self::new(scene_definition, game_catalog))
    }

    pub fn reset(&mut self) {
        let scene_definition = self.initial_scene_definition.clone();
        let game_catalog = self.game_catalog;

        *self = Self::new(scene_definition, game_catalog);
    }

    pub fn fixed_update(&mut self, input: &mut InputState, delta_secs: f32) {
        if input.reset_requested {
            self.reset();
            input.reset_requested = false;

            info!("simulation reset");
            return;
        }

        let movement = movement_from_input(input);

        self.player_controller
            .fixed_update(&mut self.scene, movement, delta_secs);

        let player_position = self.player_position();

        self.enemy_state.fixed_update(
            &mut self.scene,
            &self.game_catalog,
            player_position,
            self.fixed_updates,
            delta_secs,
        );

        self.combat_state.fixed_update(
            &mut self.scene,
            &self.game_catalog,
            player_position,
            delta_secs,
        );

        self.player_progression
            .fixed_update(&mut self.scene, player_position);

        self.fixed_updates += 1;
        self.simulation_time_secs += delta_secs as f64;

        if self.fixed_updates % 60 == 0 {
            info!(
                fixed_updates = self.fixed_updates,
                simulation_time_secs = self.simulation_time_secs,
                player_x = self.player_position()[0],
                player_y = self.player_position()[1],
                "simulation state"
            );

            info!(
                enemies = self.scene.enemies.len(),
                projectiles = self.scene.projectiles.len(),
                health = self.player_health(),
                experience = self.player_progression.experience,
                "sandbox state"
            )
        }
    }

    pub fn player_position(&self) -> [f32; 2] {
        self.scene
            .transform(self.scene.player)
            .map(|transform| transform.position)
            .unwrap_or([0.0, 0.0])
    }

    pub fn player_health(&self) -> f32 {
        self.scene
            .health(self.scene.player)
            .map(|health| health.current)
            .unwrap_or(0.0)
    }
}

fn movement_from_input(input: &InputState) -> [f32; 2] {
    let mut direction = [0.0_f32, 0.0_f32];

    if input.move_up {
        direction[1] -= 1.0;
    }

    if input.move_down {
        direction[1] += 1.0;
    }

    if input.move_left {
        direction[0] -= 1.0;
    }

    if input.move_right {
        direction[0] += 1.0;
    }

    let length = (direction[0] * direction[0] + direction[1] * direction[1]).sqrt();

    if length > 0.0 {
        direction[0] /= length;
        direction[1] /= length;
    }

    direction
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::game::{EnemyKind, EnemySpawnDefinition, PlayerSceneDefinition};

    #[test]
    fn test_reset_restores_initial_scene_definition() {
        let catalog = GameCatalog::default();
        let scene_definition = SceneDefinition {
            player: PlayerSceneDefinition {
                position: [3.0, 4.0],
            },
            enemies: vec![EnemySpawnDefinition {
                kind: EnemyKind::Basic,
                position: [6.0, 7.0],
            }],
        };
        let mut state = SandboxState::new(scene_definition, catalog);

        state
            .scene
            .transform_mut(state.scene.player)
            .unwrap()
            .position = [99.0, 99.0];
        state.scene.enemies.clear();

        state.reset();

        assert_eq!(state.player_position(), [3.0, 4.0]);
        assert_eq!(state.scene.enemies.len(), 1);
        assert_eq!(
            state
                .scene
                .transform(*state.scene.enemies.first().unwrap())
                .unwrap()
                .position,
            [6.0, 7.0]
        );
    }
}
