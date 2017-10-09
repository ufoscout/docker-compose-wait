#![feature(conservative_impl_trait)]

extern crate wait;
extern crate atomic_counter;

use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use wait::sleeper::*;
use std::time::Duration;
use std::{thread, time};
use atomic_counter::AtomicCounter;
use atomic_counter::RelaxedCounter;

#[test]
fn should_wait_5_seconds_before() {
    let wait_for : u64 = 5;
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    wait::wait(&sleeper, &new_config("", 1, wait_for, 0), &mut on_timeout);
    assert!( millisElapsed(start) >= wait_for )
}


#[test]
fn should_wait_10_seconds_after() {
    let wait_for = 10;
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    wait::wait(&sleeper, &new_config("", 1, 0, wait_for ), &mut on_timeout);
    assert!( millisElapsed(start) >= wait_for )
}

#[test]
fn should_wait_before_and_after() {
    let wait_for = 10;
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    wait::wait(&sleeper, &new_config("", 1, wait_for, wait_for ), &mut on_timeout);
    assert!( millisElapsed(start) >= (wait_for + wait_for) )
}

#[test]
fn should_execute_without_wait() {
    let start = Instant::now();
    let sleeper = MillisSleeper{};
    wait::wait(&sleeper, &new_config("", 1, 0, 0 ), &mut on_timeout);
    assert!( millisElapsed(start) <= 5 )
}

#[test]
fn should_exit_on_timeout() {
    let timeout = 25;
    let wait_before = 30;
    let wait_after = 300;
    let hosts = "localhost:".to_string() + &free_port().to_string();
    let start = Instant::now();
    let sleeper = MillisSleeper{};

    let mut count : atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || { count.inc(); };
    assert_eq!(0, count.get());
    
    wait::wait(&sleeper, &new_config(&hosts, timeout, wait_before, wait_after ), &mut  fun);
    
    // assert that the on_timeout callback was called
    assert_eq!(1, count.get());

    assert!( millisElapsed(start)  >= timeout + wait_before );
    assert!( millisElapsed(start)  < timeout + wait_after);
}

#[test]
fn should_identify_the_open_port() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let tcpListener = newTcpListener();
    let hosts = tcpListener.local_addr().unwrap().to_string();
    let start = Instant::now();
    let sleeper = MillisSleeper{};

    let mut count : atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || { count.inc(); };
    assert_eq!(0, count.get());

        thread::spawn(move || {
                loop {
                    match tcpListener.accept() {
                        Ok(_) => {  println!("Connection received!"); }
                        Err(_) => { println!("Error in received connection!"); }
                }
                }
        });

    thread::sleep(time::Duration::from_millis(250));    
    wait::wait(&sleeper, &new_config(&hosts, timeout, wait_before, wait_after ), &mut  fun);
    
    assert_eq!(0, count.get());

    assert!( millisElapsed(start)  >= wait_before + wait_after );
    assert!( millisElapsed(start)  < timeout + wait_before + wait_after);
}

#[test]
fn should_wait_multiple_hosts() {
    let timeout = 500;
    let wait_before = 30;
    let wait_after = 30;

    let tcpListener1 = newTcpListener();
    let tcpListener2 = newTcpListener();
    let hosts = tcpListener1.local_addr().unwrap().to_string() + "," + &tcpListener2.local_addr().unwrap().to_string();
    let start = Instant::now();
    let sleeper = MillisSleeper{};

    let mut count : atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || { count.inc(); };
    assert_eq!(0, count.get());

        thread::spawn(move || {
                loop {
                    match tcpListener1.accept() {
                        Ok(_) => {  println!("Connection received!"); }
                        Err(_) => { println!("Error in received connection!"); }
                    }
                }
        });

        thread::spawn(move || {
                loop {
                    match tcpListener2.accept() {
                        Ok(_) => {  println!("Connection received!"); }
                        Err(_) => { println!("Error in received connection!"); }
                    }
                }
        });

    thread::sleep(time::Duration::from_millis(250));    
    wait::wait(&sleeper, &new_config(&hosts, timeout, wait_before, wait_after ), &mut  fun);
    
    assert_eq!(0, count.get());

    assert!( millisElapsed(start)  >= wait_before + wait_after );
    assert!( millisElapsed(start)  < timeout + wait_before + wait_after);
}

#[test]
fn should_fail_if_not_all_hosts_are_available() {
    let timeout = 100;
    let wait_before = 30;
    let wait_after = 30;

    let tcpListener1 = newTcpListener();
    let hosts = tcpListener1.local_addr().unwrap().to_string() + ",127.0.0.1:" + &free_port().to_string();
    let start = Instant::now();
    let sleeper = MillisSleeper{};

    let mut count : atomic_counter::RelaxedCounter = atomic_counter::RelaxedCounter::new(0);
    let mut fun = || { count.inc(); };
    assert_eq!(0, count.get());

        thread::spawn(move || {
                loop {
                    match tcpListener1.accept() {
                        Ok(_) => {  println!("Connection received!"); }
                        Err(_) => { println!("Error in received connection!"); }
                    }
                }
        });

    thread::sleep(time::Duration::from_millis(250));    
    wait::wait(&sleeper, &new_config(&hosts, timeout, wait_before, wait_after ), &mut  fun);
    
    assert_eq!(1, count.get());

    assert!( millisElapsed(start)  >= wait_before + wait_after );
    assert!( millisElapsed(start)  >= timeout + wait_before + wait_after);
}

fn on_timeout() {}

fn new_config(hosts: &str, timeout: u64, before: u64, after: u64) -> wait::Config {
    wait::Config {
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

fn free_port() -> u16 {
    newTcpListener().local_addr().unwrap().port()
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