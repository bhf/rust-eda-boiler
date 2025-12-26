# Build stage
FROM rust:1.76 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/oms /app/app

# Install any runtime dependencies if needed (e.g., libssl)
# RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info
CMD ["./app"]
