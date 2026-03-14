FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# RUN cargo build --release
COPY ./src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runner
WORKDIR /app
COPY --from=builder /app/target/release/server ./server
EXPOSE 3000

ENTRYPOINT [ "./server" ]
