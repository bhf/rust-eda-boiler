# Build stage
FROM rust:1.83 AS builder

WORKDIR /app
COPY . .
# Install build dependencies and build CMake from source for Rusteron
RUN apt-get update && \
    apt-get install -y wget build-essential libssl-dev && \
    CMAKE_VERSION=4.2.1 && \
    wget https://github.com/Kitware/CMake/releases/download/v${CMAKE_VERSION}/cmake-${CMAKE_VERSION}.tar.gz && \
    tar -xzf cmake-${CMAKE_VERSION}.tar.gz && \
    cd cmake-${CMAKE_VERSION} && \
    ./bootstrap && \
    make -j$(nproc) && \
    make install && \
    cd /app && \
    rm -rf cmake-${CMAKE_VERSION}* && \
    apt-get remove --purge -y wget build-essential libssl-dev && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/* && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/oms /app/app

ENV RUST_LOG=info
CMD ["./app"]
