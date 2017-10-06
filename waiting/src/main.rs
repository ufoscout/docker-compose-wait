#![feature(conservative_impl_trait)]

extern crate waiting;

use std::time::Instant;
use waiting::sleeper::*;

fn main() {

    let start = Instant::now();
    // do stuff

    let sleep = waiting::sleeper::new();
    sleep.sleep(2);

    let elapsed = start.elapsed();
    
    // debug format:
    println!("{:?}", elapsed);

}
