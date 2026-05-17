use crate::app::game::{
    EnemyKind, GameCatalog,
    catalog::WeaponDefinition,
    components::{Enemy, PickupReward},
};

use super::{
    components::{
        CircleCollider, Damage, DeathDrop, Health, Pickup, Projectile, Sprite, Transform,
    },
    entity::{EntityId, SceneObjectId},
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
    pub enemy_components: Vec<(EntityId, Enemy)>,
    pub scene_object_ids: Vec<(EntityId, SceneObjectId)>,
}

impl Scene {
    pub fn empty() -> Self {
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
            enemy_components: Vec::new(),
            scene_object_ids: Vec::new(),
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
        self.enemy_components.retain(|(id, _)| *id != entity);
        self.scene_object_ids.retain(|(id, _)| *id != entity);
    }

    pub fn spawn_enemy(
        &mut self,
        position: [f32; 2],
        kind: EnemyKind,
        catalog: &GameCatalog,
    ) -> EntityId {
        let definition = catalog.enemy(kind);
        let enemy = self.spawn_entity();

        self.enemies.push(enemy);
        self.enemy_components.push((enemy, Enemy { kind }));
        self.transforms.push((enemy, Transform { position }));
        self.sprites.push((
            enemy,
            Sprite {
                size: definition.sprite_size,
                color: definition.sprite_color,
            },
        ));
        self.circle_colliders.push((
            enemy,
            CircleCollider {
                radius: definition.collider_radius,
            },
        ));
        self.health.push((
            enemy,
            Health {
                current: definition.health,
                max: definition.health,
            },
        ));
        self.death_drops.push((
            enemy,
            DeathDrop {
                reward: definition.death_reward,
            },
        ));

        enemy
    }

    pub fn spawn_projectile(
        &mut self,
        position: [f32; 2],
        velocity: [f32; 2],
        weapon: &WeaponDefinition,
    ) -> EntityId {
        let projectile = self.spawn_entity();

        self.transforms.push((projectile, Transform { position }));
        self.sprites.push((
            projectile,
            Sprite {
                size: weapon.projectile_size,
                color: weapon.projectile_color,
            },
        ));
        self.projectiles.push((
            projectile,
            Projectile {
                previous_position: position,
                velocity,
                lifetime_secs: weapon.projectile_lifetime_secs,
            },
        ));
        self.circle_colliders.push((
            projectile,
            CircleCollider {
                radius: weapon.projectile_collider_radius,
            },
        ));
        self.damage.push((
            projectile,
            Damage {
                amount: weapon.projectile_damage,
            },
        ));

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

    pub fn enemy(&self, entity: EntityId) -> Option<&Enemy> {
        self.enemy_components
            .iter()
            .find_map(|(id, enemy)| (*id == entity).then_some(enemy))
    }

    pub fn scene_object_id(&self, entity: EntityId) -> Option<&SceneObjectId> {
        self.scene_object_ids
            .iter()
            .find_map(|(id, scene_object_id)| (*id == entity).then_some(scene_object_id))
    }

    pub fn set_scene_object_id(&mut self, entity: EntityId, scene_object_id: SceneObjectId) {
        if let Some((_, existing_id)) = self
            .scene_object_ids
            .iter_mut()
            .find(|(id, _)| *id == entity)
        {
            *existing_id = scene_object_id;
            return;
        }

        self.scene_object_ids.push((entity, scene_object_id));
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::scene_with_only_player;
    use super::*;
    use crate::app::game::WeaponKind;

    #[test]
    fn test_empty_creates_valid_player() {
        let scene = Scene::empty();

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
        let catalog = GameCatalog::default();

        let enemy = scene.spawn_enemy([3.0, 4.0], EnemyKind::Basic, &catalog);

        assert!(scene.enemies.contains(&enemy));
        assert_eq!(scene.enemy(enemy).unwrap().kind, EnemyKind::Basic);
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
    fn test_scene_object_id_can_be_assigned_and_replaced() {
        let mut scene = scene_with_only_player();

        scene.set_scene_object_id(scene.player, SceneObjectId::new("player"));
        scene.set_scene_object_id(scene.player, SceneObjectId::new("player.updated"));

        assert_eq!(
            scene.scene_object_id(scene.player),
            Some(&SceneObjectId::new("player.updated"))
        );
        assert_eq!(scene.scene_object_ids.len(), 1);
    }

    #[test]
    fn test_spawn_projectile_creates_transform_sprite_and_projectile_component() {
        let mut scene = scene_with_only_player();
        let catalog = GameCatalog::default();
        let weapon = catalog.weapon(WeaponKind::Wand);

        let projectile = scene.spawn_projectile([1.0, 2.0], [8.0, 0.0], weapon);

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
        let catalog = GameCatalog::default();
        let weapon = catalog.weapon(WeaponKind::Wand);

        let enemy = scene.spawn_enemy([3.0, 4.0], EnemyKind::Basic, &catalog);
        let projectile = scene.spawn_projectile([1.0, 2.0], [8.0, 0.0], weapon);
        let pickup = scene.spawn_pickup([2.0, 3.0], PickupReward::Experience(7));
        scene.set_scene_object_id(enemy, SceneObjectId::new("enemy.test"));

        scene.despawn_entity(enemy);
        scene.despawn_entity(projectile);
        scene.despawn_entity(pickup);

        assert!(scene.transform(enemy).is_none());
        assert!(scene.sprite(enemy).is_none());
        assert!(scene.circle_collider(enemy).is_none());
        assert!(scene.health(enemy).is_none());
        assert!(scene.death_drop(enemy).is_none());
        assert!(scene.enemy(enemy).is_none());
        assert!(scene.scene_object_id(enemy).is_none());
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
        let catalog = GameCatalog::default();
        let weapon = catalog.weapon(WeaponKind::Wand);
        let projectile = scene.spawn_projectile([1.0, 2.0], [8.0, 0.0], weapon);

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
