#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    Starting,
    Started,
    Stopping,
    Stopped,
}
