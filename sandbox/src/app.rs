use engine::runtime::{EngineConfig, FrameClock, LifecycleEvent};
use softbuffer::{Context, Surface};
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
    pub window: Option<Arc<Window>>,
    pub context: Option<Context<Arc<Window>>>,
    pub surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl SandboxApp {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            frame_clock: FrameClock::new(),
            window: None,
            context: None,
            surface: None,
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

                let timing = self.frame_clock.tick();

                if timing.frame_index % 300 == 0 {
                    info!(
                        frame_index = timing.frame_index,
                        delta_ms = timing.delta.as_secs_f64() * 1000.0,
                        elapsed_secs = timing.elapsed.as_secs_f64(),
                        "frame timing"
                    );
                }

                let clear_color = clear_color_to_softbuffer_pixel(self.config.clear_color);

                for pixel in buffer.iter_mut() {
                    *pixel = clear_color;
                }

                buffer.present().expect("failed to present buffer");
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
