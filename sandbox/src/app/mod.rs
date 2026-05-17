mod input;
mod render_scene;
mod state;

use self::{
    input::InputState,
    render_scene::{build_render_scene, build_render_sprites},
    state::SandboxState,
};
use engine::{
    EngineConfig, FixedTimestep, FpsCounter, FrameClock, FrameTiming, LifecycleEvent, Renderer,
};
use std::sync::Arc;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
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
        let delta_secs = self.fixed_timestep.step().as_secs_f32();
        self.state.fixed_update(&mut self.input, delta_secs)
    }

    fn render(&mut self) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        let size = window.inner_size();

        let sprites = build_render_sprites(&self.state);
        let scene = build_render_scene(&self.state, &sprites);

        renderer.resize(size.width, size.height);

        renderer
            .render(self.config.clear_color, scene)
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
                self.input
                    .handle_keyboard_input(event.physical_key, event.state);
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
