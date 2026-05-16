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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clock_starts_at_frame_zero() {
        let clock = FrameClock::new();

        assert_eq!(clock.frame_index, 0);
    }

    #[test]
    fn test_tick_increaments_frame_index() {
        let mut clock = FrameClock::new();
        clock.tick();

        assert_eq!(clock.frame_index, 1);
    }

    #[test]
    fn test_consecutive_ticks_increament_frame_index() {
        let mut clock = FrameClock::new();
        let first = clock.tick();
        let second = clock.tick();

        assert_eq!(first.frame_index, 1);
        assert_eq!(second.frame_index, 2);
        assert_eq!(clock.frame_index, 2);
    }
}
