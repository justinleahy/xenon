use super::super::Scene;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_moves_by_direction_speed_and_delta() {
        let mut scene = Scene::empty();
        let mut player = PlayerController;

        player.fixed_update(&mut scene, [1.0, 0.0], 0.5);

        assert_eq!(scene.transform(scene.player).unwrap().position, [2.5, 0.0]);
    }
}
