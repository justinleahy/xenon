use super::{EnemyKind, GameCatalog, Scene};
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
        catalog: &GameCatalog,
        player_position: [f32; 2],
        fixed_updates: u64,
        delta_secs: f32,
    ) {
        self.update_spawning(scene, catalog, player_position, fixed_updates, delta_secs);
        self.update_movement_and_contact_damage(scene, catalog, player_position, delta_secs);
    }

    fn update_spawning(
        &mut self,
        scene: &mut Scene,
        catalog: &GameCatalog,
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

        scene.spawn_enemy(spawn_position, enemy_kind_for_spawn(fixed_updates), catalog);

        self.spawn_cooldown_secs = 1.5;
    }

    pub fn update_movement_and_contact_damage(
        &mut self,
        scene: &mut Scene,
        catalog: &GameCatalog,
        player_position: [f32; 2],
        delta_secs: f32,
    ) {
        let enemy_entities = scene.enemies.clone();

        for enemy in enemy_entities {
            let Some(enemy_component) = scene.enemy(enemy) else {
                continue;
            };

            let definition = catalog.enemy(enemy_component.kind);
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
                    transform.position[0] += direction[0] * definition.speed * delta_secs;
                    transform.position[1] += direction[1] * definition.speed * delta_secs;
                }
            }

            let Some(enemy_collider) = scene.circle_collider(enemy) else {
                continue;
            };

            let Some(player_collider) = scene.circle_collider(scene.player) else {
                continue;
            };

            let contact_distance = enemy_collider.radius + player_collider.radius;

            if distance < contact_distance {
                if let Some(health) = scene.health_mut(scene.player) {
                    let previous_health = health.current;
                    health.current = (health.current
                        - definition.contact_damage_per_second * delta_secs)
                        .max(0.0);

                    if health.current == 0.0 && previous_health > 0.0 {
                        info!("Player health depleted");
                    }
                }
            }
        }
    }
}

fn enemy_kind_for_spawn(fixed_updates: u64) -> EnemyKind {
    const FAST_ENEMY_START_TICK: u64 = 60 * 15;
    const TANK_ENEMY_START_TICK: u64 = 60 * 30;

    if fixed_updates >= TANK_ENEMY_START_TICK && fixed_updates % 4 == 0 {
        return EnemyKind::Tank;
    }

    if fixed_updates >= FAST_ENEMY_START_TICK && fixed_updates % 2 == 0 {
        return EnemyKind::Fast;
    }

    EnemyKind::Basic
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::scene_with_only_player;
    use super::*;

    #[test]
    fn test_spawning_adds_enemy_after_cooldown() {
        let mut scene = scene_with_only_player();
        let initial_enemy_count = scene.enemies.len();
        let catalog = GameCatalog::default();

        let mut enemies = EnemyState {
            spawn_cooldown_secs: 0.0,
        };

        enemies.fixed_update(&mut scene, &catalog, [0.0, 0.0], 1, 0.016);

        assert_eq!(scene.enemies.len(), initial_enemy_count + 1);
        assert_eq!(enemies.spawn_cooldown_secs, 1.5);
    }

    #[test]
    fn test_spawning_uses_basic_enemy_before_fast_threshold() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        let mut enemies = EnemyState {
            spawn_cooldown_secs: 0.0,
        };

        enemies.fixed_update(&mut scene, &catalog, [0.0, 0.0], 60 * 15 - 1, 0.016);

        let spawned_enemy = *scene.enemies.last().unwrap();

        assert_eq!(scene.enemy(spawned_enemy).unwrap().kind, EnemyKind::Basic);
    }

    #[test]
    fn test_spawning_uses_fast_enemy_after_fast_threshold() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        let mut enemies = EnemyState {
            spawn_cooldown_secs: 0.0,
        };

        enemies.fixed_update(&mut scene, &catalog, [0.0, 0.0], 60 * 15, 0.016);

        let spawned_enemy = *scene.enemies.last().unwrap();

        assert_eq!(scene.enemy(spawned_enemy).unwrap().kind, EnemyKind::Fast);
    }

    #[test]
    fn test_spawning_uses_tank_enemy_after_tank_threshold() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        let mut enemies = EnemyState {
            spawn_cooldown_secs: 0.0,
        };

        enemies.fixed_update(&mut scene, &catalog, [0.0, 0.0], 60 * 30, 0.016);

        let spawned_enemy = *scene.enemies.last().unwrap();

        assert_eq!(scene.enemy(spawned_enemy).unwrap().kind, EnemyKind::Tank);
    }

    #[test]
    fn test_enemy_moves_toward_player() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        let enemy = scene.spawn_enemy([10.0, 0.0], EnemyKind::Basic, &catalog);

        let mut enemies = EnemyState::default();

        enemies.update_movement_and_contact_damage(&mut scene, &catalog, [0.0, 0.0], 1.0);

        let enemy_position = scene.transform(enemy).unwrap().position;

        assert!(enemy_position[0] < 10.0);
        assert_eq!(enemy_position[1], 0.0);
    }

    #[test]
    fn test_enemy_contact_damage_uses_circle_colliders() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        scene.spawn_enemy([0.69, 0.0], EnemyKind::Basic, &catalog);

        let mut enemies = EnemyState::default();

        enemies.update_movement_and_contact_damage(&mut scene, &catalog, [0.0, 0.0], 1.0);

        assert_eq!(scene.health(scene.player).unwrap().current, 90.0);
    }

    #[test]
    fn test_enemy_outside_contact_distance_does_not_damage_player() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        scene.spawn_enemy([0.71, 0.0], EnemyKind::Basic, &catalog);

        let mut enemies = EnemyState::default();

        enemies.update_movement_and_contact_damage(&mut scene, &catalog, [0.0, 0.0], 1.0);

        assert_eq!(scene.health(scene.player).unwrap().current, 100.0);
    }
}
