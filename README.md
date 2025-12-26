# Rusty-OMS

A basic OMS in Rust. Uses a DDD based structure with submodules.

### Features

* SPSC Ringbuffer usage
* Repository pattern
* Logging
* Unit tests
* Integration tests

## How To Run

Run using:

```
RUST_LOG=info cargo run
```

Or:

```
cargo build --release
cd target/release/
export RUST_LOG=info
./oms
```

### Sample Output

```bash
[2025-12-26T08:51:36Z INFO  oms] Starting up OMS
[2025-12-26T08:51:36Z INFO  oms] Initialised ringbuffer, will now publish messages
[2025-12-26T08:51:36Z INFO  oms] Total events published: 1
[2025-12-26T08:51:36Z INFO  oms] Received event with sequence: 0, Id: 1
[2025-12-26T08:51:36Z INFO  oms] Processed order: 1
[2025-12-26T08:51:37Z INFO  oms] Received event with sequence: 1, Id: 2
[2025-12-26T08:51:37Z INFO  oms] Processed order: 2
[2025-12-26T08:51:37Z INFO  oms] Total events published: 2
[2025-12-26T08:51:38Z INFO  oms] Received event with sequence: 2, Id: 3
```

