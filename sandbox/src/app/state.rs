use super::input::InputState;
use tracing::info;

pub struct Enemy {
    pub position: [f32; 2],
}

pub struct SandboxState {
    pub simulation_time_secs: f64,
    pub fixed_updates: u64,
    pub player_position: [f32; 2],
    pub enemies: Vec<Enemy>,
    pub player_health: f32,
}

impl Default for SandboxState {
    fn default() -> Self {
        Self {
            simulation_time_secs: 0.0,
            fixed_updates: 0,
            player_position: [0.0, 0.0],
            enemies: vec![Enemy {
                position: [5.0, 5.0],
            }],
            player_health: 100.0,
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

        self.player_position[0] += direction[0] * player_speed * delta_secs;
        self.player_position[1] += direction[1] * player_speed * delta_secs;

        for enemy in &mut self.enemies {
            let to_player = [
                self.player_position[0] - enemy.position[0],
                self.player_position[1] - enemy.position[1],
            ];

            let distance = (to_player[0] * to_player[0] + to_player[1] * to_player[1]).sqrt();

            if distance > 0.001 {
                let direction = [to_player[0] / distance, to_player[1] / distance];

                enemy.position[0] += direction[0] * enemy_speed * delta_secs;
                enemy.position[1] += direction[1] * enemy_speed * delta_secs;
            }

            if distance < 0.25 {
                let previous_health = self.player_health;
                self.player_health =
                    (self.player_health - enemy_damage_per_second * delta_secs).max(0.0);
                if self.player_health == 0.0 && previous_health > 0.0 {
                    info!("Player health depleted");
                }
            }
        }

        self.fixed_updates += 1;
        self.simulation_time_secs += delta_secs as f64;

        if self.fixed_updates % 60 == 0 {
            info!(
                fixed_updates = self.fixed_updates,
                simulation_time_secs = self.simulation_time_secs,
                player_x = self.player_position[0],
                player_y = self.player_position[1],
                "simulation state"
            );

            info!(player_health = self.player_health, "player health")
        }
    }
}
