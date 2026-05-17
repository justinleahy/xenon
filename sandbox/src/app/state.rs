use super::input::InputState;
use tracing::info;

pub struct Enemy {
    pub position: [f32; 2],
}

pub struct Projectile {
    pub position: [f32; 2],
    pub previous_position: [f32; 2],
    pub velocity: [f32; 2],
    pub lifetime_secs: f32,
}

pub struct SandboxState {
    pub simulation_time_secs: f64,
    pub fixed_updates: u64,
    pub player_position: [f32; 2],
    pub enemies: Vec<Enemy>,
    pub player_health: f32,
    pub projectiles: Vec<Projectile>,
    pub weapon_cooldown_secs: f32,
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
            projectiles: Vec::new(),
            weapon_cooldown_secs: 0.0,
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

        self.update_weapon(delta_secs);
        self.update_projectiles(delta_secs);
        self.resolve_projectile_hits();

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

    fn update_weapon(&mut self, delta_secs: f32) {
        self.weapon_cooldown_secs = (self.weapon_cooldown_secs - delta_secs).max(0.0);

        if self.weapon_cooldown_secs > 0.0 {
            return;
        }

        let Some(target_position) = self.nearest_enemy_position() else {
            return;
        };

        let to_target = [
            target_position[0] - self.player_position[0],
            target_position[1] - self.player_position[1],
        ];

        let distance = (to_target[0] * to_target[0] + to_target[1] * to_target[1]).sqrt();

        if distance <= 0.001 {
            return;
        }

        let projectile_speed = 8.0;
        let direction = [to_target[0] / distance, to_target[1] / distance];

        self.projectiles.push(Projectile {
            position: self.player_position,
            previous_position: self.player_position,
            velocity: [
                direction[0] * projectile_speed,
                direction[1] * projectile_speed,
            ],
            lifetime_secs: 2.0,
        });

        self.weapon_cooldown_secs = 0.5;
    }

    fn update_projectiles(&mut self, delta_secs: f32) {
        for projectile in &mut self.projectiles {
            projectile.previous_position = projectile.position;
            projectile.position[0] += projectile.velocity[0] * delta_secs;
            projectile.position[1] += projectile.velocity[1] * delta_secs;
            projectile.lifetime_secs -= delta_secs;
        }

        self.projectiles.retain(|p| p.lifetime_secs > 0.0);
    }

    fn resolve_projectile_hits(&mut self) {
        let hit_radius = 0.30;

        let mut hit_enemy_indices = Vec::new();
        let mut hit_projectile_indices = Vec::new();

        for (projectile_index, projectile) in self.projectiles.iter().enumerate() {
            for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                let distance = distance_point_to_segment(
                    enemy.position,
                    projectile.previous_position,
                    projectile.position,
                );

                if distance < hit_radius {
                    hit_enemy_indices.push(enemy_index);
                    hit_projectile_indices.push(projectile_index);
                    break;
                }
            }
        }

        hit_enemy_indices.sort_unstable();
        hit_enemy_indices.dedup();

        hit_projectile_indices.sort_unstable();
        hit_projectile_indices.dedup();

        let mut enemy_index = 0;
        self.enemies.retain(|_| {
            let keep = hit_enemy_indices.binary_search(&enemy_index).is_err();
            enemy_index += 1;
            keep
        });

        let mut projectile_index = 0;
        self.projectiles.retain(|_| {
            let keep = hit_projectile_indices
                .binary_search(&projectile_index)
                .is_err();
            projectile_index += 1;
            keep
        });
    }

    fn nearest_enemy_position(&self) -> Option<[f32; 2]> {
        self.enemies
            .iter()
            .min_by(|a, b| {
                let a_distance = squared_distance(self.player_position, a.position);
                let b_distance = squared_distance(self.player_position, b.position);

                a_distance
                    .partial_cmp(&b_distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|enemy| enemy.position)
    }
}

fn squared_distance(a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];

    dx * dx + dy * dy
}

fn distance_point_to_segment(point: [f32; 2], start: [f32; 2], end: [f32; 2]) -> f32 {
    let segment = [end[0] - start[0], end[1] - start[1]];
    let point_to_start = [point[0] - start[0], point[1] - start[1]];

    let segment_length_squared = segment[0] * segment[0] + segment[1] * segment[1];

    if segment_length_squared <= f32::EPSILON {
        return squared_distance(point, start).sqrt();
    }

    let t = ((point_to_start[0] * segment[0] + point_to_start[1] * segment[1])
        / segment_length_squared)
        .clamp(0.0, 1.0);

    let closest = [start[0] + segment[0] * t, start[1] + segment[1] * t];

    squared_distance(point, closest).sqrt()
}
