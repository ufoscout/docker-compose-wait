pub mod env_reader;
pub mod sleeper;
pub mod tcp;

pub struct Config {
    pub hosts: String,
    pub timeout: u64,
    pub wait_before: u64,
    pub wait_after: u64,
    pub wait_sleep_interval: u64,
}

pub fn wait(sleep: &sleeper::Sleeper, config: &Config, on_timeout: &mut FnMut()) {
    println!("Docker-compose-wait starting with configuration:");
    println!("------------------------------------------------");
    println!(" - Hosts to be waiting for: [{}]", config.hosts);
    println!(" - Timeout before failure: {} seconds ", config.timeout);
    println!(
        " - Sleeping time before checking for hosts availability: {} seconds",
        config.wait_before
    );
    println!(
        " - Sleeping time once all hosts are available: {} seconds",
        config.wait_after
    );
    println!("------------------------------------------------");

    if config.wait_before > 0 {
        println!(
            "Waiting {} seconds before checking for hosts availability",
            config.wait_before
        );
        sleep.sleep(config.wait_before);
    }

    if !config.hosts.trim().is_empty() {
        let mut count = 0;
        //let start = Instant::now();
        for host in config.hosts.trim().split(',') {
            println!("Checking availability of {}", host);
            while !tcp::is_reachable(&host.trim().to_string()) {
                println!("Host {} not yet available", host);
                count += 1;
                if count > config.timeout {
                    //if (start.elapsed().as_secs() > wait_timeout) {
                    println!(
                        "Timeout! After {} seconds some hosts are still not reachable",
                        config.timeout
                    );
                    on_timeout();
                    return;
                }
                sleep.sleep(config.wait_sleep_interval);
            }
            println!("Host {} is now available", host);
        }
    }

    if config.wait_after > 0 {
        println!(
            "Waiting {} seconds after hosts availability",
            config.wait_after
        );
        sleep.sleep(config.wait_after);
    }
}

pub fn config_from_env() -> Config {
    Config {
        hosts: env_reader::env_var(&"WAIT_HOSTS".to_string(), "".to_string()),
        timeout: to_int(
            &env_reader::env_var(&"WAIT_HOSTS_TIMEOUT".to_string(), "".to_string()),
            30,
        ),
        wait_before: to_int(
            &env_reader::env_var(&"WAIT_BEFORE_HOSTS".to_string(), "".to_string()),
            0,
        ),
        wait_after: to_int(
            &env_reader::env_var(&"WAIT_AFTER_HOSTS".to_string(), "".to_string()),
            0,
        ),
        wait_sleep_interval: to_int(
            &env_reader::env_var(&"WAIT_SLEEP_INTERVAL".to_string(), "".to_string()),
            1,
        ),
    }
}

fn to_int(number: &str, default: u64) -> u64 {
    match number.parse::<u64>() {
        Ok(value) => value,
        Err(_e) => default,
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::env;

    #[test]
    fn should_return_int_value() {
        let value = to_int("32", 0);
        assert_eq!(32, value)
    }

    #[test]
    fn should_return_zero_when_negative_value() {
        let value = to_int("-32", 10);
        assert_eq!(10, value)
    }

    #[test]
    fn should_return_zero_when_invalid_value() {
        let value = to_int("hello", 0);
        assert_eq!(0, value)
    }

    #[test]
    fn should_return_zero_when_empty_value() {
        let value = to_int("", 11);
        assert_eq!(11, value)
    }

    /*
        #[test]
        fn default_timeout_should_be_30() {
            set_env("", "", "10o", "10");
            let config = config_from_env();
            assert_eq!("".to_string(), config.hosts);
            assert_eq!(30, config.timeout);
            assert_eq!(0, config.wait_before);
            assert_eq!(10, config.wait_after);
        }
    */

    #[test]
    fn should_get_config_values_from_env() {
        set_env("localhost:1234", "20", "2", "3", "4");
        let config = config_from_env();
        assert_eq!("localhost:1234".to_string(), config.hosts);
        assert_eq!(20, config.timeout);
        assert_eq!(2, config.wait_before);
        assert_eq!(3, config.wait_after);
        assert_eq!(4, config.wait_sleep_interval);
    }

    /*     #[test]
       fn should_get_default_config_values() {
           set_env("localhost:1234", "", "", "", "");
           let config = config_from_env();
           assert_eq!("localhost:1234".to_string(), config.hosts);
           assert_eq!(30, config.timeout);
           assert_eq!(0, config.wait_before);
           assert_eq!(0, config.wait_after);
           assert_eq!(1, config.wait_sleep_interval);
       }
    */
    fn set_env(hosts: &str, timeout: &str, before: &str, after: &str, sleep: &str) {
        env::set_var("WAIT_BEFORE_HOSTS", before.to_string());
        env::set_var("WAIT_AFTER_HOSTS", after.to_string());
        env::set_var("WAIT_HOSTS_TIMEOUT", timeout.to_string());
        env::set_var("WAIT_HOSTS", hosts.to_string());
        env::set_var("WAIT_SLEEP_INTERVAL", sleep.to_string());
    }
}
