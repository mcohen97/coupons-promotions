FROM rust:1.39

ADD Cargo.toml /build/Cargo.toml
ADD Cargo.lock /build/Cargo.lock
ADD src /build/src

WORKDIR /build

RUN apt-get update && \
    apt-get install -y --no-install-recommends cmake musl-tools openssl && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --target x86_64-unknown-linux-musl --release --locked

FROM alpine:latest
WORKDIR /app
COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/evaluations /app/

EXPOSE 8080

CMD [ "/app/evaluations" ]