use std::{
    thread,
    time::{Duration, Instant},
};

pub struct FpsCap {
    target_micros: u64,
    last_frame: Instant,
}

impl FpsCap {
    pub fn with_target_fps(target: u32) -> Self {
        let target_micros = 1_000_000 / target as u64;
        Self {
            last_frame: Instant::now(),
            target_micros,
        }
    }

    pub fn delta(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.last_frame = now;

        delta
    }

    pub fn cap(&self) {
        let frame_delta = Instant::now() - self.last_frame;
        if (frame_delta.as_micros() as u64) < self.target_micros {
            thread::sleep(Duration::from_micros(self.target_micros) - frame_delta);
        }
    }
}

impl Default for FpsCap {
    fn default() -> Self {
        FpsCap::with_target_fps(60)
    }
}
