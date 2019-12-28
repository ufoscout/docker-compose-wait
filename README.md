# docker-compose-wait

[![Build Status](https://travis-ci.org/ufoscout/docker-compose-wait.svg?branch=master)](https://travis-ci.org/ufoscout/docker-compose-wait)
[![codecov](https://codecov.io/gh/ufoscout/docker-compose-wait/branch/master/graph/badge.svg)](https://codecov.io/gh/ufoscout/docker-compose-wait)

A small command line utility to wait for other docker images to be started while using docker-compose.
It permits to wait for a fixed amount of seconds and/or to wait until a TCP port is open on a target image.

# Usage
This utility should be used in the docker build process and launched before your application starts.

For example, your application "MySuperApp" uses MongoDB, Postgres and MySql (wow!) and you want to be sure that, when it starts, all other systems are available, then simply customize your dockerfile this way:

```dockerfile
## Use whatever base image
FROM alpine

## Add your application to the docker image
ADD MySuperApp.sh /MySuperApp.sh

## Add the wait script to the image
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.6.0/wait /wait
RUN chmod +x /wait

## Launch the wait tool and then your application
CMD /wait && /MySuperApp.sh
```

Done! the image is ready.

Now let's modify the docker-compose.yml file:

```yml
version: "3"

services:

  mongo:
    image: mongo:3.4
    hostname: mongo
    ports:
      - "27017:27017"

  postgres:
    image: "postgres:9.4"
    hostname: postgres
    ports:
      - "5432:5432"

  mysql:
    image: "mysql:5.7"
    hostname: mysql
    ports:
      - "3306:3306"

  mySuperApp:
    image: "mySuperApp:latest"
    hostname: mySuperApp
    environment:
      WAIT_HOSTS: postgres:5432, mysql:3306, mongo:27017
```

When docker-compose is started (or Kubernetes or docker stack or whatever), your application will be started only when all the pairs host:port in the WAIT_HOSTS variable are available.
The WAIT_HOSTS environment variable is not mandatory, if not declared, the script executes without waiting.

If you want to use the script directly in docker-compose.yml instead of the Dockerfile, please note that the `command:` configuration option is limited to a single command so you should wrap in a `sh` call. For example:

```
command: sh -c "/wait && /MySuperApp.sh"
```

This is discussed further [here](https://stackoverflow.com/questions/30063907/using-docker-compose-how-to-execute-multiple-commands) and [here](https://github.com/docker/compose/issues/2033).

# Additional configuration options
The behaviour of the wait utility can be configured with the following environment variables:
- *WAIT_HOSTS*: comma separated list of pairs host:port for which you want to wait.
- *WAIT_HOSTS_TIMEOUT*: max number of seconds to wait for all the hosts to be available before failure. The default is 30 seconds.
- *WAIT_HOST_CONNECT_TIMEOUT*: The timeout of a single TCP connection to a remote host before attempting a new connection. The default is 5 seconds.
- *WAIT_BEFORE_HOSTS*: number of seconds to wait (sleep) before start checking for the hosts availability
- *WAIT_AFTER_HOSTS*: number of seconds to wait (sleep) once all the hosts are available
- *WAIT_SLEEP_INTERVAL*: number of seconds to sleep between retries. The default is 1 second.

# Using on non-linux systems
The simplest way of getting the _wait_ executable is to download it from

https://github.com/ufoscout/docker-compose-wait/releases/download/{{VERSION}}/wait

This is a pre-built executable for Linux x64 systems which are the default ones in Docker. 
In addition, it is built with [MUSL](https://www.musl-libc.org/) for maximum portability.

If you need it for a different architecture, you should clone this repository and build it for your target.

As it has no external dependencies, an being written in the mighty [rust](https://www.rust-lang.org) 
programming language, the build process is just a simple `cargo build --release` 
(well... of course you need to install the rust compiler before...) 

For everything involving cross-compilation, you should take a look at [Cross](https://github.com/rust-embedded/cross). 
For example, to build for a **raspberry pi**, everything you have to do is:
1. Install the latest stable rust toolchain using rustup
2. Correctly configure Docker on your machine
3. Open a terminal and type:
   ```bash
   cargo install cross
   cross build --target=armv7-unknown-linux-musleabihf --release
   ```
4. use your new shiny executable on your raspberry device!


# Notes
This utility was explicitly written to be used with docker-compose; however, it can be used everywhere since it has no dependencies on docker.
