use super::super::{Scene, components::PickupReward};

#[derive(Default)]
pub struct PlayerProgression {
    pub experience: u32,
}

impl PlayerProgression {
    pub fn fixed_update(&mut self, scene: &mut Scene, player_position: [f32; 2]) {
        let Some(player_collider) = scene.circle_collider(scene.player) else {
            return;
        };

        let mut collected_pickups = Vec::new();

        for (pickup_entity, pickup) in &scene.pickups {
            let Some(pickup_position) = scene
                .transform(*pickup_entity)
                .map(|transform| transform.position)
            else {
                continue;
            };

            let Some(pickup_collider) = scene.circle_collider(*pickup_entity) else {
                continue;
            };

            let dx = player_position[0] - pickup_position[0];
            let dy = player_position[1] - pickup_position[1];
            let collect_radius = player_collider.radius + pickup_collider.radius;

            if dx * dx + dy * dy < collect_radius * collect_radius {
                match pickup.reward {
                    PickupReward::Experience(amount) => {
                        self.experience += amount;
                    }
                }

                collected_pickups.push(*pickup_entity);
            }
        }

        collected_pickups.sort_unstable_by_key(|entity| entity.0);
        collected_pickups.dedup();

        for pickup_entity in collected_pickups {
            scene.despawn_entity(pickup_entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene_with_only_player() -> Scene {
        let mut scene = Scene::new_survivor_demo();

        scene.enemies.clear();
        scene.projectiles.clear();
        scene.damage.clear();
        scene.death_drops.clear();
        scene.pickups.clear();
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
    fn test_overlapping_pickup_adds_experience_and_despawns_pickup() {
        let mut scene = scene_with_only_player();
        let pickup = scene.spawn_pickup([0.5, 0.0], PickupReward::Experience(3));

        let mut progression = PlayerProgression::default();

        progression.fixed_update(&mut scene, [0.0, 0.0]);

        assert_eq!(progression.experience, 3);
        assert!(scene.pickup(pickup).is_none());
    }

    #[test]
    fn test_non_overlapping_pickup_remains_uncollected() {
        let mut scene = scene_with_only_player();
        let pickup = scene.spawn_pickup([2.0, 0.0], PickupReward::Experience(3));

        let mut progression = PlayerProgression::default();

        progression.fixed_update(&mut scene, [0.0, 0.0]);

        assert_eq!(progression.experience, 0);
        assert!(scene.pickup(pickup).is_some());
    }
}
