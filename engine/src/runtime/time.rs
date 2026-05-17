use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameTiming {
    pub frame_index: u64,
    pub delta: Duration,
    pub elapsed: Duration,
}

#[derive(Debug, Clone)]
pub struct FrameClock {
    started_at: Instant,
    last_frame_at: Instant,
    frame_index: u64,
}

#[derive(Debug, Clone)]
pub struct FpsCounter {
    sample_window: Duration,
    accumulated: Duration,
    frames: u32,
    last_fps: Option<f64>,
}

impl FrameClock {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            started_at: now,
            last_frame_at: now,
            frame_index: 0,
        }
    }

    pub fn tick(&mut self) -> FrameTiming {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame_at);
        let elapsed = now.duration_since(self.started_at);

        self.last_frame_at = now;
        self.frame_index += 1;

        FrameTiming {
            frame_index: self.frame_index,
            delta,
            elapsed,
        }
    }

    pub fn frame_index(&self) -> u64 {
        self.frame_index
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.started_at)
    }
}

impl Default for FrameClock {
    fn default() -> Self {
        Self::new()
    }
}

impl FpsCounter {
    pub fn new(sample_window: Duration) -> Self {
        assert!(
            !sample_window.is_zero(),
            "fps sample window must be greater than zero"
        );

        Self {
            sample_window,
            accumulated: Duration::ZERO,
            frames: 0,
            last_fps: None,
        }
    }

    pub fn per_second() -> Self {
        Self::new(Duration::from_secs(1))
    }

    pub fn record_frame(&mut self, delta: Duration) -> Option<f64> {
        self.accumulated = self.accumulated.saturating_add(delta);
        self.frames += 1;

        if self.accumulated < self.sample_window {
            return None;
        }

        let fps = self.frames as f64 / self.accumulated.as_secs_f64();

        self.accumulated = Duration::ZERO;
        self.frames = 0;
        self.last_fps = Some(fps);

        Some(fps)
    }

    pub fn last_fps(&self) -> Option<f64> {
        self.last_fps
    }

    pub fn sample_window(&self) -> Duration {
        self.sample_window
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::per_second()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clock_starts_at_frame_zero() {
        let clock = FrameClock::new();

        assert_eq!(clock.frame_index, 0);
    }

    #[test]
    fn test_tick_increments_frame_index() {
        let mut clock = FrameClock::new();
        clock.tick();

        assert_eq!(clock.frame_index, 1);
    }

    #[test]
    fn test_consecutive_ticks_increment_frame_index() {
        let mut clock = FrameClock::new();
        let first = clock.tick();
        let second = clock.tick();

        assert_eq!(first.frame_index, 1);
        assert_eq!(second.frame_index, 2);
        assert_eq!(clock.frame_index, 2);
    }

    #[test]
    fn test_fps_counter_returns_none_before_sample_window() {
        let mut counter = FpsCounter::new(Duration::from_secs(1));

        let fps = counter.record_frame(Duration::from_millis(500));

        assert_eq!(fps, None);
        assert_eq!(counter.last_fps(), None);
    }

    #[test]
    fn test_fps_counter_returns_fps_when_sample_window_elapses() {
        let mut counter = FpsCounter::new(Duration::from_secs(1));

        counter.record_frame(Duration::from_millis(500));
        let fps = counter.record_frame(Duration::from_millis(500)).unwrap();

        assert_eq!(fps, 2.0);
        assert_eq!(counter.last_fps(), Some(2.0));
    }

    #[test]
    fn test_fps_counter_resets_after_sample() {
        let mut counter = FpsCounter::new(Duration::from_secs(1));

        counter.record_frame(Duration::from_millis(500));
        counter.record_frame(Duration::from_millis(500));
        let fps = counter.record_frame(Duration::from_millis(1000)).unwrap();

        assert_eq!(fps, 1.0);
    }
}
