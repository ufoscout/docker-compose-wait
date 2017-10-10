use std::time::Duration;
use std::thread;

pub trait Sleeper {
    fn sleep(&self, duration: u64);
}

struct SecondsSleeper {}

impl Sleeper for SecondsSleeper {
    fn sleep(&self, duration: u64) {
        thread::sleep(Duration::from_secs(duration))
    }
}

struct NoOpsSleeper {}

impl Sleeper for NoOpsSleeper {
    fn sleep(&self, duration: u64) {
    }
}

pub fn new() -> impl Sleeper {
        SecondsSleeper{}
}

pub fn new_no_ops() -> impl Sleeper {
        NoOpsSleeper{}
}

#[cfg(test)]
mod test {

    use std::time::Instant;
    use super::*;

    #[test]
    fn should_wait_for_a_second() {
        let sleeper = new();

        let start = Instant::now();
        sleeper.sleep(1);
        let elapsed_sec = start.elapsed().as_secs();
        assert!( elapsed_sec >= 1);
    }

    #[test]
    fn should_not_wait() {
        let sleeper = new_no_ops();

        let start = Instant::now();
        sleeper.sleep(10);
        let elapsed_sec = start.elapsed().as_secs();
        assert!( elapsed_sec <= 1);
    }
}
