#![feature(conservative_impl_trait)]

extern crate wait;

use wait::sleeper::*;

fn main() {
    let sleep = wait::sleeper::new();
    wait::wait(&sleep, &wait::config_from_env(), on_timeout);
}

fn on_timeout() {
    std::process::exit(1);
}