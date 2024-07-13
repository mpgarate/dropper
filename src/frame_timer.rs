use std::time::{Duration, SystemTime};

pub struct FrameTimer {
    duration: Duration,
    last_lap: SystemTime,
}

impl FrameTimer {
    pub fn new(duration: Duration) -> FrameTimer {
        let time_now = SystemTime::now();

        FrameTimer {
            duration: duration,
            last_lap: time_now,
        }
    }

    pub fn next_frame(&mut self) -> bool {
        let time_now = SystemTime::now();

        match self.last_lap.elapsed() {
            Ok(elapsed) if elapsed > self.duration => {
                self.last_lap = time_now;

                true
            }
            Ok(_) => false,
            Err(_) => panic!(),
        }
    }
}
