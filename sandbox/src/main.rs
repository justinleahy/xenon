mod app;

use app::SandboxApp;
use engine::{EngineConfig, runtime::LifecycleEvent};
use tracing::info;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!(event = ?LifecycleEvent::Starting, "application starting");

    let config = EngineConfig::load_from_file("sandbox/config/runtime.toml")?;

    info!(?config, "runtime config loaded");

    let event_loop = EventLoop::new()?;
    let mut app = SandboxApp::new(config);

    event_loop.run_app(&mut app)?;

    info!(event = ?LifecycleEvent::Stopped, "application stopped");

    Ok(())
}
