use atomic_counter::AtomicCounter;
use std::fs::{File, create_dir_all};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::time::Instant;
use std::{thread, time};
use wait::sleeper::*;

#[test]
fn should_wait_for_5_seconds_before() {
    let wait_for: u64 = 5;
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();
    wait::wait(
        &mut sleeper,
        &new_config("", "", 1, wait_for, 0, 1, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) >= wait_for)
}

#[test]
fn should_wait_for_10_seconds_after() {
    let wait_for = 10;
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();
    wait::wait(
        &mut sleeper,
        &new_config("", "", 1, 0, wait_for, 1, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) >= wait_for)
}

#[test]
fn should_wait_before_and_after() {
    let wait_for = 10;
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();
    wait::wait(
        &mut sleeper,
        &new_config("", "", 1, wait_for, wait_for, 1, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) >= (wait_for + wait_for))
}

#[test]
fn should_execute_without_wait() {
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();
    wait::wait(
        &mut sleeper,
        &new_config("", "", 1, 0, 0, 1, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) <= 5)
}

#[test]
fn should_sleep_the_specified_time_between_host_checks() {
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();
    wait::wait(
        &mut sleeper,
        &new_config("198.19.255.255:1", "", 2_000, 0, 0, 10, 1),
        &mut on_timeout,
    );
    let elapsed = millis_elapsed(start);
    assert!(elapsed >= 2010);
    assert!(elapsed < 3000);
}

#[test]
fn should_sleep_the_specified_time_between_path_checks() {
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();
    wait::wait(
        &mut sleeper,
        &new_config(
            "",
            "./target/dsfasdfreworthkjiewuryiwghfsikahfsjfskjf",
            2_000,
            0,
            0,
            11,
            1,
        ),
        &mut on_timeout,
    );
    let elapsed = millis_elapsed(start);
    assert!(elapsed >= 2000);
    assert!(elapsed < 3000);
}

#[test]
fn should_exit_on_host_timeout() {
    let timeout = 25;
    let wait_before = 30;
    let wait_after = 300;
    let hosts = format!("localhost:{}", free_port());
    let paths = "";
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    wait::wait(
        &mut sleeper,
        &new_config(&hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    // assert that the on_timeout callback was called
    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= timeout + wait_before);
    assert!(millis_elapsed(start) < timeout + wait_after);
}

#[test]
fn should_exit_on_path_timeout() {
    let timeout = 25;
    let wait_before = 30;
    let wait_after = 300;
    let hosts = "";
    let paths = "./target/fsafasdfasfasfasfasfw54s664";
    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    wait::wait(
        &mut sleeper,
        &new_config(hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    // assert that the on_timeout callback was called
    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= timeout + wait_before);
    assert!(millis_elapsed(start) < timeout + wait_after);
}

#[test]
fn should_identify_the_open_port() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let tcp_listener = new_tcp_listener();
    let hosts = tcp_listener.local_addr().unwrap().to_string();
    let paths = "";

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &mut sleeper,
        &new_config(&hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(0, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

#[test]
fn should_wait_for_multiple_hosts() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let tcp_listener1 = new_tcp_listener();
    let tcp_listener2 = new_tcp_listener();
    let hosts = tcp_listener1.local_addr().unwrap().to_string()
        + ","
        + &tcp_listener2.local_addr().unwrap().to_string();

    let paths = "";

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener1);
    listen_async(tcp_listener2);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &mut sleeper,
        &new_config(&hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(0, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

#[test]
fn should_wait_for_multiple_paths() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let hosts = "";

    let path_1 = format!("./target/{}", rand::random::<u64>());
    let path_2 = format!("./target/{}", rand::random::<u64>());
    let paths = path_1.clone() + "," + path_2.as_str();

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(100));
        create_dir_all(&path_1).unwrap();
        println!("Directory created: [{}]", &path_1);
        thread::sleep(time::Duration::from_millis(10));
        File::create(&path_2).unwrap();
        println!("File created: [{}]", &path_2);
    });

    wait::wait(
        &mut sleeper,
        &new_config(hosts, &paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(0, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

#[test]
fn should_wait_for_multiple_hosts_and_paths() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let tcp_listener1 = new_tcp_listener();
    let tcp_listener2 = new_tcp_listener();
    let hosts = tcp_listener1.local_addr().unwrap().to_string()
        + ","
        + &tcp_listener2.local_addr().unwrap().to_string();

    let path_1 = format!("./target/{}", rand::random::<u64>());
    let path_2 = format!("./target/{}", rand::random::<u64>());
    let paths = path_1.clone() + "," + path_2.as_str();

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener1);
    listen_async(tcp_listener2);

    thread::sleep(time::Duration::from_millis(250));

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(100));
        create_dir_all(&path_1).unwrap();
        println!("Directory created: [{}]", &path_1);
        thread::sleep(time::Duration::from_millis(10));
        File::create(&path_2).unwrap();
        println!("File created: [{}]", &path_2);
    });

    wait::wait(
        &mut sleeper,
        &new_config(&hosts, &paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(0, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

#[test]
fn should_fail_if_not_all_hosts_are_available() {
    let timeout = 100;
    let wait_before = 30;
    let wait_after = 30;

    let tcp_listener1 = new_tcp_listener();
    let hosts =
        tcp_listener1.local_addr().unwrap().to_string() + ",127.0.0.1:" + &free_port().to_string();
    let paths = "";

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener1);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &mut sleeper,
        &new_config(&hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) >= timeout + wait_before + wait_after);
}

#[test]
fn should_fail_if_not_all_paths_are_available() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let hosts = "";

    let path_1 = format!("./target/{}", rand::random::<u64>());
    let path_2 = format!("./target/{}", rand::random::<u64>());
    let paths = path_1.clone() + "," + path_2.as_str();

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(100));
        create_dir_all(&path_1).unwrap();
        println!("Directory created: [{}]", &path_1);
    });

    wait::wait(
        &mut sleeper,
        &new_config(hosts, &paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

#[test]
fn should_fail_if_hosts_are_available_but_paths_are_not() {
    let timeout = 100;
    let wait_before = 30;
    let wait_after = 30;

    let tcp_listener1 = new_tcp_listener();
    let hosts = tcp_listener1.local_addr().unwrap().to_string();
    let paths = "./target/sfasfsfsgwe56345ybrtwet235vhffh4254";

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener1);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &mut sleeper,
        &new_config(&hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) >= timeout + wait_before + wait_after);
}

#[test]
fn should_fail_if_paths_are_available_but_hosts_are_not() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let hosts = "127.0.0.1:".to_owned() + &free_port().to_string();
    let paths = "./target";

    let start = Instant::now();
    let mut sleeper = MillisSleeper::default();

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    wait::wait(
        &mut sleeper,
        &new_config(&hosts, paths, timeout, wait_before, wait_after, 1, 1),
        &mut fun,
    );

    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

fn on_timeout() {}

fn new_config(
    hosts: &str,
    paths: &str,
    timeout: u64,
    before: u64,
    after: u64,
    sleep: u64,
    tcp_connection_timeout: u64,
) -> wait::Config {
    wait::Config {
        hosts: hosts.to_string(),
        paths: paths.to_string(),
        command: None,
        global_timeout: timeout,
        tcp_connection_timeout,
        wait_before: before,
        wait_after: after,
        wait_sleep_interval: sleep,
    }
}

fn new_tcp_listener() -> TcpListener {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    TcpListener::bind(socket).unwrap()
}

fn listen_async(listener: TcpListener) {
    thread::spawn(move || {
        loop {
            match listener.accept() {
                Ok(_) => {
                    println!("Connection received!");
                }
                Err(_) => {
                    println!("Error in received connection!");
                }
            }
        }
    });
}

fn free_port() -> u16 {
    new_tcp_listener().local_addr().unwrap().port()
}

fn millis_elapsed(start: Instant) -> u64 {
    let elapsed = start.elapsed().as_millis();
    println!("Millis elapsed {}", elapsed);
    elapsed as u64
}
