# docker-compose-wait

![Build Status](https://github.com/ufoscout/docker-compose-wait/actions/workflows/build_and_test.yml/badge.svg)
[![codecov](https://codecov.io/gh/ufoscout/docker-compose-wait/branch/master/graph/badge.svg)](https://codecov.io/gh/ufoscout/docker-compose-wait)

A small command-line utility to wait for other docker images to be started while using docker-compose (or Kubernetes or docker stack or whatever).

It permits waiting for:
- a fixed amount of seconds
- until a TCP port is open on a target image
- until a file or directory is present on the local filesystem

## Usage

This utility should be used in the docker build process and launched before your application starts.

For example, your application "MySuperApp" uses MongoDB, Postgres and MySql (wow!) and you want to be sure that, when it starts, all other systems are available, then simply customize your dockerfile this way:

```dockerfile
## Use whatever base image
FROM alpine

## Add the wait script to the image
## This is the default executable for the x86_64 architecture, other architectures are supported too
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.11.0/wait /wait
RUN chmod +x /wait

## Add your application to the docker image
ADD MySuperApp.sh /MySuperApp.sh

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

```bash
command: sh -c "/wait && /MySuperApp.sh"
```

This is discussed further [here](https://stackoverflow.com/questions/30063907/using-docker-compose-how-to-execute-multiple-commands) and [here](https://github.com/docker/compose/issues/2033).

## Usage in images that do not have a shell

When using [distroless](https://github.com/GoogleContainerTools/distroless) or building images [`FROM scratch`](https://docs.docker.com/develop/develop-images/baseimages/#create-a-simple-parent-image-using-scratch), it is common to not have `sh` available. In this case, it is necessary to [specify the command for wait to run explicitly](#additional-configuration-options). The invoked command will be invoked with any arguments configured for it and will completely replace the `wait` process in your container via a syscall to [`exec`](https://man7.org/linux/man-pages/man3/exec.3.html). Because there is no shell to expand arguments in this case, `wait` must be the `ENTRYPOINT` for the container and has to be specified in [the exec form](https://docs.docker.com/engine/reference/builder/#exec-form-entrypoint-example). Note that because there is no shell to perform expansion, arguments like `*` must be interpreted by the program that receives them.

```dockerfile
FROM golang
RUN wget -o /wait https://github.com/ufoscout/docker-compose-wait/releases/download/{{VERSION}}/wait
COPY myApp /app
WORKDIR /app
RUN go build -o /myApp -ldflags '-s -w -extldflags -static' ./...

FROM scratch
COPY --from=0 /wait /wait
COPY --from=0 /myApp /myApp
ENV WAIT_COMMAND="/myApp arg1 argN..."
ENTRYPOINT ["/wait"]
```

## Additional configuration options

The behaviour of the wait utility can be configured with the following environment variables:

- _WAIT_LOGGER_LEVEL_ : the output logger level. Valid values are: _debug_, _info_, _error_, _off_. the default is _debug_. 
- _WAIT_HOSTS_: comma-separated list of pairs host:port for which you want to wait.
- _WAIT_PATHS_: comma-separated list of paths (i.e. files or directories) on the local filesystem for which you want to wait until they exist.
- _WAIT_COMMAND_: command and arguments to run once waiting completes. The invoked command will completely replace the `wait` process. The default is none.
- _WAIT_TIMEOUT_: max number of seconds to wait for all the hosts/paths to be available before failure. The default is 30 seconds.
- _WAIT_HOST_CONNECT_TIMEOUT_: The timeout of a single TCP connection to a remote host before attempting a new connection. The default is 5 seconds.
- _WAIT_BEFORE_: number of seconds to wait (sleep) before start checking for the hosts/paths availability
- _WAIT_AFTER_: number of seconds to wait (sleep) once all the hosts/paths are available
- _WAIT_SLEEP_INTERVAL_: number of seconds to sleep between retries. The default is 1 second.

## Supported architectures

From release 2.11.0, the following executables are available for download:
- _wait_: This is the executable intended for Linux x64 systems
- *wait_x86_64*: This is the very same executable than _wait_
- *wait_aarch64*: This is the executable to be used for aarch64 architectures
- *wait_arm7*: This is the executable to be used for arm7 architectures

All executables are built with [MUSL](https://www.musl-libc.org/) for maximum portability.

To use any of these executables, simply replace the executable name in the download link:
https://github.com/ufoscout/docker-compose-wait/releases/download/{{VERSION}}/{{executable_name}}


## Using on non-linux systems

The simplest way of getting the _wait_ executable is to download it from

[https://github.com/ufoscout/docker-compose-wait/releases/download/{{VERSION}}/wait](https://github.com/ufoscout/docker-compose-wait/releases/download/{{VERSION}}/wait)

If you need it for an architecture for which a pre-built file is not available, you should clone this repository and build it for your target.

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

Use your shiny new executable on your raspberry device!

## Notes

This utility was explicitly written to be used with docker-compose; however, it can be used everywhere since it has no dependencies on docker.
