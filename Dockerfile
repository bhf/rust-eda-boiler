# Build stage
FROM rust:1.83 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/RustEDABoiler /app/app

ENV RUST_LOG=info
CMD ["./app"]
