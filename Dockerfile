# syntax=docker/dockerfile:1.9

# ─────────────────────────────────────────────────────────────────────────────
# Chef stage - install cargo-chef and sccache
# ─────────────────────────────────────────────────────────────────────────────
FROM rust:1.92-slim AS chef

ENV SCCACHE_VERSION=0.8.2
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV SCCACHE_DIR=/sccache
ENV RUSTC_WRAPPER=/usr/local/bin/sccache

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef --locked \
    && curl -fsSL "https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz" \
    | tar -xz -C /usr/local/bin --strip-components=1 --wildcards '*/sccache'

WORKDIR /app

# ─────────────────────────────────────────────────────────────────────────────
# Planner stage - create recipe.json (dependency lockfile for chef)
# ─────────────────────────────────────────────────────────────────────────────
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo chef prepare --recipe-path recipe.json

# ─────────────────────────────────────────────────────────────────────────────
# Builder stage - build with cached dependencies
# ─────────────────────────────────────────────────────────────────────────────
FROM chef AS builder

# Copy recipe and cook dependencies (cached if unchanged)
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/sccache,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json \
    && sccache --show-stats

# Build application
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/sccache,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release \
    && sccache --show-stats \
    && mkdir -p /out \
    && cp target/release/revelation-server /out/ \
    && cp target/release/revelation-gateway /out/ \
    && cp target/release/revelation-payments /out/ \
    && cp target/release/revelation-bot /out/

# ─────────────────────────────────────────────────────────────────────────────
# Runtime stage - minimal image
# ─────────────────────────────────────────────────────────────────────────────
FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app

# ─────────────────────────────────────────────────────────────────────────────
# Service images
# ─────────────────────────────────────────────────────────────────────────────
FROM runtime AS server
COPY --from=builder /out/revelation-server /usr/local/bin/
COPY migrations ./migrations
EXPOSE 3000
CMD ["revelation-server"]

FROM runtime AS gateway
COPY --from=builder /out/revelation-gateway /usr/local/bin/
EXPOSE 8080
CMD ["revelation-gateway"]

FROM runtime AS payments
COPY --from=builder /out/revelation-payments /usr/local/bin/
EXPOSE 3001
CMD ["revelation-payments"]

FROM runtime AS bot
COPY --from=builder /out/revelation-bot /usr/local/bin/
CMD ["revelation-bot"]
