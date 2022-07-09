FROM alpine:3.16
WORKDIR /tmp
RUN apk add build-base
RUN printf 'int main(){ exit(getpid()-1); }' > /tmp/is_pid_1.c \
    && gcc -w -static -o /is_pid_1 /tmp/is_pid_1.c

FROM scratch
COPY --from=0 /is_pid_1 /is_pid_1
COPY ./target/x86_64-unknown-linux-musl/release/wait /wait
ENV WAIT_LOGGER_LEVEL=off
ENV WAIT_COMMAND=/is_pid_1
ENTRYPOINT ["/wait"]