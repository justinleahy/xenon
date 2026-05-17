use super::Scene;
use tracing::info;

pub struct EnemyState {
    pub spawn_cooldown_secs: f32,
}

impl Default for EnemyState {
    fn default() -> Self {
        Self {
            spawn_cooldown_secs: 1.5,
        }
    }
}

impl EnemyState {
    pub fn fixed_update(
        &mut self,
        scene: &mut Scene,
        player_position: [f32; 2],
        fixed_updates: u64,
        delta_secs: f32,
    ) {
        self.update_spawning(scene, player_position, fixed_updates, delta_secs);
        self.update_movement_and_contact_damage(scene, player_position, delta_secs);
    }

    fn update_spawning(
        &mut self,
        scene: &mut Scene,
        player_position: [f32; 2],
        fixed_updates: u64,
        delta_secs: f32,
    ) {
        const MAX_ENEMIES: usize = 50;

        if scene.enemies.len() >= MAX_ENEMIES {
            return;
        }

        self.spawn_cooldown_secs -= delta_secs;

        if self.spawn_cooldown_secs > 0.0 {
            return;
        }

        let spawn_radius = 12.0;

        let spawn_index = fixed_updates as f32;
        let angle = spawn_index * 2.3999631;

        let spawn_position = [
            player_position[0] + spawn_radius * angle.cos(),
            player_position[1] + spawn_radius * angle.sin(),
        ];

        scene.spawn_enemy(spawn_position);

        self.spawn_cooldown_secs = 1.5;
    }

    pub fn update_movement_and_contact_damage(
        &mut self,
        scene: &mut Scene,
        player_position: [f32; 2],
        delta_secs: f32,
    ) {
        let enemy_speed = 1.5;
        let enemy_damage_per_second = 10.0;
        let enemy_entities = scene.enemies.clone();

        for enemy in enemy_entities {
            let Some(enemy_position) = scene.transform(enemy).map(|t| t.position) else {
                continue;
            };

            let to_player = [
                player_position[0] - enemy_position[0],
                player_position[1] - enemy_position[1],
            ];

            let distance = (to_player[0] * to_player[0] + to_player[1] * to_player[1]).sqrt();

            if distance > 0.001 {
                let direction = [to_player[0] / distance, to_player[1] / distance];

                if let Some(transform) = scene.transform_mut(enemy) {
                    transform.position[0] += direction[0] * enemy_speed * delta_secs;
                    transform.position[1] += direction[1] * enemy_speed * delta_secs;
                }
            }

            if distance < 0.25 {
                if let Some(health) = scene.health_mut(scene.player) {
                    let previous_health = health.current;
                    health.current =
                        (health.current - enemy_damage_per_second * delta_secs).max(0.0);

                    if health.current == 0.0 && previous_health > 0.0 {
                        info!("Player health depleted");
                    }
                }
            }
        }
    }
}
