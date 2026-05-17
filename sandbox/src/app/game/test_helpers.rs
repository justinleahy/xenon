use super::{GameCatalog, Scene};

pub fn scene_with_only_player() -> Scene {
    let catalog = GameCatalog::default();
    let mut scene = Scene::new_survivor_demo(&catalog);
    let player = scene.player;

    scene.enemies.clear();
    scene.projectiles.clear();
    scene.damage.clear();
    scene.death_drops.clear();
    scene.pickups.clear();
    scene.enemy_components.clear();
    scene.transforms.retain(|(entity, _)| *entity == player);
    scene.health.retain(|(entity, _)| *entity == player);
    scene.sprites.retain(|(entity, _)| *entity == player);
    scene
        .circle_colliders
        .retain(|(entity, _)| *entity == player);

    scene
}
