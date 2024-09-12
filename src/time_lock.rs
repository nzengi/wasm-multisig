use std::time::SystemTime;

/// A struct that represents a time-based lock mechanism, which tracks an expiration time
/// and provides functionality to check if the lock has expired.
pub struct TimeLock {
    pub expiry: SystemTime,   // The point in time when the lock will expire
    pub expired: bool,        // A boolean flag indicating whether the lock has expired
}

impl TimeLock {
    /// Checks if the TimeLock has expired by comparing the current system time with the expiration time.
    /// Once the lock has expired, it will set the `expired` flag to `true` and return the result.
    ///
    /// # Returns
    /// * `true` if the lock has expired, `false` otherwise.
    pub fn is_expired(&mut self) -> bool {
        // If the lock has not expired yet and the current time is past the expiry time, mark it as expired
        if !self.expired && SystemTime::now() >= self.expiry {
            self.expired = true;
        }
        self.expired
    }
}
