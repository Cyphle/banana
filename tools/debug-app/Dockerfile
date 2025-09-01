# ---- Build Stage ----
FROM rust:1.79 as builder
WORKDIR /app

# Create a dummy project to cache dependencies
RUN USER=root cargo new --bin debug-app
WORKDIR /app/debug-app

# Copy manifests and fetch dependencies
COPY Cargo.toml ./
RUN cargo build --release || true

# Now copy actual source
COPY src ./src
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:stable-slim
WORKDIR /app
RUN useradd -u 10001 -m appuser \
    && apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/debug-app/target/release/debug-app /usr/local/bin/debug-app
USER appuser
ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["/usr/local/bin/debug-app"]
