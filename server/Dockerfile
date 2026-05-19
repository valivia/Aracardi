# Builder
FROM rust:1.94.1-slim-bookworm AS builder
WORKDIR /app

# Install only essential build deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies separately from source
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Now copy real source and do a targeted rebuild
COPY ./src ./src
COPY ./migrations ./migrations
# TODO: fix this not actually rebuilding when updated
RUN touch src/main.rs && cargo build --release

# Runner
FROM debian:bookworm-slim AS runner
WORKDIR /app

# Install runtime deps only
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Don't run as root
RUN useradd -ms /bin/bash appuser
USER appuser

COPY --from=builder /app/target/release/server ./server

EXPOSE 3000
ENTRYPOINT ["./server"]
