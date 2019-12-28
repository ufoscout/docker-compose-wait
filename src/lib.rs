use std::time::Duration;

pub mod env_reader;
pub mod sleeper;

pub struct Config {
    pub hosts: String,
    pub global_timeout: u64,
    pub tcp_connection_timeout: u64,
    pub wait_before: u64,
    pub wait_after: u64,
    pub wait_sleep_interval: u64,
}

const LINE_SEPARATOR: &str = "--------------------------------------------------------";

pub fn wait(
    sleep: &mut dyn crate::sleeper::Sleeper,
    config: &Config,
    on_timeout: &mut dyn FnMut(),
) {
    println!("{}", LINE_SEPARATOR);
    println!("docker-compose-wait - starting with configuration:");
    println!(" - Hosts to be waiting for: [{}]", config.hosts);
    println!(" - Timeout before failure: {} seconds ", config.global_timeout);
    println!(" - TCP connection timeout before retry: {} seconds ", config.tcp_connection_timeout);
    println!(
        " - Sleeping time before checking for hosts availability: {} seconds",
        config.wait_before
    );
    println!(
        " - Sleeping time once all hosts are available: {} seconds",
        config.wait_after
    );
    println!(
        " - Sleeping time between retries: {} seconds",
        config.wait_sleep_interval
    );
    println!("{}", LINE_SEPARATOR);

    if config.wait_before > 0 {
        println!(
            "Waiting {} seconds before checking for hosts availability",
            config.wait_before
        );
        println!("{}", LINE_SEPARATOR);
        sleep.sleep(config.wait_before);
    }

    if !config.hosts.trim().is_empty() {
        sleep.reset();
        for host in config.hosts.trim().split(',') {
            println!("Checking availability of {}", host);
            while !port_check::is_port_reachable_with_timeout(&host.trim().to_string().parse().expect("The host IP should be valid"), Duration::from_secs(config.tcp_connection_timeout)) {
                println!("Host {} not yet available...", host);
                if sleep.elapsed(config.global_timeout) {
                    println!(
                        "Timeout! After {} seconds some hosts are still not reachable",
                        config.global_timeout
                    );
                    on_timeout();
                    return;
                }
                sleep.sleep(config.wait_sleep_interval);
            }
            println!("Host {} is now available!", host);
            println!("{}", LINE_SEPARATOR);
        }
    }

    if config.wait_after > 0 {
        println!(
            "Waiting {} seconds after hosts availability",
            config.wait_after
        );
        println!("{}", LINE_SEPARATOR);
        sleep.sleep(config.wait_after);
    }

    println!("docker-compose-wait - Everything's fine, the application can now start!");
    println!("{}", LINE_SEPARATOR);
}

pub fn config_from_env() -> Config {
    Config {
        hosts: crate::env_reader::env_var(&"WAIT_HOSTS".to_string(), "".to_string()),
        global_timeout: to_int(
            &crate::env_reader::env_var(&"WAIT_HOSTS_TIMEOUT".to_string(), "".to_string()),
            30,
        ),
        tcp_connection_timeout: to_int(
            &crate::env_reader::env_var(&"WAIT_HOST_CONNECT_TIMEOUT".to_string(), "".to_string()),
            5,
        ),
        wait_before: to_int(
            &crate::env_reader::env_var(&"WAIT_BEFORE_HOSTS".to_string(), "".to_string()),
            0,
        ),
        wait_after: to_int(
            &crate::env_reader::env_var(&"WAIT_AFTER_HOSTS".to_string(), "".to_string()),
            0,
        ),
        wait_sleep_interval: to_int(
            &crate::env_reader::env_var(&"WAIT_SLEEP_INTERVAL".to_string(), "".to_string()),
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
    use lazy_static::*;
    use std::env;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

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

    #[test]
    fn config_should_use_default_values() {
        let _guard = TEST_MUTEX.lock().unwrap();
        set_env("", "", "10o", "10", "", "abc");
        let config = config_from_env();
        assert_eq!("".to_string(), config.hosts);
        assert_eq!(30, config.global_timeout);
        assert_eq!(5, config.tcp_connection_timeout);
        assert_eq!(0, config.wait_before);
        assert_eq!(10, config.wait_after);
    }

    #[test]
    fn should_get_config_values_from_env() {
        let _guard = TEST_MUTEX.lock().unwrap();
        set_env("localhost:1234", "20", "2", "3", "4", "23");
        let config = config_from_env();
        assert_eq!("localhost:1234".to_string(), config.hosts);
        assert_eq!(20, config.global_timeout);
        assert_eq!(23, config.tcp_connection_timeout);
        assert_eq!(2, config.wait_before);
        assert_eq!(3, config.wait_after);
        assert_eq!(4, config.wait_sleep_interval);
    }

    #[test]
    fn should_get_default_config_values() {
        let _guard = TEST_MUTEX.lock().unwrap();
        set_env("localhost:1234", "", "", "", "", "");
        let config = config_from_env();
        assert_eq!("localhost:1234".to_string(), config.hosts);
        assert_eq!(30, config.global_timeout);
        assert_eq!(5, config.tcp_connection_timeout);
        assert_eq!(0, config.wait_before);
        assert_eq!(0, config.wait_after);
        assert_eq!(1, config.wait_sleep_interval);
    }

    fn set_env(hosts: &str, timeout: &str, before: &str, after: &str, sleep: &str, tcp_timeout: &str) {
        env::set_var("WAIT_BEFORE_HOSTS", before.to_string());
        env::set_var("WAIT_AFTER_HOSTS", after.to_string());
        env::set_var("WAIT_HOSTS_TIMEOUT", timeout.to_string());
        env::set_var("WAIT_HOST_CONNECT_TIMEOUT", tcp_timeout.to_string());
        env::set_var("WAIT_HOSTS", hosts.to_string());
        env::set_var("WAIT_SLEEP_INTERVAL", sleep.to_string());
    }
}
