use softbuffer::{Context, Surface};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct SandboxApp {
    pub window: Option<Arc<Window>>,
    pub context: Option<Context<Arc<Window>>>,
    pub surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl ApplicationHandler for SandboxApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("Xenon Sandbox")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
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
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

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

                for pixel in buffer.iter_mut() {
                    *pixel = 0x00203040;
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
