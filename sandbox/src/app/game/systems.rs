use super::{CombatState, EnemyState, GameCatalog, PlayerController, PlayerProgression, Scene};
use crate::app::input::InputState;

#[derive(Default)]
pub struct GameSystems {
    pub combat: CombatState,
    pub enemies: EnemyState,
    pub player_controller: PlayerController,
    pub player_progression: PlayerProgression,
}

impl GameSystems {
    pub fn fixed_update(
        &mut self,
        scene: &mut Scene,
        catalog: &GameCatalog,
        input: &InputState,
        fixed_updates: u64,
        delta_secs: f32,
    ) {
        let movement = movement_from_input(input);

        self.player_controller
            .fixed_update(scene, movement, delta_secs);

        let player_position = player_position(scene);

        self.enemies
            .fixed_update(scene, catalog, player_position, fixed_updates, delta_secs);

        self.combat
            .fixed_update(scene, catalog, player_position, delta_secs);

        self.player_progression.fixed_update(scene, player_position);
    }
}

fn player_position(scene: &Scene) -> [f32; 2] {
    scene
        .transform(scene.player)
        .map(|transform| transform.position)
        .unwrap_or([0.0, 0.0])
}

fn movement_from_input(input: &InputState) -> [f32; 2] {
    let mut direction = [0.0_f32, 0.0_f32];

    if input.move_up {
        direction[1] -= 1.0;
    }

    if input.move_down {
        direction[1] += 1.0;
    }

    if input.move_left {
        direction[0] -= 1.0;
    }

    if input.move_right {
        direction[0] += 1.0;
    }

    let length = (direction[0] * direction[0] + direction[1] * direction[1]).sqrt();

    if length > 0.0 {
        direction[0] /= length;
        direction[1] /= length;
    }

    direction
}

#[cfg(test)]
mod tests {
    use super::super::components::PickupReward;
    use super::*;

    #[test]
    fn test_movement_from_input_normalizes_diagonal_input() {
        let input = InputState {
            move_up: true,
            move_right: true,
            ..InputState::default()
        };

        let movement = movement_from_input(&input);

        assert_eq!(movement, [0.70710677, -0.70710677]);
    }

    #[test]
    fn test_fixed_update_runs_player_before_pickup_collection() {
        let catalog = GameCatalog::default();
        let mut scene = Scene::empty();
        let pickup = scene.spawn_pickup([2.5, 0.0], PickupReward::Experience(3));
        let mut systems = GameSystems::default();
        let input = InputState {
            move_right: true,
            ..InputState::default()
        };

        systems.fixed_update(&mut scene, &catalog, &input, 0, 0.5);

        assert_eq!(scene.transform(scene.player).unwrap().position, [2.5, 0.0]);
        assert_eq!(systems.player_progression.experience, 3);
        assert!(scene.pickup(pickup).is_none());
    }
}
