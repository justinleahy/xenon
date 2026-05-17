use engine::{
    EngineConfig, FixedTimestep, FpsCounter, FrameClock, FrameTiming, LifecycleEvent, RenderCamera,
    RenderScene, RenderSprite, Renderer,
};
use std::sync::Arc;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct SandboxApp {
    pub config: EngineConfig,
    pub frame_clock: FrameClock,
    pub fixed_timestep: FixedTimestep,
    pub fps_counter: FpsCounter,
    pub state: SandboxState,
    pub input: InputState,
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer<'static>>,
}

#[derive(Default)]
pub struct SandboxState {
    simulation_time_secs: f64,
    fixed_updates: u64,
    player_position: [f32; 2],
}

#[derive(Debug, Default)]
pub struct InputState {
    move_up: bool,
    move_down: bool,
    move_left: bool,
    move_right: bool,
    reset_requested: bool,
    quit_requested: bool,
}

impl SandboxApp {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            frame_clock: FrameClock::new(),
            fixed_timestep: FixedTimestep::default(),
            fps_counter: FpsCounter::default(),
            state: SandboxState::default(),
            input: InputState::default(),
            window: None,
            renderer: None,
        }
    }

    fn fixed_update(&mut self) {
        if self.input.reset_requested {
            self.state = SandboxState::default();
            self.input.reset_requested = false;

            info!("simulation reset");
            return;
        }

        let delta_secs = self.fixed_timestep.step().as_secs_f32();
        let speed = 5.0;

        let mut direction = [0.0_f32, 0.0_f32];

        if self.input.move_up {
            direction[1] -= 1.0;
        }

        if self.input.move_down {
            direction[1] += 1.0;
        }

        if self.input.move_left {
            direction[0] -= 1.0;
        }

        if self.input.move_right {
            direction[0] += 1.0;
        }

        let length = (direction[0] * direction[0] + direction[1] * direction[1]).sqrt();

        if length > 0.0 {
            direction[0] /= length;
            direction[1] /= length;
        }

        self.state.player_position[0] += direction[0] * speed * delta_secs;
        self.state.player_position[1] += direction[1] * speed * delta_secs;

        self.state.fixed_updates += 1;
        self.state.simulation_time_secs += self.fixed_timestep.step().as_secs_f64();

        if self.state.fixed_updates % 60 == 0 {
            info!(
                fixed_updates = self.state.fixed_updates,
                simulation_time_secs = self.state.simulation_time_secs,
                player_x = self.state.player_position[0],
                player_y = self.state.player_position[1],
                "simulation state"
            );
        }
    }

    fn render(&mut self) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        let size = window.inner_size();

        let mut sprites = Vec::new();

        append_grid_sprites(&mut sprites, self.state.player_position, 20);

        sprites.push(RenderSprite {
            position: [3.0, 3.0],
            size: [1.0, 1.0],
            color: [0.0, 0.9, 0.55, 1.0],
        });

        sprites.push(RenderSprite {
            position: self.state.player_position,
            size: [1.0, 1.0],
            color: [0.95, 0.9, 0.55, 1.0],
        });

        renderer.resize(size.width, size.height);

        renderer
            .render(
                self.config.clear_color,
                RenderScene {
                    camera: RenderCamera {
                        position: self.state.player_position,
                        pixels_per_world_unit: 32.0,
                    },
                    sprites: &sprites,
                },
            )
            .expect("failed to render frame");
    }

    fn log_frame_metrics(&mut self, timing: FrameTiming) {
        if let Some(fps) = self.fps_counter.record_frame(timing.delta) {
            info!(fps, "fps")
        }

        if timing.frame_index % 300 == 0 {
            info!(
                frame_index = timing.frame_index,
                delta_ms = timing.delta.as_secs_f64() * 1000.0,
                elapsed_secs = timing.elapsed.as_secs_f64(),
                "frame timing"
            );
        }
    }
}

impl ApplicationHandler for SandboxApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title(self.config.app_name.clone())
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.window_width as f64,
                self.config.window_height as f64,
            ))
            .with_visible(true);

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        );

        let size = window.inner_size();

        let renderer = pollster::block_on(Renderer::new(window.clone(), size.width, size.height))
            .expect("failed to create renderer");

        self.window = Some(window);
        self.renderer = Some(renderer);

        info!(event = ?LifecycleEvent::Started, "application started");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!(event = ?LifecycleEvent::Stopping, "application stopping");
                event_loop.exit()
            }

            WindowEvent::RedrawRequested => {
                let timing = self.frame_clock.tick();

                self.log_frame_metrics(timing);

                for _step in self.fixed_timestep.advance(timing.delta) {
                    self.fixed_update();
                }

                if self.input.quit_requested {
                    info!(event = ?LifecycleEvent::Stopping, "application stopping");
                    event_loop.exit();
                    return;
                }

                self.render();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let is_pressed = event.state == ElementState::Pressed;

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.input.move_up = is_pressed;
                    }

                    PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.input.move_down = is_pressed;
                    }

                    PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.input.move_left = is_pressed;
                    }

                    PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.input.move_right = is_pressed;
                    }

                    PhysicalKey::Code(KeyCode::KeyR) => {
                        if is_pressed {
                            self.input.reset_requested = true;
                        }
                    }

                    PhysicalKey::Code(KeyCode::KeyQ) => {
                        if is_pressed {
                            self.input.quit_requested = true;
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
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
