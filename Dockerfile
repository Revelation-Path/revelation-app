# Build stage
FROM rust:1.85 AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binaries
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy migrations
COPY migrations ./migrations

# Server image
FROM runtime AS server
COPY --from=builder /app/target/release/revelation-server /usr/local/bin/
CMD ["revelation-server"]

# Gateway image
FROM runtime AS gateway
COPY --from=builder /app/target/release/revelation-gateway /usr/local/bin/
CMD ["revelation-gateway"]

# Payments image
FROM runtime AS payments
COPY --from=builder /app/target/release/revelation-payments /usr/local/bin/
CMD ["revelation-payments"]

# Bot image
FROM runtime AS bot
COPY --from=builder /app/target/release/revelation-bot /usr/local/bin/
CMD ["revelation-bot"]
