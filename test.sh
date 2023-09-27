#!/bin/bash

docker build -t wait:test -f test.Dockerfile . && docker run --rm wait:test

#export WAIT_HOSTS=localhost:4748
#export WAIT_PATHS=./target/one
export WAIT_TIMEOUT=10
export WAIT_BEFORE=1
export WAIT_AFTER=2

./target/x86_64-unknown-linux-musl/release/wait && echo 'DOOOOOOONEEEEEE'

export WAIT_COMMAND="echo 'DOOOOOOONEEEEEE WITH WAIT_COMMAND (i.e. does not requrie a shell)'"

./target/x86_64-unknown-linux-musl/release/wait
