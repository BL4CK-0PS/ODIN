FROM rust:1.81-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY Cargo.toml ./
COPY crates crates/
COPY apps/api apps/api/
RUN cargo generate-lockfile && cargo build --release -p odin-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/odin-api /usr/local/bin/odin-api
EXPOSE 3001
ENV ODIN_PORT=3001
HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD curl -f http://localhost:3001/api/v1/system/health || exit 1
CMD ["odin-api"]