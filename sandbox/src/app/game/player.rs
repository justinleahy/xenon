use super::Scene;

#[derive(Default)]
pub struct PlayerController;

impl PlayerController {
    pub fn fixed_update(&mut self, scene: &mut Scene, movement: [f32; 2], delta_secs: f32) {
        let player_speed = 5.0;

        if let Some(transform) = scene.transform_mut(scene.player) {
            transform.position[0] += movement[0] * player_speed * delta_secs;
            transform.position[1] += movement[1] * player_speed * delta_secs;
        }
    }
}
