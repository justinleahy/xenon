use super::Scene;

#[derive(Default)]
pub struct CombatState {
    pub weapon_cooldown_secs: f32,
}

impl CombatState {
    pub fn fixed_update(&mut self, scene: &mut Scene, player_position: [f32; 2], delta_secs: f32) {
        self.update_weapon(scene, player_position, delta_secs);
        self.update_projectiles(scene, delta_secs);
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

        scene.spawn_projectile(
            player_position,
            [
                direction[0] * projectile_speed,
                direction[1] * projectile_speed,
            ],
        );

        self.weapon_cooldown_secs = 0.5;
    }

    fn update_projectiles(&mut self, scene: &mut Scene, delta_secs: f32) {
        let transforms = &mut scene.transforms;

        for (entity, projectile) in &mut scene.projectiles {
            let entity = *entity;

            let Some(transform) = transforms
                .iter_mut()
                .find_map(|(id, transform)| (*id == entity).then_some(transform))
            else {
                continue;
            };

            projectile.previous_position = transform.position;
            transform.position[0] += projectile.velocity[0] * delta_secs;
            transform.position[1] += projectile.velocity[1] * delta_secs;
            projectile.lifetime_secs -= delta_secs;
        }
    }

    fn resolve_projectile_hits(&mut self, scene: &mut Scene) {
        let mut hit_enemies = Vec::new();
        let mut hit_projectiles = Vec::new();

        for (projectile_entity, projectile) in &scene.projectiles {
            let Some(projectile_position) = scene.transform(*projectile_entity).map(|t| t.position)
            else {
                continue;
            };

            let Some(projectile_collider) = scene.circle_collider(*projectile_entity) else {
                continue;
            };

            for enemy in &scene.enemies {
                let enemy = *enemy;

                let Some(enemy_collider) = scene.circle_collider(enemy) else {
                    continue;
                };

                let Some(enemy_position) = scene.transform(enemy).map(|t| t.position) else {
                    continue;
                };

                let hit_radius = projectile_collider.radius + enemy_collider.radius;

                let distance = distance_point_to_segment(
                    enemy_position,
                    projectile.previous_position,
                    projectile_position,
                );

                if distance < hit_radius {
                    hit_enemies.push(enemy);
                    hit_projectiles.push(*projectile_entity);
                    break;
                }
            }
        }

        hit_enemies.sort_unstable_by_key(|entity| entity.0);
        hit_enemies.dedup();

        hit_projectiles.sort_unstable_by_key(|entity| entity.0);
        hit_projectiles.dedup();

        for enemy in hit_enemies {
            scene.despawn_entity(enemy);
        }

        for projectile_entity in hit_projectiles {
            scene.despawn_entity(projectile_entity);
        }
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
        scene.projectiles.clear();
        scene
            .transforms
            .retain(|(entity, _)| *entity == scene.player);
        scene.health.retain(|(entity, _)| *entity == scene.player);
        scene.sprites.retain(|(entity, _)| *entity == scene.player);
        scene
            .circle_colliders
            .retain(|(entity, _)| *entity == scene.player);

        scene
    }

    #[test]
    fn test_projectile_hit_despawns_enemy_and_projectile() {
        let mut scene = scene_with_only_player();

        let enemy = scene.spawn_enemy([1.0, 0.0]);
        let projectile_entity = scene.spawn_projectile([2.0, 0.0], [1.0, 0.0]);

        scene
            .projectiles
            .iter_mut()
            .find_map(|(entity, projectile)| (*entity == projectile_entity).then_some(projectile))
            .unwrap()
            .previous_position = [0.0, 0.0];

        let mut combat = CombatState::default();

        combat.resolve_projectile_hits(&mut scene);

        assert!(!scene.enemies.contains(&enemy));
        assert!(
            !scene
                .projectiles
                .iter()
                .any(|(entity, _)| *entity == projectile_entity)
        );
    }

    #[test]
    fn test_projectile_miss_keeps_enemy_and_projectile() {
        let mut scene = scene_with_only_player();

        let enemy = scene.spawn_enemy([1.0, 0.51]);
        let projectile_entity = scene.spawn_projectile([2.0, 0.0], [1.0, 0.0]);

        scene
            .projectiles
            .iter_mut()
            .find_map(|(entity, projectile)| (*entity == projectile_entity).then_some(projectile))
            .unwrap()
            .previous_position = [0.0, 0.0];

        let mut combat = CombatState::default();

        combat.resolve_projectile_hits(&mut scene);

        assert!(scene.enemies.contains(&enemy));
        assert!(
            scene
                .projectiles
                .iter()
                .any(|(entity, _)| *entity == projectile_entity)
        );
    }

    #[test]
    fn test_fixed_update_fires_at_nearest_enemy() {
        let mut scene = scene_with_only_player();
        scene.spawn_enemy([4.0, 0.0]);

        let mut combat = CombatState::default();

        combat.fixed_update(&mut scene, [0.0, 0.0], 0.0);

        assert_eq!(scene.projectiles.len(), 1);
        assert_eq!(combat.weapon_cooldown_secs, 0.5);
    }
}
