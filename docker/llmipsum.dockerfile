FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y \
    git \
    libssl-dev \
    pkg-config

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=secret,id=api-key \
    API_KEY="$(cat /run/secrets/api-key)" cargo build --release --bin llmipsum

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && \
    apt-get clean

COPY --from=builder /app/data/emojis /app/data/emojis
COPY --from=builder /app/target/release/llmipsum /usr/local/bin
