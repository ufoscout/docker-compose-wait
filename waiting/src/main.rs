#![feature(conservative_impl_trait)]

use std::time::Instant;
use sleeper::Sleeper;

mod sleeper;


fn main() {

    let start = Instant::now();
    // do stuff

    let sleep = sleeper::new();
    sleep.sleep(2);

    let elapsed = start.elapsed();
    
    // debug format:
    println!("{:?}", elapsed);

}
