use atomic_counter::AtomicCounter;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::time::Duration;
use std::time::Instant;
use std::{thread, time};
use wait::sleeper::*;

#[test]
fn should_wait_5_seconds_before() {
    let wait_for: u64 = 5;
    let start = Instant::now();
    let sleeper = MillisSleeper {};
    wait::wait(
        &sleeper,
        &new_config("", 1, wait_for, 0, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) >= wait_for)
}

#[test]
fn should_wait_10_seconds_after() {
    let wait_for = 10;
    let start = Instant::now();
    let sleeper = MillisSleeper {};
    wait::wait(
        &sleeper,
        &new_config("", 1, 0, wait_for, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) >= wait_for)
}

#[test]
fn should_wait_before_and_after() {
    let wait_for = 10;
    let start = Instant::now();
    let sleeper = MillisSleeper {};
    wait::wait(
        &sleeper,
        &new_config("", 1, wait_for, wait_for, 1),
        &mut on_timeout,
    );
    assert!(millis_elapsed(start) >= (wait_for + wait_for))
}

#[test]
fn should_execute_without_wait() {
    let start = Instant::now();
    let sleeper = MillisSleeper {};
    wait::wait(&sleeper, &new_config("", 1, 0, 0, 1), &mut on_timeout);
    assert!(millis_elapsed(start) <= 5)
}

#[test]
fn should_exit_on_timeout() {
    let timeout = 25;
    let wait_before = 30;
    let wait_after = 300;
    let hosts = "localhost:".to_string() + &free_port().to_string();
    let start = Instant::now();
    let sleeper = MillisSleeper {};

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    wait::wait(
        &sleeper,
        &new_config(&hosts, timeout, wait_before, wait_after, 1),
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
    let start = Instant::now();
    let sleeper = MillisSleeper {};

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &sleeper,
        &new_config(&hosts, timeout, wait_before, wait_after, 1),
        &mut fun,
    );

    assert_eq!(0, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) < timeout + wait_before + wait_after);
}

#[test]
fn should_wait_multiple_hosts() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let tcp_listener1 = new_tcp_listener();
    let tcp_listener2 = new_tcp_listener();
    let hosts = tcp_listener1.local_addr().unwrap().to_string()
        + ","
        + &tcp_listener2.local_addr().unwrap().to_string();
    let start = Instant::now();
    let sleeper = MillisSleeper {};

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener1);
    listen_async(tcp_listener2);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &sleeper,
        &new_config(&hosts, timeout, wait_before, wait_after, 1),
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
    let start = Instant::now();
    let sleeper = MillisSleeper {};

    let count: atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || {
        count.inc();
    };
    assert_eq!(0, count.get());

    listen_async(tcp_listener1);

    thread::sleep(time::Duration::from_millis(250));
    wait::wait(
        &sleeper,
        &new_config(&hosts, timeout, wait_before, wait_after, 1),
        &mut fun,
    );

    assert_eq!(1, count.get());

    assert!(millis_elapsed(start) >= wait_before + wait_after);
    assert!(millis_elapsed(start) >= timeout + wait_before + wait_after);
}

fn on_timeout() {}

fn new_config(hosts: &str, timeout: u64, before: u64, after: u64, sleep: u64) -> wait::Config {
    wait::Config {
        hosts: hosts.to_string(),
        timeout: timeout,
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
    thread::spawn(move || loop {
        match listener.accept() {
            Ok(_) => {
                println!("Connection received!");
            }
            Err(_) => {
                println!("Error in received connection!");
            }
        }
    });
}

fn free_port() -> u16 {
    new_tcp_listener().local_addr().unwrap().port()
}

fn millis_elapsed(start: Instant) -> u64 {
    let elapsed = start.elapsed().subsec_nanos() / 1000000;
    println!("Millis elapsed {}", elapsed);
    elapsed as u64
}

struct MillisSleeper {}

impl Sleeper for MillisSleeper {
    fn sleep(&self, duration: u64) {
        thread::sleep(Duration::from_millis(duration))
    }
}
