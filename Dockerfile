FROM rust:1.55-alpine3.13 AS build

COPY src/ /src/src/
COPY Cargo.lock/ /src/Cargo.lock
COPY Cargo.toml/ /src/Cargo.toml

WORKDIR /src/

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.13

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/wait /bin/wait

RUN chmod +x /bin/wait

ENTRYPOINT [ "/bin/wait" ]
