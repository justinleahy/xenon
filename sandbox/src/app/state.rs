use super::game::{CombatState, EnemyState, Scene};
use super::input::InputState;
use tracing::info;

pub struct SandboxState {
    pub simulation_time_secs: f64,
    pub fixed_updates: u64,
    pub scene: Scene,
    pub combat_state: CombatState,
    pub enemy_state: EnemyState,
}

impl Default for SandboxState {
    fn default() -> Self {
        Self {
            simulation_time_secs: 0.0,
            fixed_updates: 0,
            scene: Scene::new_survivor_demo(),
            combat_state: CombatState::default(),
            enemy_state: EnemyState::default(),
        }
    }
}

impl SandboxState {
    pub fn fixed_update(&mut self, input: &mut InputState, delta_secs: f32) {
        if input.reset_requested {
            *self = Self::default();
            input.reset_requested = false;

            info!("simulation reset");
            return;
        }

        let player_speed = 5.0;

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

        if let Some(transform) = self.scene.transform_mut(self.scene.player) {
            transform.position[0] += direction[0] * player_speed * delta_secs;
            transform.position[1] += direction[1] * player_speed * delta_secs;
        }

        let player_position = self.player_position();

        self.enemy_state.fixed_update(
            &mut self.scene,
            player_position,
            self.fixed_updates,
            delta_secs,
        );

        self.combat_state
            .fixed_update(&mut self.scene, player_position, delta_secs);

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
                projectiles = self.combat_state.projectiles.len(),
                health = self.player_health(),
                "scene state"
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
