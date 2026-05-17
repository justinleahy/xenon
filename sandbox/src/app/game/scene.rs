use super::{
    components::{Health, Transform},
    entity::EntityId,
};

pub struct Scene {
    next_entity_id: u64,
    pub player: EntityId,
    pub transforms: Vec<(EntityId, Transform)>,
    pub health: Vec<(EntityId, Health)>,
    pub enemies: Vec<EntityId>,
}

impl Scene {
    pub fn new_survivor_demo() -> Self {
        let mut scene = Self {
            next_entity_id: 0,
            player: EntityId(0),
            transforms: Vec::new(),
            health: Vec::new(),
            enemies: Vec::new(),
        };

        let player = scene.spawn_entity();
        scene.player = player;
        scene.transforms.push((
            player,
            Transform {
                position: [0.0, 0.0],
            },
        ));
        scene.health.push((
            player,
            Health {
                current: 100.0,
                max: 100.0,
            },
        ));

        scene.spawn_enemy([5.0, 5.0]);

        scene
    }

    pub fn spawn_entity(&mut self) -> EntityId {
        let entity = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        entity
    }

    pub fn spawn_enemy(&mut self, position: [f32; 2]) -> EntityId {
        let enemy = self.spawn_entity();
        self.enemies.push(enemy);
        self.transforms.push((enemy, Transform { position }));
        enemy
    }

    pub fn despawn_entity(&mut self, entity: EntityId) {
        self.transforms.retain(|(id, _)| *id != entity);
        self.health.retain(|(id, _)| *id != entity);
        self.enemies.retain(|id| *id != entity);
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
}
