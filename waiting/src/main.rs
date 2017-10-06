#![feature(conservative_impl_trait)]

extern crate waiting;

use waiting::sleeper::*;

fn main() {

    let sleep = waiting::sleeper::new();
    waiting::wait(&sleep, on_timeout);

}

fn on_timeout() {
    std::process::exit(1);
}