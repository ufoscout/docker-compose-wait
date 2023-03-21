FROM alpine:3.16
WORKDIR /tmp
RUN apk add build-base
RUN printf '#include <stdio.h>' > /tmp/hello.c \
    && printf 'int main() { printf("Hello World"); return 0; }' > /tmp/hello.c \
    && gcc -w -static -o /hello /tmp/hello.c

FROM scratch
COPY --from=0 /hello /hello
COPY ./target/x86_64-unknown-linux-musl/release/wait /wait
ENV WAIT_LOGGER_LEVEL=off
ENV WAIT_COMMAND=/hello
ENTRYPOINT ["/wait"]
