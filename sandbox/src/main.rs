mod app;

use app::SandboxApp;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = SandboxApp {
        window: None,
        context: None,
        surface: None,
    };

    event_loop.run_app(&mut app).expect("event loop failed");
}
