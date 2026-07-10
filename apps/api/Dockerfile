FROM rust:1.81-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates crates/
COPY apps/api apps/api/
RUN cargo build --release -p odin-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/odin-api /usr/local/bin/odin-api
EXPOSE 3001
ENV ODIN_PORT=3001
CMD ["odin-api"]