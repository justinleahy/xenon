use super::Scene;

pub fn scene_with_only_player() -> Scene {
    let mut scene = Scene::new_survivor_demo();
    let player = scene.player;

    scene.enemies.clear();
    scene.projectiles.clear();
    scene.damage.clear();
    scene.death_drops.clear();
    scene.pickups.clear();
    scene.transforms.retain(|(entity, _)| *entity == player);
    scene.health.retain(|(entity, _)| *entity == player);
    scene.sprites.retain(|(entity, _)| *entity == player);
    scene
        .circle_colliders
        .retain(|(entity, _)| *entity == player);

    scene
}
