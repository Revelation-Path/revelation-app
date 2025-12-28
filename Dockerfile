# syntax=docker/dockerfile:1.9

# ─────────────────────────────────────────────────────────────────────────────
# Web builder stage - build WASM frontend with trunk
# ─────────────────────────────────────────────────────────────────────────────
FROM rust:1.92-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl pkg-config libssl-dev ca-certificates nodejs npm \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk --locked

WORKDIR /app
COPY Cargo.toml Cargo.lock index.html build.rs manifest.json sw.js ./
COPY Trunk.docker.toml ./Trunk.toml
COPY src ./src
COPY assets ./assets
COPY icons ./icons

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    trunk build --release

# ─────────────────────────────────────────────────────────────────────────────
# Runtime - nginx serving static files
# ─────────────────────────────────────────────────────────────────────────────
FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY <<EOF /etc/nginx/conf.d/default.conf
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/wasm;
    gzip_min_length 1000;

    location / {
        try_files \$uri \$uri/ /index.html;
    }

    location ~* \.(js|css|wasm|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
EOF

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
