use std::time::{Duration, Instant};
use std::sync::Mutex;

lazy_static! {
    pub static ref TIMING_DATA: TimingData = TimingData {
        last_frame_duration: Duration::new(0, 0),
        last_frame_time: Instant::now(),
    };
}

pub struct TimingData {
    last_frame_duration: Duration,
    last_frame_time: Instant,
}

impl TimingData {
    fn update(&mut self) {
        let now = Instant::now();
        self.last_frame_duration = now - self.last_frame_time;
        self.last_frame_time = now;
    }

    pub fn get_last_frame_duration(&self) -> &Duration {
        &self.last_frame_duration
    }
}
