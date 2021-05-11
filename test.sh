#!/bin/bash

export WAIT_HOSTS=localhost:4748
#export WAIT_PATHS=./target/one
export WAIT_TIMEOUT=10
export WAIT_BEFORE=1
export WAIT_AFTER=2

./target/x86_64-unknown-linux-musl/release/wait && echo 'DOOOOOOONEEEEEE'
