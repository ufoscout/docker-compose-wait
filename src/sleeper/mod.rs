use std::thread;
use std::time::{Duration, Instant};

pub trait Sleeper {
    fn sleep(&self, duration: u64);
    fn reset(&mut self);
    fn elapsed(&self, units: u64) -> bool;
}

struct SecondsSleeper {
    started_at: Instant,
}

impl Default for SecondsSleeper {
    fn default() -> Self {
        SecondsSleeper {
            started_at: Instant::now(),
        }
    }
}

impl Sleeper for SecondsSleeper {
    fn sleep(&self, duration: u64) {
        thread::sleep(Duration::from_secs(duration))
    }

    fn reset(&mut self) {
        self.started_at = Instant::now()
    }

    fn elapsed(&self, units: u64) -> bool {
        self.started_at.elapsed().as_secs() >= units
    }
}

pub struct MillisSleeper {
    started_at: Instant,
}

impl Default for MillisSleeper {
    fn default() -> Self {
        MillisSleeper {
            started_at: Instant::now(),
        }
    }
}

impl Sleeper for MillisSleeper {
    fn sleep(&self, duration: u64) {
        thread::sleep(Duration::from_millis(duration))
    }

    fn reset(&mut self) {
        self.started_at = Instant::now()
    }

    fn elapsed(&self, units: u64) -> bool {
        self.started_at.elapsed().as_millis() >= u128::from(units)
    }
}

struct NoOpsSleeper {}

impl Sleeper for NoOpsSleeper {
    fn sleep(&self, _duration: u64) {}

    fn reset(&mut self) {}

    fn elapsed(&self, _units: u64) -> bool {
        true
    }
}

pub fn new() -> impl Sleeper {
    SecondsSleeper::default()
}

pub fn new_no_ops() -> impl Sleeper {
    NoOpsSleeper {}
}

#[cfg(test)]
mod test {

    use super::*;
    use std::time::Instant;

    #[test]
    fn should_wait_for_a_second() {
        let mut sleeper = new();
        assert!(!sleeper.elapsed(1));

        let start = Instant::now();
        sleeper.sleep(1);
        let elapsed_sec = start.elapsed().as_secs();
        assert!(elapsed_sec >= 1);
        assert!(sleeper.elapsed(1));

        sleeper.reset();
        assert!(!sleeper.elapsed(1));

        sleeper.sleep(1);
        assert!(sleeper.elapsed(1));
        assert!(!sleeper.elapsed(2));

        sleeper.sleep(1);
        assert!(sleeper.elapsed(2));
    }

    #[test]
    fn should_not_wait() {
        let sleeper = new_no_ops();

        let start = Instant::now();
        sleeper.sleep(10);
        let elapsed_sec = start.elapsed().as_secs();
        assert!(elapsed_sec <= 1);
    }
}
