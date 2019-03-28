FROM rust:1.32.0 AS builder
RUN ["mkdir", "/src"]
ADD src /src/src
ADD Cargo.lock /src/Cargo.lock
ADD Cargo.toml /src/Cargo.toml
WORKDIR /src
RUN ["cargo", "build"]

FROM debian:jessie

COPY --from=builder "/src/target/debug/ping-service" "/usr/local/bin/ping-service"

CMD [ "/usr/local/bin/ping-service" ]
