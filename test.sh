export WAIT_HOSTS=localhost:8025,localhost:8081,localhost:1433
export WAIT_HOSTS_TIMEOUT=10
export WAIT_BEFORE_HOSTS=1
export WAIT_AFTER_HOSTS=2

./target/x86_64-unknown-linux-musl/release/wait && echo 'DOOOOOOONEEEEEE'
