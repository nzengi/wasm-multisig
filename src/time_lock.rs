use std::time::{SystemTime, Duration};

pub struct TimeLock {
    pub expiry: SystemTime,
}

impl TimeLock {
    pub fn new(duration: Duration) -> Self {
        TimeLock {
            expiry: SystemTime::now() + duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expiry
    }
}
