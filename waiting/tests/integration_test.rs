extern crate waiting;

use std::time::Instant;
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use waiting::sleeper::*;
use std::time::Duration;
use std::thread;
use std::env;

#[test]
fn should_wait_5_seconds_before() {
    let wait_for = 5;
    set_env("", "", &*wait_for.to_string(), "");
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, on_timeout);
    assert!( millisElapsed(start) >= wait_for )
}

#[test]
fn should_wait_10_seconds_after() {
    let wait_for = 10;
    set_env("", "", "10o", &*wait_for.to_string());
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, on_timeout);
    assert!( millisElapsed(start) >= wait_for )
}

#[test]
fn should_wait_before_and_after() {
    let wait_for = 10;
    set_env("", "", &*wait_for.to_string(), &*wait_for.to_string());
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, on_timeout);
    assert!( millisElapsed(start) >= (wait_for + wait_for) )
}

#[test]
fn should_execute_without_waiting() {
    set_env("", "", "er", "");
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, on_timeout);
    assert!( millisElapsed(start) <= 10 )
}

#[test]
fn should_exit_on_timeout() {
    let timeout = 1;
    set_env("localhost:8080", &*timeout.to_string(), "", "");
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, on_timeout);

    check timeout should be called here

    assert!( start.elapsed().as_secs()  >= timeout )
}

fn on_timeout() {}

fn newTcpListener() -> TcpListener {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    TcpListener::bind(socket).unwrap()
}

fn port(listener: TcpListener) -> u16 {
    listener.local_addr().unwrap().port()
}

fn millisElapsed(start: Instant) -> u32 {
    start.elapsed().subsec_nanos() / 1000000
}

fn set_env(hosts: &str, timeout: &str, before: &str, after: &str) {
    env::set_var("WAIT_BEFORE_HOSTS", before.to_string());
    env::set_var("WAIT_AFTER_HOSTS", after.to_string());
    env::set_var("WAIT_HOSTS_TIMEOUT", timeout.to_string());
    env::set_var("WAIT_HOSTS", hosts.to_string());
}

struct MillisSleeper {}

impl Sleeper for MillisSleeper {
    fn sleep(&self, duration: u64) {
        thread::sleep(Duration::from_millis(duration))
    }
}