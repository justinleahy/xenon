use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedStep {
    pub step_index: u64,
    pub delta: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedSteps {
    next_step_index: u64,
    remaining: u32,
    delta: Duration,
}

impl Iterator for FixedSteps {
    type Item = FixedStep;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let step = FixedStep {
            step_index: self.next_step_index,
            delta: self.delta,
        };

        self.next_step_index += 1;
        self.remaining -= 1;

        Some(step)
    }
}

#[derive(Debug, Clone)]
pub struct FixedTimestep {
    step: Duration,
    max_accumulated: Duration,
    accumulated: Duration,
    step_index: u64,
}
impl FixedTimestep {
    pub fn new(step: Duration) -> Self {
        Self::with_max_accumulated(step, step.saturating_mul(5))
    }

    pub fn with_max_accumulated(step: Duration, max_accumulated: Duration) -> Self {
        assert!(!step.is_zero(), "fixed timestep must be greater than zero");
        assert!(
            max_accumulated >= step,
            "max accumulated time must be at least one fixed step"
        );

        Self {
            step,
            max_accumulated,
            accumulated: Duration::ZERO,
            step_index: 0,
        }
    }

    pub fn at_60_hz() -> Self {
        Self::new(Duration::from_secs_f64(1.0 / 60.0))
    }

    pub fn advance(&mut self, frame_delta: Duration) -> FixedSteps {
        self.accumulated = self.accumulated.saturating_add(frame_delta);

        if self.accumulated > self.max_accumulated {
            self.accumulated = self.max_accumulated;
        }

        let mut steps_to_run = 0;

        while self.accumulated >= self.step {
            self.accumulated = self.accumulated.saturating_sub(self.step);
            steps_to_run += 1;
        }

        let first_step_index = self.step_index + 1;
        self.step_index += steps_to_run as u64;

        FixedSteps {
            next_step_index: first_step_index,
            remaining: steps_to_run as u32,
            delta: self.step,
        }
    }

    pub fn step(&self) -> Duration {
        self.step
    }

    pub fn accumulated(&self) -> Duration {
        self.accumulated
    }

    pub fn step_index(&self) -> u64 {
        self.step_index
    }
}

impl Default for FixedTimestep {
    fn default() -> Self {
        Self::at_60_hz()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_with_less_than_one_step_returns_no_steps() {
        let mut timestep = FixedTimestep::new(Duration::from_millis(16));

        let steps: Vec<_> = timestep.advance(Duration::from_millis(8)).collect();

        assert!(steps.is_empty());
        assert_eq!(timestep.step_index(), 0);
    }

    #[test]
    fn test_advance_with_one_step_returns_one_step() {
        let mut timestep = FixedTimestep::new(Duration::from_millis(16));
        let steps: Vec<_> = timestep.advance(Duration::from_millis(16)).collect();

        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].step_index, 1);
        assert_eq!(steps[0].delta, Duration::from_millis(16));
        assert_eq!(timestep.step_index(), 1);
    }

    #[test]
    fn test_advance_accumulates_partial_steps() {
        let mut timestep = FixedTimestep::new(Duration::from_millis(16));

        let first: Vec<_> = timestep.advance(Duration::from_millis(8)).collect();
        let second: Vec<_> = timestep.advance(Duration::from_millis(8)).collect();

        assert!(first.is_empty());
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].step_index, 1);
    }

    #[test]
    fn test_advance_caps_large_frame_delta() {
        let mut timestep = FixedTimestep::with_max_accumulated(
            Duration::from_millis(10),
            Duration::from_millis(30),
        );

        let steps: Vec<_> = timestep.advance(Duration::from_millis(100)).collect();

        assert_eq!(steps.len(), 3);
        assert_eq!(timestep.step_index(), 3);
    }
}
