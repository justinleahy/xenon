use engine::{EngineConfig, FixedTimestep, FpsCounter, FrameClock, FrameTiming, LifecycleEvent};
use softbuffer::{Context, Surface};
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
    pub context: Option<Context<Arc<Window>>>,
    pub surface: Option<Surface<Arc<Window>, Arc<Window>>>,
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
            context: None,
            surface: None,
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
        let speed = 240.0;

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

        let Some(surface) = self.surface.as_mut() else {
            return;
        };

        let size = window.inner_size();

        if size.width == 0 || size.height == 0 {
            return;
        }

        surface
            .resize(
                std::num::NonZeroU32::new(size.width).unwrap(),
                std::num::NonZeroU32::new(size.height).unwrap(),
            )
            .expect("failed to resize surface");

        let mut buffer = surface.buffer_mut().expect("failed to get buffer");

        let clear_color = clear_color_to_softbuffer_pixel(self.config.clear_color);

        for pixel in buffer.iter_mut() {
            *pixel = clear_color;
        }

        let player_x = (size.width as f32 * 0.5 + self.state.player_position[0]).round() as i32;
        let player_y = (size.height as f32 * 0.5 + self.state.player_position[1]).round() as i32;

        draw_filled_rect(
            &mut buffer,
            size.width,
            size.height,
            player_x - 8,
            player_y - 8,
            16,
            16,
            0x00f0e68c,
        );

        buffer.present().expect("failed to present buffer");
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

        let context = Context::new(window.clone()).expect("failed to create softbuffer context");
        let surface =
            Surface::new(&context, window.clone()).expect("failed to create softbuffer surface");

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);

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
                        self.input.reset_requested = is_pressed;
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

fn clear_color_to_softbuffer_pixel([r, g, b, _a]: [u8; 4]) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn draw_filled_rect(
    buffer: &mut [u32],
    buffer_width: u32,
    buffer_height: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: u32,
) {
    let min_x = x.max(0) as u32;
    let min_y = y.max(0) as u32;

    let max_x = (x + width as i32).clamp(0, buffer_width as i32) as u32;
    let max_y = (y + height as i32).clamp(0, buffer_height as i32) as u32;

    for py in min_y..max_y {
        let row_start = (py * buffer_width) as usize;

        for px in min_x..max_x {
            buffer[row_start + px as usize] = color;
        }
    }
}
