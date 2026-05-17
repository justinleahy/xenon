use super::state::SandboxState;
use engine::{RenderCamera, RenderScene, RenderSprite};

const PIXELS_PER_WORLD_UNIT: f32 = 32.0;

pub fn build_render_sprites(state: &SandboxState) -> Vec<RenderSprite> {
    let mut sprites = Vec::new();

    append_grid_sprites(&mut sprites, state.player_position(), 20);

    for (entity, transform) in &state.scene.transforms {
        let Some(sprite) = state.scene.sprite(*entity) else {
            continue;
        };
        sprites.push(RenderSprite {
            position: transform.position,
            size: sprite.size,
            color: sprite.color,
        });
    }

    append_health_bar_sprites(&mut sprites, state);

    sprites
}

pub fn build_render_scene<'a>(
    state: &'a SandboxState,
    sprites: &'a [RenderSprite],
) -> RenderScene<'a> {
    RenderScene {
        camera: RenderCamera {
            position: state.player_position(),
            pixels_per_world_unit: PIXELS_PER_WORLD_UNIT,
        },
        sprites,
    }
}

fn append_grid_sprites(sprites: &mut Vec<RenderSprite>, camera_position: [f32; 2], radius: i32) {
    let pixels_per_world_unit = 32.0;
    let line_thickness = 2.0 / pixels_per_world_unit;

    let center_x = camera_position[0].floor() as i32;
    let center_y = camera_position[1].floor() as i32;

    let min_x = center_x - radius;
    let max_x = center_x + radius;
    let min_y = center_y - radius;
    let max_y = center_y + radius;

    let axis_color = [0.18, 0.24, 0.28, 1.0];
    let grid_color = [0.32, 0.38, 0.42, 1.0];

    for x in min_x..=max_x {
        sprites.push(RenderSprite {
            position: [x as f32, camera_position[1]],
            size: [line_thickness, (radius * 2) as f32],
            color: if x == 0 { axis_color } else { grid_color },
        })
    }

    for y in min_y..=max_y {
        sprites.push(RenderSprite {
            position: [camera_position[0], y as f32],
            size: [(radius * 2) as f32, line_thickness],
            color: if y == 0 { axis_color } else { grid_color },
        })
    }
}

fn append_health_bar_sprites(sprites: &mut Vec<RenderSprite>, state: &SandboxState) {
    let max_health = 100.0;
    let health_fraction = (state.player_health() / max_health).clamp(0.0, 1.0);

    let camera_position = state.player_position();

    let bar_width = 8.0;
    let bar_height = 0.35;
    let bar_offset = [-17.0, -10.0];

    let bar_center = [
        camera_position[0] + bar_offset[0],
        camera_position[1] + bar_offset[1],
    ];

    sprites.push(RenderSprite {
        position: bar_center,
        size: [bar_width, bar_height],
        color: [0.18, 0.08, 0.08, 1.0],
    });

    let fill_width = bar_width * health_fraction;
    let fill_center = [
        bar_center[0] - (bar_width - fill_width) * 0.5,
        bar_center[1],
    ];

    let fill_color = if health_fraction > 0.5 {
        [0.25, 0.9, 0.35, 1.0]
    } else if health_fraction > 0.25 {
        [0.95, 0.75, 0.25, 1.0]
    } else {
        [0.95, 0.25, 0.25, 1.0]
    };

    sprites.push(RenderSprite {
        position: fill_center,
        size: [fill_width, bar_height],
        color: fill_color,
    });
}

#[cfg(test)]
mod tests {
    use super::super::game::components::{Sprite, Transform};
    use super::super::game::test_helpers::scene_with_only_player;
    use super::super::state::SandboxState;
    use super::*;

    fn render_sprite_count(sprites: &[RenderSprite], expected: RenderSprite) -> usize {
        sprites.iter().filter(|sprite| **sprite == expected).count()
    }

    fn state_with_empty_scene() -> SandboxState {
        let mut state = SandboxState::default();
        state.scene = scene_with_only_player();
        state.scene.sprites.clear();
        state.scene.health.clear();
        state.scene.circle_colliders.clear();

        state
    }

    #[test]
    fn test_build_render_sprites_includes_entities_with_transform_and_sprite() {
        let mut state = state_with_empty_scene();
        let entity = state.scene.spawn_entity();

        state.scene.transforms.push((
            entity,
            Transform {
                position: [12.5, -3.25],
            },
        ));
        state.scene.sprites.push((
            entity,
            Sprite {
                size: [2.0, 3.0],
                color: [0.1, 0.2, 0.3, 1.0],
            },
        ));

        let sprites = build_render_sprites(&state);

        assert_eq!(
            render_sprite_count(
                &sprites,
                RenderSprite {
                    position: [12.5, -3.25],
                    size: [2.0, 3.0],
                    color: [0.1, 0.2, 0.3, 1.0],
                },
            ),
            1
        );
    }

    #[test]
    fn test_build_render_sprites_skips_entities_without_sprite() {
        let mut state = state_with_empty_scene();
        let entity = state.scene.spawn_entity();

        state.scene.transforms.push((
            entity,
            Transform {
                position: [12.5, -3.25],
            },
        ));

        let sprites = build_render_sprites(&state);

        assert!(!sprites.iter().any(|sprite| {
            sprite.position == [12.5, -3.25]
                && sprite.size == [2.0, 3.0]
                && sprite.color == [0.1, 0.2, 0.3, 1.0]
        }));
    }

    #[test]
    fn test_build_render_sprites_renders_projectiles_from_scene_components_once() {
        let mut state = state_with_empty_scene();

        state.scene.spawn_projectile([2.0, 3.0], [8.0, 0.0]);

        let sprites = build_render_sprites(&state);

        assert_eq!(
            render_sprite_count(
                &sprites,
                RenderSprite {
                    position: [2.0, 3.0],
                    size: [0.25, 0.25],
                    color: [0.35, 0.75, 1.0, 1.0],
                },
            ),
            1
        );
    }

    #[test]
    fn test_build_render_sprites_appends_grid_and_health_bar() {
        let state = SandboxState::default();

        let sprites = build_render_sprites(&state);

        assert!(sprites.iter().any(|sprite| {
            sprite.position == [0.0, 0.0]
                && sprite.size == [2.0 / 32.0, 40.0]
                && sprite.color == [0.18, 0.24, 0.28, 1.0]
        }));
        assert!(sprites.iter().any(|sprite| {
            sprite.position == [-17.0, -10.0]
                && sprite.size == [8.0, 0.35]
                && sprite.color == [0.18, 0.08, 0.08, 1.0]
        }));
    }
}
