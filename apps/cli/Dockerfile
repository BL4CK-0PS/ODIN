FROM rust:1.81-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY crates crates/
COPY apps/cli apps/cli/
RUN cargo build --release -p odin-cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/odin-cli /usr/local/bin/odin-cli
ENTRYPOINT ["odin-cli"]
