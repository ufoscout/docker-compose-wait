extern crate wait;

fn main() {
    let sleep = wait::sleeper::new();
    wait::wait(&sleep, &wait::config_from_env(), &mut on_timeout);
}

fn on_timeout() {
    std::process::exit(1);
}