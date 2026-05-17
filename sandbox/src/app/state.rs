use super::game::{CombatState, Scene};
use super::input::InputState;
use tracing::info;

pub struct SandboxState {
    pub simulation_time_secs: f64,
    pub fixed_updates: u64,
    pub enemy_spawn_cooldown_secs: f32,
    pub scene: Scene,
    pub combat_state: CombatState,
}

impl Default for SandboxState {
    fn default() -> Self {
        Self {
            simulation_time_secs: 0.0,
            fixed_updates: 0,
            enemy_spawn_cooldown_secs: 1.5,
            scene: Scene::new_survivor_demo(),
            combat_state: CombatState::default(),
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

        let enemy_speed = 1.5;
        let enemy_damage_per_second = 10.0;
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

        self.update_enemy_spawning(delta_secs);

        let player_position = self.player_position();
        let enemy_entities = self.scene.enemies.clone();

        for enemy in enemy_entities {
            let Some(enemy_position) = self.scene.transform(enemy).map(|t| t.position) else {
                continue;
            };

            let to_player = [
                player_position[0] - enemy_position[0],
                player_position[1] - enemy_position[1],
            ];

            let distance = (to_player[0] * to_player[0] + to_player[1] * to_player[1]).sqrt();

            if distance > 0.001 {
                let direction = [to_player[0] / distance, to_player[1] / distance];

                if let Some(transform) = self.scene.transform_mut(enemy) {
                    transform.position[0] += direction[0] * enemy_speed * delta_secs;
                    transform.position[1] += direction[1] * enemy_speed * delta_secs;
                }
            }

            if distance < 0.25 {
                if let Some(health) = self.scene.health_mut(self.scene.player) {
                    let previous_health = health.current;
                    health.current =
                        (health.current - enemy_damage_per_second * delta_secs).max(0.0);

                    if health.current == 0.0 && previous_health > 0.0 {
                        info!("Player health depleted");
                    }
                }
            }
        }

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

    fn update_enemy_spawning(&mut self, delta_secs: f32) {
        const MAX_ENEMIES: usize = 50;

        if self.scene.enemies.len() >= MAX_ENEMIES {
            return;
        }

        self.enemy_spawn_cooldown_secs -= delta_secs;

        if self.enemy_spawn_cooldown_secs > 0.0 {
            return;
        }

        let spawn_radius = 12.0;
        let spawn_index = self.fixed_updates as f32;

        let angle = spawn_index * 2.3999631; // Golden angle, spreads spawn around ring

        let spawn_position = [
            self.player_position()[0] + spawn_radius * angle.cos(),
            self.player_position()[1] + spawn_radius * angle.sin(),
        ];

        self.scene.spawn_enemy(spawn_position);

        self.enemy_spawn_cooldown_secs = 1.5;
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
