mod app;

use app::SandboxApp;
use engine::EngineConfig;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    let config = EngineConfig::default();
    config.validate()?;

    let event_loop = EventLoop::new()?;
    let mut app = SandboxApp::new(config);

    event_loop.run_app(&mut app).expect("event loop failed");

    Ok(())
}
