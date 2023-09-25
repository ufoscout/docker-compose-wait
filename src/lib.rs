use env_reader::env_var_exists;
use log::*;
use std::option::Option;
use std::path::Path;
use std::time::Duration;

pub mod env_reader;
pub mod sleeper;

pub struct Command {
    pub program: String,
    pub argv: Vec<String>,
}

pub struct Config {
    pub hosts: String,
    pub paths: String,
    pub command: Option<(Command, String)>,
    pub global_timeout: u64,
    pub tcp_connection_timeout: u64,
    pub wait_before: u64,
    pub wait_after: u64,
    pub wait_sleep_interval: u64,
}

const LINE_SEPARATOR: &str = "--------------------------------------------------------";

pub fn wait(sleep: &mut dyn sleeper::Sleeper, config: &Config, on_timeout: &mut dyn FnMut()) {
    info!("{}", LINE_SEPARATOR);
    info!(" docker-compose-wait {}", env!("CARGO_PKG_VERSION"));
    info!("---------------------------");
    debug!("Starting with configuration:");
    debug!(" - Hosts to be waiting for: [{}]", config.hosts);
    debug!(" - Paths to be waiting for: [{}]", config.paths);
    debug!(
        " - Timeout before failure: {} seconds ",
        config.global_timeout
    );
    debug!(
        " - TCP connection timeout before retry: {} seconds ",
        config.tcp_connection_timeout
    );

    if let Some((_, command_string)) = &config.command {
        debug!(" - Command to run once ready: {}", command_string);
    }

    debug!(
        " - Sleeping time before checking for hosts/paths availability: {} seconds",
        config.wait_before
    );
    debug!(
        " - Sleeping time once all hosts/paths are available: {} seconds",
        config.wait_after
    );
    debug!(
        " - Sleeping time between retries: {} seconds",
        config.wait_sleep_interval
    );
    debug!("{}", LINE_SEPARATOR);

    if config.wait_before > 0 {
        info!(
            "Waiting {} seconds before checking for hosts/paths availability",
            config.wait_before
        );
        info!("{}", LINE_SEPARATOR);
        sleep.sleep(config.wait_before);
    }

    sleep.reset();

    if !config.hosts.trim().is_empty() {
        for host in config.hosts.trim().split(',') {
            info!("Checking availability of host [{}]", host);
            while !port_check::is_port_reachable_with_timeout(
                &host.trim().to_string(),
                Duration::from_secs(config.tcp_connection_timeout),
            ) {
                info!("Host [{}] not yet available...", host);
                if sleep.elapsed(config.global_timeout) {
                    error!(
                        "Timeout! After {} seconds some hosts are still not reachable",
                        config.global_timeout
                    );
                    on_timeout();
                    return;
                }
                sleep.sleep(config.wait_sleep_interval);
            }
            info!("Host [{}] is now available!", host);
            info!("{}", LINE_SEPARATOR);
        }
    }

    if !config.paths.trim().is_empty() {
        for path in config.paths.trim().split(',') {
            info!("Checking availability of path [{}]", path);
            while !Path::new(path.trim()).exists() {
                info!("Path {} not yet available...", path);
                if sleep.elapsed(config.global_timeout) {
                    error!(
                        "Timeout! After [{}] seconds some paths are still not reachable",
                        config.global_timeout
                    );
                    on_timeout();
                    return;
                }
                sleep.sleep(config.wait_sleep_interval);
            }
            info!("Path [{}] is now available!", path);
            info!("{}", LINE_SEPARATOR);
        }
    }

    if config.wait_after > 0 {
        info!(
            "Waiting {} seconds after hosts/paths availability",
            config.wait_after
        );
        info!("{}", LINE_SEPARATOR);
        sleep.sleep(config.wait_after);
    }

    info!("docker-compose-wait - Everything's fine, the application can now start!");
    info!("{}", LINE_SEPARATOR);

    if let Some((command, _)) = &config.command {
        let err = exec::Command::new(&command.program)
            .args(&command.argv)
            .exec();
        panic!("{}", err);
    }
}

pub fn parse_command<S: Into<String>>(
    raw_cmd: S,
) -> Result<Option<(Command, String)>, shell_words::ParseError> {
    let s = raw_cmd.into();
    let command_string = s.trim().to_string();
    if command_string.is_empty() {
        return Ok(None);
    }
    let mut argv = shell_words::split(&command_string)?;
    Ok(Some((
        Command {
            program: argv.remove(0),
            argv,
        },
        command_string,
    )))
}

pub fn config_from_env() -> Config {
    Config {
        hosts: env_reader::env_var("WAIT_HOSTS", "".to_string()),
        paths: env_reader::env_var("WAIT_PATHS", "".to_string()),
        command: parse_command(env_reader::env_var("WAIT_COMMAND", "".to_string()))
            .expect("failed to parse command value from environment"),
        global_timeout: to_int(&legacy_or_new("WAIT_HOSTS_TIMEOUT", "WAIT_TIMEOUT", ""), 30),
        tcp_connection_timeout: to_int(
            &env_reader::env_var("WAIT_HOST_CONNECT_TIMEOUT", "".to_string()),
            5,
        ),
        wait_before: to_int(&legacy_or_new("WAIT_BEFORE_HOSTS", "WAIT_BEFORE", ""), 0),
        wait_after: to_int(&legacy_or_new("WAIT_AFTER_HOSTS", "WAIT_AFTER", ""), 0),
        wait_sleep_interval: to_int(
            &env_reader::env_var("WAIT_SLEEP_INTERVAL", "".to_string()),
            1,
        ),
    }
}

fn legacy_or_new(legacy_var_name: &str, var_name: &str, default: &str) -> String {
    let mut temp_value = default.to_string();
    if env_var_exists(legacy_var_name) {
        warn!(
            "Environment variable [{}] is deprecated. Use [{}] instead.",
            legacy_var_name, var_name
        );
        temp_value = env_reader::env_var(legacy_var_name, temp_value);
    }
    temp_value = env_reader::env_var(var_name, temp_value);
    temp_value
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
        set_env("", "", "10o", "10", "", "abc", "");
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
        set_env("localhost:1234", "20", "2", "3", "4", "23", "");
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
        set_env("localhost:1234", "", "", "", "", "", "");
        let config = config_from_env();
        assert_eq!("localhost:1234".to_string(), config.hosts);
        assert_eq!(30, config.global_timeout);
        assert_eq!(5, config.tcp_connection_timeout);
        assert_eq!(0, config.wait_before);
        assert_eq!(0, config.wait_after);
        assert_eq!(1, config.wait_sleep_interval);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_given_an_invalid_command() {
        let _guard = TEST_MUTEX.lock().unwrap();
        set_env("", "", "", "", "", "", "a 'b");
        config_from_env();
    }

    fn set_env(
        hosts: &str,
        timeout: &str,
        before: &str,
        after: &str,
        sleep: &str,
        tcp_timeout: &str,
        command: &str,
    ) {
        env::set_var("WAIT_BEFORE_HOSTS", before);
        env::set_var("WAIT_AFTER_HOSTS", after);
        env::set_var("WAIT_HOSTS_TIMEOUT", timeout);
        env::set_var("WAIT_HOST_CONNECT_TIMEOUT", tcp_timeout);
        env::set_var("WAIT_HOSTS", hosts);
        env::set_var("WAIT_SLEEP_INTERVAL", sleep);
        env::set_var("WAIT_COMMAND", command);
    }

    #[test]
    fn parse_command_fails_when_command_is_invalid() {
        assert!(parse_command(" intentionally 'invalid").is_err())
    }

    #[test]
    fn parse_command_returns_none_when_command_is_empty() {
        for c in &["", " \t\n\r\n"] {
            let p = parse_command(c.to_string()).unwrap();
            assert!(p.is_none());
        }
    }

    #[test]
    fn parse_command_handles_commands_without_args() {
        let (command, command_string) = parse_command("ls".to_string()).unwrap().unwrap();
        assert_eq!("ls", command_string);
        assert_eq!("ls", command.program);
        assert_eq!(vec!["ls"], command.argv);
    }

    #[test]
    fn parse_command_handles_commands_with_args() {
        let (command, command_string) = parse_command("ls -al".to_string()).unwrap().unwrap();
        assert_eq!("ls -al", command_string);
        assert_eq!("ls", command.program);
        assert_eq!(vec!["ls", "-al"], command.argv);
    }

    #[test]
    fn parse_command_discards_leading_and_trailing_whitespace() {
        let (command, command_string) = parse_command("     hello world    ".to_string())
            .unwrap()
            .unwrap();
        assert_eq!("hello world", command_string);
        assert_eq!("hello", command.program);
        assert_eq!(vec!["hello", "world"], command.argv);
    }

    #[test]
    fn parse_command_strips_shell_quotes() {
        let (command, command_string) =
            parse_command(" find . -type \"f\" -name '*.rs' ".to_string())
                .unwrap()
                .unwrap();
        assert_eq!("find . -type \"f\" -name '*.rs'", command_string);
        assert_eq!("find", command.program);
        assert_eq!(
            vec!["find", ".", "-type", "f", "-name", "*.rs"],
            command.argv
        );
    }
}
