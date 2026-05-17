use super::Scene;

pub struct Projectile {
    pub position: [f32; 2],
    pub previous_position: [f32; 2],
    pub velocity: [f32; 2],
    pub lifetime_secs: f32,
}

#[derive(Default)]
pub struct CombatState {
    pub projectiles: Vec<Projectile>,
    pub weapon_cooldown_secs: f32,
}

impl CombatState {
    pub fn fixed_update(&mut self, scene: &mut Scene, player_position: [f32; 2], delta_secs: f32) {
        self.update_weapon(scene, player_position, delta_secs);
        self.update_projectiles(delta_secs);
        self.resolve_projectile_hits(scene);
    }

    fn update_weapon(&mut self, scene: &mut Scene, player_position: [f32; 2], delta_secs: f32) {
        self.weapon_cooldown_secs = (self.weapon_cooldown_secs - delta_secs).max(0.0);

        if self.weapon_cooldown_secs > 0.0 {
            return;
        }

        let Some(target_position) = nearest_enemy_position(&scene, player_position) else {
            return;
        };

        let to_target = [
            target_position[0] - player_position[0],
            target_position[1] - player_position[1],
        ];

        let distance = (to_target[0] * to_target[0] + to_target[1] * to_target[1]).sqrt();

        if distance <= 0.001 {
            return;
        }

        let projectile_speed = 8.0;
        let direction = [to_target[0] / distance, to_target[1] / distance];

        self.projectiles.push(Projectile {
            position: player_position,
            previous_position: player_position,
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

    fn resolve_projectile_hits(&mut self, scene: &mut Scene) {
        let hit_radius = 0.30;

        let mut hit_enemies = Vec::new();
        let mut hit_projectile_indices = Vec::new();

        for (projectile_index, projectile) in self.projectiles.iter().enumerate() {
            for enemy in &scene.enemies {
                let enemy = *enemy;

                let Some(enemy_position) = scene.transform(enemy).map(|t| t.position) else {
                    continue;
                };

                let distance = distance_point_to_segment(
                    enemy_position,
                    projectile.previous_position,
                    projectile.position,
                );

                if distance < hit_radius {
                    hit_enemies.push(enemy);
                    hit_projectile_indices.push(projectile_index);
                    break;
                }
            }
        }

        hit_enemies.sort_unstable_by_key(|entity| entity.0);
        hit_enemies.dedup();

        hit_projectile_indices.sort_unstable();
        hit_projectile_indices.dedup();

        for enemy in hit_enemies {
            scene.despawn_entity(enemy);
        }

        let mut projectile_index = 0;
        self.projectiles.retain(|_| {
            let keep = hit_projectile_indices
                .binary_search(&projectile_index)
                .is_err();
            projectile_index += 1;
            keep
        });
    }
}

fn nearest_enemy_position(scene: &Scene, player_position: [f32; 2]) -> Option<[f32; 2]> {
    scene
        .enemies
        .iter()
        .filter_map(|enemy| scene.transform(*enemy))
        .map(|transform| transform.position)
        .min_by(|a, b| {
            let player_position = player_position;
            let a_distance = squared_distance(player_position, *a);
            let b_distance = squared_distance(player_position, *b);
            a_distance
                .partial_cmp(&b_distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn scene_with_only_player() -> Scene {
        let mut scene = Scene::new_survivor_demo();

        scene.enemies.clear();
        scene
            .transforms
            .retain(|(entity, _)| *entity == scene.player);
        scene.health.retain(|(entity, _)| *entity == scene.player);

        scene
    }

    #[test]
    fn test_projectile_hit_despawns_enemy_and_projectile() {
        let mut scene = scene_with_only_player();

        let enemy = scene.spawn_enemy([1.0, 0.0]);

        let mut combat = CombatState {
            projectiles: vec![Projectile {
                position: [2.0, 0.0],
                previous_position: [0.0, 0.0],
                velocity: [1.0, 0.0],
                lifetime_secs: 1.0,
            }],
            weapon_cooldown_secs: 0.0,
        };

        combat.resolve_projectile_hits(&mut scene);

        assert!(!scene.enemies.contains(&enemy));
        assert!(combat.projectiles.is_empty());
    }

    #[test]
    fn test_projectile_miss_keeps_enemy_and_projectile() {
        let mut scene = scene_with_only_player();

        let enemy = scene.spawn_enemy([1.0, 1.0]);

        let mut combat = CombatState {
            projectiles: vec![Projectile {
                position: [2.0, 0.0],
                previous_position: [0.0, 0.0],
                velocity: [1.0, 0.0],
                lifetime_secs: 1.0,
            }],
            weapon_cooldown_secs: 0.0,
        };

        combat.resolve_projectile_hits(&mut scene);

        assert!(scene.enemies.contains(&enemy));
        assert_eq!(combat.projectiles.len(), 1);
    }

    #[test]
    fn test_fixed_update_fires_at_nearest_enemy() {
        let mut scene = scene_with_only_player();
        scene.spawn_enemy([4.0, 0.0]);

        let mut combat = CombatState::default();

        combat.fixed_update(&mut scene, [0.0, 0.0], 0.0);

        assert_eq!(combat.projectiles.len(), 1);
        assert_eq!(combat.weapon_cooldown_secs, 0.5);
    }
}
