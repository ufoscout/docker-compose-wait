fn main() {
    let mut sleep = wait::sleeper::new();
    wait::wait(&mut sleep, &wait::config_from_env(), &mut on_timeout);
}

fn on_timeout() {
    std::process::exit(1);
}
