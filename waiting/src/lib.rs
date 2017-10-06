#![feature(conservative_impl_trait)]

use sleeper::Sleeper;
use std::time::Instant;

pub mod env_reader;
pub mod sleeper;
pub mod tcp;

pub fn wait(sleep: &sleeper::Sleeper, on_timeout : fn()) {
    let hosts = env_reader::env_var(&"WAIT_HOSTS".to_string(), "".to_string());
    let wait_timeout = to_int(env_reader::env_var(&"WAIT_HOSTS_TIMEOUT".to_string(), "".to_string()), 30);
    let wait_before = to_int(env_reader::env_var(&"WAIT_BEFORE_HOSTS".to_string(), "".to_string()), 0);
    let wait_after = to_int(env_reader::env_var(&"WAIT_AFTER_HOSTS".to_string(), "".to_string()), 0);

    if (wait_before > 0) {
        println!("Waiting {} seconds before checking for hosts availability", wait_before);
        sleep.sleep(wait_before);
    }

    if (!hosts.trim().is_empty()) {
        let start = Instant::now();
        for host in hosts.trim().split(',') {
            println!("Checking availability of {}", host);
            while (!tcp::is_reachable(&host.trim().to_string())) {
                println!("Host {} not yet availabile", host);
                if (start.elapsed().as_secs() > wait_timeout) {
                    println!("Timeout! After {} seconds some hosts are still not reachable", wait_timeout);
                    on_timeout();
                    return;
                }
                sleep.sleep(1);
            }
            println!("Host {} is now availabile", host);
        }
    }

    if (wait_after > 0) {
        println!("Waiting {} seconds after hosts availability", wait_after);
        sleep.sleep(wait_after);
    }
}

fn to_int(number: String, default : u64) -> u64 {
    match number.parse::<u64>() {
        Ok(value) => value,
        Err(e) => default
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_int_value() {
        let value = to_int("32".to_string(), 0);
        assert!(32 == value)
    }

    #[test]
    fn should_return_zero_when_negative_value() {
        let value = to_int("-32".to_string(), 10);
        assert!(10 == value)
    }

    #[test]
    fn should_return_zero_when_Invalid_value() {
        let value = to_int("hello".to_string(), 0);
        assert!(0 == value)
    }

    #[test]
    fn should_return_zero_when_empty_value() {
        let value = to_int("".to_string(), 11);
        assert!(11 == value)
    }
}
