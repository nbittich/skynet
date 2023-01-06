FROM rust:1.66 as builder

WORKDIR /build
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM ubuntu

WORKDIR /app

COPY --from=builder /build/target/release/skynet skynet
ENTRYPOINT [ "./skynet"]
