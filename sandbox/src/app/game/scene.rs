use crate::app::game::components::PickupReward;

use super::{
    components::{
        CircleCollider, Damage, DeathDrop, Health, Pickup, Projectile, Sprite, Transform,
    },
    entity::EntityId,
};

pub struct Scene {
    next_entity_id: u64,
    pub player: EntityId,
    pub transforms: Vec<(EntityId, Transform)>,
    pub health: Vec<(EntityId, Health)>,
    pub enemies: Vec<EntityId>,
    pub sprites: Vec<(EntityId, Sprite)>,
    pub projectiles: Vec<(EntityId, Projectile)>,
    pub circle_colliders: Vec<(EntityId, CircleCollider)>,
    pub damage: Vec<(EntityId, Damage)>,
    pub death_drops: Vec<(EntityId, DeathDrop)>,
    pub pickups: Vec<(EntityId, Pickup)>,
}

impl Scene {
    pub fn new_survivor_demo() -> Self {
        let mut scene = Self {
            next_entity_id: 0,
            player: EntityId(0),
            transforms: Vec::new(),
            health: Vec::new(),
            enemies: Vec::new(),
            sprites: Vec::new(),
            projectiles: Vec::new(),
            circle_colliders: Vec::new(),
            damage: Vec::new(),
            death_drops: Vec::new(),
            pickups: Vec::new(),
        };

        let player = scene.spawn_entity();
        scene.player = player;
        scene.transforms.push((
            player,
            Transform {
                position: [0.0, 0.0],
            },
        ));
        scene.sprites.push((
            player,
            Sprite {
                size: [1.0, 1.0],
                color: [0.95, 0.9, 0.55, 1.0],
            },
        ));
        scene.health.push((
            player,
            Health {
                current: 100.0,
                max: 100.0,
            },
        ));
        scene
            .circle_colliders
            .push((player, CircleCollider { radius: 0.35 }));

        scene.spawn_enemy([5.0, 5.0]);

        scene
    }

    pub fn spawn_entity(&mut self) -> EntityId {
        let entity = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        entity
    }

    pub fn despawn_entity(&mut self, entity: EntityId) {
        self.transforms.retain(|(id, _)| *id != entity);
        self.health.retain(|(id, _)| *id != entity);
        self.enemies.retain(|id| *id != entity);
        self.sprites.retain(|(id, _)| *id != entity);
        self.projectiles.retain(|(id, _)| *id != entity);
        self.circle_colliders.retain(|(id, _)| *id != entity);
        self.damage.retain(|(id, _)| *id != entity);
        self.death_drops.retain(|(id, _)| *id != entity);
        self.pickups.retain(|(id, _)| *id != entity);
    }

    pub fn spawn_enemy(&mut self, position: [f32; 2]) -> EntityId {
        let enemy = self.spawn_entity();

        self.enemies.push(enemy);
        self.transforms.push((enemy, Transform { position }));
        self.sprites.push((
            enemy,
            Sprite {
                size: [1.0, 1.0],
                color: [0.9, 0.35, 0.55, 1.0],
            },
        ));
        self.circle_colliders
            .push((enemy, CircleCollider { radius: 0.35 }));
        self.health.push((
            enemy,
            Health {
                current: 20.0,
                max: 20.0,
            },
        ));
        self.death_drops.push((
            enemy,
            DeathDrop {
                reward: PickupReward::Experience(1),
            },
        ));

        enemy
    }

    pub fn spawn_projectile(&mut self, position: [f32; 2], velocity: [f32; 2]) -> EntityId {
        let projectile = self.spawn_entity();

        self.transforms.push((projectile, Transform { position }));
        self.sprites.push((
            projectile,
            Sprite {
                size: [0.25, 0.25],
                color: [0.35, 0.75, 1.0, 1.0],
            },
        ));
        self.projectiles.push((
            projectile,
            Projectile {
                previous_position: position,
                velocity,
                lifetime_secs: 2.0,
            },
        ));
        self.circle_colliders
            .push((projectile, CircleCollider { radius: 0.15 }));
        self.damage.push((projectile, Damage { amount: 10.0 }));

        projectile
    }

    pub fn spawn_pickup(&mut self, position: [f32; 2], reward: PickupReward) -> EntityId {
        let pickup = self.spawn_entity();

        self.transforms.push((pickup, Transform { position }));
        self.sprites.push((
            pickup,
            Sprite {
                size: [0.25, 0.25],
                color: [0.45, 1.0, 0.55, 1.0],
            },
        ));
        self.circle_colliders
            .push((pickup, CircleCollider { radius: 0.25 }));
        self.pickups.push((pickup, Pickup { reward }));

        pickup
    }

    pub fn transform(&self, entity: EntityId) -> Option<&Transform> {
        self.transforms
            .iter()
            .find_map(|(id, transform)| (*id == entity).then_some(transform))
    }

    pub fn transform_mut(&mut self, entity: EntityId) -> Option<&mut Transform> {
        self.transforms
            .iter_mut()
            .find_map(|(id, transform)| (*id == entity).then_some(transform))
    }

    pub fn health(&self, entity: EntityId) -> Option<&Health> {
        self.health
            .iter()
            .find_map(|(id, health)| (*id == entity).then_some(health))
    }

    pub fn health_mut(&mut self, entity: EntityId) -> Option<&mut Health> {
        self.health
            .iter_mut()
            .find_map(|(id, health)| (*id == entity).then_some(health))
    }

    pub fn sprite(&self, entity: EntityId) -> Option<&Sprite> {
        self.sprites
            .iter()
            .find_map(|(id, sprite)| (*id == entity).then_some(sprite))
    }

    pub fn sprite_mut(&mut self, entity: EntityId) -> Option<&mut Sprite> {
        self.sprites
            .iter_mut()
            .find_map(|(id, sprite)| (*id == entity).then_some(sprite))
    }

    pub fn circle_collider(&self, entity: EntityId) -> Option<&CircleCollider> {
        self.circle_colliders
            .iter()
            .find_map(|(id, collider)| (*id == entity).then_some(collider))
    }

    pub fn circle_collider_mut(&mut self, entity: EntityId) -> Option<&mut CircleCollider> {
        self.circle_colliders
            .iter_mut()
            .find_map(|(id, collider)| (*id == entity).then_some(collider))
    }

    pub fn damage(&self, entity: EntityId) -> Option<&Damage> {
        self.damage
            .iter()
            .find_map(|(id, damage)| (*id == entity).then_some(damage))
    }

    pub fn damage_mut(&mut self, entity: EntityId) -> Option<&mut Damage> {
        self.damage
            .iter_mut()
            .find_map(|(id, damage)| (*id == entity).then_some(damage))
    }

    pub fn death_drop(&self, entity: EntityId) -> Option<&DeathDrop> {
        self.death_drops
            .iter()
            .find_map(|(id, drop)| (*id == entity).then_some(drop))
    }

    pub fn pickup(&self, entity: EntityId) -> Option<&Pickup> {
        self.pickups
            .iter()
            .find_map(|(id, pickup)| (*id == entity).then_some(pickup))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::scene_with_only_player;
    use super::*;

    #[test]
    fn test_new_survivor_demo_creates_valid_player() {
        let scene = Scene::new_survivor_demo();

        assert!(scene.transform(scene.player).is_some());
        assert!(scene.sprite(scene.player).is_some());
        assert!(scene.health(scene.player).is_some());
        assert_eq!(
            scene.circle_collider(scene.player).unwrap(),
            &CircleCollider { radius: 0.35 }
        );
    }

    #[test]
    fn test_spawn_enemy_creates_transform_sprite_and_enemy_tag() {
        let mut scene = scene_with_only_player();

        let enemy = scene.spawn_enemy([3.0, 4.0]);

        assert!(scene.enemies.contains(&enemy));
        assert_eq!(scene.transform(enemy).unwrap().position, [3.0, 4.0]);
        assert_eq!(
            scene.sprite(enemy).unwrap(),
            &Sprite {
                size: [1.0, 1.0],
                color: [0.9, 0.35, 0.55, 1.0],
            }
        );
        assert_eq!(
            scene.circle_collider(enemy).unwrap(),
            &CircleCollider { radius: 0.35 }
        );
        assert_eq!(
            scene.health(enemy).unwrap(),
            &Health {
                current: 20.0,
                max: 20.0,
            }
        );
        assert_eq!(
            scene.death_drop(enemy).unwrap(),
            &DeathDrop {
                reward: PickupReward::Experience(1),
            }
        );
    }

    #[test]
    fn test_spawn_projectile_creates_transform_sprite_and_projectile_component() {
        let mut scene = scene_with_only_player();

        let projectile = scene.spawn_projectile([1.0, 2.0], [8.0, 0.0]);

        assert_eq!(scene.transform(projectile).unwrap().position, [1.0, 2.0]);
        assert_eq!(
            scene.sprite(projectile).unwrap(),
            &Sprite {
                size: [0.25, 0.25],
                color: [0.35, 0.75, 1.0, 1.0],
            }
        );
        assert_eq!(
            scene
                .projectiles
                .iter()
                .find_map(|(entity, projectile_component)| {
                    (*entity == projectile).then_some(projectile_component)
                })
                .unwrap(),
            &Projectile {
                previous_position: [1.0, 2.0],
                velocity: [8.0, 0.0],
                lifetime_secs: 2.0,
            }
        );
        assert_eq!(
            scene.circle_collider(projectile).unwrap(),
            &CircleCollider { radius: 0.15 }
        );
        assert_eq!(scene.damage(projectile).unwrap(), &Damage { amount: 10.0 });
    }

    #[test]
    fn test_spawn_pickup_creates_transform_sprite_collider_and_pickup_component() {
        let mut scene = scene_with_only_player();

        let pickup = scene.spawn_pickup([2.0, 3.0], PickupReward::Experience(7));

        assert_eq!(scene.transform(pickup).unwrap().position, [2.0, 3.0]);
        assert_eq!(
            scene.sprite(pickup).unwrap(),
            &Sprite {
                size: [0.25, 0.25],
                color: [0.45, 1.0, 0.55, 1.0],
            }
        );
        assert_eq!(
            scene.circle_collider(pickup).unwrap(),
            &CircleCollider { radius: 0.25 }
        );
        assert_eq!(
            scene.pickup(pickup).unwrap(),
            &Pickup {
                reward: PickupReward::Experience(7),
            }
        );
    }

    #[test]
    fn test_despawn_entity_removes_components_and_tags() {
        let mut scene = scene_with_only_player();

        let enemy = scene.spawn_enemy([3.0, 4.0]);
        let projectile = scene.spawn_projectile([1.0, 2.0], [8.0, 0.0]);
        let pickup = scene.spawn_pickup([2.0, 3.0], PickupReward::Experience(7));

        scene.despawn_entity(enemy);
        scene.despawn_entity(projectile);
        scene.despawn_entity(pickup);

        assert!(scene.transform(enemy).is_none());
        assert!(scene.sprite(enemy).is_none());
        assert!(scene.circle_collider(enemy).is_none());
        assert!(scene.health(enemy).is_none());
        assert!(scene.death_drop(enemy).is_none());
        assert!(!scene.enemies.contains(&enemy));

        assert!(scene.transform(projectile).is_none());
        assert!(scene.sprite(projectile).is_none());
        assert!(scene.circle_collider(projectile).is_none());
        assert!(scene.damage(projectile).is_none());
        assert!(
            !scene
                .projectiles
                .iter()
                .any(|(entity, _)| *entity == projectile)
        );

        assert!(scene.transform(pickup).is_none());
        assert!(scene.sprite(pickup).is_none());
        assert!(scene.circle_collider(pickup).is_none());
        assert!(scene.pickup(pickup).is_none());
    }

    #[test]
    fn test_mut_helpers_update_sprite_circle_collider_and_damage_components() {
        let mut scene = scene_with_only_player();
        let projectile = scene.spawn_projectile([1.0, 2.0], [8.0, 0.0]);

        scene.sprite_mut(scene.player).unwrap().size = [2.0, 2.0];
        scene.circle_collider_mut(scene.player).unwrap().radius = 0.5;
        scene.damage_mut(projectile).unwrap().amount = 5.0;

        assert_eq!(scene.sprite(scene.player).unwrap().size, [2.0, 2.0]);
        assert_eq!(
            scene.circle_collider(scene.player).unwrap(),
            &CircleCollider { radius: 0.5 }
        );
        assert_eq!(scene.damage(projectile).unwrap(), &Damage { amount: 5.0 });
    }
}
