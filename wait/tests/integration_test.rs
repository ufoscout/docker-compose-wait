#![feature(conservative_impl_trait)]

extern crate waiting;
extern crate atomic_counter;

use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use waiting::sleeper::*;
use std::time::Duration;
use std::thread;
use atomic_counter::AtomicCounter;
use atomic_counter::RelaxedCounter;

#[test]
fn should_wait_5_seconds_before() {
    let wait_for : u64 = 5;
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, &new_config("", 1, wait_for, 0), on_timeout);
    assert!( millisElapsed(start) >= wait_for )
}


#[test]
fn should_wait_10_seconds_after() {
    let wait_for = 10;
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, &new_config("", 1, 0, wait_for ), on_timeout);
    assert!( millisElapsed(start) >= wait_for )
}

#[test]
fn should_wait_before_and_after() {
    let wait_for = 10;
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, &new_config("", 1, wait_for, wait_for ), on_timeout);
    assert!( millisElapsed(start) >= (wait_for + wait_for) )
}

#[test]
fn should_execute_without_waiting() {
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    waiting::wait(&sleeper, &new_config("", 1, 0, 0 ), on_timeout);
    assert!( millisElapsed(start) <= 5 )
}

/*
#[test]
fn should_exit_on_timeout() {
    let timeout = 12;
    let hosts = "localhost:8080";
    let start = Instant::now();
    let sleeper = MillisSleeper{};

    let count : atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    println!("Count is {}", count.get());
    count.inc();
    println!("Count is {}", count.get());

    let fun = || { count.inc()};
    waiting::wait(&sleeper, &new_config(hosts, timeout, 0, 0 ), callback(count));

//    check timeout should be called here

    assert!( start.elapsed().as_secs()  >= timeout )
}
*/

fn callback(a: atomic_counter::RelaxedCounter) -> Box< Fn() > {
    Box::new(move || a.inc())
}

fn on_timeout() {}

fn new_config(hosts: &str, timeout: u64, before: u64, after: u64) -> waiting::Config {
    waiting::Config {
        hosts: hosts.to_string(),
        timeout: timeout,
        wait_before: before,
        wait_after: after,
    }
}

fn newTcpListener() -> TcpListener {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    TcpListener::bind(socket).unwrap()
}

fn port(listener: TcpListener) -> u16 {
    listener.local_addr().unwrap().port()
}

fn millisElapsed(start: Instant) -> u64 {
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